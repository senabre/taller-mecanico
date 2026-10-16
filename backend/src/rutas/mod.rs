//! Rutas de la API (`/api/...`). Cada spec añade aquí su módulo con `.merge(modulo::rutas())`.

use axum::Router;

use crate::EstadoApp;

pub fn api() -> Router<EstadoApp> {
    Router::new()
}
