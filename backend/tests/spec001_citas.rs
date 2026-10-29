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

// ---------------------------------------------------------------------------------------------
// US2 · Cliente con vehículo nuevo
// ---------------------------------------------------------------------------------------------

/// Prueba 1 (FR-004; US2-1, US2-2): alta del cliente, del vehículo y de la cita.
#[sqlx::test]
async fn prueba_1_alta_completa(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let (codigo, _) = vehiculo_publico(&app, "5678CDF").await;
    assert_eq!(codigo, StatusCode::NOT_FOUND);

    let fecha = laborable(1);
    let mut cuerpo = cita_nueva("5678 cdf", fecha);
    cuerpo["vehiculo"] = json!({ "marca": "  Kia ", "modelo": " Ceed  " });
    cuerpo["cliente"] =
        json!({ "nombre": "  Luis Gil ", "telefono": " 612-345-678 ", "email": " luis@ejemplo.test  " });
    let (codigo, respuesta) = pedir(&app, cuerpo).await;
    assert_eq!(codigo, StatusCode::CREATED, "{respuesta}");
    assert_eq!(
        respuesta,
        json!({ "fecha": fecha.to_string(), "motivo": "Ruido al frenar", "matricula": "5678CDF" })
    );

    for tabla in ["cliente", "vehiculo", "cita"] {
        assert_eq!(contar(&pool, tabla).await, 1, "{tabla}");
    }
    // Tal como se escribieron, sin los espacios del principio y del final.
    assert_eq!(
        cliente_de(&pool, "5678CDF").await,
        ("Luis Gil".into(), "612-345-678".into(), "luis@ejemplo.test".into())
    );
    let (codigo, vehiculo) = vehiculo_publico(&app, "5678CDF").await;
    assert_eq!(codigo, StatusCode::OK, "{vehiculo}");
    assert_eq!(
        vehiculo,
        json!({ "matricula": "5678CDF", "marca": "Kia", "modelo": "Ceed", "cita_pendiente": true })
    );
}

/// Petición con matrícula nueva y el teléfono y el correo indicados.
fn con_contacto(matricula: &str, fecha: NaiveDate, telefono: &str, email: &str) -> Value {
    let mut cuerpo = cita_nueva(matricula, fecha);
    cuerpo["cliente"]["telefono"] = json!(telefono);
    cuerpo["cliente"]["email"] = json!(email);
    cuerpo
}

/// Prueba 4 (FR-004a; US2-3, US2-4): reglas del teléfono y del correo, y su lugar en el orden
/// de las comprobaciones.
#[sqlx::test]
async fn prueba_4_telefono_y_correo(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let fecha = laborable(1);
    const TELEFONO: &str = "El teléfono no es válido";
    const CORREO: &str = "El correo no es válido";

    let incorrectos = [
        ("12345", "ana@correo.es", TELEFONO, "cliente.telefono"),
        ("", "ana@correo.es", TELEFONO, "cliente.telefono"),
        ("61234567", "ana@correo.es", TELEFONO, "cliente.telefono"), // 8 cifras
        ("6123456789012345", "ana@correo.es", TELEFONO, "cliente.telefono"), // 16 cifras
        ("6123+45678", "ana@correo.es", TELEFONO, "cliente.telefono"),
        ("612345678", "ana@correo", CORREO, "cliente.email"),
        ("612345678", "", CORREO, "cliente.email"),
        ("612345678", "a@@b.es", CORREO, "cliente.email"),
        ("612345678", "@b.es", CORREO, "cliente.email"),
        ("612345678", "a@.es", CORREO, "cliente.email"),
        ("612345678", "a@b.", CORREO, "cliente.email"),
    ];
    for (telefono, email, mensaje, campo) in incorrectos {
        let (codigo, cuerpo) = pedir(&app, con_contacto("5678CDF", fecha, telefono, email)).await;
        assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY, "{telefono:?} {email:?}: {cuerpo}");
        assert_eq!(cuerpo["error"], mensaje, "{telefono:?} {email:?}");
        assert_eq!(cuerpo["detalle"]["campo"], campo, "{telefono:?} {email:?}");
    }
    for tabla in ["cliente", "vehiculo", "cita"] {
        assert_eq!(contar(&pool, tabla).await, 0, "{tabla}");
    }

    // Válidos: cada uno con su matrícula y su día, para no llenar el cupo (T029).
    let correctos = [
        ("5678CDF", "+34 612 345 678"),
        ("5679CDF", "612-345-678"),
        ("5680CDF", "612345678901234"), // 15 cifras
    ];
    for (n, (matricula, telefono)) in correctos.into_iter().enumerate() {
        let cuerpo = con_contacto(matricula, laborable(n + 1), telefono, "ana@correo.es");
        let (codigo, respuesta) = pedir(&app, cuerpo).await;
        assert_eq!(codigo, StatusCode::CREATED, "{telefono:?}: {respuesta}");
    }

    // Con el día ya lleno, un teléfono incorrecto da 422 y no el 409 del cupo: los datos del
    // vehículo nuevo se validan antes de los bloqueos (hallazgo I3).
    let lleno = laborable(5);
    for matricula in ["1111AAA", "2222AAA"] {
        let id = insertar_vehiculo(&pool, matricula, "Seat", "Ibiza", "Ana", "600000001", "a@ejemplo.test").await;
        insertar_cita(&pool, id, lleno, "Revisión").await;
    }
    let (codigo, cuerpo) = pedir(&app, con_contacto("9012DFG", lleno, "12345", "ana@correo.es")).await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY, "{cuerpo}");
    assert_eq!(cuerpo["error"], TELEFONO);
}

/// Prueba 5 (FR-004, FR-005): campos obligatorios con una matrícula nueva.
#[sqlx::test]
async fn prueba_5_obligatorios(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let fecha = laborable(1);

    let mut sin_motivo = cita_nueva("5678CDF", fecha);
    sin_motivo["motivo"] = json!("   ");
    let mut sin_marca = cita_nueva("5678CDF", fecha);
    sin_marca["vehiculo"]["marca"] = json!("");
    let mut sin_modelo = cita_nueva("5678CDF", fecha);
    sin_modelo["vehiculo"]["modelo"] = json!("  ");
    let mut sin_vehiculo = cita_nueva("5678CDF", fecha);
    sin_vehiculo.as_object_mut().unwrap().remove("vehiculo");
    let mut sin_nombre = cita_nueva("5678CDF", fecha);
    sin_nombre["cliente"]["nombre"] = json!(" ");
    let mut sin_cliente = cita_nueva("5678CDF", fecha);
    sin_cliente.as_object_mut().unwrap().remove("cliente");

    let casos = [
        (sin_motivo, "El motivo es obligatorio", "motivo"),
        (sin_marca, "La marca es obligatoria", "vehiculo.marca"),
        (sin_modelo, "El modelo es obligatorio", "vehiculo.modelo"),
        (sin_vehiculo, "La marca es obligatoria", "vehiculo.marca"),
        (sin_nombre, "El nombre es obligatorio", "cliente.nombre"),
        (sin_cliente, "El nombre es obligatorio", "cliente.nombre"),
    ];
    for (cuerpo, mensaje, campo) in casos {
        let (codigo, respuesta) = pedir(&app, cuerpo).await;
        assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY, "{campo}: {respuesta}");
        assert_eq!(respuesta["error"], mensaje, "{campo}");
        assert_eq!(respuesta["detalle"]["campo"], campo, "{campo}");
    }
    for tabla in ["cliente", "vehiculo", "cita"] {
        assert_eq!(contar(&pool, tabla).await, 0, "{tabla}");
    }
}

// ---------------------------------------------------------------------------------------------
// US3 · Los días completos no se ofrecen
// ---------------------------------------------------------------------------------------------

const DIA_COMPLETO: &str = "Ese día ya está completo";

async fn dias_disponibles(app: &axum::Router) -> Vec<Value> {
    let (codigo, dias, _) =
        comun::peticion(app, "GET", "/api/publico/dias-disponibles", None, None).await;
    assert_eq!(codigo, StatusCode::OK, "{dias}");
    dias.as_array().unwrap().clone()
}

/// Prueba 8 (FR-007; US3-1): con `cupo_diario = 2`, un día con dos citas deja de ofrecerse y no
/// admite una tercera. Todos los vehículos, por SQL (no depende del alta de US2).
#[sqlx::test]
async fn prueba_8_cupo(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let (lleno, libre) = (laborable(1), laborable(2));
    for matricula in ["1111AAA", "2222AAA"] {
        let id = insertar_vehiculo(&pool, matricula, "Seat", "Ibiza", "Ana", "600000001", "a@ejemplo.test").await;
        insertar_cita(&pool, id, lleno, "Revisión").await;
    }

    let dias = dias_disponibles(&app).await;
    assert!(!dias.contains(&json!(lleno.to_string())), "{lleno} no debería ofrecerse: {dias:?}");
    assert!(dias.contains(&json!(libre.to_string())), "{libre} debería ofrecerse: {dias:?}");

    insertar_vehiculo(&pool, "3333AAA", "Kia", "Ceed", "Luis", "600000002", "l@ejemplo.test").await;
    let (codigo, cuerpo) = pedir(&app, cita_conocida("3333AAA", lleno)).await;
    assert_eq!(codigo, StatusCode::CONFLICT, "{cuerpo}");
    assert_eq!(cuerpo["error"], DIA_COMPLETO);

    // Con una cita pendiente además del día lleno, gana el aviso de cita pendiente (research D3).
    let id = insertar_vehiculo(&pool, "4444AAA", "Kia", "Ceed", "Eva", "600000003", "e@ejemplo.test").await;
    insertar_cita(&pool, id, laborable(3), "Revisión").await;
    let (codigo, cuerpo) = pedir(&app, cita_conocida("4444AAA", lleno)).await;
    assert_eq!(codigo, StatusCode::CONFLICT, "{cuerpo}");
    assert_eq!(cuerpo["error"], CITA_PENDIENTE);

    let en_el_dia: i64 = sqlx::query_scalar("SELECT count(*) FROM cita WHERE fecha = $1")
        .bind(lleno)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(en_el_dia, 2);
}

/// Prueba 9 (FR-007; US3-2, SC-003): dos peticiones a la vez por la última plaza del día.
#[sqlx::test]
async fn prueba_9_concurrencia_cupo(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let fecha = laborable(1);
    let primero = insertar_vehiculo(&pool, "1111AAA", "Seat", "Ibiza", "Ana", "600000001", "a@ejemplo.test").await;
    insertar_vehiculo(&pool, "2222AAA", "Kia", "Ceed", "Luis", "600000002", "l@ejemplo.test").await;
    insertar_vehiculo(&pool, "3333AAA", "Opel", "Corsa", "Eva", "600000003", "e@ejemplo.test").await;
    insertar_cita(&pool, primero, fecha, "Revisión").await;

    for vuelta in 0..20 {
        let (a, b) = tokio::join!(
            pedir(&app, cita_conocida("2222AAA", fecha)),
            pedir(&app, cita_conocida("3333AAA", fecha)),
        );
        let mut codigos = [a.0, b.0];
        codigos.sort();
        assert_eq!(codigos, [StatusCode::CREATED, StatusCode::CONFLICT], "vuelta {vuelta}: {a:?} {b:?}");
        let rechazada = if a.0 == StatusCode::CONFLICT { &a.1 } else { &b.1 };
        assert_eq!(rechazada["error"], DIA_COMPLETO, "vuelta {vuelta}");
        let en_el_dia: i64 = sqlx::query_scalar("SELECT count(*) FROM cita WHERE fecha = $1")
            .bind(fecha)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(en_el_dia, 2, "vuelta {vuelta}");
        // Se borra la cita que ha entrado, para que la vuelta siguiente vuelva a pelear por la
        // última plaza (y para que ninguno de los dos tenga cita pendiente).
        sqlx::query("DELETE FROM cita WHERE vehiculo_id <> $1")
            .bind(primero)
            .execute(&pool)
            .await
            .unwrap();
    }
}

// ---------------------------------------------------------------------------------------------
// Convergencia (converge-1)
// ---------------------------------------------------------------------------------------------

/// T036 (converge-1 F2; FR-004, FR-010, research D3 y D4): dos altas a la vez de la misma
/// matrícula nueva, para días distintos (el bloqueo por fecha no las serializa). La segunda
/// búsqueda, con el bloqueo por matrícula, ve el vehículo de la primera: D4 y 409 de cita
/// pendiente, sin chocar con el `UNIQUE` de `vehiculo`.
#[sqlx::test]
async fn alta_simultanea_de_la_misma_matricula_nueva(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());

    for vuelta in 0..20 {
        let mut de_ana = cita_nueva("5678CDF", laborable(1));
        de_ana["cliente"] = json!({ "nombre": "Ana Pérez", "telefono": "612345678", "email": "ana@ejemplo.test" });
        let mut de_luis = cita_nueva("5678CDF", laborable(2));
        de_luis["cliente"] = json!({ "nombre": "Luis Gil", "telefono": "699999999", "email": "luis@ejemplo.test" });

        let (a, b) = tokio::join!(pedir(&app, de_ana), pedir(&app, de_luis));
        let mut codigos = [a.0, b.0];
        codigos.sort();
        assert_eq!(codigos, [StatusCode::CREATED, StatusCode::CONFLICT], "vuelta {vuelta}: {a:?} {b:?}");
        let rechazada = if a.0 == StatusCode::CONFLICT { &a.1 } else { &b.1 };
        assert_eq!(rechazada["error"], CITA_PENDIENTE, "vuelta {vuelta}");

        for tabla in ["cliente", "vehiculo", "cita"] {
            assert_eq!(contar(&pool, tabla).await, 1, "vuelta {vuelta}: {tabla}");
        }
        // El propietario es el de la petición que ha recibido el 201.
        let ganador = if a.0 == StatusCode::CREATED { "Ana Pérez" } else { "Luis Gil" };
        assert_eq!(cliente_de(&pool, "5678CDF").await.0, ganador, "vuelta {vuelta}");

        for tabla in ["cita", "vehiculo", "cliente"] {
            sqlx::query(&format!("DELETE FROM {tabla}")).execute(&pool).await.unwrap();
        }
    }
}

/// T038 (converge-1 F4): los dos comportamientos que el contrato común recoge desde converge-1.
#[sqlx::test]
async fn errores_documentados_en_el_contrato(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());

    // El 422 del GET público lleva `detalle.campo`, igual que el del POST.
    let (codigo, cuerpo) = vehiculo_publico(&app, "A").await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY, "{cuerpo}");
    assert_eq!(cuerpo, json!({ "error": "La matrícula no es válida", "detalle": { "campo": "matricula" } }));

    // Una fecha mal escrita en la lista de recepción → 400.
    let (_, contrasena) = comun::crear_usuario(&pool, "anabel", "recepcion", true).await;
    let cookie = comun::entrar(&app, "anabel", &contrasena).await;
    let (codigo, cuerpo, _) = comun::peticion(&app, "GET", "/api/citas?fecha=ayer", Some(&cookie), None).await;
    assert_eq!(codigo, StatusCode::BAD_REQUEST, "{cuerpo}");
    assert_eq!(cuerpo["error"], "La fecha no es válida");
}
