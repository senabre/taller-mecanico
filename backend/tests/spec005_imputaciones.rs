//! Imputar tiempo y piezas, bloqueo del 10 %, ampliación y terminar (spec-005; plan, «Pruebas»;
//! contrato, sección spec-005).

mod comun;

use std::time::Duration;

use axum::Router;
use axum::http::StatusCode;
use serde_json::{Value, json};
use sqlx::PgPool;
use taller::estados::{CambioEstado, Estado};
use taller::eventos::Eventos;
use taller::rutas::imputaciones::resumen_imputaciones;
use taller::{app, fechas};
use tokio::sync::broadcast;

// ---------------------------------------------------------------------------------------------
// Utilidades propias de spec-005 (el resto, en `comun`)
// ---------------------------------------------------------------------------------------------

/// App de prueba con acceso a sus eventos (`app_prueba` crea su propio canal y no lo devuelve).
fn app_con_eventos(pool: &PgPool) -> (Router, Eventos) {
    let estado = comun::estado_prueba(pool.clone());
    let eventos = estado.eventos.clone();
    (app(estado), eventos)
}

async fn entrar_como(app: &Router, pool: &PgPool, login: &str, rol: &str) -> (i64, String) {
    let (id, contrasena) = comun::crear_usuario(pool, login, rol, true).await;
    (id, comun::entrar(app, login, &contrasena).await)
}

async fn insertar_pieza(
    pool: &PgPool,
    referencia: &str,
    descripcion: &str,
    precio: &str,
    stock: i32,
) -> i64 {
    sqlx::query_scalar(
        "INSERT INTO pieza (referencia, descripcion, precio_venta, stock)
         VALUES ($1, $2, $3::numeric, $4) RETURNING id",
    )
    .bind(referencia)
    .bind(descripcion)
    .bind(precio)
    .bind(stock)
    .fetch_one(pool)
    .await
    .unwrap()
}

/// Presupuesto `aceptado` por SQL, con una línea de mano de obra por el importe de la base.
async fn presupuesto_aceptado(pool: &PgPool, orden_id: i64, tipo: &str, base: &str) -> i64 {
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO presupuesto (orden_id, tipo, estado, iva_porcentaje, base, iva, total,
                                  emitido_en, caduca_el, respondido_en)
         VALUES ($1, $2::tipo_presupuesto, 'aceptado', 21.00, $3::numeric,
                 round($3::numeric * 0.21, 2), $3::numeric + round($3::numeric * 0.21, 2),
                 now(), current_date + 15, now())
         RETURNING id",
    )
    .bind(orden_id)
    .bind(tipo)
    .bind(base)
    .fetch_one(pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO linea_presupuesto (presupuesto_id, tipo, concepto, cantidad,
                                        precio_unitario, importe)
         VALUES ($1, 'mano_obra', 'Trabajo presupuestado', 1, $2::numeric, $2::numeric)",
    )
    .bind(id)
    .bind(base)
    .execute(pool)
    .await
    .unwrap();
    id
}

async fn stock_de(pool: &PgPool, pieza_id: i64) -> i32 {
    sqlx::query_scalar("SELECT stock FROM pieza WHERE id = $1")
        .bind(pieza_id)
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

async fn imputaciones_guardadas(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM imputacion")
        .fetch_one(pool)
        .await
        .unwrap()
}

/// Luis con sesión, la orden 42 suya en el estado dado y, si se indica, un presupuesto aceptado.
struct Escenario {
    app: Router,
    eventos: Eventos,
    luis_id: i64,
    luis: String,
    orden: i64,
}

async fn escenario(pool: &PgPool, estado: Estado, presupuestado: Option<&str>) -> Escenario {
    let (app, eventos) = app_con_eventos(pool);
    let (luis_id, luis) = entrar_como(&app, pool, "luis", "mecanico").await;
    let orden = comun::crear_orden(pool, 42, estado, luis_id).await;
    if let Some(base) = presupuestado {
        presupuesto_aceptado(pool, orden, "inicial", base).await;
    }
    Escenario {
        app,
        eventos,
        luis_id,
        luis,
        orden,
    }
}

async fn imputar(e: &Escenario, cookie: &str, linea: Value) -> (StatusCode, Value) {
    let ruta = format!("/api/ordenes/{}/imputaciones", e.orden);
    let (codigo, cuerpo, _) =
        comun::peticion(&e.app, "POST", &ruta, Some(cookie), Some(linea)).await;
    (codigo, cuerpo)
}

async fn terminar(e: &Escenario, cookie: &str) -> (StatusCode, Value) {
    let ruta = format!("/api/ordenes/{}/terminar", e.orden);
    let (codigo, cuerpo, _) = comun::peticion(&e.app, "POST", &ruta, Some(cookie), None).await;
    (codigo, cuerpo)
}

fn horas(concepto: &str, cantidad: &str) -> Value {
    json!({ "tipo": "mano_obra", "concepto": concepto, "cantidad": cantidad })
}

fn pieza(id: i64, unidades: &str) -> Value {
    json!({ "tipo": "pieza", "pieza_id": id, "cantidad": unidades })
}

async fn evento(tablero: &mut broadcast::Receiver<CambioEstado>) -> Value {
    let recibido = tokio::time::timeout(Duration::from_secs(1), tablero.recv())
        .await
        .expect("no llegó ningún evento al tablero")
        .unwrap();
    serde_json::to_value(recibido).unwrap()
}

fn sin_eventos(tablero: &mut broadcast::Receiver<CambioEstado>) -> bool {
    matches!(
        tablero.try_recv(),
        Err(broadcast::error::TryRecvError::Empty)
    )
}

// ---------------------------------------------------------------------------------------------
// US1 · Imputar tiempo y piezas (T011–T015)
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn imputar_pieza_descuenta_stock_y_suma_al_acumulado(pool: PgPool) {
    let e = escenario(&pool, Estado::EnReparacion, Some("110.00")).await;
    let pf123 = insertar_pieza(&pool, "PF-123", "Pastillas freno", "25.00", 10).await;

    let (codigo, r) = imputar(&e, &e.luis, pieza(pf123, "2")).await;
    assert_eq!(codigo, StatusCode::CREATED, "{r}");
    assert_eq!(stock_de(&pool, pf123).await, 8);
    assert_eq!(r["imputaciones"][0]["concepto"], "PF-123 Pastillas freno");
    assert_eq!(r["imputaciones"][0]["importe"], "50.00");

    let (codigo, r) = imputar(&e, &e.luis, horas("Cambio de pastillas", "1")).await;
    assert_eq!(codigo, StatusCode::CREATED, "{r}");
    assert_eq!(r["orden_id"], e.orden);
    assert_eq!(r["estado"], "en_reparacion");
    assert_eq!(r["coste_acumulado"], "90.00");
    assert_eq!(r["total_presupuestado"], "110.00");
    let lineas = r["imputaciones"].as_array().unwrap();
    assert_eq!(lineas.len(), 2);
    assert_eq!(lineas[1]["tipo"], "mano_obra");
    assert_eq!(lineas[1]["concepto"], "Cambio de pastillas");
    assert_eq!(lineas[1]["cantidad"], "1.00");
    assert_eq!(lineas[1]["precio_unitario"], "40.00");
    assert_eq!(lineas[1]["importe"], "40.00");

    let (usuario, hoy): (i64, bool) = sqlx::query_as(
        "SELECT usuario_id, (creada_en AT TIME ZONE 'Europe/Madrid')::date = $1
           FROM imputacion ORDER BY id LIMIT 1",
    )
    .bind(fechas::hoy())
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!((usuario, hoy), (e.luis_id, true));
}

#[sqlx::test]
async fn sin_stock_suficiente_no_se_imputa(pool: PgPool) {
    let e = escenario(&pool, Estado::EnReparacion, Some("110.00")).await;
    let pf123 = insertar_pieza(&pool, "PF-123", "Pastillas freno", "25.00", 1).await;

    let (codigo, cuerpo) = imputar(&e, &e.luis, pieza(pf123, "2")).await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        cuerpo["error"],
        "Solo hay 1 unidad en stock de PF-123; el almacén puede pedirla al proveedor"
    );
    assert_eq!(stock_de(&pool, pf123).await, 1);
    assert_eq!(imputaciones_guardadas(&pool).await, 0);

    let fa77 = insertar_pieza(&pool, "FA-77", "Filtro aire", "12.50", 0).await;
    let (codigo, cuerpo) = imputar(&e, &e.luis, pieza(fa77, "1")).await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        cuerpo["error"],
        "Solo hay 0 unidades en stock de FA-77; el almacén puede pedirla al proveedor"
    );
}

#[sqlx::test]
async fn imputaciones_no_validas_con_los_mensajes_del_presupuesto(pool: PgPool) {
    let e = escenario(&pool, Estado::EnReparacion, Some("110.00")).await;
    let pf123 = insertar_pieza(&pool, "PF-123", "Pastillas freno", "25.00", 10).await;

    let malas = [
        (
            horas("Cambio", "1.3"),
            "Las horas van en múltiplos de 0,25 (p. ej. 1,5)",
        ),
        (
            horas("Cambio", "0"),
            "Indica las horas: un número mayor que 0",
        ),
        (
            horas("Cambio", "-1"),
            "Indica las horas: un número mayor que 0",
        ),
        (
            horas("Cambio", ""),
            "Indica las horas: un número mayor que 0",
        ),
        (horas("", "1"), "Escribe el concepto de la mano de obra"),
        (horas("   ", "1"), "Escribe el concepto de la mano de obra"),
        (
            pieza(pf123, "2.5"),
            "Las unidades de pieza deben ser un número entero",
        ),
        (
            pieza(pf123, "0"),
            "Indica las unidades: un número mayor que 0",
        ),
        (
            pieza(999_999, "1"),
            "Elige una referencia de pieza dada de alta",
        ),
        (
            json!({ "tipo": "descuento", "cantidad": "1" }),
            "El tipo de línea debe ser mano de obra o pieza",
        ),
    ];
    for (linea, mensaje) in malas {
        let (codigo, cuerpo) = imputar(&e, &e.luis, linea.clone()).await;
        assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY, "{linea}");
        assert_eq!(cuerpo["error"], mensaje, "{linea}");
    }
    assert_eq!(imputaciones_guardadas(&pool).await, 0);
    assert_eq!(stock_de(&pool, pf123).await, 10);

    // Una pieza que no estaba en el presupuesto se admite.
    let fa77 = insertar_pieza(&pool, "FA-77", "Filtro aire", "12.50", 3).await;
    assert_eq!(
        imputar(&e, &e.luis, pieza(fa77, "1")).await.0,
        StatusCode::CREATED
    );

    // La base de datos también rechaza piezas no enteras (migración 0003).
    let directa = sqlx::query(
        "INSERT INTO imputacion (orden_id, tipo, concepto, pieza_id, cantidad, precio_unitario,
                                 importe, usuario_id)
         VALUES ($1, 'pieza', 'X', $2, 2.50, 25, 62.50, $3)",
    )
    .bind(e.orden)
    .bind(pf123)
    .bind(e.luis_id)
    .execute(&pool)
    .await;
    assert!(directa.is_err());
}

#[sqlx::test]
async fn solo_se_imputa_en_reparacion_con_presupuesto_aceptado(pool: PgPool) {
    let (app, _) = app_con_eventos(&pool);
    let (luis_id, luis) = entrar_como(&app, &pool, "luis", "mecanico").await;
    let estados = [
        (Estado::EnEsperaPiezas, "en espera de piezas"),
        (Estado::PendienteConformidad, "pendiente de conformidad"),
        (Estado::Terminada, "terminada"),
        (Estado::EnDiagnostico, "en diagnóstico"),
    ];
    for (numero, (estado, texto)) in (1..).zip(estados) {
        let orden = comun::crear_orden(&pool, numero, estado, luis_id).await;
        presupuesto_aceptado(&pool, orden, "inicial", "110.00").await;
        let ruta = format!("/api/ordenes/{orden}/imputaciones");
        let (codigo, cuerpo, _) =
            comun::peticion(&app, "POST", &ruta, Some(&luis), Some(horas("X", "1"))).await;
        assert_eq!(codigo, StatusCode::CONFLICT, "{texto}");
        assert_eq!(
            cuerpo["error"],
            format!("La orden está {texto}; solo se puede imputar en órdenes en reparación")
        );
    }

    let sin_presupuesto = comun::crear_orden(&pool, 42, Estado::EnReparacion, luis_id).await;
    let ruta = format!("/api/ordenes/{sin_presupuesto}/imputaciones");
    let (codigo, cuerpo, _) =
        comun::peticion(&app, "POST", &ruta, Some(&luis), Some(horas("X", "1"))).await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(cuerpo["error"], "La orden no tiene un presupuesto aceptado");
    assert_eq!(imputaciones_guardadas(&pool).await, 0);
}

#[sqlx::test]
async fn permisos_para_imputar(pool: PgPool) {
    let e = escenario(&pool, Estado::EnReparacion, Some("110.00")).await;

    let (_, marta) = entrar_como(&e.app, &pool, "marta", "mecanico").await;
    assert_eq!(
        imputar(&e, &marta, horas("X", "1")).await.0,
        StatusCode::FORBIDDEN
    );

    let ruta = format!("/api/ordenes/{}/imputaciones", e.orden);
    let (codigo, _, _) = comun::peticion(&e.app, "POST", &ruta, None, Some(horas("X", "1"))).await;
    assert_eq!(codigo, StatusCode::UNAUTHORIZED);

    let (codigo, _, _) = comun::peticion(
        &e.app,
        "POST",
        "/api/ordenes/999999/imputaciones",
        Some(&e.luis),
        Some(horas("X", "1")),
    )
    .await;
    assert_eq!(codigo, StatusCode::NOT_FOUND);
    assert_eq!(imputaciones_guardadas(&pool).await, 0);

    let (_, paco) = entrar_como(&e.app, &pool, "paco", "jefe").await;
    assert_eq!(
        imputar(&e, &paco, horas("Ajuste", "1")).await.0,
        StatusCode::CREATED
    );
}

#[sqlx::test]
async fn el_resumen_suma_solo_lo_aceptado_de_esa_orden(pool: PgPool) {
    let e = escenario(&pool, Estado::EnReparacion, Some("110.00")).await;
    imputar(&e, &e.luis, horas("Cambio de pastillas", "1")).await;
    // Ni los emitidos, ni los rechazados, ni los borradores cuentan como presupuestado.
    for estado in ["emitido", "rechazado", "borrador"] {
        sqlx::query(
            "INSERT INTO presupuesto (orden_id, tipo, estado, base)
             VALUES ($1, 'ampliacion', $2::estado_presupuesto, 999)",
        )
        .bind(e.orden)
        .bind(estado)
        .execute(&pool)
        .await
        .unwrap();
    }
    // Otra orden con su imputación no cuenta.
    let otra = comun::crear_orden(&pool, 43, Estado::EnReparacion, e.luis_id).await;
    presupuesto_aceptado(&pool, otra, "inicial", "500.00").await;
    let ruta = format!("/api/ordenes/{otra}/imputaciones");
    comun::peticion(
        &e.app,
        "POST",
        &ruta,
        Some(&e.luis),
        Some(horas("Otra", "2")),
    )
    .await;

    let mut conexion = pool.acquire().await.unwrap();
    let resumen = resumen_imputaciones(&mut conexion, e.orden).await.unwrap();
    let resumen = serde_json::to_value(resumen).unwrap();

    assert_eq!(resumen["coste_acumulado"], "40.00");
    assert_eq!(resumen["total_presupuestado"], "110.00");
    assert_eq!(resumen["imputaciones"].as_array().unwrap().len(), 1);
}

// ---------------------------------------------------------------------------------------------
// US2 · Bloqueo por desviación de más del 10 % (T018, T019)
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn pasar_del_110_por_ciento_bloquea_la_orden(pool: PgPool) {
    let e = escenario(&pool, Estado::EnReparacion, Some("100.00")).await;
    let mut tablero = e.eventos.suscribir();

    // 2,75 h × 40,00 = 110,00: justo el 110 %, no bloquea.
    let (codigo, r) = imputar(&e, &e.luis, horas("Cambio de pastillas", "2.75")).await;
    assert_eq!(codigo, StatusCode::CREATED);
    assert_eq!(r["coste_acumulado"], "110.00");
    assert_eq!(r["estado"], "en_reparacion");
    assert!(sin_eventos(&mut tablero));

    // 0,25 h más → 120,00: se guarda y la orden queda pendiente de conformidad.
    let (codigo, r) = imputar(&e, &e.luis, horas("Ajuste", "0.25")).await;
    assert_eq!(codigo, StatusCode::CREATED, "{r}");
    assert_eq!(r["coste_acumulado"], "120.00");
    assert_eq!(r["estado"], "pendiente_conformidad");
    assert_eq!(r["imputaciones"].as_array().unwrap().len(), 2);
    assert_eq!(
        estado_de_orden(&pool, e.orden).await,
        Estado::PendienteConformidad
    );
    assert_eq!(
        evento(&mut tablero).await,
        json!({ "id": e.orden, "estado": "pendiente_conformidad" })
    );

    // Las siguientes se rechazan y no tocan el stock.
    let pf123 = insertar_pieza(&pool, "PF-123", "Pastillas freno", "25.00", 5).await;
    let (codigo, _) = imputar(&e, &e.luis, pieza(pf123, "1")).await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(stock_de(&pool, pf123).await, 5);
    assert_eq!(imputaciones_guardadas(&pool).await, 2);
}

#[sqlx::test]
async fn el_limite_del_10_por_ciento_es_exacto_al_centimo(pool: PgPool) {
    let e = escenario(&pool, Estado::EnReparacion, Some("100.00")).await;
    let justa = insertar_pieza(&pool, "J-1", "Justa", "110.00", 1).await;
    let (_, r) = imputar(&e, &e.luis, pieza(justa, "1")).await;
    assert_eq!(r["estado"], "en_reparacion");

    let e2_orden = comun::crear_orden(&pool, 43, Estado::EnReparacion, e.luis_id).await;
    presupuesto_aceptado(&pool, e2_orden, "inicial", "100.00").await;
    let pasada = insertar_pieza(&pool, "P-1", "Pasada", "110.01", 1).await;
    let ruta = format!("/api/ordenes/{e2_orden}/imputaciones");
    let (_, r, _) = comun::peticion(
        &e.app,
        "POST",
        &ruta,
        Some(&e.luis),
        Some(pieza(pasada, "1")),
    )
    .await;
    assert_eq!(r["estado"], "pendiente_conformidad");
}

// ---------------------------------------------------------------------------------------------
// US3 · Ampliación del presupuesto y reanudación (T022, T023)
// ---------------------------------------------------------------------------------------------

/// La orden 42 bloqueada: 100,00 aceptados y 115,00 imputados.
async fn orden_bloqueada(pool: &PgPool) -> Escenario {
    let e = escenario(pool, Estado::EnReparacion, Some("100.00")).await;
    imputar(&e, &e.luis, horas("Cambio de pastillas", "2.5")).await;
    let pieza_cara = insertar_pieza(pool, "D-1", "Disco", "15.00", 1).await;
    let (_, r) = imputar(&e, &e.luis, pieza(pieza_cara, "1")).await;
    assert_eq!(r["estado"], "pendiente_conformidad");
    assert_eq!(r["coste_acumulado"], "115.00");
    e
}

/// Crea, rellena (1,25 h = 50,00) y emite una ampliación con los endpoints de spec-003.
async fn emitir_ampliacion(e: &Escenario) -> i64 {
    let ruta = format!("/api/ordenes/{}/presupuestos", e.orden);
    let (codigo, p, _) = comun::peticion(&e.app, "POST", &ruta, Some(&e.luis), None).await;
    assert_eq!(codigo, StatusCode::CREATED, "{p}");
    assert_eq!(p["tipo"], "ampliacion");
    let id = p["id"].as_i64().unwrap();
    let ruta = format!("/api/presupuestos/{id}/lineas");
    let linea = horas("Cambio de discos", "1.25");
    let (codigo, _, _) = comun::peticion(&e.app, "POST", &ruta, Some(&e.luis), Some(linea)).await;
    assert_eq!(codigo, StatusCode::CREATED);
    let ruta = format!("/api/presupuestos/{id}/emitir");
    let (codigo, p, _) = comun::peticion(&e.app, "POST", &ruta, Some(&e.luis), None).await;
    assert_eq!(codigo, StatusCode::OK, "{p}");
    id
}

/// Lo que hace spec-004 al aceptar una ampliación.
async fn simular_respuesta(pool: &PgPool, presupuesto: i64, orden: i64, acepta: bool) {
    let estado = if acepta { "aceptado" } else { "rechazado" };
    sqlx::query(
        "UPDATE presupuesto SET estado = $2::estado_presupuesto, respondido_en = now()
          WHERE id = $1",
    )
    .bind(presupuesto)
    .bind(estado)
    .execute(pool)
    .await
    .unwrap();
    if acepta {
        sqlx::query("UPDATE orden SET estado = 'en_reparacion' WHERE id = $1")
            .bind(orden)
            .execute(pool)
            .await
            .unwrap();
    }
}

#[sqlx::test]
async fn aceptar_la_ampliacion_suma_lo_presupuestado_y_reanuda(pool: PgPool) {
    let e = orden_bloqueada(&pool).await;
    let ampliacion = emitir_ampliacion(&e).await;

    // Emitida y sin respuesta: no se puede imputar.
    assert_eq!(
        imputar(&e, &e.luis, horas("X", "1")).await.0,
        StatusCode::CONFLICT
    );

    simular_respuesta(&pool, ampliacion, e.orden, true).await;
    let mut conexion = pool.acquire().await.unwrap();
    let resumen =
        serde_json::to_value(resumen_imputaciones(&mut conexion, e.orden).await.unwrap()).unwrap();
    assert_eq!(resumen["total_presupuestado"], "150.00");

    // 110 % de 150,00 = 165,00: hasta ahí se puede seguir.
    let (codigo, r) = imputar(&e, &e.luis, horas("Montaje", "1.25")).await;
    assert_eq!(codigo, StatusCode::CREATED);
    assert_eq!(r["coste_acumulado"], "165.00");
    assert_eq!(r["estado"], "en_reparacion");
    let centimo = insertar_pieza(&pool, "C-1", "Clip", "0.01", 1).await;
    let (_, r) = imputar(&e, &e.luis, pieza(centimo, "1")).await;
    assert_eq!(r["estado"], "pendiente_conformidad");
}

#[sqlx::test]
async fn con_la_ampliacion_rechazada_sigue_bloqueada(pool: PgPool) {
    let e = orden_bloqueada(&pool).await;
    let ampliacion = emitir_ampliacion(&e).await;
    simular_respuesta(&pool, ampliacion, e.orden, false).await;

    assert_eq!(
        estado_de_orden(&pool, e.orden).await,
        Estado::PendienteConformidad
    );
    assert_eq!(
        imputar(&e, &e.luis, horas("X", "1")).await.0,
        StatusCode::CONFLICT
    );
    assert_eq!(terminar(&e, &e.luis).await.0, StatusCode::CONFLICT);
    // Se puede preparar otra ampliación (límite conocido de la spec).
    let ruta = format!("/api/ordenes/{}/presupuestos", e.orden);
    let (codigo, _, _) = comun::peticion(&e.app, "POST", &ruta, Some(&e.luis), None).await;
    assert_eq!(codigo, StatusCode::CREATED);
}

// ---------------------------------------------------------------------------------------------
// US4 · Terminar la reparación (T025, T026)
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn terminar_una_orden_con_imputaciones(pool: PgPool) {
    let e = escenario(&pool, Estado::EnReparacion, Some("110.00")).await;
    imputar(&e, &e.luis, horas("Cambio de pastillas", "1.5")).await;
    let mut tablero = e.eventos.suscribir();

    let (codigo, r) = terminar(&e, &e.luis).await;

    assert_eq!(codigo, StatusCode::OK, "{r}");
    assert_eq!(r["estado"], "terminada");
    assert_eq!(r["coste_acumulado"], "60.00");
    let terminada: bool =
        sqlx::query_scalar("SELECT terminada_en IS NOT NULL FROM orden WHERE id = $1")
            .bind(e.orden)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(terminada);
    assert_eq!(
        evento(&mut tablero).await,
        json!({ "id": e.orden, "estado": "terminada" })
    );
    let (codigo, cuerpo) = imputar(&e, &e.luis, horas("Más", "1")).await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(
        cuerpo["error"],
        "La orden está terminada; solo se puede imputar en órdenes en reparación"
    );
}

#[sqlx::test]
async fn no_se_termina_sin_imputaciones_ni_fuera_de_reparacion(pool: PgPool) {
    let e = escenario(&pool, Estado::EnReparacion, Some("110.00")).await;
    let (codigo, cuerpo) = terminar(&e, &e.luis).await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        cuerpo["error"],
        "Primero hay que imputar el trabajo realizado"
    );
    assert_eq!(estado_de_orden(&pool, e.orden).await, Estado::EnReparacion);

    imputar(&e, &e.luis, horas("Cambio de pastillas", "1")).await;
    let (_, marta) = entrar_como(&e.app, &pool, "marta", "mecanico").await;
    assert_eq!(terminar(&e, &marta).await.0, StatusCode::FORBIDDEN);

    for (estado, texto) in [
        ("pendiente_conformidad", "pendiente de conformidad"),
        ("en_espera_piezas", "en espera de piezas"),
    ] {
        sqlx::query("UPDATE orden SET estado = $2::estado_orden WHERE id = $1")
            .bind(e.orden)
            .bind(estado)
            .execute(&pool)
            .await
            .unwrap();
        let (codigo, cuerpo) = terminar(&e, &e.luis).await;
        assert_eq!(codigo, StatusCode::CONFLICT);
        assert_eq!(
            cuerpo["error"],
            format!("La orden está {texto}; solo se puede terminar una orden en reparación")
        );
    }

    sqlx::query("UPDATE orden SET estado = 'en_reparacion' WHERE id = $1")
        .bind(e.orden)
        .execute(&pool)
        .await
        .unwrap();
    let (_, paco) = entrar_como(&e.app, &pool, "paco", "jefe").await;
    assert_eq!(terminar(&e, &paco).await.0, StatusCode::OK);
}
