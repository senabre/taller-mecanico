//! Órdenes de reparación (spec-002; contrato, sección «spec-002 · Orden de reparación»): abrir la
//! orden de una cita de hoy, la lista de órdenes y la ficha de una orden. La ficha añade las
//! secciones de spec-003, 005 y 006 llamando a una función de cada módulo (research D6).

use axum::Router;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::PgConnection;

use crate::EstadoApp;
use crate::auth::{Rol, Usuario};
use crate::contador;
use crate::error::{ErrorApi, Json, Resultado};
use crate::estados::{CambioEstado, Estado};
use crate::fechas;
use crate::lineas::Linea;
use crate::rutas::almacen::{self, Pedido};
use crate::rutas::imputaciones;
use crate::rutas::presupuestos::{self, PresupuestoResumen};

pub fn rutas() -> Router<EstadoApp> {
    Router::new()
        .route("/ordenes", get(listar_ordenes).post(abrir_orden))
        .route("/ordenes/{id}", get(ver_orden))
}

/// 422 con el campo afectado, para que el formulario muestre el mensaje bajo ese campo
/// (contrato común, tabla de validación de `POST /api/ordenes`).
fn error_campo(campo: &str, mensaje: &str) -> ErrorApi {
    ErrorApi::Validacion {
        mensaje: mensaje.to_string(),
        detalle: Some(json!({ "campo": campo })),
    }
}

// ---------------------------------------------------------------------------------------------
// OrdenResumen: listas y tablero (spec-008 la reutiliza; hallazgo I2)
// ---------------------------------------------------------------------------------------------

#[derive(Serialize)]
pub struct MecanicoDeOrden {
    pub id: i64,
    pub nombre: String,
}

/// El objeto `OrdenResumen` del contrato.
#[derive(Serialize)]
pub struct OrdenResumen {
    pub id: i64,
    pub numero: i64,
    pub matricula: String,
    pub estado: Estado,
    pub mecanico: MecanicoDeOrden,
    pub fecha_entrega_prometida: NaiveDate,
    /// Data-model común: no facturada y con la entrega anterior a hoy (Europe/Madrid).
    pub retrasada: bool,
}

#[derive(sqlx::FromRow)]
struct FilaResumen {
    id: i64,
    numero: i64,
    matricula: String,
    estado: Estado,
    mecanico_id: i64,
    mecanico_nombre: String,
    fecha_entrega_prometida: NaiveDate,
    retrasada: bool,
}

impl From<FilaResumen> for OrdenResumen {
    fn from(f: FilaResumen) -> Self {
        OrdenResumen {
            id: f.id,
            numero: f.numero,
            matricula: f.matricula,
            estado: f.estado,
            mecanico: MecanicoDeOrden {
                id: f.mecanico_id,
                nombre: f.mecanico_nombre,
            },
            fecha_entrega_prometida: f.fecha_entrega_prometida,
            retrasada: f.retrasada,
        }
    }
}

/// Columnas de `OrdenResumen`; `$1` es hoy en Europe/Madrid.
const COLUMNAS_RESUMEN: &str = "o.id, o.numero, v.matricula, o.estado,
       u.id AS mecanico_id, u.nombre AS mecanico_nombre, o.fecha_entrega_prometida,
       (o.estado <> 'facturada' AND o.fecha_entrega_prometida < $1) AS retrasada";
const DESDE: &str = "FROM orden o
  JOIN vehiculo v ON v.id = o.vehiculo_id
  JOIN usuario u ON u.id = o.mecanico_id";

/// Órdenes de la más reciente a la más antigua; con `mecanico_id`, solo las de ese mecánico.
pub async fn resumenes(
    conexion: &mut PgConnection,
    mecanico_id: Option<i64>,
) -> Resultado<Vec<OrdenResumen>> {
    let filas: Vec<FilaResumen> = sqlx::query_as(&format!(
        "SELECT {COLUMNAS_RESUMEN} {DESDE}
          WHERE $2::bigint IS NULL OR o.mecanico_id = $2
          ORDER BY o.abierta_en DESC, o.id DESC"
    ))
    .bind(fechas::hoy())
    .bind(mecanico_id)
    .fetch_all(conexion)
    .await?;
    Ok(filas.into_iter().map(OrdenResumen::from).collect())
}

// ---------------------------------------------------------------------------------------------
// Orden: la ficha (research D6)
// ---------------------------------------------------------------------------------------------

#[derive(Serialize)]
struct CitaDeOrden {
    fecha: NaiveDate,
    motivo: String,
}

#[derive(Serialize)]
struct VehiculoDeOrden {
    marca: String,
    modelo: String,
}

/// El objeto `Orden` del contrato: los campos de spec-002 y una sección por spec de la Persona B.
#[derive(Serialize)]
pub struct Orden {
    #[serde(flatten)]
    resumen: OrdenResumen,
    cita: CitaDeOrden,
    vehiculo: VehiculoDeOrden,
    km: i32,
    danos: String,
    diagnostico: Option<String>,
    #[serde(with = "rust_decimal::serde::str")]
    total_presupuestado: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    coste_acumulado: Decimal,
    presupuestos: Vec<PresupuestoResumen>,
    imputaciones: Vec<Linea>,
    pedidos: Vec<Pedido>,
    factura_id: Option<i64>,
}

#[derive(sqlx::FromRow)]
struct FilaOrden {
    #[sqlx(flatten)]
    resumen: FilaResumen,
    cita_fecha: NaiveDate,
    cita_motivo: String,
    marca: String,
    modelo: String,
    km: i32,
    danos: String,
    diagnostico: Option<String>,
    factura_id: Option<i64>,
}

/// La ficha completa. Cada sección de B es una llamada a su propio módulo; para cambiar lo que
/// muestra una sección se cambia su función, no esta.
pub async fn orden_completa(
    conexion: &mut PgConnection,
    id: i64,
    iva_porcentaje: Decimal,
) -> Resultado<Orden> {
    let fila: FilaOrden = sqlx::query_as(&format!(
        "SELECT {COLUMNAS_RESUMEN},
                c.fecha AS cita_fecha, c.motivo AS cita_motivo, v.marca, v.modelo,
                o.km, o.danos, o.diagnostico,
                (SELECT f.id FROM factura f WHERE f.orden_id = o.id) AS factura_id
           {DESDE}
           JOIN cita c ON c.id = o.cita_id
          WHERE o.id = $2"
    ))
    .bind(fechas::hoy())
    .bind(id)
    .fetch_optional(&mut *conexion)
    .await?
    .ok_or_else(|| ErrorApi::NoEncontrado("No existe la orden".to_string()))?;

    // spec-003
    let presupuestos = presupuestos::presupuestos_de_orden(conexion, id, iva_porcentaje).await?;
    // spec-005
    let resumen = imputaciones::resumen_imputaciones(conexion, id).await?;
    // spec-006
    let pedidos = almacen::pedidos_de_orden(conexion, id).await?;

    Ok(Orden {
        resumen: fila.resumen.into(),
        cita: CitaDeOrden {
            fecha: fila.cita_fecha,
            motivo: fila.cita_motivo,
        },
        vehiculo: VehiculoDeOrden {
            marca: fila.marca,
            modelo: fila.modelo,
        },
        km: fila.km,
        danos: fila.danos,
        diagnostico: fila.diagnostico,
        total_presupuestado: resumen.total_presupuestado,
        coste_acumulado: resumen.coste_acumulado,
        presupuestos,
        imputaciones: resumen.imputaciones,
        pedidos,
        factura_id: fila.factura_id,
    })
}

// ---------------------------------------------------------------------------------------------
// POST /api/ordenes (research D1–D5)
// ---------------------------------------------------------------------------------------------

/// Los campos llegan como valores JSON libres: un tipo incorrecto da el 422 de su fila de la
/// tabla de validación, no el 400 de «JSON mal formado» (contrato común; hallazgo A1).
#[derive(Deserialize)]
struct PeticionOrden {
    cita_id: Option<Value>,
    km: Option<Value>,
    danos: Option<Value>,
    mecanico_id: Option<Value>,
    fecha_entrega_prometida: Option<Value>,
}

/// Entero de 0 a `i32::MAX`, como número JSON o como texto con solo cifras.
fn leer_km(valor: Option<&Value>) -> Option<i32> {
    let km = match valor? {
        Value::Number(n) => n.as_i64(),
        Value::String(t) if !t.is_empty() && t.chars().all(|c| c.is_ascii_digit()) => {
            t.parse().ok()
        }
        _ => None,
    }?;
    i32::try_from(km).ok().filter(|km| *km >= 0)
}

async fn abrir_orden(
    usuario: Usuario,
    State(estado): State<EstadoApp>,
    Json(peticion): Json<PeticionOrden>,
) -> Resultado<Response> {
    usuario.exigir_rol(&[Rol::Recepcion])?;
    let hoy = fechas::hoy();

    // 1. La cita existe (404).
    let cita_id = peticion.cita_id.as_ref().and_then(Value::as_i64);
    let cita: Option<(NaiveDate, i64)> = match cita_id {
        Some(id) => {
            sqlx::query_as("SELECT fecha, vehiculo_id FROM cita WHERE id = $1")
                .bind(id)
                .fetch_optional(&estado.db)
                .await?
        }
        None => None,
    };
    let (Some(cita_id), Some((fecha_cita, vehiculo_id))) = (cita_id, cita) else {
        return Err(ErrorApi::NoEncontrado("No existe la cita".to_string()));
    };

    // 2. Sin transacción, los campos en el orden de la tabla de validación (422).
    let km = leer_km(peticion.km.as_ref()).ok_or_else(|| {
        error_campo("km", "El kilometraje debe ser un número entero igual o mayor que 0")
    })?;
    let danos = peticion
        .danos
        .as_ref()
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|danos| !danos.is_empty())
        .ok_or_else(|| {
            error_campo(
                "danos",
                "Describe los daños visibles o escribe “Sin daños visibles”",
            )
        })?;
    // La misma condición que GET /api/mecanicos (research D4).
    let mecanico_id: Option<i64> = match peticion.mecanico_id.as_ref().and_then(Value::as_i64) {
        Some(id) => {
            sqlx::query_scalar(
                "SELECT id FROM usuario WHERE id = $1 AND rol = 'mecanico' AND activo",
            )
            .bind(id)
            .fetch_optional(&estado.db)
            .await?
        }
        None => None,
    };
    let mecanico_id =
        mecanico_id.ok_or_else(|| error_campo("mecanico_id", "Elige un mecánico de la lista"))?;
    let entrega = peticion
        .fecha_entrega_prometida
        .as_ref()
        .and_then(Value::as_str)
        .and_then(|texto| texto.parse::<NaiveDate>().ok())
        .ok_or_else(|| {
            error_campo(
                "fecha_entrega_prometida",
                "Indica la fecha de entrega prometida",
            )
        })?;
    if entrega < hoy {
        return Err(error_campo(
            "fecha_entrega_prometida",
            "La fecha de entrega prometida no puede ser anterior a hoy",
        ));
    }
    // Hueco de T028 (US2): km frente a la visita anterior, última fila de la tabla.

    // 3. Solo el día de la cita (409, research D1).
    if fecha_cita != hoy {
        return Err(ErrorApi::Conflicto(
            "Solo se puede abrir la orden el día de la cita".to_string(),
        ));
    }

    // 4. Con la cita bloqueada hasta el commit, ¿ya tiene orden? (409, research D2).
    let mut tx = estado.db.begin().await?;
    sqlx::query("SELECT id FROM cita WHERE id = $1 FOR UPDATE")
        .bind(cita_id)
        .execute(&mut *tx)
        .await?;
    let existente: Option<(i64, i64)> =
        sqlx::query_as("SELECT id, numero FROM orden WHERE cita_id = $1")
            .bind(cita_id)
            .fetch_optional(&mut *tx)
            .await?;
    if let Some((orden_id, numero)) = existente {
        // `ErrorApi::Conflicto` no lleva `detalle`: misma forma `{error, detalle}` a mano.
        let cuerpo = json!({
            "error": format!("Esta cita ya tiene una orden (Nº {numero})"),
            "detalle": { "orden_id": orden_id, "numero": numero },
        });
        return Ok((StatusCode::CONFLICT, Json(cuerpo)).into_response());
    }

    // 5. Solo ahora el número (R5): ningún rechazo anterior lo ha gastado.
    let numero = contador::siguiente(&mut tx, contador::ORDEN).await?;
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO orden (numero, cita_id, vehiculo_id, mecanico_id, km, danos,
                            fecha_entrega_prometida)
         VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
    )
    .bind(numero)
    .bind(cita_id)
    .bind(vehiculo_id)
    .bind(mecanico_id)
    .bind(km)
    .bind(danos)
    .bind(entrega)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;

    // 6. Después del commit, el evento del tablero (research D5).
    CambioEstado {
        id,
        estado: Estado::Recibida,
    }
    .publicar(&estado.eventos);

    let mut conexion = estado.db.acquire().await?;
    let orden = orden_completa(&mut conexion, id, estado.config.iva_porcentaje).await?;
    Ok((StatusCode::CREATED, Json(orden)).into_response())
}

// ---------------------------------------------------------------------------------------------
// GET /api/ordenes y GET /api/ordenes/{id}
// ---------------------------------------------------------------------------------------------

#[derive(Deserialize)]
struct FiltroOrdenes {
    mias: Option<bool>,
}

/// La lista de «Órdenes» y «Mis órdenes» (menú común y quickstart §3; hallazgo E1).
async fn listar_ordenes(
    usuario: Usuario,
    State(estado): State<EstadoApp>,
    Query(filtro): Query<FiltroOrdenes>,
) -> Resultado<Json<Vec<OrdenResumen>>> {
    usuario.exigir_rol(&[Rol::Recepcion, Rol::Mecanico])?;
    let mecanico_id = filtro.mias.unwrap_or(false).then_some(usuario.id);
    let mut conexion = estado.db.acquire().await?;
    Ok(Json(resumenes(&mut conexion, mecanico_id).await?))
}

/// La ficha: todos los roles con sesión.
async fn ver_orden(
    _usuario: Usuario,
    State(estado): State<EstadoApp>,
    Path(id): Path<i64>,
) -> Resultado<Json<Orden>> {
    let mut conexion = estado.db.acquire().await?;
    Ok(Json(
        orden_completa(&mut conexion, id, estado.config.iva_porcentaje).await?,
    ))
}
