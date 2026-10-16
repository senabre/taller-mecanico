//! Entrar con usuario y contraseña, roles y sesión (spec-000 US1, FR-001…FR-003; research R8).

mod comun;

use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;
use taller::auth::{Rol, Usuario};
use taller::error::ErrorApi;

const INCORRECTOS: &str = "Usuario o contraseña incorrectos";

#[sqlx::test]
async fn entrar_da_204_y_una_cookie_de_sesion(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let (_, contrasena) = comun::crear_usuario(&pool, "luis", "mecanico", true).await;

    let (codigo, _, set_cookie) = comun::peticion(
        &app,
        "POST",
        "/api/sesion",
        None,
        Some(json!({ "login": "luis", "contrasena": contrasena })),
    )
    .await;

    assert_eq!(codigo, StatusCode::NO_CONTENT);
    let set_cookie = set_cookie.unwrap();
    assert!(set_cookie.starts_with("sesion="), "{set_cookie}");
    for atributo in ["HttpOnly", "SameSite=Lax", "Path=/"] {
        assert!(set_cookie.contains(atributo), "falta {atributo}: {set_cookie}");
    }
    assert!(!set_cookie.contains("Secure"), "con URL_PUBLICA http no lleva Secure");
}

#[sqlx::test]
async fn el_login_se_acepta_en_mayusculas(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let (_, contrasena) = comun::crear_usuario(&pool, "luis", "mecanico", true).await;

    comun::entrar(&app, "LUIS", &contrasena).await;
}

#[sqlx::test]
async fn credenciales_incorrectas_dan_siempre_el_mismo_401(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let (_, contrasena) = comun::crear_usuario(&pool, "luis", "mecanico", true).await;
    let (_, contrasena_inactivo) = comun::crear_usuario(&pool, "baja", "mecanico", false).await;

    for (login, clave) in [
        ("luis", "no-es-la-suya".to_string()),
        ("nadie", contrasena.clone()),
        ("baja", contrasena_inactivo),
    ] {
        let (codigo, cuerpo, set_cookie) = comun::peticion(
            &app,
            "POST",
            "/api/sesion",
            None,
            Some(json!({ "login": login, "contrasena": clave })),
        )
        .await;
        assert_eq!(codigo, StatusCode::UNAUTHORIZED, "{login}");
        assert_eq!(cuerpo, json!({ "error": INCORRECTOS }), "{login}");
        assert!(set_cookie.is_none(), "{login}");
    }
}

#[sqlx::test]
async fn yo_exige_sesion_y_no_devuelve_la_contrasena(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let (id, contrasena) = comun::crear_usuario(&pool, "luis", "mecanico", true).await;

    let (codigo, _, _) = comun::peticion(&app, "GET", "/api/yo", None, None).await;
    assert_eq!(codigo, StatusCode::UNAUTHORIZED);

    let cookie = comun::entrar(&app, "luis", &contrasena).await;
    let (codigo, cuerpo, _) = comun::peticion(&app, "GET", "/api/yo", Some(&cookie), None).await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(cuerpo, json!({ "id": id, "nombre": "Luis", "rol": "mecanico" }));
}

#[sqlx::test]
async fn una_sesion_caducada_da_401(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let (_, contrasena) = comun::crear_usuario(&pool, "luis", "mecanico", true).await;
    let cookie = comun::entrar(&app, "luis", &contrasena).await;

    sqlx::query("UPDATE sesion SET expira_en = now() - interval '1 minute'")
        .execute(&pool)
        .await
        .unwrap();

    let (codigo, _, _) = comun::peticion(&app, "GET", "/api/yo", Some(&cookie), None).await;
    assert_eq!(codigo, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn cada_peticion_renueva_la_sesion(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let (_, contrasena) = comun::crear_usuario(&pool, "luis", "mecanico", true).await;
    let cookie = comun::entrar(&app, "luis", &contrasena).await;
    sqlx::query("UPDATE sesion SET expira_en = now() + interval '1 minute'")
        .execute(&pool)
        .await
        .unwrap();

    comun::peticion(&app, "GET", "/api/yo", Some(&cookie), None).await;

    let renovada: bool =
        sqlx::query_scalar("SELECT expira_en > now() + interval '11 hours' FROM sesion")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(renovada);
}

#[sqlx::test]
async fn un_usuario_desactivado_pierde_la_sesion(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let (_, contrasena) = comun::crear_usuario(&pool, "luis", "mecanico", true).await;
    let cookie = comun::entrar(&app, "luis", &contrasena).await;

    sqlx::query("UPDATE usuario SET activo = false WHERE login = 'luis'")
        .execute(&pool)
        .await
        .unwrap();

    let (codigo, _, _) = comun::peticion(&app, "GET", "/api/yo", Some(&cookie), None).await;
    assert_eq!(codigo, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn salir_cierra_la_sesion(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let (_, contrasena) = comun::crear_usuario(&pool, "luis", "mecanico", true).await;
    let cookie = comun::entrar(&app, "luis", &contrasena).await;

    let (codigo, _, set_cookie) =
        comun::peticion(&app, "DELETE", "/api/sesion", Some(&cookie), None).await;
    assert_eq!(codigo, StatusCode::NO_CONTENT);
    assert!(set_cookie.unwrap().contains("Max-Age=0"));

    let (codigo, _, _) = comun::peticion(&app, "GET", "/api/yo", Some(&cookie), None).await;
    assert_eq!(codigo, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn mecanicos_activos_por_nombre(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let (marta, _) = comun::crear_usuario(&pool, "marta", "mecanico", true).await;
    let (luis, contrasena) = comun::crear_usuario(&pool, "luis", "mecanico", true).await;
    comun::crear_usuario(&pool, "baja", "mecanico", false).await;
    comun::crear_usuario(&pool, "ana", "recepcion", true).await;
    let cookie = comun::entrar(&app, "luis", &contrasena).await;

    let (codigo, cuerpo, _) =
        comun::peticion(&app, "GET", "/api/mecanicos", Some(&cookie), None).await;

    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(
        cuerpo,
        json!([{ "id": luis, "nombre": "Luis" }, { "id": marta, "nombre": "Marta" }])
    );
}

#[sqlx::test]
async fn eventos_exige_sesion(pool: PgPool) {
    let app = comun::app_prueba(pool);

    let (codigo, _, _) = comun::peticion(&app, "GET", "/api/eventos", None, None).await;
    assert_eq!(codigo, StatusCode::UNAUTHORIZED);
}

fn usuario(id: i64, rol: Rol) -> Usuario {
    Usuario {
        id,
        nombre: format!("Usuario {id}"),
        rol,
    }
}

#[test]
fn exigir_rol_deja_pasar_siempre_al_jefe() {
    let recepcion = &[Rol::Recepcion];
    assert!(usuario(1, Rol::Recepcion).exigir_rol(recepcion).is_ok());
    assert!(matches!(
        usuario(2, Rol::Mecanico).exigir_rol(recepcion),
        Err(ErrorApi::Prohibido)
    ));
    assert!(usuario(3, Rol::Jefe).exigir_rol(recepcion).is_ok());
}

#[test]
fn exigir_mecanico_de_solo_deja_al_asignado_y_al_jefe() {
    let id_de_luis = 10;
    let luis = usuario(id_de_luis, Rol::Mecanico);
    let marta = usuario(11, Rol::Mecanico);
    let paco = usuario(12, Rol::Jefe);

    assert!(luis.exigir_mecanico_de(id_de_luis).is_ok());
    assert!(matches!(
        marta.exigir_mecanico_de(id_de_luis),
        Err(ErrorApi::Prohibido)
    ));
    assert!(paco.exigir_mecanico_de(id_de_luis).is_ok());
}
