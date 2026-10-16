//! Errores de la API: cada variante se convierte en el código HTTP del contrato y en un cuerpo
//! `{"error": "...", "detalle": {...}}` con el mensaje en español (contracts/api.md).

use axum::extract::FromRequest;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Debug)]
pub enum ErrorApi {
    /// 400: JSON mal formado.
    PeticionMala(String),
    /// 401: sin sesión.
    SinSesion,
    /// 403: rol no permitido o la orden no es del mecánico.
    Prohibido,
    /// 404: no existe.
    NoEncontrado(String),
    /// 409: conflicto de estado.
    Conflicto(String),
    /// 422: validación.
    Validacion { mensaje: String, detalle: Option<Value> },
    /// 500: el texto es técnico y solo va al registro, nunca al cliente.
    Interno(String),
}

pub type Resultado<T> = Result<T, ErrorApi>;

impl IntoResponse for ErrorApi {
    fn into_response(self) -> Response {
        let (codigo, mensaje, detalle) = match self {
            ErrorApi::PeticionMala(m) => (StatusCode::BAD_REQUEST, m, None),
            ErrorApi::SinSesion => (
                StatusCode::UNAUTHORIZED,
                "Tienes que entrar con tu usuario".to_string(),
                None,
            ),
            ErrorApi::Prohibido => (
                StatusCode::FORBIDDEN,
                "No tienes permiso para hacer esto".to_string(),
                None,
            ),
            ErrorApi::NoEncontrado(m) => (StatusCode::NOT_FOUND, m, None),
            ErrorApi::Conflicto(m) => (StatusCode::CONFLICT, m, None),
            ErrorApi::Validacion { mensaje, detalle } => {
                (StatusCode::UNPROCESSABLE_ENTITY, mensaje, detalle)
            }
            ErrorApi::Interno(tecnico) => {
                tracing::error!("{tecnico}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Ha ocurrido un error interno".to_string(),
                    None,
                )
            }
        };
        let mut cuerpo = json!({ "error": mensaje });
        if let Some(detalle) = detalle {
            cuerpo["detalle"] = detalle;
        }
        (codigo, axum::Json(cuerpo)).into_response()
    }
}

impl From<sqlx::Error> for ErrorApi {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => ErrorApi::NoEncontrado("No existe".to_string()),
            otro => ErrorApi::Interno(otro.to_string()),
        }
    }
}

impl From<JsonRejection> for ErrorApi {
    fn from(rechazo: JsonRejection) -> Self {
        tracing::debug!("JSON rechazado: {rechazo}");
        ErrorApi::PeticionMala("Los datos enviados no son válidos".to_string())
    }
}

/// Igual que `axum::Json`, pero un cuerpo inválido responde con un error JSON en español.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ErrorApi))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}
