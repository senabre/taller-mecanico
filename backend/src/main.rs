//! Arranque del servidor en http://localhost:3000. Las migraciones no se ejecutan aquí: se
//! aplican con `cargo sqlx migrate run` (quickstart).

use std::process;
use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use taller::config::Config;
use taller::correo::Correo;
use taller::eventos::Eventos;
use taller::{EstadoApp, app};
use tracing_subscriber::EnvFilter;

const DIRECCION: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    // La configuración va primero porque carga el .env, donde está RUST_LOG.
    let config = Config::desde_entorno().unwrap_or_else(|mensaje| salir(&mensaje));

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .unwrap_or_else(|e| salir(&format!("No se pudo conectar con la base de datos: {e}")));
    let correo = Correo::desde_config(&config).unwrap_or_else(|mensaje| salir(&mensaje));

    let estado = EstadoApp {
        db,
        config: Arc::new(config),
        eventos: Eventos::nuevo(),
        correo,
    };

    let escucha = tokio::net::TcpListener::bind(DIRECCION)
        .await
        .unwrap_or_else(|e| salir(&format!("No se pudo escuchar en {DIRECCION}: {e}")));
    tracing::info!("Talleres Rueda en http://localhost:3000");

    if let Err(e) = axum::serve(escucha, app(estado)).await {
        salir(&format!("El servidor se ha detenido por un error: {e}"));
    }
}

fn salir(mensaje: &str) -> ! {
    eprintln!("{mensaje}");
    process::exit(1);
}
