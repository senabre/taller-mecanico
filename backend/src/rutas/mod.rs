//! Rutas de la API (`/api/...`). Cada spec añade aquí su módulo con `.merge(modulo::rutas())`.

pub mod almacen;
pub mod citas;
pub mod imputaciones;
pub mod presupuestos;
pub mod sesion;

use axum::Router;

use crate::{EstadoApp, eventos};

pub fn api() -> Router<EstadoApp> {
    Router::new()
        .merge(sesion::rutas())
        .merge(eventos::rutas())
        .merge(almacen::rutas())
        .merge(presupuestos::rutas())
        .merge(citas::rutas())
        .merge(imputaciones::rutas())
}