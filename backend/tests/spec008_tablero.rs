//! Tablero de órdenes abiertas (spec-008; plan, «Pruebas»; contrato, sección «spec-008 · Tablero»).
//! Cada prueba crea sus propios datos; ninguna usa valores demo (constitución, principio V). Las
//! fechas de entrega se fijan con `fechas::hoy()` (Europe/Madrid, R12).

mod comun;

use std::future::Future;

use axum::Router;
use axum::http::StatusCode;
use serde_json::Value;
use sqlx::{Connection, PgConnection, PgPool};
use taller::estados::Estado;
use taller::fechas;

// ---------------------------------------------------------------------------------------------
// Utilidades propias de spec-008 (el resto, en `comun`).
// ---------------------------------------------------------------------------------------------

/// Cambia la entrega prometida de una orden a `dias` desde hoy (negativo = ya pasó).
async fn entrega(pool: &PgPool, orden_id: i64, dias: i64) {
    sqlx::query("UPDATE orden SET fecha_entrega_prometida = $2::date + $3::int WHERE id = $1")
        .bind(orden_id)
        .bind(fechas::hoy())
        .bind(dias as i32)
        .execute(pool)
        .await
        .unwrap();
}

/// Fija el momento de apertura de una orden a `horas` antes de ahora.
async fn abierta_hace(pool: &PgPool, orden_id: i64, horas: i32) {
    sqlx::query("UPDATE orden SET abierta_en = now() - make_interval(hours => $2) WHERE id = $1")
        .bind(orden_id)
        .bind(horas)
        .execute(pool)
        .await
        .unwrap();
}

/// Un usuario del rol indicado que puede entrar, y su cookie.
async fn sesion(pool: &PgPool, app: &Router, login: &str, rol: &str) -> String {
    let (_, contrasena) = comun::crear_usuario(pool, login, rol, true).await;
    comun::entrar(app, login, &contrasena).await
}

/// Los números de orden de una lista de `OrdenResumen`.
fn numeros(ordenes: &Value) -> Vec<i64> {
    ordenes
        .as_array()
        .expect("una lista de órdenes")
        .iter()
        .map(|o| o["numero"].as_i64().unwrap())
        .collect()
}

// ---------------------------------------------------------------------------------------------
// Prueba 8 · D3: la lista de órdenes de spec-002 no cambia con `resumenes(…, solo_abiertas)`.
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn prueba8_la_lista_de_ordenes_sigue_trayendo_las_facturadas_en_el_mismo_orden(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let recepcion = sesion(&pool, &app, "ana", "recepcion").await;
    let mecanico = comun::insertar_usuario(&pool, "luis", "mecanico").await;

    let facturada = comun::crear_orden(&pool, 1, Estado::Facturada, mecanico).await;
    let antigua = comun::crear_orden(&pool, 2, Estado::EnReparacion, mecanico).await;
    let reciente = comun::crear_orden(&pool, 3, Estado::Recibida, mecanico).await;
    abierta_hace(&pool, facturada, 3).await;
    abierta_hace(&pool, antigua, 2).await;
    abierta_hace(&pool, reciente, 1).await;

    let (codigo, lista, _) =
        comun::peticion(&app, "GET", "/api/ordenes", Some(&recepcion), None).await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(
        numeros(&lista),
        vec![3, 2, 1],
        "las tres, de la abierta más recientemente a la más antigua, facturada incluida"
    );
}

/// Los 6 estados abiertos, en el orden de las columnas (FR-001).
const ESTADOS_ABIERTOS: [&str; 6] = [
    "recibida",
    "en_diagnostico",
    "en_reparacion",
    "en_espera_piezas",
    "pendiente_conformidad",
    "terminada",
];

/// `GET /api/tablero` con la cookie dada.
async fn tablero(app: &Router, cookie: Option<&str>) -> (StatusCode, Value) {
    let (codigo, cuerpo, _) = comun::peticion(app, "GET", "/api/tablero", cookie, None).await;
    (codigo, cuerpo)
}

/// Las órdenes de la columna de un estado.
fn columna<'a>(tablero: &'a Value, estado: &str) -> &'a Value {
    &tablero["columnas"]
        .as_array()
        .expect("columnas")
        .iter()
        .find(|c| c["estado"] == estado)
        .unwrap_or_else(|| panic!("falta la columna {estado}"))["ordenes"]
}

/// Los estados de las columnas, en su orden.
fn estados_de_columnas(tablero: &Value) -> Vec<String> {
    tablero["columnas"]
        .as_array()
        .expect("columnas")
        .iter()
        .map(|c| c["estado"].as_str().unwrap().to_string())
        .collect()
}

/// La carga como `(nombre, órdenes abiertas, activo)`, en su orden.
fn carga(tablero: &Value) -> Vec<(String, i64, bool)> {
    tablero["carga_mecanicos"]
        .as_array()
        .expect("carga_mecanicos")
        .iter()
        .map(|m| {
            (
                m["nombre"].as_str().unwrap().to_string(),
                m["ordenes_abiertas"].as_i64().unwrap(),
                m["activo"].as_bool().unwrap(),
            )
        })
        .collect()
}

/// SC-003: la suma de la carga es el número total de tarjetas.
fn cuadra_carga(tablero: &Value) {
    let tarjetas: usize = ESTADOS_ABIERTOS
        .iter()
        .map(|e| columna(tablero, e).as_array().unwrap().len())
        .sum();
    let suma: i64 = carga(tablero).iter().map(|(_, n, _)| n).sum();
    assert_eq!(
        suma, tarjetas as i64,
        "SC-003: la carga no cuadra con las tarjetas"
    );
}

// ---------------------------------------------------------------------------------------------
// US1 · Ver las órdenes abiertas agrupadas por estado
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn prueba1_seis_columnas_cada_orden_en_la_suya_y_sin_facturadas(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let jefe = sesion(&pool, &app, "paco", "jefe").await;
    let mecanico = comun::insertar_usuario(&pool, "luis", "mecanico").await;
    let estados = [
        Estado::Recibida,
        Estado::EnDiagnostico,
        Estado::EnReparacion,
        Estado::EnEsperaPiezas,
        Estado::PendienteConformidad,
        Estado::Terminada,
    ];
    for (i, estado) in estados.into_iter().enumerate() {
        comun::crear_orden(&pool, i as i64 + 1, estado, mecanico).await;
    }
    comun::crear_orden(&pool, 7, Estado::Facturada, mecanico).await;

    let (codigo, t) = tablero(&app, Some(&jefe)).await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(estados_de_columnas(&t), ESTADOS_ABIERTOS);
    for (i, estado) in ESTADOS_ABIERTOS.iter().enumerate() {
        let ordenes = columna(&t, estado);
        assert_eq!(numeros(ordenes), vec![i as i64 + 1], "columna {estado}");
        let o = &ordenes[0];
        assert_eq!(o["estado"], *estado);
        assert_eq!(o["matricula"], format!("{:04}PRB", i + 1));
        assert_eq!(o["mecanico"]["id"], mecanico);
        assert_eq!(o["mecanico"]["nombre"], "luis");
        assert!(o["fecha_entrega_prometida"].is_string());
        assert_eq!(o["retrasada"], false);
    }
    let total: usize = ESTADOS_ABIERTOS
        .iter()
        .map(|e| columna(&t, e).as_array().unwrap().len())
        .sum();
    assert_eq!(total, 6, "la facturada (nº 7) no aparece");
    cuadra_carga(&t);
}

#[sqlx::test]
async fn prueba2_sin_ordenes_seis_columnas_vacias(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let jefe = sesion(&pool, &app, "paco", "jefe").await;
    comun::insertar_usuario(&pool, "luis", "mecanico").await;

    let (codigo, t) = tablero(&app, Some(&jefe)).await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(estados_de_columnas(&t), ESTADOS_ABIERTOS);
    for estado in ESTADOS_ABIERTOS {
        assert_eq!(
            columna(&t, estado),
            &Value::Array(vec![]),
            "columna {estado}"
        );
    }
    assert_eq!(
        carga(&t),
        vec![("luis".to_string(), 0, true)],
        "un mecánico activo sin órdenes sale con 0"
    );
    cuadra_carga(&t);
}

#[sqlx::test]
async fn prueba3_solo_el_jefe(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    for (login, rol) in [
        ("ana", "recepcion"),
        ("luis", "mecanico"),
        ("pepe", "almacen"),
    ] {
        let cookie = sesion(&pool, &app, login, rol).await;
        let (codigo, cuerpo) = tablero(&app, Some(&cookie)).await;
        assert_eq!(codigo, StatusCode::FORBIDDEN, "rol {rol}");
        assert_eq!(
            cuerpo["error"], "No tienes permiso para hacer esto",
            "rol {rol}"
        );
    }
    let jefe = sesion(&pool, &app, "paco", "jefe").await;
    assert_eq!(tablero(&app, Some(&jefe)).await.0, StatusCode::OK);
    assert_eq!(tablero(&app, None).await.0, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn prueba4_tarjetas_por_entrega_y_numero(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let jefe = sesion(&pool, &app, "paco", "jefe").await;
    let mecanico = comun::insertar_usuario(&pool, "luis", "mecanico").await;
    for (numero, dias) in [(20, 2), (30, -1), (12, 0), (7, 0)] {
        let id = comun::crear_orden(&pool, numero, Estado::EnReparacion, mecanico).await;
        entrega(&pool, id, dias).await;
    }

    let (codigo, t) = tablero(&app, Some(&jefe)).await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(
        numeros(columna(&t, "en_reparacion")),
        vec![30, 7, 12, 20],
        "ayer, las dos de hoy por número y pasado mañana"
    );
    cuadra_carga(&t);
}

// ---------------------------------------------------------------------------------------------
// US2 · Carga de cada mecánico
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn prueba5_carga_por_mecanico_con_los_activos_a_cero(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let jefe = sesion(&pool, &app, "paco", "jefe").await;
    let luis = comun::insertar_usuario(&pool, "luis", "mecanico").await;
    let marta = comun::insertar_usuario(&pool, "marta", "mecanico").await;
    comun::insertar_usuario(&pool, "nuria", "mecanico").await;
    comun::crear_orden(&pool, 1, Estado::Recibida, luis).await;
    comun::crear_orden(&pool, 2, Estado::EnReparacion, luis).await;
    comun::crear_orden(&pool, 3, Estado::Terminada, luis).await;
    comun::crear_orden(&pool, 4, Estado::Facturada, luis).await;
    comun::crear_orden(&pool, 5, Estado::EnDiagnostico, marta).await;

    let (codigo, t) = tablero(&app, Some(&jefe)).await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(
        carga(&t),
        vec![
            ("luis".to_string(), 3, true),
            ("marta".to_string(), 1, true),
            ("nuria".to_string(), 0, true),
        ],
        "por nombre; la facturada no cuenta; el jefe y los demás roles no salen"
    );
    cuadra_carga(&t);
}

#[sqlx::test]
async fn prueba6_inactivos_solo_con_ordenes_abiertas(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let jefe = sesion(&pool, &app, "paco", "jefe").await;
    // Los tres con `crear_usuario` («Luis», «Pedro»): el orden no depende de mayúsculas.
    comun::crear_usuario(&pool, "luis", "mecanico", true).await;
    let (pedro, _) = comun::crear_usuario(&pool, "pedro", "mecanico", false).await;
    comun::crear_usuario(&pool, "rosa", "mecanico", false).await;
    comun::crear_orden(&pool, 1, Estado::EnReparacion, pedro).await;

    let (codigo, t) = tablero(&app, Some(&jefe)).await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(
        carga(&t),
        vec![
            ("Luis".to_string(), 0, true),
            ("Pedro".to_string(), 1, false)
        ],
        "Pedro, de baja, sale porque tiene una orden abierta; Rosa, de baja y sin órdenes, no"
    );
    cuadra_carga(&t);
}

// ---------------------------------------------------------------------------------------------
// US3 · Órdenes con retraso destacadas
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn prueba7_retrasada_ayer_si_hoy_no_y_terminada_sin_facturar_si(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let jefe = sesion(&pool, &app, "paco", "jefe").await;
    let mecanico = comun::insertar_usuario(&pool, "luis", "mecanico").await;
    let ayer = comun::crear_orden(&pool, 1, Estado::EnReparacion, mecanico).await;
    let hoy = comun::crear_orden(&pool, 2, Estado::EnReparacion, mecanico).await;
    let terminada = comun::crear_orden(&pool, 3, Estado::Terminada, mecanico).await;
    entrega(&pool, ayer, -1).await;
    entrega(&pool, hoy, 0).await;
    entrega(&pool, terminada, -1).await;

    let (codigo, t) = tablero(&app, Some(&jefe)).await;
    assert_eq!(codigo, StatusCode::OK);
    let reparacion = columna(&t, "en_reparacion");
    assert_eq!(numeros(reparacion), vec![1, 2]);
    assert_eq!(reparacion[0]["retrasada"], true, "entrega ayer: retrasada");
    assert_eq!(
        reparacion[1]["retrasada"], false,
        "entrega hoy: no retrasada"
    );
    assert_eq!(
        columna(&t, "terminada")[0]["retrasada"],
        true,
        "terminada sin facturar con la entrega ayer: retrasada"
    );
}

/// Pide el tablero y lo abandona tras `pasos` vueltas del planificador, como un navegador que corta
/// la petición. Entre una vuelta y otra, `yield_now` deja que avance la red. Devuelve si se cortó.
async fn cortar_tras(app: &Router, cookie: &str, pasos: usize) -> bool {
    let peticion = comun::peticion(app, "GET", "/api/tablero", Some(cookie), None);
    let mut peticion = std::pin::pin!(peticion);
    for _ in 0..pasos {
        let lista = std::future::poll_fn(|cx| {
            std::task::Poll::Ready(peticion.as_mut().poll(cx).is_ready())
        })
        .await;
        if lista {
            return false;
        }
        tokio::task::yield_now().await;
    }
    true
}

// ---------------------------------------------------------------------------------------------
// Prueba 9 · V3: una petición cortada a mitad no deja ninguna conexión dentro de una transacción.
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
#[ignore = "tarda ~100 s; lanzar con: cargo test --test spec008_tablero -- --ignored"]
async fn prueba9_peticiones_cortadas_no_dejan_conexiones_en_transaccion(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let jefe = sesion(&pool, &app, "paco", "jefe").await;
    let luis = comun::insertar_usuario(&pool, "luis", "mecanico").await;
    let marta = comun::insertar_usuario(&pool, "marta", "mecanico").await;
    for (numero, estado, mecanico) in [
        (1, Estado::Recibida, luis),
        (2, Estado::EnReparacion, luis),
        (3, Estado::Terminada, marta),
        (4, Estado::Facturada, marta),
    ] {
        comun::crear_orden(&pool, numero, estado, mecanico).await;
    }

    // Una conexión aparte, fuera del pool de la aplicación, para mirar `pg_stat_activity` sin
    // recibir nunca una de las conexiones que se comprueban.
    let mut observador = PgConnection::connect_with(&pool.connect_options())
        .await
        .unwrap();

    // Como el navegador al cambiar de pantalla: la petición se corta a mitad. El temporizador de
    // tokio va de milisegundo en milisegundo, demasiado grueso para la ventana en que se espera la
    // respuesta del `BEGIN` (una décima de milisegundo); por eso se corta tras `pasos` vueltas del
    // planificador, recorriendo todos los puntos de espera de la petición. Acertar justo en esa
    // ventana es poco frecuente (unas 2 de cada 1700 peticiones cortadas con la transacción de D1),
    // así que se recorre 25 veces: unos 4000 cortes, con una probabilidad de fallar de verdad de
    // más del 99 % si la transacción vuelve. Se comprueba
    // después de **cada** corte: la siguiente petición podría limpiar la conexión de rebote.
    let mut cortadas = 0;
    let mut en_transaccion = Vec::new();
    for _ in 0..25 {
        for pasos in 1..=200 {
            if !cortar_tras(&app, &jefe, pasos).await {
                continue;
            }
            cortadas += 1;
            // Lo que hubiera que deshacer al soltar la conexión ya ha tenido tiempo de hacerse.
            tokio::time::sleep(std::time::Duration::from_millis(2)).await;
            let n: i64 = sqlx::query_scalar(
                "SELECT count(*) FROM pg_stat_activity
                  WHERE datname = current_database() AND state LIKE 'idle in transaction%'",
            )
            .fetch_one(&mut observador)
            .await
            .unwrap();
            if n > 0 {
                en_transaccion.push(pasos);
                // Se deja la base como estaba para que el resto del recorrido siga midiendo.
                sqlx::query(
                    "SELECT pg_terminate_backend(pid) FROM pg_stat_activity
                      WHERE datname = current_database() AND state LIKE 'idle in transaction%'",
                )
                .execute(&mut observador)
                .await
                .unwrap();
            }
        }
    }
    assert!(
        cortadas > 0,
        "ninguna petición se cortó: la prueba no estaría probando nada"
    );
    assert_eq!(
        en_transaccion,
        Vec::<usize>::new(),
        "de {cortadas} peticiones cortadas, estas dejaron una conexión dentro de una transacción \
         (pasos del corte)"
    );

    // Y la aplicación sigue escribiendo: entrar actualiza la tabla de sesiones.
    let (_, contrasena) = comun::crear_usuario(&pool, "ana", "recepcion", true).await;
    for _ in 0..10 {
        comun::entrar(&app, "ana", &contrasena).await;
    }
    assert_eq!(tablero(&app, Some(&jefe)).await.0, StatusCode::OK);
}
