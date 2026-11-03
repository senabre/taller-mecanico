//! Abrir la orden de reparación (spec-002; plan, «Pruebas»; contrato, sección «spec-002 · Orden de
//! reparación»). Cada prueba crea sus propios datos; ninguna usa valores demo (constitución,
//! principio V). Las citas de hoy usan `fechas::hoy()` (Europe/Madrid, R12).

mod comun;

use chrono::{Days, NaiveDate};
use serde_json::{Value, json};
use sqlx::PgPool;
use taller::fechas;

// ---------------------------------------------------------------------------------------------
// Utilidades propias de spec-002 (el resto, en `comun`). Las que aún no se usan llevan
// `#[allow(dead_code)]`, que se quita al usarlas en las pruebas de las fases 3 y 4.
// ---------------------------------------------------------------------------------------------

/// Cliente, vehículo y cita por SQL. Devuelve `(cita_id, vehiculo_id)`.
#[allow(dead_code)]
async fn cita(pool: &PgPool, matricula: &str, fecha: NaiveDate) -> (i64, i64) {
    let vehiculo_id: i64 = sqlx::query_scalar(
        "WITH nuevo AS (
             INSERT INTO cliente (nombre, telefono, email)
             VALUES ('Cliente de prueba', '600000000', 'cliente@ejemplo.test') RETURNING id
         )
         INSERT INTO vehiculo (matricula, marca, modelo, cliente_id)
         SELECT $1, 'Seat', 'Ibiza', id FROM nuevo RETURNING id",
    )
    .bind(matricula)
    .fetch_one(pool)
    .await
    .unwrap();
    let cita_id = sqlx::query_scalar(
        "INSERT INTO cita (vehiculo_id, fecha, motivo) VALUES ($1, $2, 'Revisión') RETURNING id",
    )
    .bind(vehiculo_id)
    .bind(fecha)
    .fetch_one(pool)
    .await
    .unwrap();
    (cita_id, vehiculo_id)
}

/// Una orden anterior por SQL (para el kilometraje). Las insertadas después tienen un
/// `abierta_en` posterior, porque cada consulta va en su propia transacción.
#[allow(dead_code)]
async fn orden_previa(
    pool: &PgPool,
    cita_id: i64,
    vehiculo_id: i64,
    mecanico_id: i64,
    km: i32,
    numero: i64,
) -> i64 {
    sqlx::query_scalar(
        "INSERT INTO orden (numero, cita_id, vehiculo_id, mecanico_id, km, danos,
                            fecha_entrega_prometida)
         VALUES ($1, $2, $3, $4, $5, 'Sin daños visibles', $6) RETURNING id",
    )
    .bind(numero)
    .bind(cita_id)
    .bind(vehiculo_id)
    .bind(mecanico_id)
    .bind(km)
    .bind(fechas::hoy())
    .fetch_one(pool)
    .await
    .unwrap()
}

/// Un usuario con rol `mecanico`, activo o de baja. Devuelve su id.
#[allow(dead_code)]
async fn mecanico(pool: &PgPool, login: &str, activo: bool) -> i64 {
    comun::crear_usuario(pool, login, "mecanico", activo).await.0
}

/// Una petición válida para `POST /api/ordenes`.
#[allow(dead_code)]
fn peticion(cita_id: i64, mecanico_id: i64) -> Value {
    json!({
        "cita_id": cita_id,
        "km": 85000,
        "danos": "Rayón puerta trasera izquierda",
        "mecanico_id": mecanico_id,
        "fecha_entrega_prometida": (fechas::hoy() + Days::new(3)).to_string(),
    })
}

/// El último número dado por `contador('orden')`.
#[allow(dead_code)]
async fn contador(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT ultimo FROM contador WHERE nombre = 'orden'")
        .fetch_one(pool)
        .await
        .unwrap()
}

/// Número de órdenes en la base de datos.
#[allow(dead_code)]
async fn ordenes(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM orden")
        .fetch_one(pool)
        .await
        .unwrap()
}
