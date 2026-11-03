//! Órdenes de reparación (spec-002; contrato, sección «spec-002 · Orden de reparación»): abrir la
//! orden de una cita de hoy, la lista de órdenes y la ficha de una orden. La ficha añade las
//! secciones de spec-003, 005 y 006 llamando a una función de cada módulo (research D6).

use axum::Router;
use axum::http::StatusCode;
use axum::routing::get;

use crate::EstadoApp;

pub fn rutas() -> Router<EstadoApp> {
    Router::new()
        .route("/ordenes", get(listar_ordenes).post(abrir_orden))
        .route("/ordenes/{id}", get(ver_orden))
}

/// Provisional hasta T017.
async fn abrir_orden() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

/// Provisional hasta T021.
async fn listar_ordenes() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

/// Provisional hasta T019.
async fn ver_orden() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}
