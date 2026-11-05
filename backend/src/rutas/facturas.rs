//! Facturar una orden terminada, avisar al cliente e historial por matrícula (spec-007; contrato,
//! sección «spec-007 · Factura e historial»).

use axum::Router;
use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Serialize;
use sqlx::PgConnection;

use crate::EstadoApp;
use crate::auth::{Rol, Usuario};
use crate::config::Config;
use crate::contador;
use crate::dinero::totales;
use crate::error::{ErrorApi, Json, Resultado};
use crate::estados::{Estado, cambiar_estado};
use crate::fechas;
use crate::formato;
use crate::matricula::normalizar_matricula;

pub fn rutas() -> Router<EstadoApp> {
    Router::new()
        .route("/ordenes/{id}/factura", post(facturar))
        .route("/facturas/{id}", get(ver_factura))
        .route("/vehiculos/{matricula}/historial", get(historial))
        .route("/taller/logo", get(logo))
}

// ---------------------------------------------------------------------------------------------
// Logo del taller (FR-009; research D8)
// ---------------------------------------------------------------------------------------------

const RUTA_LOGO: &str = "/api/taller/logo";

/// El archivo del logo, si está configurado y existe. Un archivo que falta cuenta como «sin logo».
fn archivo_logo(config: &Config) -> Option<&std::path::Path> {
    let ruta = config.taller_logo.as_deref()?;
    if ruta.is_file() {
        Some(ruta)
    } else {
        tracing::warn!("TALLER_LOGO apunta a «{}», que no existe", ruta.display());
        None
    }
}

/// Cualquier rol con sesión: es la imagen que sale en la cabecera de la factura.
async fn logo(State(estado): State<EstadoApp>, _usuario: Usuario) -> Resultado<impl IntoResponse> {
    let sin_logo = || ErrorApi::NoEncontrado("No hay logo configurado".to_string());
    let ruta = archivo_logo(&estado.config).ok_or_else(sin_logo)?;
    let tipo = match ruta
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("svg") => "image/svg+xml",
        Some("webp") => "image/webp",
        _ => "application/octet-stream",
    };
    let bytes = tokio::fs::read(ruta).await.map_err(|e| {
        tracing::warn!("No se pudo leer el logo «{}»: {e}", ruta.display());
        sin_logo()
    })?;
    Ok(([(header::CONTENT_TYPE, tipo)], bytes))
}

// ---------------------------------------------------------------------------------------------
// Objetos del contrato (T004)
// ---------------------------------------------------------------------------------------------

/// Una línea de factura: copia de una imputación, sin tipo (research D6).
#[derive(Serialize, sqlx::FromRow)]
pub struct LineaFactura {
    concepto: String,
    #[serde(with = "rust_decimal::serde::str")]
    cantidad: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    precio_unitario: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    importe: Decimal,
}

#[derive(Serialize)]
struct Taller {
    nombre: String,
    nif: String,
    direccion: String,
    /// `/api/taller/logo` si hay logo configurado; `null` si no (FR-009).
    logo: Option<&'static str>,
}

/// El objeto `Factura` del contrato.
#[derive(Serialize)]
pub struct Factura {
    id: i64,
    numero: i64,
    orden_id: i64,
    fecha: NaiveDate,
    taller: Taller,
    cliente_nombre: String,
    matricula: String,
    lineas: Vec<LineaFactura>,
    #[serde(with = "rust_decimal::serde::str")]
    base: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    iva_porcentaje: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    iva: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    total: Decimal,
    aviso_enviado: bool,
}

#[derive(sqlx::FromRow)]
struct FilaFactura {
    id: i64,
    numero: i64,
    orden_id: i64,
    fecha: NaiveDate,
    cliente_nombre: String,
    matricula: String,
    base: Decimal,
    iva_porcentaje: Decimal,
    iva: Decimal,
    total: Decimal,
    aviso_enviado: bool,
}

async fn lineas_de(conexion: &mut PgConnection, factura_id: i64) -> Resultado<Vec<LineaFactura>> {
    Ok(sqlx::query_as(
        "SELECT concepto, cantidad, precio_unitario, importe
           FROM linea_factura WHERE factura_id = $1 ORDER BY id",
    )
    .bind(factura_id)
    .fetch_all(conexion)
    .await?)
}

async fn factura_por_id(
    conexion: &mut PgConnection,
    id: i64,
    config: &Config,
) -> Resultado<Factura> {
    let fila: FilaFactura = sqlx::query_as(
        "SELECT id, numero, orden_id, fecha, cliente_nombre, matricula, base, iva_porcentaje, iva,
                total, aviso_enviado
           FROM factura WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&mut *conexion)
    .await?
    .ok_or_else(|| ErrorApi::NoEncontrado("No existe la factura".to_string()))?;
    Ok(Factura {
        lineas: lineas_de(conexion, id).await?,
        id: fila.id,
        numero: fila.numero,
        orden_id: fila.orden_id,
        fecha: fila.fecha,
        taller: Taller {
            nombre: config.taller_nombre.clone(),
            nif: config.taller_nif.clone(),
            direccion: config.taller_direccion.clone(),
            logo: archivo_logo(config).map(|_| RUTA_LOGO),
        },
        cliente_nombre: fila.cliente_nombre,
        matricula: fila.matricula,
        base: fila.base,
        iva_porcentaje: fila.iva_porcentaje,
        iva: fila.iva,
        total: fila.total,
        aviso_enviado: fila.aviso_enviado,
    })
}

/// La factura de una orden, si la tiene. Spec-002 la llama desde `ordenes.rs` para llenar
/// `factura_id` de `GET /api/ordenes/{id}` (research D7).
pub async fn factura_de_orden(
    conexion: &mut PgConnection,
    orden_id: i64,
) -> Resultado<Option<i64>> {
    Ok(
        sqlx::query_scalar("SELECT id FROM factura WHERE orden_id = $1")
            .bind(orden_id)
            .fetch_optional(conexion)
            .await?,
    )
}

/// `true` si el error es la violación de esa restricción `UNIQUE`.
fn viola(error: &sqlx::Error, restriccion: &str) -> bool {
    matches!(error, sqlx::Error::Database(e) if e.constraint() == Some(restriccion))
}

// ---------------------------------------------------------------------------------------------
// Facturar y avisar (US1, US2; research D1, D2)
// ---------------------------------------------------------------------------------------------

async fn facturar(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
    Path(id): Path<i64>,
) -> Resultado<(StatusCode, Json<Factura>)> {
    usuario.exigir_rol(&[Rol::Recepcion])?;

    let mut tx = estado.db.begin().await?;
    // Bloquea la orden: un segundo «Facturar» espera y la encuentra ya facturada.
    let orden: Option<(Estado, String, String, String)> = sqlx::query_as(
        "SELECT o.estado, c.nombre, c.email, v.matricula
           FROM orden o JOIN vehiculo v ON v.id = o.vehiculo_id JOIN cliente c ON c.id = v.cliente_id
          WHERE o.id = $1
            FOR UPDATE OF o",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    let (estado_orden, cliente, email, matricula) =
        orden.ok_or_else(|| ErrorApi::NoEncontrado("No existe la orden".to_string()))?;
    if estado_orden != Estado::Terminada {
        return Err(ErrorApi::Conflicto(format!(
            "La orden está {}; solo se puede facturar una orden terminada",
            estado_orden.texto()
        )));
    }

    let imputaciones: Vec<LineaFactura> = sqlx::query_as(
        "SELECT concepto, cantidad, precio_unitario, importe
           FROM imputacion WHERE orden_id = $1 ORDER BY creada_en, id",
    )
    .bind(id)
    .fetch_all(&mut *tx)
    .await?;
    if imputaciones.is_empty() {
        return Err(ErrorApi::Validacion {
            mensaje: "La orden no tiene imputaciones; no se puede facturar".to_string(),
            detalle: None,
        });
    }

    // Dentro de la transacción: si algo falla, el número vuelve y no queda hueco (R5).
    let numero = contador::siguiente(&mut tx, contador::FACTURA).await?;
    let t = totales(
        imputaciones.iter().map(|l| l.importe),
        estado.config.iva_porcentaje,
    );
    let factura_id: i64 = sqlx::query_scalar(
        "INSERT INTO factura (numero, orden_id, fecha, cliente_nombre, matricula, iva_porcentaje,
                              base, iva, total)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING id",
    )
    .bind(numero)
    .bind(id)
    .bind(fechas::hoy())
    .bind(&cliente)
    .bind(&matricula)
    .bind(t.iva_porcentaje)
    .bind(t.base)
    .bind(t.iva)
    .bind(t.total)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        if viola(&e, "factura_orden_id_key") {
            ErrorApi::Conflicto("Esta orden ya tiene factura".to_string())
        } else {
            e.into()
        }
    })?;
    for linea in &imputaciones {
        sqlx::query(
            "INSERT INTO linea_factura (factura_id, concepto, cantidad, precio_unitario, importe)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(factura_id)
        .bind(&linea.concepto)
        .bind(linea.cantidad)
        .bind(linea.precio_unitario)
        .bind(linea.importe)
        .execute(&mut *tx)
        .await?;
    }
    let cambio = cambiar_estado(&mut tx, id, &[Estado::Terminada], Estado::Facturada).await?;
    tx.commit().await?;
    // La orden facturada sale del tablero (R11, R14).
    cambio.publicar(&estado.eventos);

    // Después del commit: si el correo falla, la factura ya está emitida (R10).
    let cuerpo = format!(
        "Hola, {cliente}:\n\n\
         Su vehículo {matricula} ya está reparado y puede pasar a recogerlo.\n\n\
         Total de la factura nº {numero}: {}.\n\nUn saludo,\n{}",
        formato::euros(t.total),
        estado.config.taller_nombre
    );
    let aviso_enviado = estado
        .correo
        .enviar(
            &email,
            &format!("Su vehículo {matricula} está listo para recoger"),
            &cuerpo,
        )
        .await;
    sqlx::query("UPDATE factura SET aviso_enviado = $2 WHERE id = $1")
        .bind(factura_id)
        .bind(aviso_enviado)
        .execute(&estado.db)
        .await?;

    let mut conexion = estado.db.acquire().await?;
    let factura = factura_por_id(&mut conexion, factura_id, &estado.config).await?;
    Ok((StatusCode::CREATED, Json(factura)))
}

async fn ver_factura(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
    Path(id): Path<i64>,
) -> Resultado<Json<Factura>> {
    usuario.exigir_rol(&[Rol::Recepcion])?;
    let mut conexion = estado.db.acquire().await?;
    Ok(Json(
        factura_por_id(&mut conexion, id, &estado.config).await?,
    ))
}

// ---------------------------------------------------------------------------------------------
// Historial por matrícula (US3; research D5)
// ---------------------------------------------------------------------------------------------

#[derive(Serialize)]
struct Reparacion {
    orden_numero: i64,
    fecha: NaiveDate,
    km: i32,
    diagnostico: Option<String>,
    factura_numero: i64,
    lineas: Vec<LineaFactura>,
    #[serde(with = "rust_decimal::serde::str")]
    total: Decimal,
}

#[derive(Serialize)]
struct Historial {
    matricula: String,
    marca: String,
    modelo: String,
    reparaciones: Vec<Reparacion>,
}

/// Cualquier rol con sesión.
async fn historial(
    State(estado): State<EstadoApp>,
    _usuario: Usuario,
    Path(escrita): Path<String>,
) -> Resultado<Json<Historial>> {
    let matricula = normalizar_matricula(&escrita).ok_or_else(|| ErrorApi::Validacion {
        mensaje: "Matrícula no válida".to_string(),
        detalle: None,
    })?;
    let mut conexion = estado.db.acquire().await?;
    let (vehiculo_id, marca, modelo): (i64, String, String) =
        sqlx::query_as("SELECT id, marca, modelo FROM vehiculo WHERE matricula = $1")
            .bind(&matricula)
            .fetch_optional(&mut *conexion)
            .await?
            .ok_or_else(|| {
                ErrorApi::NoEncontrado(format!(
                    "No hay ningún vehículo con la matrícula {matricula}"
                ))
            })?;

    let filas: Vec<(i64, i64, NaiveDate, i32, Option<String>, i64, Decimal)> = sqlx::query_as(
        "SELECT f.id, o.numero, f.fecha, o.km, o.diagnostico, f.numero, f.total
           FROM factura f JOIN orden o ON o.id = f.orden_id
          WHERE o.vehiculo_id = $1
          ORDER BY f.fecha DESC, f.numero DESC",
    )
    .bind(vehiculo_id)
    .fetch_all(&mut *conexion)
    .await?;
    let mut reparaciones = Vec::with_capacity(filas.len());
    for (factura_id, orden_numero, fecha, km, diagnostico, factura_numero, total) in filas {
        reparaciones.push(Reparacion {
            orden_numero,
            fecha,
            km,
            diagnostico,
            factura_numero,
            lineas: lineas_de(&mut conexion, factura_id).await?,
            total,
        });
    }
    Ok(Json(Historial {
        matricula,
        marca,
        modelo,
        reparaciones,
    }))
}
