//! Citas (spec-001; contrato, sección «spec-001 · Citas»): pedir cita desde la página pública y
//! lista de citas del día para recepción. El orden de las comprobaciones del `POST` es el de la
//! tabla de validación del contrato: todos los 422 antes que cualquier 409 (research D2 y D3).

use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use chrono::{Datelike, Days, NaiveDate, Weekday};
use serde::Deserialize;
use sqlx::PgExecutor;

use crate::EstadoApp;
use crate::error::{ErrorApi, Json, Resultado};
use crate::fechas;
use crate::matricula::normalizar_matricula;

pub fn rutas() -> Router<EstadoApp> {
    Router::new()
        .route("/publico/vehiculos/{matricula}", get(vehiculo_publico))
        .route("/publico/dias-disponibles", get(dias_disponibles))
        .route("/publico/citas", post(crear_cita))
        .route("/citas", get(citas_del_dia))
}

/// 422 con el campo afectado, para que el formulario muestre el mensaje bajo ese campo
/// (contrato, tabla de validación de spec-001).
fn error_campo(campo: &str, mensaje: &str) -> ErrorApi {
    ErrorApi::Validacion {
        mensaje: mensaje.to_string(),
        detalle: Some(serde_json::json!({ "campo": campo })),
    }
}

/// Días que se pueden pedir: de lunes a viernes, desde mañana hasta `hoy + 30` incluido (FR-006).
fn dias_ofrecidos(hoy: NaiveDate) -> Vec<NaiveDate> {
    (1..=30)
        .map(|d| hoy + Days::new(d))
        .filter(|dia| !matches!(dia.weekday(), Weekday::Sat | Weekday::Sun))
        .collect()
}

/// Id del vehículo con esa matrícula (ya normalizada), si existe.
async fn buscar_vehiculo(db: impl PgExecutor<'_>, matricula: &str) -> Resultado<Option<i64>> {
    let id = sqlx::query_scalar("SELECT id FROM vehiculo WHERE matricula = $1")
        .bind(matricula)
        .fetch_optional(db)
        .await?;
    Ok(id)
}

async fn vehiculo_publico() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

/// De momento, todos los días del rango; el filtro por cupo lo añade T029.
async fn dias_disponibles() -> Json<Vec<NaiveDate>> {
    Json(dias_ofrecidos(fechas::hoy()))
}

#[derive(Deserialize)]
struct PeticionCita {
    matricula: Option<String>,
    fecha: Option<String>,
    motivo: Option<String>,
}

async fn crear_cita(
    State(estado): State<EstadoApp>,
    Json(peticion): Json<PeticionCita>,
) -> Resultado<Response> {
    // 1. Matrícula, fecha y motivo, en el orden de la tabla de validación.
    let matricula = peticion
        .matricula
        .as_deref()
        .and_then(normalizar_matricula)
        .ok_or_else(|| error_campo("matricula", "La matrícula no es válida"))?;
    let fecha = peticion
        .fecha
        .as_deref()
        .and_then(|texto| texto.parse::<NaiveDate>().ok())
        .filter(|fecha| dias_ofrecidos(fechas::hoy()).contains(fecha))
        .ok_or_else(|| error_campo("fecha", "Elige uno de los días disponibles"))?;
    let motivo = peticion.motivo.as_deref().map(str::trim).unwrap_or_default();
    if motivo.is_empty() {
        return Err(error_campo("motivo", "El motivo es obligatorio"));
    }

    // 2. Antes de los bloqueos: si el vehículo es nuevo, sus datos se validan aquí, para que
    //    todos los 422 lleguen antes que cualquier 409.
    if buscar_vehiculo(&estado.db, &matricula).await?.is_none() {
        // Hueco de T024 (US2): validar marca, modelo, nombre, teléfono y correo.
        return Err(ErrorApi::Validacion {
            mensaje: "Todavía no se pueden dar de alta vehículos nuevos".to_string(),
            detalle: None,
        });
    }

    // 3. Los dos bloqueos, siempre en este orden para que no haya interbloqueos (R6 del plan
    //    maestro, tabla de claves; research D3). Primero la fecha, con la forma de una clave de
    //    64 bits; después la matrícula, con la de dos claves de 32 bits. PostgreSQL no solapa
    //    los dos espacios de claves.
    let mut tx = estado.db.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(($1::date - DATE '2000-01-01')::bigint)")
        .bind(fecha)
        .execute(&mut *tx)
        .await?;
    sqlx::query("SELECT pg_advisory_xact_lock(1, hashtext($1))")
        .bind(&matricula)
        .execute(&mut *tx)
        .await?;

    // 4. Con los bloqueos tomados, se vuelve a buscar: si otra persona lo ha dado de alta entre
    //    el paso 2 y el 3, se trata como vehículo conocido (D4) y se ignoran `vehiculo` y
    //    `cliente`.
    let _vehiculo = buscar_vehiculo(&mut *tx, &matricula).await?;

    // Huecos: T016 (cita pendiente, 409), T029 (cupo, 409) y T024 (alta del vehículo nuevo),
    // y la inserción de la cita. Sin `commit`, la transacción se deshace al salir.
    Ok(StatusCode::NOT_IMPLEMENTED.into_response())
}

async fn citas_del_dia() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}
