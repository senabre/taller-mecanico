//! Talleres Rueda: módulos comunes del backend y montaje del router (plan maestro, research R2).

pub mod auth;
pub mod config;
pub mod contador;
pub mod correo;
pub mod dinero;
pub mod error;
pub mod estados;
pub mod eventos;
pub mod fechas;
pub mod formato;
pub mod lineas;
pub mod matricula;
pub mod rutas;

use std::sync::Arc;

use axum::Router;
use sqlx::PgPool;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::correo::Correo;
use crate::error::ErrorApi;
use crate::eventos::Eventos;

#[derive(Clone)]
pub struct EstadoApp {
    pub db: PgPool,
    pub config: Arc<Config>,
    pub eventos: Eventos,
    pub correo: Correo,
}

/// Un solo proceso y un solo origen: la API en `/api` y el frontend compilado en el resto.
pub fn app(estado: EstadoApp) -> Router {
    // Una ruta /api desconocida responde 404 en JSON; nunca devuelve index.html.
    let api = rutas::api().fallback(|| async { ErrorApi::NoEncontrado("No existe".to_string()) });
    let frontend = ServeDir::new("../frontend/build")
        .fallback(ServeFile::new("../frontend/build/index.html"));

    Router::new()
        .nest("/api", api)
        .fallback_service(frontend)
        .layer(TraceLayer::new_for_http())
        .with_state(estado)
}
