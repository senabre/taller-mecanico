//! Diagnóstico y presupuesto (spec-003; plan, «Pruebas»; contrato, sección spec-003).

mod comun;

use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::http::StatusCode;
use chrono::{DateTime, NaiveDate, Utc};
use serde_json::{Value, json};
use sqlx::PgPool;
use taller::correo::Correo;
use taller::estados::{CambioEstado, Estado};
use taller::eventos::Eventos;
use taller::rutas::presupuestos::presupuestos_de_orden;
use taller::{EstadoApp, app, fechas};
use tokio::sync::broadcast;

// ---------------------------------------------------------------------------------------------
// Utilidades propias de spec-003 (el resto, en `comun`)
// ---------------------------------------------------------------------------------------------

/// App de prueba con acceso a sus eventos (`app_prueba` crea su propio canal y no lo devuelve).
fn app_con_eventos(pool: &PgPool) -> (Router, Eventos) {
    let estado = comun::estado_prueba(pool.clone());
    let eventos = estado.eventos.clone();
    (app(estado), eventos)
}

/// App cuyo SMTP apunta a un puerto cerrado: el correo siempre falla.
fn app_sin_correo(pool: &PgPool) -> Router {
    let mut estado: EstadoApp = comun::estado_prueba(pool.clone());
    let mut config = (*estado.config).clone();
    config.smtp_url = "smtp://127.0.0.1:9".to_string();
    estado.correo = Correo::desde_config(&config).unwrap();
    estado.config = Arc::new(config);
    app(estado)
}

async fn entrar_como(app: &Router, pool: &PgPool, login: &str, rol: &str) -> (i64, String) {
    let (id, contrasena) = comun::crear_usuario(pool, login, rol, true).await;
    (id, comun::entrar(app, login, &contrasena).await)
}

async fn insertar_pieza(pool: &PgPool, referencia: &str, descripcion: &str, precio: &str) -> i64 {
    sqlx::query_scalar(
        "INSERT INTO pieza (referencia, descripcion, precio_venta, stock)
         VALUES ($1, $2, $3::numeric, 0) RETURNING id",
    )
    .bind(referencia)
    .bind(descripcion)
    .bind(precio)
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

async fn diagnostico_de(pool: &PgPool, id: i64) -> Option<String> {
    sqlx::query_scalar("SELECT diagnostico FROM orden WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

/// Presupuesto por SQL directo, sin líneas, en el estado y con la caducidad dados.
async fn insertar_presupuesto(
    pool: &PgPool,
    orden_id: i64,
    estado: &str,
    caduca_el: Option<NaiveDate>,
) -> i64 {
    sqlx::query_scalar(
        "INSERT INTO presupuesto (orden_id, tipo, estado, caduca_el, emitido_en)
         VALUES ($1, 'inicial', $2::estado_presupuesto, $3,
                 CASE WHEN $2 = 'borrador' THEN NULL ELSE now() END)
         RETURNING id",
    )
    .bind(orden_id)
    .bind(estado)
    .bind(caduca_el)
    .fetch_one(pool)
    .await
    .unwrap()
}

fn hoy() -> NaiveDate {
    fechas::hoy()
}

fn ayer() -> NaiveDate {
    fechas::hoy().pred_opt().unwrap()
}

/// Luis con sesión y una orden suya (nº 42) en el estado dado.
struct Escenario {
    app: Router,
    eventos: Eventos,
    luis: String,
    orden: i64,
}

async fn escenario(pool: &PgPool, estado: Estado) -> Escenario {
    let (app, eventos) = app_con_eventos(pool);
    let (luis_id, luis) = entrar_como(&app, pool, "luis", "mecanico").await;
    let orden = comun::crear_orden(pool, 42, estado, luis_id).await;
    Escenario {
        app,
        eventos,
        luis,
        orden,
    }
}

async fn diagnosticar(e: &Escenario, cookie: &str, texto: &str) -> (StatusCode, Value) {
    let ruta = format!("/api/ordenes/{}/diagnostico", e.orden);
    let (codigo, cuerpo, _) = comun::peticion(
        &e.app,
        "PUT",
        &ruta,
        Some(cookie),
        Some(json!({ "diagnostico": texto })),
    )
    .await;
    (codigo, cuerpo)
}

async fn nuevo_presupuesto(e: &Escenario, cookie: &str) -> (StatusCode, Value) {
    let ruta = format!("/api/ordenes/{}/presupuestos", e.orden);
    let (codigo, cuerpo, _) = comun::peticion(&e.app, "POST", &ruta, Some(cookie), None).await;
    (codigo, cuerpo)
}

async fn anadir(
    e: &Escenario,
    cookie: &str,
    presupuesto: i64,
    linea: Value,
) -> (StatusCode, Value) {
    let ruta = format!("/api/presupuestos/{presupuesto}/lineas");
    let (codigo, cuerpo, _) =
        comun::peticion(&e.app, "POST", &ruta, Some(cookie), Some(linea)).await;
    (codigo, cuerpo)
}

async fn quitar(e: &Escenario, cookie: &str, presupuesto: i64, linea: i64) -> (StatusCode, Value) {
    let ruta = format!("/api/presupuestos/{presupuesto}/lineas/{linea}");
    let (codigo, cuerpo, _) = comun::peticion(&e.app, "DELETE", &ruta, Some(cookie), None).await;
    (codigo, cuerpo)
}

async fn emitir(app: &Router, cookie: &str, presupuesto: i64) -> (StatusCode, Value) {
    let ruta = format!("/api/presupuestos/{presupuesto}/emitir");
    let (codigo, cuerpo, _) = comun::peticion(app, "POST", &ruta, Some(cookie), None).await;
    (codigo, cuerpo)
}

fn mano_obra(concepto: &str, horas: &str) -> Value {
    json!({ "tipo": "mano_obra", "concepto": concepto, "cantidad": horas })
}

fn pieza(id: i64, unidades: &str) -> Value {
    json!({ "tipo": "pieza", "pieza_id": id, "cantidad": unidades })
}

fn id_de(cuerpo: &Value) -> i64 {
    cuerpo["id"].as_i64().unwrap()
}

/// El presupuesto del ejemplo de la spec: 1,5 h de «Cambio de pastillas» y PF-123 × 2.
async fn presupuesto_del_ejemplo(pool: &PgPool, e: &Escenario) -> (i64, Value) {
    let pf123 = insertar_pieza(pool, "PF-123", "Pastillas freno", "25.00").await;
    let (_, p) = nuevo_presupuesto(e, &e.luis).await;
    let id = id_de(&p);
    assert_eq!(
        anadir(e, &e.luis, id, mano_obra("Cambio de pastillas", "1.5"))
            .await
            .0,
        StatusCode::CREATED
    );
    let (codigo, p) = anadir(e, &e.luis, id, pieza(pf123, "2")).await;
    assert_eq!(codigo, StatusCode::CREATED, "{p}");
    (id, p)
}

fn totales(p: &Value) -> (&str, &str, &str) {
    (
        p["base"].as_str().unwrap(),
        p["iva"].as_str().unwrap(),
        p["total"].as_str().unwrap(),
    )
}

async fn evento(tablero: &mut broadcast::Receiver<CambioEstado>) -> Value {
    let recibido = tokio::time::timeout(Duration::from_secs(1), tablero.recv())
        .await
        .expect("no llegó ningún evento al tablero")
        .unwrap();
    serde_json::to_value(recibido).unwrap()
}

// ---------------------------------------------------------------------------------------------
// US1 · Registrar el diagnóstico (T010, T011)
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn diagnostico_en_orden_recibida_la_pasa_a_en_diagnostico(pool: PgPool) {
    let e = escenario(&pool, Estado::Recibida).await;
    let mut tablero = e.eventos.suscribir();

    let (codigo, orden) = diagnosticar(&e, &e.luis, "  Pastillas delanteras desgastadas ").await;

    assert_eq!(codigo, StatusCode::OK, "{orden}");
    assert_eq!(
        orden,
        json!({ "id": e.orden, "numero": 42, "estado": "en_diagnostico",
                "diagnostico": "Pastillas delanteras desgastadas" })
    );
    assert_eq!(estado_de_orden(&pool, e.orden).await, Estado::EnDiagnostico);
    assert_eq!(
        evento(&mut tablero).await,
        json!({ "id": e.orden, "estado": "en_diagnostico" })
    );
}

#[sqlx::test]
async fn el_diagnostico_se_puede_cambiar_mientras_no_haya_presupuesto_emitido(pool: PgPool) {
    let e = escenario(&pool, Estado::Recibida).await;
    assert_eq!(
        diagnosticar(&e, &e.luis, "Pastillas delanteras desgastadas")
            .await
            .0,
        StatusCode::OK
    );
    insertar_presupuesto(&pool, e.orden, "borrador", None).await;
    let mut tablero = e.eventos.suscribir();

    let (codigo, orden) =
        diagnosticar(&e, &e.luis, "Pastillas y discos delanteros desgastados").await;

    assert_eq!(codigo, StatusCode::OK, "{orden}");
    assert_eq!(orden["estado"], "en_diagnostico");
    assert_eq!(
        diagnostico_de(&pool, e.orden).await.as_deref(),
        Some("Pastillas y discos delanteros desgastados")
    );
    // No cambia de estado: no hay nada que avisar al tablero.
    assert!(matches!(
        tablero.try_recv(),
        Err(broadcast::error::TryRecvError::Empty)
    ));
}

#[sqlx::test]
async fn con_un_presupuesto_emitido_el_diagnostico_queda_fijo(pool: PgPool) {
    for estado in ["emitido", "rechazado"] {
        let e = escenario(&pool, Estado::Recibida).await;
        assert_eq!(
            diagnosticar(&e, &e.luis, "Original").await.0,
            StatusCode::OK
        );
        insertar_presupuesto(&pool, e.orden, estado, Some(hoy())).await;

        let (codigo, cuerpo) = diagnosticar(&e, &e.luis, "Otro").await;

        assert_eq!(codigo, StatusCode::CONFLICT, "con un presupuesto {estado}");
        assert_eq!(
            cuerpo["error"],
            "Ya se ha emitido un presupuesto para esta orden; el diagnóstico no se puede cambiar"
        );
        assert_eq!(
            diagnostico_de(&pool, e.orden).await.as_deref(),
            Some("Original")
        );
        // Limpia para la siguiente vuelta (el login y el nº de orden son únicos).
        sqlx::query(
            "TRUNCATE presupuesto, orden, cita, vehiculo, cliente, sesion, usuario CASCADE",
        )
        .execute(&pool)
        .await
        .unwrap();
    }
}

#[sqlx::test]
async fn diagnostico_no_valido_o_sin_permiso(pool: PgPool) {
    let e = escenario(&pool, Estado::Recibida).await;

    for vacio in ["", "   "] {
        let (codigo, _) = diagnosticar(&e, &e.luis, vacio).await;
        assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY);
    }

    let (_, marta) = entrar_como(&e.app, &pool, "marta", "mecanico").await;
    assert_eq!(
        diagnosticar(&e, &marta, "Algo").await.0,
        StatusCode::FORBIDDEN
    );

    let ruta = format!("/api/ordenes/{}/diagnostico", e.orden);
    let (codigo, _, _) = comun::peticion(
        &e.app,
        "PUT",
        &ruta,
        None,
        Some(json!({ "diagnostico": "Algo" })),
    )
    .await;
    assert_eq!(codigo, StatusCode::UNAUTHORIZED);
    assert_eq!(estado_de_orden(&pool, e.orden).await, Estado::Recibida);

    let (_, paco) = entrar_como(&e.app, &pool, "paco", "jefe").await;
    assert_eq!(
        diagnosticar(&e, &paco, "Lo hace el jefe").await.0,
        StatusCode::OK
    );
}

#[sqlx::test]
async fn diagnostico_de_una_orden_en_reparacion_da_409(pool: PgPool) {
    let e = escenario(&pool, Estado::EnReparacion).await;

    let (codigo, cuerpo) = diagnosticar(&e, &e.luis, "Algo").await;

    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(
        cuerpo["error"],
        "La orden está en reparación; el diagnóstico solo se registra en órdenes recibidas o en \
         diagnóstico"
    );
}

// ---------------------------------------------------------------------------------------------
// US2 · Preparar el presupuesto línea a línea (T014–T019)
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn crear_presupuesto_segun_el_estado_de_la_orden(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    let (codigo, p) = nuevo_presupuesto(&e, &e.luis).await;
    assert_eq!(codigo, StatusCode::CREATED, "{p}");
    assert_eq!(p["tipo"], "inicial");
    assert_eq!(p["estado"], "borrador");
    assert_eq!(p["orden_id"], e.orden);
    assert_eq!(p["lineas"], json!([]));
    assert_eq!(p["enlace"], Value::Null);
    assert_eq!(p["caducado"], false);
    assert_eq!(totales(&p), ("0.00", "0.00", "0.00"));

    let luis: i64 = sqlx::query_scalar("SELECT id FROM usuario WHERE login = 'luis'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let ampliable = comun::crear_orden(&pool, 43, Estado::PendienteConformidad, luis).await;
    let ruta = format!("/api/ordenes/{ampliable}/presupuestos");
    let (codigo, p, _) = comun::peticion(&e.app, "POST", &ruta, Some(&e.luis), None).await;
    assert_eq!(codigo, StatusCode::CREATED);
    assert_eq!(p["tipo"], "ampliacion");

    let recibida = comun::crear_orden(&pool, 44, Estado::Recibida, luis).await;
    let ruta = format!("/api/ordenes/{recibida}/presupuestos");
    let (codigo, p, _) = comun::peticion(&e.app, "POST", &ruta, Some(&e.luis), None).await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(
        p["error"],
        "La orden está recibida; no se puede preparar un presupuesto"
    );
}

#[sqlx::test]
async fn como_mucho_un_presupuesto_abierto_por_orden(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    assert_eq!(nuevo_presupuesto(&e, &e.luis).await.0, StatusCode::CREATED);

    let (codigo, cuerpo) = nuevo_presupuesto(&e, &e.luis).await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(
        cuerpo["error"],
        "Ya hay un presupuesto en borrador para esta orden"
    );

    sqlx::query("DELETE FROM presupuesto")
        .execute(&pool)
        .await
        .unwrap();
    let vigente = insertar_presupuesto(&pool, e.orden, "emitido", Some(hoy())).await;
    let (codigo, cuerpo) = nuevo_presupuesto(&e, &e.luis).await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(
        cuerpo["error"],
        "Ya hay un presupuesto emitido sin caducar para esta orden"
    );

    // Caducado ayer: ya no cuenta.
    sqlx::query("UPDATE presupuesto SET caduca_el = $2 WHERE id = $1")
        .bind(vigente)
        .bind(ayer())
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(nuevo_presupuesto(&e, &e.luis).await.0, StatusCode::CREATED);
}

#[sqlx::test]
async fn otro_mecanico_no_puede_preparar_el_presupuesto(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    let (_, marta) = entrar_como(&e.app, &pool, "marta", "mecanico").await;
    assert_eq!(nuevo_presupuesto(&e, &marta).await.0, StatusCode::FORBIDDEN);

    let (_, p) = nuevo_presupuesto(&e, &e.luis).await;
    let id = id_de(&p);
    assert_eq!(
        anadir(&e, &marta, id, mano_obra("X", "1")).await.0,
        StatusCode::FORBIDDEN
    );
}

#[sqlx::test]
async fn lineas_importes_y_totales_del_ejemplo(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;

    let (_, p) = presupuesto_del_ejemplo(&pool, &e).await;

    let lineas = p["lineas"].as_array().unwrap();
    assert_eq!(lineas.len(), 2);
    assert_eq!(lineas[0]["tipo"], "mano_obra");
    assert_eq!(lineas[0]["concepto"], "Cambio de pastillas");
    assert_eq!(lineas[0]["pieza_id"], Value::Null);
    assert_eq!(lineas[0]["cantidad"], "1.50");
    assert_eq!(lineas[0]["precio_unitario"], "40.00");
    assert_eq!(lineas[0]["importe"], "60.00");
    // El concepto de una pieza lo pone el servidor; la pieza no tiene stock y se admite.
    assert_eq!(lineas[1]["tipo"], "pieza");
    assert_eq!(lineas[1]["concepto"], "PF-123 Pastillas freno");
    assert_eq!(lineas[1]["cantidad"], "2.00");
    assert_eq!(lineas[1]["precio_unitario"], "25.00");
    assert_eq!(lineas[1]["importe"], "50.00");
    assert_eq!(p["iva_porcentaje"], "21.00");
    assert_eq!(totales(&p), ("110.00", "23.10", "133.10"));
}

#[sqlx::test]
async fn el_concepto_enviado_para_una_pieza_se_ignora(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    let pf123 = insertar_pieza(&pool, "PF-123", "Pastillas freno", "25.00").await;
    let (_, p) = nuevo_presupuesto(&e, &e.luis).await;

    let mut linea = pieza(pf123, "1");
    linea["concepto"] = json!("Otra cosa");
    let (codigo, p) = anadir(&e, &e.luis, id_de(&p), linea).await;

    assert_eq!(codigo, StatusCode::CREATED);
    assert_eq!(p["lineas"][0]["concepto"], "PF-123 Pastillas freno");
}

#[sqlx::test]
async fn lineas_no_validas_dan_422(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    let pf123 = insertar_pieza(&pool, "PF-123", "Pastillas freno", "25.00").await;
    let (_, p) = nuevo_presupuesto(&e, &e.luis).await;
    let id = id_de(&p);

    let malas = [
        mano_obra("Cambio", "1.3"),
        mano_obra("Cambio", "0"),
        mano_obra("Cambio", "-1"),
        mano_obra("Cambio", "abc"),
        mano_obra("Cambio", ""),
        json!({ "tipo": "mano_obra", "concepto": "Cambio" }),
        mano_obra("", "1"),
        mano_obra("   ", "1"),
        pieza(pf123, "2.5"),
        pieza(pf123, "0"),
        pieza(999_999, "1"),
        json!({ "tipo": "pieza", "cantidad": "1" }),
        json!({ "tipo": "descuento", "concepto": "X", "cantidad": "1" }),
    ];
    for linea in malas {
        let (codigo, cuerpo) = anadir(&e, &e.luis, id, linea.clone()).await;
        assert_eq!(
            codigo,
            StatusCode::UNPROCESSABLE_ENTITY,
            "{linea}: {cuerpo}"
        );
        assert!(cuerpo["error"].is_string());
    }

    let (_, cuerpo) = anadir(&e, &e.luis, id, mano_obra("Cambio", "1.3")).await;
    assert_eq!(
        cuerpo["error"],
        "Las horas van en múltiplos de 0,25 (p. ej. 1,5)"
    );
    let (_, cuerpo) = anadir(&e, &e.luis, id, pieza(pf123, "2.5")).await;
    assert_eq!(
        cuerpo["error"],
        "Las unidades de pieza deben ser un número entero"
    );

    let lineas: i64 = sqlx::query_scalar("SELECT count(*) FROM linea_presupuesto")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(lineas, 0);
}

#[sqlx::test]
async fn la_base_de_datos_tambien_rechaza_cantidades_no_validas(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    let pf123 = insertar_pieza(&pool, "PF-123", "Pastillas freno", "25.00").await;
    let p = insertar_presupuesto(&pool, e.orden, "borrador", None).await;

    let horas = sqlx::query(
        "INSERT INTO linea_presupuesto (presupuesto_id, tipo, concepto, cantidad,
                                        precio_unitario, importe)
         VALUES ($1, 'mano_obra', 'X', 1.30, 40, 52)",
    )
    .bind(p)
    .execute(&pool)
    .await;
    assert!(horas.is_err());

    let unidades = sqlx::query(
        "INSERT INTO linea_presupuesto (presupuesto_id, tipo, concepto, pieza_id, cantidad,
                                        precio_unitario, importe)
         VALUES ($1, 'pieza', 'X', $2, 2.50, 25, 62.50)",
    )
    .bind(p)
    .bind(pf123)
    .execute(&pool)
    .await;
    assert!(unidades.is_err());
}

#[sqlx::test]
async fn quitar_una_linea_recalcula_los_totales(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    let (id, p) = presupuesto_del_ejemplo(&pool, &e).await;
    let mano_de_obra = id_de(&p["lineas"][0]);

    let (codigo, p) = quitar(&e, &e.luis, id, mano_de_obra).await;

    assert_eq!(codigo, StatusCode::OK, "{p}");
    assert_eq!(p["lineas"].as_array().unwrap().len(), 1);
    assert_eq!(p["lineas"][0]["tipo"], "pieza");
    assert_eq!(totales(&p), ("50.00", "10.50", "60.50"));

    // Una línea de otro presupuesto (o ya quitada) → 404.
    let (codigo, _) = quitar(&e, &e.luis, id, mano_de_obra).await;
    assert_eq!(codigo, StatusCode::NOT_FOUND);

    let (_, marta) = entrar_como(&e.app, &pool, "marta", "mecanico").await;
    let pieza_id = id_de(&p["lineas"][0]);
    assert_eq!(
        quitar(&e, &marta, id, pieza_id).await.0,
        StatusCode::FORBIDDEN
    );
}

#[sqlx::test]
async fn el_iva_se_redondea_una_vez_sobre_la_base(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    let a = insertar_pieza(&pool, "A-1", "Arandela", "0.35").await;
    let b = insertar_pieza(&pool, "B-1", "Brida", "1.15").await;
    let c = insertar_pieza(&pool, "C-1", "Casquillo", "2.50").await;
    let (_, p) = nuevo_presupuesto(&e, &e.luis).await;
    let id = id_de(&p);

    anadir(&e, &e.luis, id, mano_obra("Ajuste", "0.25")).await;
    anadir(&e, &e.luis, id, pieza(a, "3")).await;
    let (_, p) = anadir(&e, &e.luis, id, pieza(b, "7")).await;
    // 10,00 + 1,05 + 8,05 = 19,10; IVA 4,011 → 4,01.
    assert_eq!(totales(&p), ("19.10", "4.01", "23.11"));

    sqlx::query("DELETE FROM linea_presupuesto")
        .execute(&pool)
        .await
        .unwrap();
    let (_, p) = anadir(&e, &e.luis, id, pieza(c, "1")).await;
    // IVA 0,525 → 0,53 (mitad hacia arriba).
    assert_eq!(totales(&p), ("2.50", "0.53", "3.03"));
}

#[sqlx::test]
async fn cualquier_rol_con_sesion_puede_leer_el_presupuesto(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    let (id, _) = presupuesto_del_ejemplo(&pool, &e).await;
    let (_, pepe) = entrar_como(&e.app, &pool, "pepe", "almacen").await;

    let ruta = format!("/api/presupuestos/{id}");
    let (codigo, p, _) = comun::peticion(&e.app, "GET", &ruta, Some(&pepe), None).await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(totales(&p), ("110.00", "23.10", "133.10"));

    let (codigo, _, _) =
        comun::peticion(&e.app, "GET", "/api/presupuestos/999999", Some(&pepe), None).await;
    assert_eq!(codigo, StatusCode::NOT_FOUND);
    let (codigo, _, _) = comun::peticion(&e.app, "GET", &ruta, None, None).await;
    assert_eq!(codigo, StatusCode::UNAUTHORIZED);
}

// ---------------------------------------------------------------------------------------------
// US3 · Emitir el presupuesto (T025–T027)
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn emitir_fija_totales_caducidad_y_token(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    let (id, _) = presupuesto_del_ejemplo(&pool, &e).await;

    let (codigo, p) = emitir(&e.app, &e.luis, id).await;

    assert_eq!(codigo, StatusCode::OK, "{p}");
    assert_eq!(p["estado"], "emitido");
    let quince_dias = hoy() + chrono::Days::new(15);
    assert_eq!(p["caduca_el"], quince_dias.format("%Y-%m-%d").to_string());
    let emitido: DateTime<Utc> = p["emitido_en"].as_str().unwrap().parse().unwrap();
    assert_eq!(fechas::fecha_en_madrid(emitido), hoy());
    assert!(p["aviso_enviado"].is_boolean());
    assert_eq!(p["caducado"], false);

    let enlace = p["enlace"].as_str().unwrap();
    let token = enlace
        .strip_prefix("http://localhost:3000/presupuesto/")
        .unwrap();
    assert_eq!(token.len(), 43);
    assert!(
        token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    );

    let guardado: (String, String, String, String, Option<String>) = sqlx::query_as(
        "SELECT base::text, iva::text, total::text, iva_porcentaje::text, token
           FROM presupuesto WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        guardado,
        (
            "110.00".to_string(),
            "23.10".to_string(),
            "133.10".to_string(),
            "21.00".to_string(),
            Some(token.to_string())
        )
    );
}

#[sqlx::test]
async fn un_presupuesto_emitido_no_se_puede_modificar(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    let (id, p) = presupuesto_del_ejemplo(&pool, &e).await;
    let linea = id_de(&p["lineas"][0]);
    assert_eq!(emitir(&e.app, &e.luis, id).await.0, StatusCode::OK);

    let (codigo, cuerpo) = anadir(&e, &e.luis, id, mano_obra("Más", "1")).await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(
        cuerpo["error"],
        "El presupuesto ya está emitido; no se puede modificar"
    );

    let (codigo, cuerpo) = quitar(&e, &e.luis, id, linea).await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(
        cuerpo["error"],
        "El presupuesto ya está emitido; no se puede modificar"
    );

    assert_eq!(emitir(&e.app, &e.luis, id).await.0, StatusCode::CONFLICT);
}

#[sqlx::test]
async fn no_se_emite_sin_lineas_ni_por_otro_mecanico(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    let (_, p) = nuevo_presupuesto(&e, &e.luis).await;
    let id = id_de(&p);

    let (codigo, cuerpo) = emitir(&e.app, &e.luis, id).await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(cuerpo["error"], "El presupuesto no tiene líneas");

    anadir(&e, &e.luis, id, mano_obra("Cambio", "1")).await;
    let (_, marta) = entrar_como(&e.app, &pool, "marta", "mecanico").await;
    assert_eq!(emitir(&e.app, &marta, id).await.0, StatusCode::FORBIDDEN);
}

#[sqlx::test]
async fn si_el_correo_falla_el_presupuesto_queda_emitido(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    let (id, _) = presupuesto_del_ejemplo(&pool, &e).await;
    let sin_correo = app_sin_correo(&pool);

    let (codigo, p) = emitir(&sin_correo, &e.luis, id).await;

    assert_eq!(codigo, StatusCode::OK, "{p}");
    assert_eq!(p["estado"], "emitido");
    assert_eq!(p["aviso_enviado"], false);
}

#[sqlx::test]
async fn presupuestos_de_una_orden_con_su_caducidad(pool: PgPool) {
    let e = escenario(&pool, Estado::EnDiagnostico).await;
    let caducado = insertar_presupuesto(&pool, e.orden, "emitido", Some(ayer())).await;
    let vigente = insertar_presupuesto(&pool, e.orden, "emitido", Some(hoy())).await;
    let luis: i64 = sqlx::query_scalar("SELECT id FROM usuario WHERE login = 'luis'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let otra = comun::crear_orden(&pool, 43, Estado::EnDiagnostico, luis).await;
    insertar_presupuesto(&pool, otra, "borrador", None).await;

    let mut conexion = pool.acquire().await.unwrap();
    let lista = presupuestos_de_orden(&mut conexion, e.orden, rust_decimal::Decimal::new(21, 0))
        .await
        .unwrap();

    let lista = serde_json::to_value(lista).unwrap();
    let lista = lista.as_array().unwrap();
    assert_eq!(lista.len(), 2);
    assert_eq!(lista[0]["id"], caducado);
    assert_eq!(lista[0]["caducado"], true);
    assert_eq!(lista[1]["id"], vigente);
    assert_eq!(lista[1]["caducado"], false);
    for campo in ["tipo", "estado", "total", "emitido_en", "caduca_el"] {
        assert!(lista[0].get(campo).is_some(), "falta {campo}");
    }
}
