//! Pedir cita en línea (spec-001; plan, «Pruebas»; contrato, sección «spec-001 · Citas»).
//! Cada prueba crea sus propios datos; ninguna usa valores demo (constitución, principio V).

mod comun;

use axum::http::StatusCode;
use chrono::{Datelike, Days, NaiveDate, Weekday};
use serde_json::{Value, json};
use sqlx::PgPool;
use taller::fechas;

// ---------------------------------------------------------------------------------------------
// Utilidades propias de spec-001 (el resto, en `comun`)
// ---------------------------------------------------------------------------------------------

/// Cliente y vehículo por SQL, sin pasar por el alta de US2 (hallazgo I1). Devuelve el id del
/// vehículo.
#[allow(dead_code)] // la usan las pruebas de las fases 3–5
async fn insertar_vehiculo(
    pool: &PgPool,
    matricula: &str,
    marca: &str,
    modelo: &str,
    nombre: &str,
    telefono: &str,
    email: &str,
) -> i64 {
    sqlx::query_scalar(
        "WITH nuevo AS (
             INSERT INTO cliente (nombre, telefono, email) VALUES ($4, $5, $6) RETURNING id
         )
         INSERT INTO vehiculo (matricula, marca, modelo, cliente_id)
         SELECT $1, $2, $3, id FROM nuevo RETURNING id",
    )
    .bind(matricula)
    .bind(marca)
    .bind(modelo)
    .bind(nombre)
    .bind(telefono)
    .bind(email)
    .fetch_one(pool)
    .await
    .unwrap()
}

#[allow(dead_code)] // la usan las pruebas de las fases 3–5
async fn insertar_cita(pool: &PgPool, vehiculo_id: i64, fecha: NaiveDate, motivo: &str) {
    sqlx::query("INSERT INTO cita (vehiculo_id, fecha, motivo) VALUES ($1, $2, $3)")
        .bind(vehiculo_id)
        .bind(fecha)
        .bind(motivo)
        .execute(pool)
        .await
        .unwrap();
}

fn es_laborable(dia: NaiveDate) -> bool {
    !matches!(dia.weekday(), Weekday::Sat | Weekday::Sun)
}

/// El n-ésimo día de lunes a viernes a partir de mañana (n = 1 es el primero).
#[allow(dead_code)] // la usan las pruebas de las fases 3–5
fn laborable(n: usize) -> NaiveDate {
    (1..)
        .map(|d| fechas::hoy() + Days::new(d))
        .filter(|dia| es_laborable(*dia))
        .nth(n - 1)
        .unwrap()
}

async fn contar(pool: &PgPool, tabla: &str) -> i64 {
    sqlx::query_scalar(&format!("SELECT count(*) FROM {tabla}"))
        .fetch_one(pool)
        .await
        .unwrap()
}

/// `(nombre, telefono, email)` del propietario de la matrícula.
#[allow(dead_code)] // la usan las pruebas de las fases 3–5
async fn cliente_de(pool: &PgPool, matricula: &str) -> (String, String, String) {
    sqlx::query_as(
        "SELECT c.nombre, c.telefono, c.email
           FROM cliente c JOIN vehiculo v ON v.cliente_id = c.id
          WHERE v.matricula = $1",
    )
    .bind(matricula)
    .fetch_one(pool)
    .await
    .unwrap()
}

/// Petición de cita con matrícula nueva y todos los datos válidos.
fn cita_nueva(matricula: &str, fecha: NaiveDate) -> Value {
    json!({
        "matricula": matricula,
        "fecha": fecha.to_string(),
        "motivo": "Ruido al frenar",
        "vehiculo": { "marca": "Seat", "modelo": "León" },
        "cliente": { "nombre": "Ana Pérez", "telefono": "612 345 678", "email": "ana@ejemplo.test" }
    })
}

// ---------------------------------------------------------------------------------------------
// Prueba 11 · Fechas que se pueden pedir (FR-006)
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn prueba_11_solo_dias_laborables_de_manana_a_30_dias(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let hoy = fechas::hoy();
    let sabado = (1..=7)
        .map(|d| hoy + Days::new(d))
        .find(|dia| dia.weekday() == Weekday::Sat)
        .unwrap();

    for fecha in [sabado, hoy, hoy + Days::new(31)] {
        let (codigo, cuerpo, _) = comun::peticion(
            &app,
            "POST",
            "/api/publico/citas",
            None,
            Some(cita_nueva("5678CDF", fecha)),
        )
        .await;
        assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY, "{fecha}: {cuerpo}");
        assert_eq!(cuerpo["error"], "Elige uno de los días disponibles", "{fecha}");
        assert_eq!(cuerpo["detalle"]["campo"], "fecha", "{fecha}");
    }
    assert_eq!(contar(&pool, "cita").await, 0);

    // Límite positivo (hallazgo A3): el último día de lunes a viernes que sea ≤ hoy + 30.
    let ultimo = (1..=30)
        .rev()
        .map(|d| hoy + Days::new(d))
        .find(|dia| es_laborable(*dia))
        .unwrap();
    let (codigo, dias, _) =
        comun::peticion(&app, "GET", "/api/publico/dias-disponibles", None, None).await;
    assert_eq!(codigo, StatusCode::OK);
    assert!(
        dias.as_array().unwrap().contains(&json!(ultimo.to_string())),
        "{ultimo} no está en {dias}"
    );
    let (_, cuerpo, _) = comun::peticion(
        &app,
        "POST",
        "/api/publico/citas",
        None,
        Some(cita_nueva("5678CDF", ultimo)),
    )
    .await;
    assert_ne!(cuerpo["detalle"]["campo"], "fecha", "{ultimo}: {cuerpo}");
}
