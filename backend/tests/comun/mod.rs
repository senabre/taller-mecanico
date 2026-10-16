//! Utilidades de prueba para todas las specs (`mod comun;` en cada fichero de `tests/`).
//! Parte 1 (fase 2): no necesitan `auth.rs`. Parte 2 (US1): `crear_usuario` y `entrar`.
#![allow(dead_code)]

use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use rust_decimal_macros::dec;
use serde_json::{Value, json};
use sqlx::PgPool;
use taller::auth::{hash_contrasena, nuevo_token};
use taller::config::Config;
use taller::correo::Correo;
use taller::estados::Estado;
use taller::eventos::Eventos;
use taller::{EstadoApp, app};
use tower::ServiceExt;

/// Estado de la app con una configuración de prueba fija (los valores demo del quickstart).
pub fn estado_prueba(pool: PgPool) -> EstadoApp {
    let config = Config {
        database_url: String::new(),
        smtp_url: "smtp://localhost:1025".to_string(),
        correo_remitente: "Talleres Rueda <taller@talleres-rueda.test>".to_string(),
        url_publica: "http://localhost:3000".to_string(),
        cupo_diario: 2,
        precio_hora: dec!(40.00),
        iva_porcentaje: dec!(21.00),
        taller_nombre: "Talleres Rueda".to_string(),
        taller_nif: "B00000000".to_string(),
        taller_direccion: "Calle Ejemplo 1, 03001 Alicante".to_string(),
    };
    let correo = Correo::desde_config(&config).expect("configuración de correo de prueba");
    EstadoApp {
        db: pool,
        config: Arc::new(config),
        eventos: Eventos::nuevo(),
        correo,
    }
}

pub fn app_prueba(pool: PgPool) -> Router {
    app(estado_prueba(pool))
}

/// Hace una petición al router y devuelve el código, el cuerpo JSON (`Null` si no hay) y la
/// cabecera `Set-Cookie`, si la hay.
pub async fn peticion(
    app: &Router,
    metodo: &str,
    ruta: &str,
    cookie: Option<&str>,
    cuerpo: Option<Value>,
) -> (StatusCode, Value, Option<String>) {
    let mut req = Request::builder().method(metodo).uri(ruta);
    if let Some(cookie) = cookie {
        req = req.header(header::COOKIE, cookie);
    }
    let req = match cuerpo {
        Some(json) => req
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json.to_string())),
        None => req.body(Body::empty()),
    }
    .unwrap();

    let respuesta = app.clone().oneshot(req).await.unwrap();
    let codigo = respuesta.status();
    let set_cookie = respuesta
        .headers()
        .get(header::SET_COOKIE)
        .map(|v| v.to_str().unwrap().to_string());
    let bytes = respuesta.into_body().collect().await.unwrap().to_bytes();
    let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (codigo, json, set_cookie)
}

/// Usuario insertado por SQL, con un hash que no sirve para entrar: solo para asignarle órdenes.
pub async fn insertar_usuario(pool: &PgPool, login: &str, rol: &str) -> i64 {
    sqlx::query_scalar(
        "INSERT INTO usuario (nombre, login, hash_contrasena, rol)
         VALUES ($1, $1, 'no-sirve-para-entrar', $2::rol) RETURNING id",
    )
    .bind(login)
    .bind(rol)
    .fetch_one(pool)
    .await
    .unwrap()
}

/// Inserta cliente, vehículo, cita y orden por SQL, sin pasar por spec-001 ni spec-002.
pub async fn crear_orden(pool: &PgPool, numero: i64, estado: Estado, mecanico_id: i64) -> i64 {
    let cliente_id: i64 = sqlx::query_scalar(
        "INSERT INTO cliente (nombre, telefono, email)
         VALUES ('Cliente de prueba', '600000000', 'cliente@ejemplo.test') RETURNING id",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    let vehiculo_id: i64 = sqlx::query_scalar(
        "INSERT INTO vehiculo (matricula, marca, modelo, cliente_id)
         VALUES ($1, 'Seat', 'Ibiza', $2) RETURNING id",
    )
    .bind(format!("{numero:04}PRB"))
    .bind(cliente_id)
    .fetch_one(pool)
    .await
    .unwrap();
    let cita_id: i64 = sqlx::query_scalar(
        "INSERT INTO cita (vehiculo_id, fecha, motivo)
         VALUES ($1, current_date, 'Revisión') RETURNING id",
    )
    .bind(vehiculo_id)
    .fetch_one(pool)
    .await
    .unwrap();
    sqlx::query_scalar(
        "INSERT INTO orden (numero, cita_id, vehiculo_id, mecanico_id, km, danos,
                            fecha_entrega_prometida, estado)
         VALUES ($1, $2, $3, $4, 1000, 'Sin daños visibles', current_date + 3, $5)
         RETURNING id",
    )
    .bind(numero)
    .bind(cita_id)
    .bind(vehiculo_id)
    .bind(mecanico_id)
    .bind(estado)
    .fetch_one(pool)
    .await
    .unwrap()
}

/// Usuario que puede entrar: la contraseña es aleatoria y se devuelve junto al id. Las pruebas
/// nunca contienen contraseñas escritas a mano (principio V, constitución v1.2.1).
/// El nombre es el login con la inicial en mayúscula («luis» → «Luis»).
pub async fn crear_usuario(pool: &PgPool, login: &str, rol: &str, activo: bool) -> (i64, String) {
    let contrasena = nuevo_token();
    let mut letras = login.chars();
    let nombre: String = letras
        .next()
        .map(|c| c.to_uppercase().chain(letras).collect())
        .unwrap_or_default();
    let id = sqlx::query_scalar(
        "INSERT INTO usuario (nombre, login, hash_contrasena, rol, activo)
         VALUES ($1, $2, $3, $4::rol, $5) RETURNING id",
    )
    .bind(nombre)
    .bind(login)
    .bind(hash_contrasena(&contrasena))
    .bind(rol)
    .bind(activo)
    .fetch_one(pool)
    .await
    .unwrap();
    (id, contrasena)
}

/// Entra con `POST /api/sesion` y devuelve la cookie lista para la cabecera `Cookie`.
pub async fn entrar(app: &Router, login: &str, contrasena: &str) -> String {
    let (codigo, _, set_cookie) = peticion(
        app,
        "POST",
        "/api/sesion",
        None,
        Some(json!({ "login": login, "contrasena": contrasena })),
    )
    .await;
    assert_eq!(codigo, StatusCode::NO_CONTENT, "no se pudo entrar como {login}");
    let set_cookie = set_cookie.expect("POST /api/sesion devuelve Set-Cookie");
    set_cookie.split(';').next().unwrap().to_string()
}
