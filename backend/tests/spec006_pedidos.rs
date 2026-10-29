//! Almacén y pedidos a proveedor (spec-006; plan, «Pruebas»; contrato, sección spec-006).

mod comun;

use std::time::Duration;

use axum::Router;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use sqlx::PgPool;
use taller::app;
use taller::estados::{CambioEstado, Estado};
use taller::eventos::Eventos;
use taller::fechas;
use taller::rutas::almacen::pedidos_de_orden;
use tokio::sync::broadcast;

// ---------------------------------------------------------------------------------------------
// Utilidades propias de spec-006 (el resto, en `comun`)
// ---------------------------------------------------------------------------------------------

/// App de prueba con acceso a sus eventos (`app_prueba` crea su propio canal y no lo devuelve).
fn app_con_eventos(pool: &PgPool) -> (Router, Eventos) {
    let estado = comun::estado_prueba(pool.clone());
    let eventos = estado.eventos.clone();
    (app(estado), eventos)
}

async fn entrar_como(app: &Router, pool: &PgPool, login: &str, rol: &str) -> String {
    let (_, contrasena) = comun::crear_usuario(pool, login, rol, true).await;
    comun::entrar(app, login, &contrasena).await
}

async fn insertar_pieza(pool: &PgPool, referencia: &str, stock: i32) -> i64 {
    sqlx::query_scalar(
        "INSERT INTO pieza (referencia, descripcion, precio_venta, stock)
         VALUES ($1, 'Pieza de prueba', 25.00, $2) RETURNING id",
    )
    .bind(referencia)
    .bind(stock)
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn insertar_proveedor(pool: &PgPool, nombre: &str) -> i64 {
    sqlx::query_scalar("INSERT INTO proveedor (nombre) VALUES ($1) RETURNING id")
        .bind(nombre)
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn estado_de_orden(pool: &PgPool, id: i64) -> Estado {
    sqlx::query_scalar("SELECT estado FROM orden WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn stock_de(pool: &PgPool, pieza_id: i64) -> i32 {
    sqlx::query_scalar("SELECT stock FROM pieza WHERE id = $1")
        .bind(pieza_id)
        .fetch_one(pool)
        .await
        .unwrap()
}

/// Lo necesario para pedir: el almacén con sesión, un proveedor, PF-123 con stock 0 y la orden
/// 42 «en reparación» de luis.
struct Escenario {
    app: Router,
    eventos: Eventos,
    pepe: String,
    proveedor: i64,
    pf123: i64,
    orden: i64,
}

async fn escenario(pool: &PgPool) -> Escenario {
    let (app, eventos) = app_con_eventos(pool);
    let pepe = entrar_como(&app, pool, "pepe", "almacen").await;
    let proveedor = insertar_proveedor(pool, "Recambios Sur").await;
    let pf123 = insertar_pieza(pool, "PF-123", 0).await;
    let luis = comun::insertar_usuario(pool, "luis", "mecanico").await;
    let orden = comun::crear_orden(pool, 42, Estado::EnReparacion, luis).await;
    Escenario {
        app,
        eventos,
        pepe,
        proveedor,
        pf123,
        orden,
    }
}

async fn pedir(
    e: &Escenario,
    pieza: i64,
    cantidad: Value,
    orden_numero: i64,
) -> (StatusCode, Value) {
    let (codigo, cuerpo, _) = comun::peticion(
        &e.app,
        "POST",
        "/api/pedidos",
        Some(&e.pepe),
        Some(json!({
            "proveedor_id": e.proveedor,
            "pieza_id": pieza,
            "cantidad": cantidad,
            "orden_numero": orden_numero
        })),
    )
    .await;
    (codigo, cuerpo)
}

async fn recibir(e: &Escenario, pedido: i64) -> (StatusCode, Value) {
    let ruta = format!("/api/pedidos/{pedido}/recibir");
    let (codigo, cuerpo, _) = comun::peticion(&e.app, "POST", &ruta, Some(&e.pepe), None).await;
    (codigo, cuerpo)
}

fn id_de(cuerpo: &Value) -> i64 {
    cuerpo["id"].as_i64().unwrap()
}

fn es_hoy(instante: &Value) -> bool {
    let instante: DateTime<Utc> = instante.as_str().unwrap().parse().unwrap();
    fechas::fecha_en_madrid(instante) == fechas::hoy()
}

async fn evento(tablero: &mut broadcast::Receiver<CambioEstado>) -> Value {
    let recibido = tokio::time::timeout(Duration::from_secs(1), tablero.recv())
        .await
        .expect("no llegó ningún evento al tablero")
        .unwrap();
    serde_json::to_value(recibido).unwrap()
}

// ---------------------------------------------------------------------------------------------
// US1 · Dar de alta referencias de piezas (T009, T010)
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn alta_con_stock_y_sin_stock(pool: PgPool) {
    let (app, _) = app_con_eventos(&pool);
    let pepe = entrar_como(&app, &pool, "pepe", "almacen").await;

    let (codigo, pieza, _) = comun::peticion(
        &app,
        "POST",
        "/api/piezas",
        Some(&pepe),
        Some(
            json!({ "referencia": "PF-123", "descripcion": "Pastillas freno",
                     "precio_venta": "25.00", "stock": 4 }),
        ),
    )
    .await;
    assert_eq!(codigo, StatusCode::CREATED, "{pieza}");
    assert_eq!(pieza["referencia"], "PF-123");
    assert_eq!(pieza["descripcion"], "Pastillas freno");
    assert_eq!(pieza["precio_venta"], "25.00");
    assert_eq!(pieza["stock"], 4);
    assert!(pieza["id"].is_i64());

    let (codigo, pieza, _) = comun::peticion(
        &app,
        "POST",
        "/api/piezas",
        Some(&pepe),
        Some(
            json!({ "referencia": "  FA-77 ", "descripcion": " Filtro aire ",
                     "precio_venta": "12.5" }),
        ),
    )
    .await;
    assert_eq!(codigo, StatusCode::CREATED, "{pieza}");
    assert_eq!(pieza["referencia"], "FA-77");
    assert_eq!(pieza["descripcion"], "Filtro aire");
    assert_eq!(pieza["precio_venta"], "12.50");
    assert_eq!(pieza["stock"], 0);
}

#[sqlx::test]
async fn referencia_repetida_da_409(pool: PgPool) {
    let (app, _) = app_con_eventos(&pool);
    let pepe = entrar_como(&app, &pool, "pepe", "almacen").await;
    insertar_pieza(&pool, "PF-123", 4).await;

    let (codigo, cuerpo, _) = comun::peticion(
        &app,
        "POST",
        "/api/piezas",
        Some(&pepe),
        Some(json!({ "referencia": "PF-123", "descripcion": "Otra", "precio_venta": "1.00" })),
    )
    .await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(cuerpo["error"], "Ya existe la referencia PF-123");
}

#[sqlx::test]
async fn alta_con_datos_no_validos_da_422(pool: PgPool) {
    let (app, _) = app_con_eventos(&pool);
    let pepe = entrar_como(&app, &pool, "pepe", "almacen").await;
    let base = json!({ "referencia": "PF-1", "descripcion": "Pieza", "precio_venta": "1.00" });

    let malos = [
        ("stock", json!(-1)),
        ("stock", json!(1.5)),
        ("stock", json!("3")),
        ("precio_venta", json!("-1.00")),
        ("precio_venta", json!("1.001")),
        ("precio_venta", json!("abc")),
        ("precio_venta", Value::Null),
        ("referencia", json!("")),
        ("referencia", json!("   ")),
        ("descripcion", json!("")),
        ("descripcion", json!("   ")),
    ];
    for (campo, valor) in malos {
        let mut cuerpo = base.clone();
        cuerpo[campo] = valor.clone();
        let (codigo, respuesta, _) =
            comun::peticion(&app, "POST", "/api/piezas", Some(&pepe), Some(cuerpo)).await;
        assert_eq!(
            codigo,
            StatusCode::UNPROCESSABLE_ENTITY,
            "{campo} = {valor}: {respuesta}"
        );
        assert!(respuesta["error"].is_string());
    }

    let piezas: i64 = sqlx::query_scalar("SELECT count(*) FROM pieza")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(piezas, 0);
}

#[sqlx::test]
async fn un_mecanico_no_puede_dar_de_alta_piezas(pool: PgPool) {
    let (app, _) = app_con_eventos(&pool);
    let luis = entrar_como(&app, &pool, "luis", "mecanico").await;

    let (codigo, _, _) = comun::peticion(
        &app,
        "POST",
        "/api/piezas",
        Some(&luis),
        Some(json!({ "referencia": "PF-1", "descripcion": "Pieza", "precio_venta": "1.00" })),
    )
    .await;
    assert_eq!(codigo, StatusCode::FORBIDDEN);
}

#[sqlx::test]
async fn el_jefe_puede_dar_de_alta_piezas(pool: PgPool) {
    let (app, _) = app_con_eventos(&pool);
    let paco = entrar_como(&app, &pool, "paco", "jefe").await;

    let (codigo, _, _) = comun::peticion(
        &app,
        "POST",
        "/api/piezas",
        Some(&paco),
        Some(json!({ "referencia": "PF-1", "descripcion": "Pieza", "precio_venta": "1.00" })),
    )
    .await;
    assert_eq!(codigo, StatusCode::CREATED);
}

#[sqlx::test]
async fn lista_de_piezas_ordenada_y_con_busqueda(pool: PgPool) {
    let (app, _) = app_con_eventos(&pool);
    // Cualquier rol con sesión puede consultarla (la usan spec-003 y spec-005).
    let luis = entrar_como(&app, &pool, "luis", "mecanico").await;
    for (referencia, descripcion) in [
        ("PF-123", "Pastillas freno"),
        ("FA-77", "Filtro aire"),
        ("FC-1", "Filtro combustible"),
    ] {
        sqlx::query("INSERT INTO pieza (referencia, descripcion, precio_venta) VALUES ($1, $2, 3)")
            .bind(referencia)
            .bind(descripcion)
            .execute(&pool)
            .await
            .unwrap();
    }

    let (codigo, lista, _) = comun::peticion(&app, "GET", "/api/piezas", Some(&luis), None).await;
    assert_eq!(codigo, StatusCode::OK);
    let referencias: Vec<&str> = lista
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["referencia"].as_str().unwrap())
        .collect();
    assert_eq!(referencias, ["FA-77", "FC-1", "PF-123"]);
    assert_eq!(lista[0]["precio_venta"], "3.00");
    assert_eq!(lista[0]["stock"], 0);

    let (_, lista, _) =
        comun::peticion(&app, "GET", "/api/piezas?q=FILTRO", Some(&luis), None).await;
    assert_eq!(lista.as_array().unwrap().len(), 2);

    let (_, lista, _) = comun::peticion(&app, "GET", "/api/piezas?q=pf-", Some(&luis), None).await;
    assert_eq!(lista.as_array().unwrap().len(), 1);
    assert_eq!(lista[0]["referencia"], "PF-123");

    let (codigo, _, _) = comun::peticion(&app, "GET", "/api/piezas", None, None).await;
    assert_eq!(codigo, StatusCode::UNAUTHORIZED);
}

// ---------------------------------------------------------------------------------------------
// US2 · Pedir una pieza vinculada a una orden (T014, T015, T016)
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn lista_de_proveedores_ordenada(pool: PgPool) {
    let e = escenario(&pool).await;
    insertar_proveedor(&pool, "Autopartes Norte").await;

    let (codigo, lista, _) =
        comun::peticion(&e.app, "GET", "/api/proveedores", Some(&e.pepe), None).await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(lista[0]["nombre"], "Autopartes Norte");
    assert_eq!(lista[1]["nombre"], "Recambios Sur");

    // Proveedores precargados: no hay alta (FR-010).
    let (codigo, _, _) = comun::peticion(
        &e.app,
        "POST",
        "/api/proveedores",
        Some(&e.pepe),
        Some(json!({ "nombre": "Nuevo" })),
    )
    .await;
    assert_eq!(codigo, StatusCode::METHOD_NOT_ALLOWED);
}

#[sqlx::test]
async fn pedido_para_orden_en_reparacion_la_pone_en_espera(pool: PgPool) {
    let e = escenario(&pool).await;
    let mut tablero = e.eventos.suscribir();

    let (codigo, pedido) = pedir(&e, e.pf123, json!(2), 42).await;

    assert_eq!(codigo, StatusCode::CREATED, "{pedido}");
    assert_eq!(pedido["estado"], "pendiente");
    assert_eq!(pedido["cantidad"], 2);
    assert_eq!(
        pedido["proveedor"],
        json!({ "id": e.proveedor, "nombre": "Recambios Sur" })
    );
    assert_eq!(
        pedido["pieza"],
        json!({ "id": e.pf123, "referencia": "PF-123", "descripcion": "Pieza de prueba" })
    );
    assert_eq!(pedido["orden"], json!({ "id": e.orden, "numero": 42 }));
    assert!(es_hoy(&pedido["pedido_en"]));
    assert_eq!(pedido["recibido_en"], Value::Null);
    assert_eq!(
        estado_de_orden(&pool, e.orden).await,
        Estado::EnEsperaPiezas
    );
    assert_eq!(
        evento(&mut tablero).await,
        json!({ "id": e.orden, "estado": "en_espera_piezas" })
    );
}

#[sqlx::test]
async fn otro_pedido_con_la_orden_en_espera_se_admite(pool: PgPool) {
    let e = escenario(&pool).await;
    let fa77 = insertar_pieza(&pool, "FA-77", 0).await;
    assert_eq!(
        pedir(&e, e.pf123, json!(2), 42).await.0,
        StatusCode::CREATED
    );
    let mut tablero = e.eventos.suscribir();

    let (codigo, pedido) = pedir(&e, fa77, json!(1), 42).await;

    assert_eq!(codigo, StatusCode::CREATED, "{pedido}");
    assert_eq!(
        estado_de_orden(&pool, e.orden).await,
        Estado::EnEsperaPiezas
    );
    // La orden no cambia de estado: no hay nada que avisar al tablero.
    assert!(matches!(
        tablero.try_recv(),
        Err(broadcast::error::TryRecvError::Empty)
    ));
}

#[sqlx::test]
async fn misma_referencia_pendiente_para_la_misma_orden_da_409(pool: PgPool) {
    let e = escenario(&pool).await;
    assert_eq!(
        pedir(&e, e.pf123, json!(2), 42).await.0,
        StatusCode::CREATED
    );

    let (codigo, cuerpo) = pedir(&e, e.pf123, json!(1), 42).await;

    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(
        cuerpo["error"],
        "Ya hay un pedido pendiente de PF-123 para la orden 42"
    );
    let pedidos: i64 = sqlx::query_scalar("SELECT count(*) FROM pedido")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(pedidos, 1);
}

#[sqlx::test]
async fn orden_en_diagnostico_da_409_y_no_cambia(pool: PgPool) {
    let e = escenario(&pool).await;
    let luis: i64 = sqlx::query_scalar("SELECT id FROM usuario WHERE login = 'luis'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let orden50 = comun::crear_orden(&pool, 50, Estado::EnDiagnostico, luis).await;

    let (codigo, cuerpo) = pedir(&e, e.pf123, json!(2), 50).await;

    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(
        cuerpo["error"],
        "La orden 50 está en diagnóstico; solo se puede pedir para órdenes en reparación o en \
         espera de piezas"
    );
    assert_eq!(estado_de_orden(&pool, orden50).await, Estado::EnDiagnostico);
}

#[sqlx::test]
async fn pedido_con_datos_no_validos_da_422(pool: PgPool) {
    let e = escenario(&pool).await;

    let (codigo, cuerpo) = pedir(&e, e.pf123, json!(2), 99).await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(cuerpo["error"], "No existe la orden 99");

    for cantidad in [json!(0), json!(-1), json!(1.5), json!("2"), Value::Null] {
        let (codigo, cuerpo) = pedir(&e, e.pf123, cantidad.clone(), 42).await;
        assert_eq!(
            codigo,
            StatusCode::UNPROCESSABLE_ENTITY,
            "cantidad {cantidad}: {cuerpo}"
        );
    }

    let (codigo, _) = pedir(&e, 999_999, json!(1), 42).await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY);

    let (codigo, _, _) = comun::peticion(
        &e.app,
        "POST",
        "/api/pedidos",
        Some(&e.pepe),
        Some(
            json!({ "proveedor_id": 999_999, "pieza_id": e.pf123, "cantidad": 1,
                     "orden_numero": 42 }),
        ),
    )
    .await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY);

    let (codigo, _, _) = comun::peticion(
        &e.app,
        "POST",
        "/api/pedidos",
        Some(&e.pepe),
        Some(json!({ "proveedor_id": e.proveedor, "pieza_id": e.pf123, "cantidad": 1 })),
    )
    .await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY);

    assert_eq!(estado_de_orden(&pool, e.orden).await, Estado::EnReparacion);
}

#[sqlx::test]
async fn un_mecanico_no_puede_pedir(pool: PgPool) {
    let e = escenario(&pool).await;
    let marta = entrar_como(&e.app, &pool, "marta", "mecanico").await;

    let (codigo, _, _) = comun::peticion(
        &e.app,
        "POST",
        "/api/pedidos",
        Some(&marta),
        Some(
            json!({ "proveedor_id": e.proveedor, "pieza_id": e.pf123, "cantidad": 1,
                     "orden_numero": 42 }),
        ),
    )
    .await;
    assert_eq!(codigo, StatusCode::FORBIDDEN);
    assert_eq!(estado_de_orden(&pool, e.orden).await, Estado::EnReparacion);
}

/// FR-006 y FR-008a: con la orden en espera no se puede terminar ni imputar. Los endpoints son de
/// spec-005 (activada en spec-005 T027).
#[sqlx::test]
async fn orden_en_espera_no_se_puede_terminar_ni_imputar(pool: PgPool) {
    let e = escenario(&pool).await;
    assert_eq!(
        pedir(&e, e.pf123, json!(2), 42).await.0,
        StatusCode::CREATED
    );
    let paco = entrar_como(&e.app, &pool, "paco", "jefe").await;

    let terminar = format!("/api/ordenes/{}/terminar", e.orden);
    let (codigo, _, _) = comun::peticion(&e.app, "POST", &terminar, Some(&paco), None).await;
    assert_eq!(codigo, StatusCode::CONFLICT);

    let imputar = format!("/api/ordenes/{}/imputaciones", e.orden);
    let (codigo, _, _) = comun::peticion(
        &e.app,
        "POST",
        &imputar,
        Some(&paco),
        Some(json!({ "tipo": "mano_obra", "concepto": "Cambio de pastillas", "cantidad": "1.00" })),
    )
    .await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(
        estado_de_orden(&pool, e.orden).await,
        Estado::EnEsperaPiezas
    );
}

// ---------------------------------------------------------------------------------------------
// US3 · Recibir el pedido (T020, T021, T022)
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn recibir_suma_el_stock_y_la_orden_vuelve_a_reparacion(pool: PgPool) {
    let e = escenario(&pool).await;
    let (_, pedido) = pedir(&e, e.pf123, json!(2), 42).await;
    let mut tablero = e.eventos.suscribir();

    let (codigo, recibido) = recibir(&e, id_de(&pedido)).await;

    assert_eq!(codigo, StatusCode::OK, "{recibido}");
    assert_eq!(recibido["estado"], "recibido");
    assert!(es_hoy(&recibido["recibido_en"]));
    assert_eq!(stock_de(&pool, e.pf123).await, 2);
    assert_eq!(estado_de_orden(&pool, e.orden).await, Estado::EnReparacion);
    assert_eq!(
        evento(&mut tablero).await,
        json!({ "id": e.orden, "estado": "en_reparacion" })
    );
}

#[sqlx::test]
async fn con_dos_pedidos_la_orden_espera_al_ultimo(pool: PgPool) {
    let e = escenario(&pool).await;
    let fa77 = insertar_pieza(&pool, "FA-77", 0).await;
    let (_, primero) = pedir(&e, e.pf123, json!(2), 42).await;
    let (_, segundo) = pedir(&e, fa77, json!(1), 42).await;
    let mut tablero = e.eventos.suscribir();

    assert_eq!(recibir(&e, id_de(&primero)).await.0, StatusCode::OK);
    assert_eq!(
        estado_de_orden(&pool, e.orden).await,
        Estado::EnEsperaPiezas
    );
    assert!(matches!(
        tablero.try_recv(),
        Err(broadcast::error::TryRecvError::Empty)
    ));

    assert_eq!(recibir(&e, id_de(&segundo)).await.0, StatusCode::OK);
    assert_eq!(estado_de_orden(&pool, e.orden).await, Estado::EnReparacion);
    assert_eq!(
        evento(&mut tablero).await,
        json!({ "id": e.orden, "estado": "en_reparacion" })
    );
}

#[sqlx::test]
async fn tras_recibir_se_puede_volver_a_pedir_la_misma_referencia(pool: PgPool) {
    let e = escenario(&pool).await;
    let (_, pedido) = pedir(&e, e.pf123, json!(2), 42).await;
    assert_eq!(recibir(&e, id_de(&pedido)).await.0, StatusCode::OK);

    let (codigo, _) = pedir(&e, e.pf123, json!(1), 42).await;
    assert_eq!(codigo, StatusCode::CREATED);
    assert_eq!(
        estado_de_orden(&pool, e.orden).await,
        Estado::EnEsperaPiezas
    );
}

#[sqlx::test]
async fn recibir_dos_veces_da_409_y_no_suma_otra_vez(pool: PgPool) {
    let e = escenario(&pool).await;
    let (_, pedido) = pedir(&e, e.pf123, json!(2), 42).await;
    assert_eq!(recibir(&e, id_de(&pedido)).await.0, StatusCode::OK);

    let (codigo, cuerpo) = recibir(&e, id_de(&pedido)).await;

    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(cuerpo["error"], "Este pedido ya está recibido");
    assert_eq!(stock_de(&pool, e.pf123).await, 2);
}

#[sqlx::test]
async fn recibir_un_pedido_que_no_existe_da_404(pool: PgPool) {
    let e = escenario(&pool).await;
    assert_eq!(recibir(&e, 999_999).await.0, StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn un_mecanico_no_puede_recibir(pool: PgPool) {
    let e = escenario(&pool).await;
    let (_, pedido) = pedir(&e, e.pf123, json!(2), 42).await;
    let marta = entrar_como(&e.app, &pool, "marta", "mecanico").await;

    let ruta = format!("/api/pedidos/{}/recibir", id_de(&pedido));
    let (codigo, _, _) = comun::peticion(&e.app, "POST", &ruta, Some(&marta), None).await;

    assert_eq!(codigo, StatusCode::FORBIDDEN);
    assert_eq!(stock_de(&pool, e.pf123).await, 0);
}

#[sqlx::test]
async fn lista_de_pedidos_pendientes_de_todas_las_ordenes(pool: PgPool) {
    let e = escenario(&pool).await;
    let luis: i64 = sqlx::query_scalar("SELECT id FROM usuario WHERE login = 'luis'")
        .fetch_one(&pool)
        .await
        .unwrap();
    comun::crear_orden(&pool, 43, Estado::EnReparacion, luis).await;
    let (_, recibido) = pedir(&e, e.pf123, json!(2), 42).await;
    assert_eq!(recibir(&e, id_de(&recibido)).await.0, StatusCode::OK);
    let (_, pendiente42) = pedir(&e, e.pf123, json!(1), 42).await;
    let (_, pendiente43) = pedir(&e, e.pf123, json!(3), 43).await;

    let (codigo, lista, _) = comun::peticion(
        &e.app,
        "GET",
        "/api/pedidos?estado=pendiente",
        Some(&e.pepe),
        None,
    )
    .await;
    assert_eq!(codigo, StatusCode::OK);
    let lista = lista.as_array().unwrap();
    let ids: Vec<i64> = lista.iter().map(id_de).collect();
    assert_eq!(ids, [id_de(&pendiente42), id_de(&pendiente43)]);
    assert_eq!(lista[0]["orden"]["numero"], 42);
    assert_eq!(lista[1]["orden"]["numero"], 43);

    let (_, todos, _) = comun::peticion(&e.app, "GET", "/api/pedidos", Some(&e.pepe), None).await;
    assert_eq!(todos.as_array().unwrap().len(), 3);

    let (codigo, _, _) = comun::peticion(
        &e.app,
        "GET",
        "/api/pedidos?estado=otro",
        Some(&e.pepe),
        None,
    )
    .await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY);
}

// ---------------------------------------------------------------------------------------------
// US4 · Ver los pedidos de una orden (T026, T027)
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn pedidos_de_una_orden_sin_los_de_otras(pool: PgPool) {
    let e = escenario(&pool).await;
    let luis: i64 = sqlx::query_scalar("SELECT id FROM usuario WHERE login = 'luis'")
        .fetch_one(&pool)
        .await
        .unwrap();
    comun::crear_orden(&pool, 43, Estado::EnReparacion, luis).await;
    let fa77 = insertar_pieza(&pool, "FA-77", 0).await;
    let (_, recibido) = pedir(&e, e.pf123, json!(2), 42).await;
    assert_eq!(recibir(&e, id_de(&recibido)).await.0, StatusCode::OK);
    let (_, pendiente) = pedir(&e, fa77, json!(1), 42).await;
    pedir(&e, e.pf123, json!(3), 43).await;

    let mut conexion = pool.acquire().await.unwrap();
    let pedidos = pedidos_de_orden(&mut conexion, e.orden).await.unwrap();

    let pedidos = serde_json::to_value(pedidos).unwrap();
    let pedidos = pedidos.as_array().unwrap();
    assert_eq!(pedidos.len(), 2);
    assert_eq!(pedidos[0]["id"], recibido["id"]);
    assert_eq!(pedidos[0]["estado"], "recibido");
    assert_eq!(pedidos[1]["id"], pendiente["id"]);
    assert_eq!(pedidos[1]["estado"], "pendiente");
}

/// El campo `pedidos` de la ficha de la orden. El endpoint es de spec-002; quitar el `#[ignore]`
/// cuando exista (T031).
#[sqlx::test]
#[ignore = "necesita GET /api/ordenes/{id} de spec-002"]
async fn la_ficha_de_la_orden_trae_sus_pedidos(pool: PgPool) {
    let e = escenario(&pool).await;
    let (_, pedido) = pedir(&e, e.pf123, json!(2), 42).await;
    let paco = entrar_como(&e.app, &pool, "paco", "jefe").await;

    let ruta = format!("/api/ordenes/{}", e.orden);
    let (codigo, orden, _) = comun::peticion(&e.app, "GET", &ruta, Some(&paco), None).await;

    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(orden["pedidos"].as_array().unwrap().len(), 1);
    assert_eq!(orden["pedidos"][0]["id"], pedido["id"]);
}
