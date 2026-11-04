//! Abrir la orden de reparación (spec-002; plan, «Pruebas»; contrato, sección «spec-002 · Orden de
//! reparación»). Cada prueba crea sus propios datos; ninguna usa valores demo (constitución,
//! principio V). Las citas de hoy usan `fechas::hoy()` (Europe/Madrid, R12).

mod comun;

use axum::http::StatusCode;
use chrono::{Days, NaiveDate};
use serde_json::{Value, json};
use sqlx::PgPool;
use taller::estados::{CambioEstado, Estado};
use taller::fechas;

// ---------------------------------------------------------------------------------------------
// Utilidades propias de spec-002 (el resto, en `comun`).
// ---------------------------------------------------------------------------------------------

/// Cliente, vehículo y cita por SQL. Devuelve `(cita_id, vehiculo_id)`.
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
async fn mecanico(pool: &PgPool, login: &str, activo: bool) -> i64 {
    comun::crear_usuario(pool, login, "mecanico", activo).await.0
}

/// Una petición válida para `POST /api/ordenes`.
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
async fn contador(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT ultimo FROM contador WHERE nombre = 'orden'")
        .fetch_one(pool)
        .await
        .unwrap()
}

/// Número de órdenes en la base de datos.
async fn ordenes(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM orden")
        .fetch_one(pool)
        .await
        .unwrap()
}

/// Una recepcionista con sesión: devuelve la cookie.
async fn recepcion(app: &axum::Router, pool: &PgPool, login: &str) -> String {
    let (_, contrasena) = comun::crear_usuario(pool, login, "recepcion", true).await;
    comun::entrar(app, login, &contrasena).await
}

/// `POST /api/ordenes` con la cookie de `quien`.
async fn abrir(app: &axum::Router, quien: &str, cuerpo: Value) -> (StatusCode, Value) {
    let (codigo, cuerpo, _) =
        comun::peticion(app, "POST", "/api/ordenes", Some(quien), Some(cuerpo)).await;
    (codigo, cuerpo)
}

async fn get(app: &axum::Router, quien: Option<&str>, ruta: &str) -> (StatusCode, Value) {
    let (codigo, cuerpo, _) = comun::peticion(app, "GET", ruta, quien, None).await;
    (codigo, cuerpo)
}

const YA_TIENE_ORDEN: &str = "Esta cita ya tiene una orden";
const OTRO_DIA: &str = "Solo se puede abrir la orden el día de la cita";
const KM: &str = "El kilometraje debe ser un número entero igual o mayor que 0";
const DANOS: &str = "Describe los daños visibles o escribe “Sin daños visibles”";
const MECANICO: &str = "Elige un mecánico de la lista";
const FECHA_FALTA: &str = "Indica la fecha de entrega prometida";
const FECHA_PASADA: &str = "La fecha de entrega prometida no puede ser anterior a hoy";

fn hoy() -> NaiveDate {
    fechas::hoy()
}

// ---------------------------------------------------------------------------------------------
// US1 · Abrir la orden al recibir el coche
// ---------------------------------------------------------------------------------------------

/// Prueba 9 (FR-001): acceso y cita inexistente.
#[sqlx::test]
async fn prueba_9_acceso_y_cita_inexistente(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let luis = mecanico(&pool, "luis", true).await;
    let (cita_id, _) = cita(&pool, "1000AAA", hoy()).await;

    let (codigo, _) = {
        let (c, b, _) =
            comun::peticion(&app, "POST", "/api/ordenes", None, Some(peticion(cita_id, luis))).await;
        (c, b)
    };
    assert_eq!(codigo, StatusCode::UNAUTHORIZED);
    for (login, rol) in [("marta", "mecanico"), ("pepe", "almacen")] {
        let (_, contrasena) = comun::crear_usuario(&pool, login, rol, true).await;
        let cookie = comun::entrar(&app, login, &contrasena).await;
        let (codigo, cuerpo) = abrir(&app, &cookie, peticion(cita_id, luis)).await;
        assert_eq!(codigo, StatusCode::FORBIDDEN, "{rol}: {cuerpo}");
    }
    assert_eq!(ordenes(&pool).await, 0);

    let ana = recepcion(&app, &pool, "ana").await;
    let (codigo, cuerpo) = abrir(&app, &ana, peticion(cita_id, luis)).await;
    assert_eq!(codigo, StatusCode::CREATED, "recepción: {cuerpo}");
    let (_, contrasena) = comun::crear_usuario(&pool, "paco", "jefe", true).await;
    let paco = comun::entrar(&app, "paco", &contrasena).await;
    let (otra, _) = cita(&pool, "1001AAA", hoy()).await;
    let (codigo, cuerpo) = abrir(&app, &paco, peticion(otra, luis)).await;
    assert_eq!(codigo, StatusCode::CREATED, "jefe: {cuerpo}");

    let (codigo, cuerpo) = abrir(&app, &ana, peticion(999_999, luis)).await;
    assert_eq!(codigo, StatusCode::NOT_FOUND, "{cuerpo}");
    assert_eq!(cuerpo["error"], "No existe la cita");
}

/// Prueba 1 (FR-005, FR-006, FR-007, US1-1; research D5): abrir y publicar el evento.
#[sqlx::test]
async fn prueba_1_abrir_y_publicar_evento(pool: PgPool) {
    let estado = comun::estado_prueba(pool.clone());
    let mut tablero = estado.eventos.suscribir();
    let app = taller::app(estado);
    let luis = mecanico(&pool, "luis", true).await;
    let ana = recepcion(&app, &pool, "ana").await;
    let (cita_id, vehiculo_id) = cita(&pool, "1234BCD", hoy()).await;
    let antes = contador(&pool).await;

    // Un 422 no publica nada.
    let mut mala = peticion(cita_id, luis);
    mala["km"] = json!(-1);
    let (codigo, _) = abrir(&app, &ana, mala).await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(tablero.try_recv().is_err(), "un 422 no debe publicar ningún evento");

    let (codigo, orden) = abrir(&app, &ana, peticion(cita_id, luis)).await;
    assert_eq!(codigo, StatusCode::CREATED, "{orden}");
    assert_eq!(orden["estado"], "recibida");
    assert_eq!(orden["numero"], antes + 1);
    assert_eq!(orden["km"], 85000);
    assert_eq!(orden["danos"], "Rayón puerta trasera izquierda");
    assert_eq!(orden["mecanico"], json!({ "id": luis, "nombre": "Luis" }));
    assert_eq!(orden["fecha_entrega_prometida"], (hoy() + Days::new(3)).to_string());
    assert_eq!(orden["matricula"], "1234BCD");
    assert_eq!(orden["vehiculo"], json!({ "marca": "Seat", "modelo": "Ibiza" }));

    let id = orden["id"].as_i64().unwrap();
    let guardado: i64 = sqlx::query_scalar("SELECT vehiculo_id FROM orden WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(guardado, vehiculo_id, "el vehículo es el de la cita");

    let evento = tablero.try_recv().expect("abrir la orden publica su evento");
    assert_eq!(evento, CambioEstado { id, estado: Estado::Recibida });
}

/// Prueba 2 (FR-002, US1-2, US1-9, SC-002): ningún intento rechazado gasta número.
#[sqlx::test]
async fn prueba_2_correlativo_sin_huecos(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let luis = mecanico(&pool, "luis", true).await;
    let pedro = mecanico(&pool, "pedro", false).await;
    let ana = recepcion(&app, &pool, "ana").await;
    sqlx::query("UPDATE contador SET ultimo = 41 WHERE nombre = 'orden'")
        .execute(&pool)
        .await
        .unwrap();

    let (primera, _) = cita(&pool, "1000AAA", hoy()).await;
    let (segunda, _) = cita(&pool, "2000AAA", hoy()).await;
    let (manana, _) = cita(&pool, "3000AAA", hoy() + Days::new(1)).await;
    let (con_orden, v) = cita(&pool, "4000AAA", hoy()).await;
    orden_previa(&pool, con_orden, v, luis, 1000, 1000).await;

    let rechazos: Vec<(&str, Value)> = vec![
        ("km", json!(-1)),
        ("danos", json!("")),
        ("mecanico_id", json!(pedro)),
        ("fecha_entrega_prometida", json!((hoy() - Days::new(1)).to_string())),
    ];
    for (campo, valor) in rechazos {
        let mut cuerpo = peticion(primera, luis);
        cuerpo[campo] = valor;
        let (codigo, respuesta) = abrir(&app, &ana, cuerpo).await;
        assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY, "{campo}: {respuesta}");
    }
    let (codigo, _) = abrir(&app, &ana, peticion(manana, luis)).await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    let (codigo, _) = abrir(&app, &ana, peticion(con_orden, luis)).await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(contador(&pool).await, 41, "ningún rechazo gasta número");

    let (codigo, orden) = abrir(&app, &ana, peticion(primera, luis)).await;
    assert_eq!(codigo, StatusCode::CREATED, "{orden}");
    assert_eq!(orden["numero"], 42);

    // Escenario 9: la segunda cita, en dos pestañas.
    let (codigo, orden) = abrir(&app, &ana, peticion(segunda, luis)).await;
    assert_eq!(codigo, StatusCode::CREATED, "{orden}");
    assert_eq!(orden["numero"], 43);
    let (codigo, _) = abrir(&app, &ana, peticion(segunda, luis)).await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(contador(&pool).await, 43);
}

/// Prueba 3 (FR-002): diez aperturas a la vez, números consecutivos.
#[sqlx::test]
async fn prueba_3_concurrencia_del_numero(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let luis = mecanico(&pool, "luis", true).await;
    let ana = recepcion(&app, &pool, "ana").await;
    let antes = contador(&pool).await;
    let mut citas = Vec::new();
    for n in 0..10 {
        citas.push(cita(&pool, &format!("{:04}CCC", 5000 + n), hoy()).await.0);
    }

    let respuestas = futures_util::future::join_all(
        citas.iter().map(|&c| abrir(&app, &ana, peticion(c, luis))),
    )
    .await;
    let mut numeros: Vec<i64> = respuestas
        .iter()
        .map(|(codigo, orden)| {
            assert_eq!(*codigo, StatusCode::CREATED, "{orden}");
            orden["numero"].as_i64().unwrap()
        })
        .collect();
    numeros.sort();
    assert_eq!(numeros, ((antes + 1)..=(antes + 10)).collect::<Vec<_>>());
    assert_eq!(contador(&pool).await, antes + 10);
}

/// Prueba 4 (FR-008, FR-002, US1-4; research D2): dos aperturas de la misma cita a la vez.
#[sqlx::test]
async fn prueba_4_misma_cita_a_la_vez(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let luis = mecanico(&pool, "luis", true).await;
    let ana = recepcion(&app, &pool, "ana").await;

    for vuelta in 0..20 {
        let (cita_id, _) = cita(&pool, &format!("{:04}DDD", 6000 + vuelta), hoy()).await;
        let antes = contador(&pool).await;
        let (a, b) = tokio::join!(
            abrir(&app, &ana, peticion(cita_id, luis)),
            abrir(&app, &ana, peticion(cita_id, luis)),
        );
        let mut codigos = [a.0, b.0];
        codigos.sort();
        assert_eq!(codigos, [StatusCode::CREATED, StatusCode::CONFLICT], "vuelta {vuelta}: {a:?} {b:?}");
        let (creada, rechazo) = if a.0 == StatusCode::CREATED { (&a.1, &b.1) } else { (&b.1, &a.1) };
        let numero = creada["numero"].as_i64().unwrap();
        assert_eq!(rechazo["error"], format!("{YA_TIENE_ORDEN} (Nº {numero})"), "vuelta {vuelta}");
        assert_eq!(rechazo["detalle"]["orden_id"], creada["id"], "vuelta {vuelta}");
        assert_eq!(rechazo["detalle"]["numero"], numero, "vuelta {vuelta}");
        let de_la_cita: i64 = sqlx::query_scalar("SELECT count(*) FROM orden WHERE cita_id = $1")
            .bind(cita_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(de_la_cita, 1, "vuelta {vuelta}");
        assert_eq!(contador(&pool).await, antes + 1, "vuelta {vuelta}");
    }
}

/// Prueba 5 (FR-001, US1-5; Q2): solo el día de la cita.
#[sqlx::test]
async fn prueba_5_solo_el_dia_de_la_cita(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let luis = mecanico(&pool, "luis", true).await;
    let ana = recepcion(&app, &pool, "ana").await;
    let (manana, _) = cita(&pool, "1000AAA", hoy() + Days::new(1)).await;
    let (ayer, _) = cita(&pool, "2000AAA", hoy() - Days::new(1)).await;

    for cita_id in [manana, ayer] {
        let (codigo, cuerpo) = abrir(&app, &ana, peticion(cita_id, luis)).await;
        assert_eq!(codigo, StatusCode::CONFLICT, "{cuerpo}");
        assert_eq!(cuerpo["error"], OTRO_DIA);
    }
    assert_eq!(ordenes(&pool).await, 0);
    assert_eq!(contador(&pool).await, 0);

    // Todos los 422 antes que cualquier 409.
    let mut cuerpo = peticion(manana, luis);
    cuerpo["km"] = json!(-1);
    let (codigo, respuesta) = abrir(&app, &ana, cuerpo).await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY, "{respuesta}");
    assert_eq!(respuesta["error"], KM);
}

/// Prueba 7 (FR-003, FR-006, US1-8; Q4; contrato, tabla de validación).
#[sqlx::test]
async fn prueba_7_campos(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let luis = mecanico(&pool, "luis", true).await;
    let ana = recepcion(&app, &pool, "ana").await;
    let (cita_id, _) = cita(&pool, "1000AAA", hoy()).await;
    let ayer = (hoy() - Days::new(1)).to_string();

    let casos: Vec<(&str, Option<Value>, &str, &str)> = vec![
        ("km", None, KM, "km"),
        ("km", Some(Value::Null), KM, "km"),
        ("km", Some(json!(-1)), KM, "km"),
        ("km", Some(json!(1.5)), KM, "km"),
        ("km", Some(json!("")), KM, "km"),
        ("km", Some(json!("abc")), KM, "km"),
        ("danos", None, DANOS, "danos"),
        ("danos", Some(json!("")), DANOS, "danos"),
        ("danos", Some(json!("   ")), DANOS, "danos"),
        ("mecanico_id", None, MECANICO, "mecanico_id"),
        ("fecha_entrega_prometida", None, FECHA_FALTA, "fecha_entrega_prometida"),
        ("fecha_entrega_prometida", Some(json!("ayer")), FECHA_FALTA, "fecha_entrega_prometida"),
        ("fecha_entrega_prometida", Some(json!(ayer)), FECHA_PASADA, "fecha_entrega_prometida"),
    ];
    for (campo, valor, mensaje, detalle) in casos {
        let mut cuerpo = peticion(cita_id, luis);
        match valor {
            Some(v) => cuerpo[campo] = v,
            None => {
                cuerpo.as_object_mut().unwrap().remove(campo);
            }
        }
        let (codigo, respuesta) = abrir(&app, &ana, cuerpo).await;
        assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY, "{campo}: {respuesta}");
        assert_eq!(respuesta["error"], mensaje, "{campo}");
        assert_eq!(respuesta["detalle"]["campo"], detalle, "{campo}");
    }
    // Varios errores: el primero según el orden de la tabla.
    let mut cuerpo = peticion(cita_id, luis);
    cuerpo["km"] = json!(-1);
    cuerpo["danos"] = json!("");
    let (_, respuesta) = abrir(&app, &ana, cuerpo).await;
    assert_eq!(respuesta["error"], KM);
    // Sin cita_id → 404.
    let mut cuerpo = peticion(cita_id, luis);
    cuerpo.as_object_mut().unwrap().remove("cita_id");
    let (codigo, respuesta) = abrir(&app, &ana, cuerpo).await;
    assert_eq!(codigo, StatusCode::NOT_FOUND, "{respuesta}");
    assert_eq!(respuesta["error"], "No existe la cita");
    assert_eq!(ordenes(&pool).await, 0);
    assert_eq!(contador(&pool).await, 0);

    // Válidos: km como texto, entrega hoy, daños con espacios, «Sin daños visibles».
    let validos: Vec<(&str, Value, Value)> = vec![
        ("km", json!("85000"), json!(85000)),
        ("fecha_entrega_prometida", json!(hoy().to_string()), json!(hoy().to_string())),
        ("danos", json!("  Rayón  "), json!("Rayón")),
        ("danos", json!("Sin daños visibles"), json!("Sin daños visibles")),
    ];
    for (n, (campo, valor, guardado)) in validos.into_iter().enumerate() {
        let (otra, _) = cita(&pool, &format!("{:04}EEE", 7000 + n), hoy()).await;
        let mut cuerpo = peticion(otra, luis);
        cuerpo[campo] = valor;
        let (codigo, orden) = abrir(&app, &ana, cuerpo).await;
        assert_eq!(codigo, StatusCode::CREATED, "{campo}: {orden}");
        assert_eq!(orden[campo], guardado, "{campo}");
    }
}

/// Prueba 8 (FR-005, US1-7; Q3): solo mecánicos activos.
#[sqlx::test]
async fn prueba_8_mecanico(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let luis = mecanico(&pool, "luis", true).await;
    let pedro = mecanico(&pool, "pedro", false).await;
    let (bea, _) = comun::crear_usuario(&pool, "bea", "recepcion", true).await;
    let (paco, _) = comun::crear_usuario(&pool, "paco", "jefe", true).await;
    let ana = recepcion(&app, &pool, "ana").await;
    let (cita_id, _) = cita(&pool, "1000AAA", hoy()).await;

    for (quien, id) in [("de baja", pedro), ("recepción", bea), ("jefe", paco), ("inexistente", 999_999)] {
        let (codigo, cuerpo) = abrir(&app, &ana, peticion(cita_id, id)).await;
        assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY, "{quien}: {cuerpo}");
        assert_eq!(cuerpo["error"], MECANICO, "{quien}");
        assert_eq!(cuerpo["detalle"]["campo"], "mecanico_id", "{quien}");
    }
    let (codigo, cuerpo) = abrir(&app, &ana, peticion(cita_id, luis)).await;
    assert_eq!(codigo, StatusCode::CREATED, "{cuerpo}");
}

/// Prueba 10 (FR-005, US1-3, SC-004): la ficha de la orden.
#[sqlx::test]
async fn prueba_10_ficha(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let luis = mecanico(&pool, "luis", true).await;
    let ana = recepcion(&app, &pool, "ana").await;
    let (cita_id, _) = cita(&pool, "1234BCD", hoy()).await;
    let (_, orden) = abrir(&app, &ana, peticion(cita_id, luis)).await;
    let id = orden["id"].as_i64().unwrap();
    let ruta = format!("/api/ordenes/{id}");

    let esperado = json!({
        "id": id,
        "numero": orden["numero"],
        "matricula": "1234BCD",
        "estado": "recibida",
        "mecanico": { "id": luis, "nombre": "Luis" },
        "fecha_entrega_prometida": (hoy() + Days::new(3)).to_string(),
        "retrasada": false,
        "cita": { "fecha": hoy().to_string(), "motivo": "Revisión" },
        "vehiculo": { "marca": "Seat", "modelo": "Ibiza" },
        "km": 85000,
        "danos": "Rayón puerta trasera izquierda",
        "diagnostico": null,
        "total_presupuestado": "0.00",
        "coste_acumulado": "0.00",
        "presupuestos": [],
        "imputaciones": [],
        "pedidos": [],
        "factura_id": null,
    });
    assert_eq!(orden, esperado, "la respuesta del POST es la ficha");

    let mut cookies = vec![ana.clone()];
    for (login, rol) in [("marta", "mecanico"), ("pepe", "almacen"), ("paco", "jefe")] {
        let (_, contrasena) = comun::crear_usuario(&pool, login, rol, true).await;
        cookies.push(comun::entrar(&app, login, &contrasena).await);
    }
    for cookie in &cookies {
        let (codigo, ficha) = get(&app, Some(cookie), &ruta).await;
        assert_eq!(codigo, StatusCode::OK, "{ficha}");
        assert_eq!(ficha, esperado);
    }
    let (codigo, _) = get(&app, None, &ruta).await;
    assert_eq!(codigo, StatusCode::UNAUTHORIZED);
    let (codigo, cuerpo) = get(&app, Some(&ana), "/api/ordenes/999999").await;
    assert_eq!(codigo, StatusCode::NOT_FOUND, "{cuerpo}");
    assert_eq!(cuerpo["error"], "No existe la orden");
}

/// Prueba 11 (FR-008, US1-4; Q5): la lista de citas trae el número de la orden.
#[sqlx::test]
async fn prueba_11_lista_de_citas(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let luis = mecanico(&pool, "luis", true).await;
    let ana = recepcion(&app, &pool, "ana").await;
    let (con_orden, _) = cita(&pool, "1000AAA", hoy()).await;
    cita(&pool, "2000AAA", hoy()).await;
    let (codigo, orden) = abrir(&app, &ana, peticion(con_orden, luis)).await;
    assert_eq!(codigo, StatusCode::CREATED, "{orden}");

    let (codigo, lista) = get(&app, Some(&ana), &format!("/api/citas?fecha={}", hoy())).await;
    assert_eq!(codigo, StatusCode::OK, "{lista}");
    let lista = lista.as_array().unwrap();
    let de = |m: &str| lista.iter().find(|c| c["matricula"] == m).unwrap().clone();
    assert_eq!(de("1000AAA")["orden_id"], orden["id"]);
    assert!(de("1000AAA")["orden_numero"].is_i64(), "falta orden_numero: {:?}", de("1000AAA"));
    assert_eq!(de("1000AAA")["orden_numero"], orden["numero"]);
    assert_eq!(de("2000AAA")["orden_id"], Value::Null);
    assert!(de("2000AAA").get("orden_numero").is_some_and(Value::is_null), "orden_numero debe ser null");
}

/// Prueba 12 (menú común y quickstart §3; hallazgo E1): la lista de órdenes.
#[sqlx::test]
async fn prueba_12_lista_de_ordenes(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let (luis, clave_luis) = comun::crear_usuario(&pool, "luis", "mecanico", true).await;
    let marta = mecanico(&pool, "marta", true).await;
    let ana = recepcion(&app, &pool, "ana").await;
    let mut ids = Vec::new();
    for (n, quien) in [luis, marta, luis].into_iter().enumerate() {
        let (c, _) = cita(&pool, &format!("{:04}FFF", 8000 + n), hoy()).await;
        let (_, orden) = abrir(&app, &ana, peticion(c, quien)).await;
        ids.push(orden["id"].as_i64().unwrap());
    }
    // La primera, con la entrega de ayer: retrasada.
    sqlx::query("UPDATE orden SET fecha_entrega_prometida = $2 WHERE id = $1")
        .bind(ids[0])
        .bind(hoy() - Days::new(1))
        .execute(&pool)
        .await
        .unwrap();

    let (codigo, lista) = get(&app, Some(&ana), "/api/ordenes").await;
    assert_eq!(codigo, StatusCode::OK, "{lista}");
    let orden_ids: Vec<i64> = lista.as_array().unwrap().iter().map(|o| o["id"].as_i64().unwrap()).collect();
    assert_eq!(orden_ids, [ids[2], ids[1], ids[0]], "de la más reciente a la más antigua");
    let primera = &lista[2];
    for clave in ["id", "numero", "matricula", "estado", "mecanico", "fecha_entrega_prometida", "retrasada"] {
        assert!(primera.get(clave).is_some(), "falta {clave}: {primera}");
    }
    assert_eq!(primera["retrasada"], true);
    assert_eq!(lista[0]["retrasada"], false);

    let cookie_luis = comun::entrar(&app, "luis", &clave_luis).await;
    let (codigo, mias) = get(&app, Some(&cookie_luis), "/api/ordenes?mias=true").await;
    assert_eq!(codigo, StatusCode::OK, "{mias}");
    let mias: Vec<i64> = mias.as_array().unwrap().iter().map(|o| o["id"].as_i64().unwrap()).collect();
    assert_eq!(mias, [ids[2], ids[0]]);

    let (_, contrasena) = comun::crear_usuario(&pool, "pepe", "almacen", true).await;
    let pepe = comun::entrar(&app, "pepe", &contrasena).await;
    let (codigo, _) = get(&app, Some(&pepe), "/api/ordenes").await;
    assert_eq!(codigo, StatusCode::FORBIDDEN);
}
