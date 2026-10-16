//! Entrar, salir, quién soy y lista de mecánicos (contrato, sección «Base común»).

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use serde::{Deserialize, Serialize};

use crate::EstadoApp;
use crate::auth::{
    Usuario, cookie_borrada, cookie_sesion, hash_token, nueva_expiracion, nuevo_token,
    token_de_cookie, verificar_contrasena,
};
use crate::error::{ErrorApi, Json, Resultado};

pub fn rutas() -> Router<EstadoApp> {
    Router::new()
        .route("/sesion", post(entrar).delete(salir))
        .route("/yo", get(yo))
        .route("/mecanicos", get(mecanicos))
}

#[derive(Deserialize)]
struct Credenciales {
    login: String,
    contrasena: String,
}

async fn entrar(
    State(estado): State<EstadoApp>,
    Json(credenciales): Json<Credenciales>,
) -> Resultado<impl IntoResponse> {
    let login = credenciales.login.trim().to_lowercase();
    let usuario: Option<(i64, String, bool)> =
        sqlx::query_as("SELECT id, hash_contrasena, activo FROM usuario WHERE login = $1")
            .bind(&login)
            .fetch_optional(&estado.db)
            .await?;
    // Mismo error si no existe, si está inactivo o si la contraseña no coincide.
    let usuario_id = match usuario {
        Some((id, hash, true)) if verificar_contrasena(&credenciales.contrasena, &hash) => id,
        _ => return Err(ErrorApi::CredencialesIncorrectas),
    };

    let token = nuevo_token();
    let mut tx = estado.db.begin().await?;
    sqlx::query("DELETE FROM sesion WHERE expira_en <= now()")
        .execute(&mut *tx)
        .await?;
    sqlx::query("INSERT INTO sesion (token_hash, usuario_id, expira_en) VALUES ($1, $2, $3)")
        .bind(hash_token(&token))
        .bind(usuario_id)
        .bind(nueva_expiracion())
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    Ok((
        StatusCode::NO_CONTENT,
        [(
            header::SET_COOKIE,
            cookie_sesion(&token, estado.config.cookie_segura()),
        )],
    ))
}

async fn salir(
    State(estado): State<EstadoApp>,
    _usuario: Usuario,
    cabeceras: HeaderMap,
) -> Resultado<impl IntoResponse> {
    if let Some(token) = token_de_cookie(&cabeceras) {
        sqlx::query("DELETE FROM sesion WHERE token_hash = $1")
            .bind(hash_token(&token))
            .execute(&estado.db)
            .await?;
    }
    Ok((
        StatusCode::NO_CONTENT,
        [(header::SET_COOKIE, cookie_borrada())],
    ))
}

async fn yo(usuario: Usuario) -> Json<Usuario> {
    Json(usuario)
}

#[derive(Serialize, sqlx::FromRow)]
struct Mecanico {
    id: i64,
    nombre: String,
}

async fn mecanicos(
    State(estado): State<EstadoApp>,
    _usuario: Usuario,
) -> Resultado<Json<Vec<Mecanico>>> {
    let lista = sqlx::query_as(
        "SELECT id, nombre FROM usuario WHERE rol = 'mecanico' AND activo ORDER BY nombre",
    )
    .fetch_all(&estado.db)
    .await?;
    Ok(Json(lista))
}
