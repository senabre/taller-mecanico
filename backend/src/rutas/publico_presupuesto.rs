//! Respuesta del cliente al presupuesto desde su enlace (spec-004; contrato, sección «spec-004 ·
//! Respuesta del cliente»). Sin sesión: el token del enlace es lo que da acceso.

use axum::Router;
use axum::http::StatusCode;
use axum::routing::{get, post};

use crate::EstadoApp;

pub fn rutas() -> Router<EstadoApp> {
    Router::new()
        .route("/publico/presupuestos/{token}", get(provisional))
        .route("/publico/presupuestos/{token}/respuesta", post(provisional))
}

/// Provisional: el `GET` lo implementa T012 y el `POST`, T013.
async fn provisional() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}
