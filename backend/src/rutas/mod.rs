//! Rutas de la API (`/api/...`). Cada spec añade aquí su módulo con `.merge(modulo::rutas())`.

pub mod sesion;

use axum::Router;

use crate::{EstadoApp, eventos};

pub fn api() -> Router<EstadoApp> {
    Router::new()
        .merge(sesion::rutas())
        .merge(eventos::rutas())
}
