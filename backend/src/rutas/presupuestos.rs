//! Diagnóstico y presupuesto (spec-003; contrato, sección «spec-003 · Diagnóstico y
//! presupuesto»). Las ampliaciones de spec-005 usan estos mismos endpoints.

use std::str::FromStr;

use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use chrono::{DateTime, Days, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgConnection;

use crate::EstadoApp;
use crate::auth::{Usuario, nuevo_token};
use crate::config::Config;
use crate::dinero::{importe_linea, totales};
use crate::error::{ErrorApi, Json, Resultado};
use crate::estados::{Estado, cambiar_estado};
use crate::fechas;

pub fn rutas() -> Router<EstadoApp> {
    Router::new()
        .route("/ordenes/{id}/diagnostico", put(guardar_diagnostico))
        .route("/ordenes/{id}/presupuestos", post(crear_presupuesto))
        .route("/presupuestos/{id}", get(ver_presupuesto))
        .route("/presupuestos/{id}/lineas", post(anadir_linea))
        .route("/presupuestos/{id}/lineas/{linea_id}", delete(quitar_linea))
        .route("/presupuestos/{id}/emitir", post(emitir))
}

fn validacion(mensaje: impl Into<String>) -> ErrorApi {
    ErrorApi::Validacion {
        mensaje: mensaje.into(),
        detalle: None,
    }
}

/// `true` si el error es la violación de esa restricción `UNIQUE` (o índice único).
fn viola(error: &sqlx::Error, restriccion: &str) -> bool {
    matches!(error, sqlx::Error::Database(e) if e.constraint() == Some(restriccion))
}

const EMITIDO: &str = "El presupuesto ya está emitido; no se puede modificar";

// ---------------------------------------------------------------------------------------------
// Lo común (T005)
// ---------------------------------------------------------------------------------------------

/// Bloquea la orden hasta el `commit` y devuelve su estado, su mecánico y su número.
async fn orden_bloqueada(tx: &mut PgConnection, orden_id: i64) -> Resultado<(Estado, i64, i64)> {
    sqlx::query_as("SELECT estado, mecanico_id, numero FROM orden WHERE id = $1 FOR UPDATE")
        .bind(orden_id)
        .fetch_optional(tx)
        .await?
        .ok_or_else(|| ErrorApi::NoEncontrado("No existe la orden".to_string()))
}

/// Bloquea el presupuesto (y, con `con_orden`, también su orden) y devuelve su estado, su orden y
/// el mecánico de la orden.
async fn presupuesto_bloqueado(
    tx: &mut PgConnection,
    id: i64,
    con_orden: bool,
) -> Resultado<(String, i64, i64)> {
    let bloqueo = if con_orden {
        "FOR UPDATE OF p, o"
    } else {
        "FOR UPDATE OF p"
    };
    sqlx::query_as(&format!(
        "SELECT p.estado::text, o.id, o.mecanico_id
           FROM presupuesto p JOIN orden o ON o.id = p.orden_id
          WHERE p.id = $1 {bloqueo}"
    ))
    .bind(id)
    .fetch_optional(tx)
    .await?
    .ok_or_else(|| ErrorApi::NoEncontrado("No existe el presupuesto".to_string()))
}

/// El objeto `Linea` del contrato.
#[derive(Serialize, sqlx::FromRow)]
struct Linea {
    id: i64,
    tipo: String,
    concepto: String,
    pieza_id: Option<i64>,
    #[serde(with = "rust_decimal::serde::str")]
    cantidad: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    precio_unitario: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    importe: Decimal,
}

/// El objeto `Presupuesto` del contrato.
#[derive(Serialize)]
pub struct Presupuesto {
    id: i64,
    orden_id: i64,
    tipo: String,
    estado: String,
    caducado: bool,
    lineas: Vec<Linea>,
    #[serde(flatten)]
    totales: crate::dinero::Totales,
    emitido_en: Option<DateTime<Utc>>,
    caduca_el: Option<NaiveDate>,
    respondido_en: Option<DateTime<Utc>>,
    enlace: Option<String>,
}

#[derive(sqlx::FromRow)]
struct FilaPresupuesto {
    id: i64,
    orden_id: i64,
    tipo: String,
    estado: String,
    token: Option<String>,
    iva_porcentaje: Option<Decimal>,
    base: Option<Decimal>,
    iva: Option<Decimal>,
    total: Option<Decimal>,
    emitido_en: Option<DateTime<Utc>>,
    caduca_el: Option<NaiveDate>,
    respondido_en: Option<DateTime<Utc>>,
}

const SELECT_PRESUPUESTO: &str = "
    SELECT id, orden_id, tipo::text AS tipo, estado::text AS estado, token, iva_porcentaje,
           base, iva, total, emitido_en, caduca_el, respondido_en
      FROM presupuesto";

/// Un emitido pasada su fecha de caducidad se considera caducado; no es un estado guardado.
fn caducado(estado: &str, caduca_el: Option<NaiveDate>) -> bool {
    estado == "emitido" && caduca_el.is_some_and(|c| c < fechas::hoy())
}

/// En borrador los totales se calculan al momento; una vez emitido, son los guardados (R4).
fn totales_de(
    fila: &FilaPresupuesto,
    lineas: &[Linea],
    iva_config: Decimal,
) -> crate::dinero::Totales {
    match (fila.base, fila.iva_porcentaje, fila.iva, fila.total) {
        (Some(base), Some(iva_porcentaje), Some(iva), Some(total)) if fila.estado != "borrador" => {
            crate::dinero::Totales {
                base,
                iva_porcentaje,
                iva,
                total,
            }
        }
        _ => totales(lineas.iter().map(|l| l.importe), iva_config),
    }
}

async fn presupuesto_por_id(
    conexion: &mut PgConnection,
    id: i64,
    config: &Config,
) -> Resultado<Presupuesto> {
    let fila: FilaPresupuesto = sqlx::query_as(&format!("{SELECT_PRESUPUESTO} WHERE id = $1"))
        .bind(id)
        .fetch_optional(&mut *conexion)
        .await?
        .ok_or_else(|| ErrorApi::NoEncontrado("No existe el presupuesto".to_string()))?;
    let lineas: Vec<Linea> = sqlx::query_as(
        "SELECT id, tipo::text AS tipo, concepto, pieza_id, cantidad, precio_unitario, importe
           FROM linea_presupuesto WHERE presupuesto_id = $1 ORDER BY id",
    )
    .bind(id)
    .fetch_all(&mut *conexion)
    .await?;

    let totales = totales_de(&fila, &lineas, config.iva_porcentaje);
    Ok(Presupuesto {
        id: fila.id,
        orden_id: fila.orden_id,
        caducado: caducado(&fila.estado, fila.caduca_el),
        enlace: fila.token.as_ref().map(|t| {
            format!(
                "{}/presupuesto/{t}",
                config.url_publica.trim_end_matches('/')
            )
        }),
        tipo: fila.tipo,
        estado: fila.estado,
        lineas,
        totales,
        emitido_en: fila.emitido_en,
        caduca_el: fila.caduca_el,
        respondido_en: fila.respondido_en,
    })
}

/// El objeto `PresupuestoResumen` del contrato.
#[derive(Serialize)]
pub struct PresupuestoResumen {
    id: i64,
    tipo: String,
    estado: String,
    caducado: bool,
    #[serde(with = "rust_decimal::serde::str")]
    total: Decimal,
    emitido_en: Option<DateTime<Utc>>,
    caduca_el: Option<NaiveDate>,
}

/// Los presupuestos de una orden, del más antiguo al más reciente. Es el campo `presupuestos` de
/// `GET /api/ordenes/{id}`: spec-002 lo llama desde `ordenes.rs` con
/// `config.iva_porcentaje`, que se usa para el total de un borrador (research D8; T029).
pub async fn presupuestos_de_orden(
    conexion: &mut PgConnection,
    orden_id: i64,
    iva_porcentaje: Decimal,
) -> Resultado<Vec<PresupuestoResumen>> {
    let filas: Vec<FilaPresupuesto> = sqlx::query_as(&format!(
        "{SELECT_PRESUPUESTO} WHERE orden_id = $1 ORDER BY id"
    ))
    .bind(orden_id)
    .fetch_all(&mut *conexion)
    .await?;
    let mut lista = Vec::with_capacity(filas.len());
    for fila in filas {
        let importes: Vec<Decimal> =
            sqlx::query_scalar("SELECT importe FROM linea_presupuesto WHERE presupuesto_id = $1")
                .bind(fila.id)
                .fetch_all(&mut *conexion)
                .await?;
        let total = match (fila.total, fila.estado.as_str()) {
            (Some(total), estado) if estado != "borrador" => total,
            _ => totales(importes, iva_porcentaje).total,
        };
        lista.push(PresupuestoResumen {
            id: fila.id,
            caducado: caducado(&fila.estado, fila.caduca_el),
            tipo: fila.tipo,
            estado: fila.estado,
            total,
            emitido_en: fila.emitido_en,
            caduca_el: fila.caduca_el,
        });
    }
    Ok(lista)
}

// ---------------------------------------------------------------------------------------------
// Diagnóstico (US1; research D2)
// ---------------------------------------------------------------------------------------------

#[derive(Deserialize)]
struct NuevoDiagnostico {
    diagnostico: Option<String>,
}

#[derive(Serialize)]
struct OrdenDiagnostico {
    id: i64,
    numero: i64,
    estado: Estado,
    diagnostico: String,
}

async fn guardar_diagnostico(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
    Path(id): Path<i64>,
    Json(nuevo): Json<NuevoDiagnostico>,
) -> Resultado<Json<OrdenDiagnostico>> {
    let diagnostico = nuevo
        .diagnostico
        .as_deref()
        .unwrap_or("")
        .trim()
        .to_string();

    let mut tx = estado.db.begin().await?;
    let (estado_orden, mecanico_id, numero) = orden_bloqueada(&mut tx, id).await?;
    usuario.exigir_mecanico_de(mecanico_id)?;
    if diagnostico.is_empty() {
        return Err(validacion("Escribe el diagnóstico"));
    }

    let cambio = match estado_orden {
        Estado::Recibida => {
            Some(cambiar_estado(&mut tx, id, &[Estado::Recibida], Estado::EnDiagnostico).await?)
        }
        Estado::EnDiagnostico => {
            let emitido: bool = sqlx::query_scalar(
                "SELECT EXISTS (SELECT 1 FROM presupuesto WHERE orden_id = $1 AND estado <> 'borrador')",
            )
            .bind(id)
            .fetch_one(&mut *tx)
            .await?;
            if emitido {
                return Err(ErrorApi::Conflicto(
                    "Ya se ha emitido un presupuesto para esta orden; el diagnóstico no se puede \
                     cambiar"
                        .to_string(),
                ));
            }
            None
        }
        otro => {
            return Err(ErrorApi::Conflicto(format!(
                "La orden está {}; el diagnóstico solo se registra en órdenes recibidas o en \
                 diagnóstico",
                otro.texto()
            )));
        }
    };
    sqlx::query("UPDATE orden SET diagnostico = $2 WHERE id = $1")
        .bind(id)
        .bind(&diagnostico)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    // Después del commit, para que el tablero nunca lea un estado sin confirmar (R11, R14).
    if let Some(cambio) = cambio {
        cambio.publicar(&estado.eventos);
    }
    Ok(Json(OrdenDiagnostico {
        id,
        numero,
        estado: Estado::EnDiagnostico,
        diagnostico,
    }))
}

// ---------------------------------------------------------------------------------------------
// Presupuesto y líneas (US2; research D1, D3, D4, D5)
// ---------------------------------------------------------------------------------------------

async fn crear_presupuesto(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
    Path(orden_id): Path<i64>,
) -> Resultado<(StatusCode, Json<Presupuesto>)> {
    let mut tx = estado.db.begin().await?;
    let (estado_orden, mecanico_id, _) = orden_bloqueada(&mut tx, orden_id).await?;
    usuario.exigir_mecanico_de(mecanico_id)?;

    let tipo = match estado_orden {
        Estado::EnDiagnostico => "inicial",
        Estado::PendienteConformidad => "ampliacion",
        otro => {
            return Err(ErrorApi::Conflicto(format!(
                "La orden está {}; no se puede preparar un presupuesto",
                otro.texto()
            )));
        }
    };
    let vigente: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM presupuesto
                         WHERE orden_id = $1 AND estado = 'emitido' AND caduca_el >= $2)",
    )
    .bind(orden_id)
    .bind(fechas::hoy())
    .fetch_one(&mut *tx)
    .await?;
    if vigente {
        return Err(ErrorApi::Conflicto(
            "Ya hay un presupuesto emitido sin caducar para esta orden".to_string(),
        ));
    }

    let id: i64 = sqlx::query_scalar(
        "INSERT INTO presupuesto (orden_id, tipo) VALUES ($1, $2::tipo_presupuesto) RETURNING id",
    )
    .bind(orden_id)
    .bind(tipo)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        if viola(&e, "presupuesto_borrador_unico") {
            ErrorApi::Conflicto("Ya hay un presupuesto en borrador para esta orden".to_string())
        } else {
            e.into()
        }
    })?;
    let presupuesto = presupuesto_por_id(&mut tx, id, &estado.config).await?;
    tx.commit().await?;
    Ok((StatusCode::CREATED, Json(presupuesto)))
}

/// Cualquier rol con sesión (contrato).
async fn ver_presupuesto(
    State(estado): State<EstadoApp>,
    _usuario: Usuario,
    Path(id): Path<i64>,
) -> Resultado<Json<Presupuesto>> {
    let mut conexion = estado.db.acquire().await?;
    Ok(Json(
        presupuesto_por_id(&mut conexion, id, &estado.config).await?,
    ))
}

/// Los campos llegan como JSON sin tipar para responder 422 (y no 400) a un tipo equivocado.
#[derive(Deserialize)]
struct NuevaLinea {
    tipo: Option<String>,
    concepto: Option<String>,
    pieza_id: Option<Value>,
    cantidad: Option<Value>,
}

/// Cantidad > 0 con 2 decimales como máximo, como texto (`"1.5"`) o número.
fn cantidad(valor: &Option<Value>) -> Option<Decimal> {
    let decimal = match valor.as_ref()? {
        Value::String(texto) => Decimal::from_str(texto.trim()).ok()?,
        Value::Number(numero) => Decimal::from_str(&numero.to_string()).ok()?,
        _ => return None,
    };
    if decimal <= Decimal::ZERO || decimal.normalize().scale() > 2 {
        return None;
    }
    let mut decimal = decimal;
    decimal.rescale(2);
    Some(decimal)
}

async fn anadir_linea(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
    Path(id): Path<i64>,
    Json(nueva): Json<NuevaLinea>,
) -> Resultado<(StatusCode, Json<Presupuesto>)> {
    let mut tx = estado.db.begin().await?;
    let (estado_presupuesto, _, mecanico_id) = presupuesto_bloqueado(&mut tx, id, false).await?;
    usuario.exigir_mecanico_de(mecanico_id)?;
    if estado_presupuesto != "borrador" {
        return Err(ErrorApi::Conflicto(EMITIDO.to_string()));
    }

    let (tipo, concepto, pieza_id, cantidad, precio_unitario) = match nueva.tipo.as_deref() {
        Some("mano_obra") => {
            let horas = cantidad(&nueva.cantidad)
                .ok_or_else(|| validacion("Indica las horas: un número mayor que 0"))?;
            if (horas * Decimal::from(4)).fract() != Decimal::ZERO {
                return Err(validacion(
                    "Las horas van en múltiplos de 0,25 (p. ej. 1,5)",
                ));
            }
            let concepto = nueva.concepto.as_deref().unwrap_or("").trim().to_string();
            if concepto.is_empty() {
                return Err(validacion("Escribe el concepto de la mano de obra"));
            }
            (
                "mano_obra",
                concepto,
                None,
                horas,
                estado.config.precio_hora,
            )
        }
        Some("pieza") => {
            let unidades = cantidad(&nueva.cantidad)
                .ok_or_else(|| validacion("Indica las unidades: un número mayor que 0"))?;
            if unidades.fract() != Decimal::ZERO {
                return Err(validacion(
                    "Las unidades de pieza deben ser un número entero",
                ));
            }
            let pieza: Option<(i64, String, String, Decimal)> =
                match nueva.pieza_id.as_ref().and_then(Value::as_i64) {
                    Some(pieza_id) => sqlx::query_as(
                        "SELECT id, referencia, descripcion, precio_venta FROM pieza WHERE id = $1",
                    )
                    .bind(pieza_id)
                    .fetch_optional(&mut *tx)
                    .await?,
                    None => None,
                };
            let (pieza_id, referencia, descripcion, precio_venta) =
                pieza.ok_or_else(|| validacion("Elige una referencia de pieza dada de alta"))?;
            // El concepto de una pieza lo pone el servidor (research D3). No se mira el stock.
            (
                "pieza",
                format!("{referencia} {descripcion}"),
                Some(pieza_id),
                unidades,
                precio_venta,
            )
        }
        _ => return Err(validacion("El tipo de línea debe ser mano de obra o pieza")),
    };

    sqlx::query(
        "INSERT INTO linea_presupuesto (presupuesto_id, tipo, concepto, pieza_id, cantidad,
                                        precio_unitario, importe)
         VALUES ($1, $2::tipo_linea, $3, $4, $5, $6, $7)",
    )
    .bind(id)
    .bind(tipo)
    .bind(concepto)
    .bind(pieza_id)
    .bind(cantidad)
    .bind(precio_unitario)
    .bind(importe_linea(cantidad, precio_unitario))
    .execute(&mut *tx)
    .await?;
    let presupuesto = presupuesto_por_id(&mut tx, id, &estado.config).await?;
    tx.commit().await?;
    Ok((StatusCode::CREATED, Json(presupuesto)))
}

async fn quitar_linea(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
    Path((id, linea_id)): Path<(i64, i64)>,
) -> Resultado<Json<Presupuesto>> {
    let mut tx = estado.db.begin().await?;
    let (estado_presupuesto, _, mecanico_id) = presupuesto_bloqueado(&mut tx, id, false).await?;
    usuario.exigir_mecanico_de(mecanico_id)?;
    if estado_presupuesto != "borrador" {
        return Err(ErrorApi::Conflicto(EMITIDO.to_string()));
    }

    let borradas =
        sqlx::query("DELETE FROM linea_presupuesto WHERE id = $2 AND presupuesto_id = $1")
            .bind(id)
            .bind(linea_id)
            .execute(&mut *tx)
            .await?
            .rows_affected();
    if borradas == 0 {
        return Err(ErrorApi::NoEncontrado(
            "No existe esa línea en el presupuesto".to_string(),
        ));
    }
    let presupuesto = presupuesto_por_id(&mut tx, id, &estado.config).await?;
    tx.commit().await?;
    Ok(Json(presupuesto))
}

// ---------------------------------------------------------------------------------------------
// Emitir (US3; research D6)
// ---------------------------------------------------------------------------------------------

#[derive(Serialize)]
struct PresupuestoEmitido {
    #[serde(flatten)]
    presupuesto: Presupuesto,
    aviso_enviado: bool,
}

async fn emitir(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
    Path(id): Path<i64>,
) -> Resultado<Json<PresupuestoEmitido>> {
    let mut tx = estado.db.begin().await?;
    // También la orden: así no se cruza con un cambio de diagnóstico (FR-001a).
    let (estado_presupuesto, orden_id, mecanico_id) =
        presupuesto_bloqueado(&mut tx, id, true).await?;
    usuario.exigir_mecanico_de(mecanico_id)?;
    if estado_presupuesto != "borrador" {
        return Err(ErrorApi::Conflicto(
            "El presupuesto ya está emitido".to_string(),
        ));
    }

    let importes: Vec<Decimal> =
        sqlx::query_scalar("SELECT importe FROM linea_presupuesto WHERE presupuesto_id = $1")
            .bind(id)
            .fetch_all(&mut *tx)
            .await?;
    if importes.is_empty() {
        return Err(validacion("El presupuesto no tiene líneas"));
    }
    let t = totales(importes, estado.config.iva_porcentaje);
    let caduca_el = fechas::hoy() + Days::new(15);
    sqlx::query(
        "UPDATE presupuesto
            SET estado = 'emitido', base = $2, iva = $3, total = $4, iva_porcentaje = $5,
                emitido_en = now(), caduca_el = $6, token = $7
          WHERE id = $1",
    )
    .bind(id)
    .bind(t.base)
    .bind(t.iva)
    .bind(t.total)
    .bind(t.iva_porcentaje)
    .bind(caduca_el)
    .bind(nuevo_token())
    .execute(&mut *tx)
    .await?;

    let (cliente, email, matricula): (String, String, String) = sqlx::query_as(
        "SELECT c.nombre, c.email, v.matricula
           FROM orden o JOIN vehiculo v ON v.id = o.vehiculo_id JOIN cliente c ON c.id = v.cliente_id
          WHERE o.id = $1",
    )
    .bind(orden_id)
    .fetch_one(&mut *tx)
    .await?;
    let presupuesto = presupuesto_por_id(&mut tx, id, &estado.config).await?;
    tx.commit().await?;

    // Después del commit; si falla no se deshace nada (R10). Sin importes (spec-004 FR-001).
    let enlace = presupuesto.enlace.clone().unwrap_or_default();
    let cuerpo = format!(
        "Hola, {cliente}:\n\n\
         Ya tiene disponible el presupuesto de la reparación de su vehículo {matricula}. \
         Puede verlo y aceptarlo o rechazarlo en este enlace:\n\n{enlace}\n\n\
         El presupuesto es válido hasta el {}.\n\nUn saludo,\n{}",
        caduca_el.format("%d/%m/%Y"),
        estado.config.taller_nombre
    );
    let aviso_enviado = estado
        .correo
        .enviar(
            &email,
            &format!("Presupuesto de su vehículo {matricula}"),
            &cuerpo,
        )
        .await;

    Ok(Json(PresupuestoEmitido {
        presupuesto,
        aviso_enviado,
    }))
}
