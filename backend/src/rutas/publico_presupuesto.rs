//! Respuesta del cliente al presupuesto desde su enlace (spec-004; contrato, sección «spec-004 ·
//! Respuesta del cliente»). Sin sesión: el token del enlace es lo que da acceso.

use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

use crate::EstadoApp;
use crate::config::Config;
use crate::dinero::Totales;
use crate::error::{ErrorApi, Json, Resultado};
use crate::estados::{Estado, cambiar_estado};
use crate::fechas;
use crate::lineas::Linea;

const ENLACE_NO_VALIDO: &str = "Este enlace no es válido";

pub fn rutas() -> Router<EstadoApp> {
    Router::new()
        .route("/publico/presupuestos/{token}", get(ver))
        .route("/publico/presupuestos/{token}/respuesta", post(responder))
}

#[derive(Serialize)]
struct Taller {
    nombre: String,
}

/// Lo que ve el cliente. La consulta no une con `cliente`: no puede llevar sus datos (FR-002).
#[derive(Serialize)]
struct PresupuestoPublico {
    taller: Taller,
    matricula: String,
    marca: String,
    modelo: String,
    diagnostico: Option<String>,
    tipo: String,
    lineas: Vec<Linea>,
    #[serde(flatten)]
    totales: Totales,
    caduca_el: NaiveDate,
    caducado: bool,
    decision: Option<String>,
    respondido_en: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
struct FilaPublica {
    id: i64,
    matricula: String,
    marca: String,
    modelo: String,
    diagnostico: Option<String>,
    tipo: String,
    estado: String,
    iva_porcentaje: Decimal,
    base: Decimal,
    iva: Decimal,
    total: Decimal,
    caduca_el: NaiveDate,
    respondido_en: Option<DateTime<Utc>>,
}

async fn presupuesto_publico(
    conexion: &mut PgConnection,
    token: &str,
    config: &Config,
) -> Resultado<PresupuestoPublico> {
    let fila: FilaPublica = sqlx::query_as(
        "SELECT p.id, v.matricula, v.marca, v.modelo, o.diagnostico, p.tipo::text AS tipo,
                p.estado::text AS estado, p.iva_porcentaje, p.base, p.iva, p.total, p.caduca_el,
                p.respondido_en
           FROM presupuesto p
           JOIN orden o ON o.id = p.orden_id
           JOIN vehiculo v ON v.id = o.vehiculo_id
          WHERE p.token = $1",
    )
    .bind(token)
    .fetch_optional(&mut *conexion)
    .await?
    .ok_or_else(|| ErrorApi::NoEncontrado(ENLACE_NO_VALIDO.to_string()))?;
    let lineas: Vec<Linea> = sqlx::query_as(
        "SELECT id, tipo::text AS tipo, concepto, pieza_id, cantidad, precio_unitario, importe
           FROM linea_presupuesto WHERE presupuesto_id = $1 ORDER BY id",
    )
    .bind(fila.id)
    .fetch_all(&mut *conexion)
    .await?;

    Ok(PresupuestoPublico {
        taller: Taller {
            nombre: config.taller_nombre.clone(),
        },
        matricula: fila.matricula,
        marca: fila.marca,
        modelo: fila.modelo,
        diagnostico: fila.diagnostico,
        tipo: fila.tipo,
        lineas,
        totales: Totales {
            base: fila.base,
            iva_porcentaje: fila.iva_porcentaje,
            iva: fila.iva,
            total: fila.total,
        },
        caduca_el: fila.caduca_el,
        // Provisional hasta T012, que llamará a `presupuestos::caducado()`. Solo llega aquí un
        // presupuesto ya respondido, y uno respondido nunca está caducado.
        caducado: false,
        decision: (fila.estado != "emitido").then_some(fila.estado),
        respondido_en: fila.respondido_en,
    })
}

async fn ver(State(estado): State<EstadoApp>, Path(token): Path<String>) -> Resultado<Response> {
    let mut conexion = estado.db.acquire().await?;
    let presupuesto = presupuesto_publico(&mut conexion, &token, &estado.config).await?;
    // Provisional: un presupuesto sin responder necesita `caducado()`, que llega con T012.
    if presupuesto.decision.is_none() {
        return Ok(StatusCode::NOT_IMPLEMENTED.into_response());
    }
    Ok(Json(presupuesto).into_response())
}

#[derive(Deserialize)]
struct Respuesta {
    decision: Option<String>,
}

#[derive(sqlx::FromRow)]
struct Respondido {
    orden_id: i64,
    tipo: String,
}

/// FR-003, FR-004 y FR-006 (research D2 y D3).
async fn responder(
    State(estado): State<EstadoApp>,
    Path(token): Path<String>,
    Json(respuesta): Json<Respuesta>,
) -> Resultado<Response> {
    let nuevo_estado = match respuesta.decision.as_deref() {
        Some("aceptar") => "aceptado",
        // Provisional: rechazar llega con T021.
        Some("rechazar") => return Ok(StatusCode::NOT_IMPLEMENTED.into_response()),
        _ => {
            return Err(ErrorApi::Validacion {
                mensaje: "Elija aceptar o rechazar".to_string(),
                detalle: None,
            });
        }
    };

    let mut tx = estado.db.begin().await?;
    // La única versión de esta regla (research R9): solo cuenta la primera respuesta.
    let respondido: Option<Respondido> = sqlx::query_as(
        "UPDATE presupuesto
            SET estado = $3::estado_presupuesto, respondido_en = now()
          WHERE token = $1 AND estado = 'emitido' AND caduca_el >= $2
         RETURNING orden_id, tipo::text AS tipo",
    )
    .bind(&token)
    .bind(fechas::hoy())
    .bind(nuevo_estado)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(respondido) = respondido else {
        let existe: bool =
            sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM presupuesto WHERE token = $1)")
                .bind(&token)
                .fetch_one(&mut *tx)
                .await?;
        return Err(if existe {
            // Provisional: el 409 de caducidad llega con T025.
            ErrorApi::Conflicto("Este presupuesto ya fue respondido".to_string())
        } else {
            ErrorApi::NoEncontrado(ENLACE_NO_VALIDO.to_string())
        });
    };

    // Provisional: aceptar una ampliación llega con T021. Sin `commit` no se guarda nada.
    if respondido.tipo != "inicial" {
        return Ok(StatusCode::NOT_IMPLEMENTED.into_response());
    }
    let cambio = cambiar_estado(
        &mut tx,
        respondido.orden_id,
        &[Estado::EnDiagnostico],
        Estado::EnReparacion,
    )
    .await?;
    let presupuesto = presupuesto_publico(&mut tx, &token, &estado.config).await?;
    tx.commit().await?;

    // Después del commit, para que el tablero nunca lea un estado sin confirmar (R11, R14).
    cambio.publicar(&estado.eventos);
    Ok(Json(presupuesto).into_response())
}
