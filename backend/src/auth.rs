//! Sesión del personal y roles (research R8, spec-000 FR-001/FR-002).
//!
//! La sesión se guarda en el servidor (tabla `sesion`); la cookie `sesion` solo lleva un token
//! aleatorio, y en la base de datos se guarda su SHA-256. Las contraseñas se guardan con argon2id.

use std::time::Duration;

use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderValue, header};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::{DateTime, TimeDelta, Utc};
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::EstadoApp;
use crate::error::{ErrorApi, Resultado};

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[sqlx(type_name = "rol", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Rol {
    Recepcion,
    Mecanico,
    Almacen,
    Jefe,
}

/// Una sesión caduca tras 12 horas sin actividad: cada petición la renueva.
pub const DURACION_SESION: Duration = Duration::from_secs(12 * 60 * 60);

const NOMBRE_COOKIE: &str = "sesion";

pub fn hash_contrasena(contrasena: &str) -> String {
    let sal = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(contrasena.as_bytes(), &sal)
        .expect("argon2 con parámetros por defecto no falla")
        .to_string()
}

pub fn verificar_contrasena(contrasena: &str, hash: &str) -> bool {
    PasswordHash::new(hash)
        .map(|hash| {
            Argon2::default()
                .verify_password(contrasena.as_bytes(), &hash)
                .is_ok()
        })
        .unwrap_or(false)
}

/// 32 bytes aleatorios en base64url sin relleno.
pub fn nuevo_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

/// SHA-256 en hexadecimal: es lo que se guarda en `sesion.token_hash`.
pub fn hash_token(token: &str) -> String {
    format!("{:x}", Sha256::digest(token.as_bytes()))
}

pub fn nueva_expiracion() -> DateTime<Utc> {
    Utc::now() + TimeDelta::from_std(DURACION_SESION).expect("12 horas caben en TimeDelta")
}

/// Cookie de sesión del navegador (sin `Max-Age`): la caducidad la decide el servidor con
/// `expira_en`, que se renueva con cada petición. `Secure` solo en producción (HTTPS).
pub fn cookie_sesion(token: &str, segura: bool) -> HeaderValue {
    let segura = if segura { "; Secure" } else { "" };
    HeaderValue::from_str(&format!(
        "{NOMBRE_COOKIE}={token}; HttpOnly; SameSite=Lax; Path=/{segura}"
    ))
    .expect("el token es base64url, siempre válido en una cabecera")
}

pub fn cookie_borrada() -> HeaderValue {
    HeaderValue::from_static("sesion=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0")
}

/// Lee el token de la cookie `sesion` de la cabecera `Cookie`.
pub fn token_de_cookie(cabeceras: &HeaderMap) -> Option<String> {
    cabeceras
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .filter_map(|par| par.trim().split_once('='))
        .find(|(nombre, valor)| *nombre == NOMBRE_COOKIE && !valor.is_empty())
        .map(|(_, valor)| valor.to_string())
}

/// El empleado que ha entrado. Usarlo como parámetro de un handler exige sesión (401 si no hay).
#[derive(Clone, Debug, Serialize)]
pub struct Usuario {
    pub id: i64,
    pub nombre: String,
    pub rol: Rol,
}

impl FromRequestParts<EstadoApp> for Usuario {
    type Rejection = ErrorApi;

    async fn from_request_parts(parts: &mut Parts, estado: &EstadoApp) -> Resultado<Usuario> {
        let token = token_de_cookie(&parts.headers).ok_or(ErrorApi::SinSesion)?;
        // Busca la sesión vigente de un usuario activo y la renueva en la misma consulta.
        let fila: Option<(i64, String, Rol)> = sqlx::query_as(
            "UPDATE sesion s
                SET expira_en = $2
               FROM usuario u
              WHERE s.token_hash = $1
                AND s.expira_en > now()
                AND u.id = s.usuario_id
                AND u.activo
          RETURNING u.id, u.nombre, u.rol",
        )
        .bind(hash_token(&token))
        .bind(nueva_expiracion())
        .fetch_optional(&estado.db)
        .await?;
        let (id, nombre, rol) = fila.ok_or(ErrorApi::SinSesion)?;
        Ok(Usuario { id, nombre, rol })
    }
}

impl Usuario {
    /// El jefe de taller puede hacer todo lo que hacen los demás roles (FR-002).
    pub fn exigir_rol(&self, roles: &[Rol]) -> Resultado<()> {
        if self.rol == Rol::Jefe || roles.contains(&self.rol) {
            Ok(())
        } else {
            Err(ErrorApi::Prohibido)
        }
    }

    /// Solo el mecánico asignado a la orden (o el jefe) puede diagnosticar, presupuestar e
    /// imputar en ella (spec-003, spec-005).
    pub fn exigir_mecanico_de(&self, mecanico_id: i64) -> Resultado<()> {
        if self.rol == Rol::Jefe || self.id == mecanico_id {
            Ok(())
        } else {
            Err(ErrorApi::Prohibido)
        }
    }
}
