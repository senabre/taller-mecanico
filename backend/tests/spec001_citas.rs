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

/// `POST /api/publico/citas` de un vehículo ya registrado (sin `vehiculo` ni `cliente`).
fn cita_conocida(matricula: &str, fecha: NaiveDate) -> Value {
    json!({ "matricula": matricula, "fecha": fecha.to_string(), "motivo": "Revisión" })
}

async fn pedir(app: &axum::Router, cuerpo: Value) -> (StatusCode, Value) {
    let (codigo, cuerpo, _) =
        comun::peticion(app, "POST", "/api/publico/citas", None, Some(cuerpo)).await;
    (codigo, cuerpo)
}

async fn vehiculo_publico(app: &axum::Router, matricula: &str) -> (StatusCode, Value) {
    let ruta = format!("/api/publico/vehiculos/{matricula}");
    let (codigo, cuerpo, _) = comun::peticion(app, "GET", &ruta, None, None).await;
    (codigo, cuerpo)
}

const CITA_PENDIENTE: &str = "Este vehículo ya tiene una cita pendiente.";

// ---------------------------------------------------------------------------------------------
// US1 · Cliente con vehículo ya conocido
// ---------------------------------------------------------------------------------------------

/// Prueba 2 (FR-002, FR-003, FR-008; US1-1, US1-2).
#[sqlx::test]
async fn prueba_2_vehiculo_conocido_se_reconoce(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    insertar_vehiculo(&pool, "1234BCD", "Seat", "Ibiza", "Ana Pérez", "600000001", "ana@ejemplo.test")
        .await;

    let (codigo, cuerpo) = vehiculo_publico(&app, "1234-bcd").await;
    assert_eq!(codigo, StatusCode::OK, "{cuerpo}");
    assert_eq!(
        cuerpo,
        json!({ "matricula": "1234BCD", "marca": "Seat", "modelo": "Ibiza", "cita_pendiente": false })
    );
    for clave in ["nombre", "telefono", "email", "cliente"] {
        assert!(cuerpo.get(clave).is_none(), "el GET público no puede devolver {clave}");
    }

    let fecha = laborable(1);
    let (codigo, cuerpo) = pedir(&app, json!({ "matricula": "1234 bcd", "fecha": fecha.to_string(), "motivo": "  Ruido al frenar  " })).await;
    assert_eq!(codigo, StatusCode::CREATED, "{cuerpo}");
    assert_eq!(
        cuerpo,
        json!({ "fecha": fecha.to_string(), "motivo": "Ruido al frenar", "matricula": "1234BCD" })
    );
    assert_eq!(contar(&pool, "vehiculo").await, 1);
    assert_eq!(contar(&pool, "cita").await, 1);
}

/// Prueba 3 (FR-002; US1-5).
#[sqlx::test]
async fn prueba_3_matricula_invalida(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    // «12#4» va codificada en la URL: «#» empezaría el fragmento.
    for (en_url, escrita) in [("12%234", "12#4"), ("A", "A"), ("ABCDE123456", "ABCDE123456")] {
        let (codigo, cuerpo) = vehiculo_publico(&app, en_url).await;
        assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY, "GET {escrita}: {cuerpo}");
        assert_eq!(cuerpo["error"], "La matrícula no es válida", "GET {escrita}");

        let (codigo, cuerpo) = pedir(&app, cita_nueva(escrita, laborable(1))).await;
        assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY, "POST {escrita}: {cuerpo}");
        assert_eq!(cuerpo["error"], "La matrícula no es válida", "POST {escrita}");
        assert_eq!(cuerpo["detalle"]["campo"], "matricula", "POST {escrita}");
    }
    for tabla in ["cliente", "vehiculo", "cita"] {
        assert_eq!(contar(&pool, tabla).await, 0, "{tabla}");
    }

    // «AB» es válida: como no existe, 404 (y el formulario pediría los datos del vehículo).
    let (codigo, cuerpo) = vehiculo_publico(&app, "AB").await;
    assert_eq!(codigo, StatusCode::NOT_FOUND, "{cuerpo}");
    assert_eq!(cuerpo["error"], "No hay ningún vehículo con esa matrícula");
}

/// Prueba 6 (FR-003; US1-7): con un vehículo conocido, el POST nunca toca vehículo ni cliente.
#[sqlx::test]
async fn prueba_6_vehiculo_conocido_no_cambia(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    insertar_vehiculo(&pool, "1234BCD", "Seat", "Ibiza", "Ana Pérez", "600000001", "ana@ejemplo.test")
        .await;

    let mut cuerpo = cita_conocida("1234BCD", laborable(1));
    cuerpo["vehiculo"] = json!({ "marca": "Otra", "modelo": "Otro" });
    cuerpo["cliente"] =
        json!({ "nombre": "Intruso", "telefono": "699999999", "email": "otro@ejemplo.test" });
    let (codigo, respuesta) = pedir(&app, cuerpo).await;
    assert_eq!(codigo, StatusCode::CREATED, "{respuesta}");

    assert_eq!(
        cliente_de(&pool, "1234BCD").await,
        ("Ana Pérez".into(), "600000001".into(), "ana@ejemplo.test".into())
    );
    let (_, vehiculo) = vehiculo_publico(&app, "1234BCD").await;
    assert_eq!((vehiculo["marca"].as_str(), vehiculo["modelo"].as_str()), (Some("Seat"), Some("Ibiza")));
    assert_eq!(contar(&pool, "cliente").await, 1);
    assert_eq!(contar(&pool, "vehiculo").await, 1);
}

/// Prueba 7 (FR-010; US1-4, US1-6).
#[sqlx::test]
async fn prueba_7_una_sola_cita_pendiente(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let hoy = fechas::hoy();

    // Con una cita para hoy o para un día posterior: aviso, sin fecha, y no se puede pedir otra.
    for (matricula, pendiente) in [("1111AAA", hoy), ("2222AAA", laborable(3))] {
        let id = insertar_vehiculo(&pool, matricula, "Seat", "Ibiza", "Ana", "600000001", "a@ejemplo.test").await;
        insertar_cita(&pool, id, pendiente, "Revisión").await;

        let (codigo, cuerpo) = vehiculo_publico(&app, matricula).await;
        assert_eq!(codigo, StatusCode::OK, "{cuerpo}");
        assert_eq!(cuerpo["cita_pendiente"], true, "{matricula}");
        assert!(cuerpo.get("fecha").is_none(), "el GET no puede devolver la fecha: {cuerpo}");

        let (codigo, cuerpo) = pedir(&app, cita_conocida(matricula, laborable(1))).await;
        assert_eq!(codigo, StatusCode::CONFLICT, "{matricula}: {cuerpo}");
        assert_eq!(cuerpo["error"], CITA_PENDIENTE);
    }

    // Si la única cita ya pasó, se puede pedir otra.
    let id = insertar_vehiculo(&pool, "3333AAA", "Seat", "Ibiza", "Ana", "600000001", "a@ejemplo.test").await;
    insertar_cita(&pool, id, hoy - Days::new(1), "Revisión").await;
    let (_, cuerpo) = vehiculo_publico(&app, "3333AAA").await;
    assert_eq!(cuerpo["cita_pendiente"], false);
    let (codigo, cuerpo) = pedir(&app, cita_conocida("3333AAA", laborable(1))).await;
    assert_eq!(codigo, StatusCode::CREATED, "{cuerpo}");
}

/// Prueba 10 (FR-010, research D3): misma matrícula para dos días distintos a la vez. El
/// bloqueo por fecha no las serializa; el bloqueo por matrícula sí.
#[sqlx::test]
async fn prueba_10_concurrencia_cita_pendiente(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let id = insertar_vehiculo(&pool, "1234BCD", "Seat", "Ibiza", "Ana", "600000001", "a@ejemplo.test").await;

    for vuelta in 0..20 {
        let (a, b) = tokio::join!(
            pedir(&app, cita_conocida("1234BCD", laborable(1))),
            pedir(&app, cita_conocida("1234BCD", laborable(2))),
        );
        let mut codigos = [a.0, b.0];
        codigos.sort();
        assert_eq!(codigos, [StatusCode::CREATED, StatusCode::CONFLICT], "vuelta {vuelta}: {a:?} {b:?}");
        let citas: i64 = sqlx::query_scalar("SELECT count(*) FROM cita WHERE vehiculo_id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(citas, 1, "vuelta {vuelta}");
        // Se borra para la vuelta siguiente (y para no llenar el cupo de esos días).
        sqlx::query("DELETE FROM cita").execute(&pool).await.unwrap();
    }
}

/// Prueba 12 (FR-009; US1-3, US1-8, US1-9, US1-10): lista de recepción.
#[sqlx::test]
async fn prueba_12_lista_de_citas(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let fecha = laborable(1);
    let ruta = format!("/api/citas?fecha={fecha}");

    // Sin sesión → 401; mecánico y almacén → 403.
    let (codigo, _, _) = comun::peticion(&app, "GET", &ruta, None, None).await;
    assert_eq!(codigo, StatusCode::UNAUTHORIZED);
    for (login, rol) in [("luisa", "mecanico"), ("pepa", "almacen")] {
        let (_, contrasena) = comun::crear_usuario(&pool, login, rol, true).await;
        let cookie = comun::entrar(&app, login, &contrasena).await;
        let (codigo, _, _) = comun::peticion(&app, "GET", &ruta, Some(&cookie), None).await;
        assert_eq!(codigo, StatusCode::FORBIDDEN, "{rol}");
    }

    // Vehículos por SQL (hallazgo I1); las citas, por POST, primero 1234BCD y después 5678CDF.
    insertar_vehiculo(&pool, "1234BCD", "Seat", "Ibiza", "Ana Pérez", "600000001", "ana@ejemplo.test").await;
    insertar_vehiculo(&pool, "5678CDF", "Kia", "Ceed", "Luis Gil", "600000002", "luis@ejemplo.test").await;
    for matricula in ["1234BCD", "5678CDF"] {
        let (codigo, cuerpo) = pedir(&app, cita_conocida(matricula, fecha)).await;
        assert_eq!(codigo, StatusCode::CREATED, "{matricula}: {cuerpo}");
    }
    // Una cita de hoy (por SQL: el POST no admite hoy) para la lista sin `fecha`.
    let hoy_id = insertar_vehiculo(&pool, "9999ZZZ", "Opel", "Corsa", "Eva Ruiz", "600000003", "eva@ejemplo.test").await;
    insertar_cita(&pool, hoy_id, fechas::hoy(), "Cambio de aceite").await;

    for (login, rol) in [("anabel", "recepcion"), ("paca", "jefe")] {
        let (_, contrasena) = comun::crear_usuario(&pool, login, rol, true).await;
        let cookie = comun::entrar(&app, login, &contrasena).await;

        let (codigo, lista, _) = comun::peticion(&app, "GET", &ruta, Some(&cookie), None).await;
        assert_eq!(codigo, StatusCode::OK, "{rol}: {lista}");
        let lista = lista.as_array().unwrap();
        let matriculas: Vec<&str> = lista.iter().map(|c| c["matricula"].as_str().unwrap()).collect();
        assert_eq!(matriculas, ["1234BCD", "5678CDF"], "{rol}");
        assert_eq!(lista[0]["marca"], "Seat");
        assert_eq!(lista[0]["modelo"], "Ibiza");
        assert_eq!(lista[0]["motivo"], "Revisión");
        assert_eq!(lista[0]["cliente"], json!({ "nombre": "Ana Pérez", "telefono": "600000001" }));
        assert_eq!(lista[1]["cliente"], json!({ "nombre": "Luis Gil", "telefono": "600000002" }));
        assert!(lista[0]["id"].is_i64());
        assert_eq!(lista[0]["orden_id"], Value::Null);
        assert!(!lista.iter().any(|c| c.to_string().contains("email")), "la lista no puede llevar el correo");

        // Sin `fecha` → las de hoy.
        let (_, de_hoy, _) = comun::peticion(&app, "GET", "/api/citas", Some(&cookie), None).await;
        let de_hoy = de_hoy.as_array().unwrap();
        assert_eq!(de_hoy.len(), 1, "{rol}");
        assert_eq!(de_hoy[0]["matricula"], "9999ZZZ");

        // Un día sin citas → [].
        let vacio = format!("/api/citas?fecha={}", laborable(5));
        let (codigo, nada, _) = comun::peticion(&app, "GET", &vacio, Some(&cookie), None).await;
        assert_eq!(codigo, StatusCode::OK);
        assert_eq!(nada, json!([]));
    }
}
