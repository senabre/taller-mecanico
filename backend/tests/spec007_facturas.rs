//! Facturar, avisar al cliente e historial por matrícula (spec-007; plan, «Pruebas»; contrato,
//! sección spec-007).

mod comun;

use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::http::StatusCode;
use serde_json::{Value, json};
use sqlx::PgPool;
use taller::correo::Correo;
use taller::estados::{CambioEstado, Estado};
use taller::eventos::Eventos;
use taller::rutas::facturas::factura_de_orden;
use taller::{EstadoApp, app, fechas};
use tokio::sync::broadcast;

// ---------------------------------------------------------------------------------------------
// Utilidades propias de spec-007 (el resto, en `comun`)
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

async fn entrar_como(app: &Router, pool: &PgPool, login: &str, rol: &str) -> String {
    let (_, contrasena) = comun::crear_usuario(pool, login, rol, true).await;
    comun::entrar(app, login, &contrasena).await
}

/// Imputación por SQL directo, con su importe (cantidad × precio).
async fn imputar_sql(
    pool: &PgPool,
    orden_id: i64,
    tipo: &str,
    concepto: &str,
    pieza_id: Option<i64>,
    cantidad: &str,
    precio: &str,
) {
    sqlx::query(
        "INSERT INTO imputacion (orden_id, tipo, concepto, pieza_id, cantidad, precio_unitario,
                                 importe, usuario_id)
         SELECT $1, $2::tipo_linea, $3, $4, $5::numeric, $6::numeric, $5::numeric * $6::numeric,
                mecanico_id
           FROM orden WHERE id = $1",
    )
    .bind(orden_id)
    .bind(tipo)
    .bind(concepto)
    .bind(pieza_id)
    .bind(cantidad)
    .bind(precio)
    .execute(pool)
    .await
    .unwrap();
}

async fn estado_de_orden(pool: &PgPool, id: i64) -> Estado {
    sqlx::query_scalar("SELECT estado FROM orden WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn contador_factura(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT ultimo FROM contador WHERE nombre = 'factura'")
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn facturas_guardadas(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM factura")
        .fetch_one(pool)
        .await
        .unwrap()
}

/// Orden `terminada` con las imputaciones del ejemplo de la spec: 2 h a 40,00 y PF-123 × 2 a 25,00.
async fn orden_terminada(pool: &PgPool, numero: i64, mecanico_id: i64) -> i64 {
    let orden = comun::crear_orden(pool, numero, Estado::Terminada, mecanico_id).await;
    let pf123: i64 = sqlx::query_scalar(
        "INSERT INTO pieza (referencia, descripcion, precio_venta) VALUES ($1, 'Pastillas freno', 25)
         ON CONFLICT (referencia) DO UPDATE SET descripcion = excluded.descripcion RETURNING id",
    )
    .bind("PF-123")
    .fetch_one(pool)
    .await
    .unwrap();
    imputar_sql(
        pool,
        orden,
        "mano_obra",
        "Cambio de pastillas",
        None,
        "2",
        "40",
    )
    .await;
    imputar_sql(
        pool,
        orden,
        "pieza",
        "PF-123 Pastillas freno",
        Some(pf123),
        "2",
        "25",
    )
    .await;
    orden
}

/// Ana (recepción) con sesión, Luis (mecánico) y la orden 42 terminada con el ejemplo.
struct Escenario {
    app: Router,
    eventos: Eventos,
    ana: String,
    luis_id: i64,
    orden: i64,
}

async fn escenario(pool: &PgPool) -> Escenario {
    let (app, eventos) = app_con_eventos(pool);
    let ana = entrar_como(&app, pool, "ana", "recepcion").await;
    let luis_id = comun::insertar_usuario(pool, "luis", "mecanico").await;
    let orden = orden_terminada(pool, 42, luis_id).await;
    Escenario {
        app,
        eventos,
        ana,
        luis_id,
        orden,
    }
}

async fn facturar(app: &Router, cookie: &str, orden: i64) -> (StatusCode, Value) {
    let ruta = format!("/api/ordenes/{orden}/factura");
    let (codigo, cuerpo, _) = comun::peticion(app, "POST", &ruta, Some(cookie), None).await;
    (codigo, cuerpo)
}

async fn historial(app: &Router, cookie: Option<&str>, matricula: &str) -> (StatusCode, Value) {
    let ruta = format!("/api/vehiculos/{matricula}/historial");
    let (codigo, cuerpo, _) = comun::peticion(app, "GET", &ruta, cookie, None).await;
    (codigo, cuerpo)
}

async fn evento(tablero: &mut broadcast::Receiver<CambioEstado>) -> Value {
    let recibido = tokio::time::timeout(Duration::from_secs(1), tablero.recv())
        .await
        .expect("no llegó ningún evento al tablero")
        .unwrap();
    serde_json::to_value(recibido).unwrap()
}

// ---------------------------------------------------------------------------------------------
// US1 · Generar la factura de una orden terminada (T009–T012)
// ---------------------------------------------------------------------------------------------

#[sqlx::test]
async fn facturar_copia_las_imputaciones_y_cierra_la_orden(pool: PgPool) {
    let e = escenario(&pool).await;
    let mut tablero = e.eventos.suscribir();

    let (codigo, f) = facturar(&e.app, &e.ana, e.orden).await;

    assert_eq!(codigo, StatusCode::CREATED, "{f}");
    assert_eq!(f["numero"], 1);
    assert_eq!(f["orden_id"], e.orden);
    assert_eq!(f["fecha"], fechas::hoy().format("%Y-%m-%d").to_string());
    assert_eq!(f["cliente_nombre"], "Cliente de prueba");
    assert_eq!(f["matricula"], "0042PRB");
    assert_eq!(
        f["taller"],
        json!({ "nombre": "Talleres Rueda", "nif": "B00000000",
                "direccion": "Calle Ejemplo 1, 03001 Alicante", "logo": null })
    );
    assert_eq!(
        f["lineas"],
        json!([
            { "concepto": "Cambio de pastillas", "cantidad": "2.00",
              "precio_unitario": "40.00", "importe": "80.00" },
            { "concepto": "PF-123 Pastillas freno", "cantidad": "2.00",
              "precio_unitario": "25.00", "importe": "50.00" }
        ])
    );
    assert_eq!(f["base"], "130.00");
    assert_eq!(f["iva_porcentaje"], "21.00");
    assert_eq!(f["iva"], "27.30");
    assert_eq!(f["total"], "157.30");
    assert_eq!(estado_de_orden(&pool, e.orden).await, Estado::Facturada);
    assert_eq!(
        evento(&mut tablero).await,
        json!({ "id": e.orden, "estado": "facturada" })
    );
}

#[sqlx::test]
async fn los_numeros_de_factura_son_correlativos_y_sin_repetir(pool: PgPool) {
    let e = escenario(&pool).await;
    assert_eq!(facturar(&e.app, &e.ana, e.orden).await.1["numero"], 1);

    // Un intento rechazado (sin imputaciones) no gasta número.
    let vacia = comun::crear_orden(&pool, 43, Estado::Terminada, e.luis_id).await;
    assert_eq!(
        facturar(&e.app, &e.ana, vacia).await.0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(contador_factura(&pool).await, 1);

    // Dos a la vez: números distintos y seguidos.
    let a = orden_terminada(&pool, 44, e.luis_id).await;
    let b = orden_terminada(&pool, 45, e.luis_id).await;
    let (ra, rb) = tokio::join!(facturar(&e.app, &e.ana, a), facturar(&e.app, &e.ana, b));
    assert_eq!((ra.0, rb.0), (StatusCode::CREATED, StatusCode::CREATED));
    let mut numeros = [
        ra.1["numero"].as_i64().unwrap(),
        rb.1["numero"].as_i64().unwrap(),
    ];
    numeros.sort();
    assert_eq!(numeros, [2, 3]);
}

#[sqlx::test]
async fn solo_se_factura_una_vez_y_solo_si_esta_terminada(pool: PgPool) {
    let e = escenario(&pool).await;

    for (numero, (estado, texto)) in (1..).zip([
        (Estado::EnReparacion, "en reparación"),
        (Estado::PendienteConformidad, "pendiente de conformidad"),
        (Estado::EnDiagnostico, "en diagnóstico"),
    ]) {
        let orden = comun::crear_orden(&pool, numero, estado, e.luis_id).await;
        imputar_sql(&pool, orden, "mano_obra", "Algo", None, "1", "40").await;
        let (codigo, cuerpo) = facturar(&e.app, &e.ana, orden).await;
        assert_eq!(codigo, StatusCode::CONFLICT, "{texto}");
        assert_eq!(
            cuerpo["error"],
            format!("La orden está {texto}; solo se puede facturar una orden terminada")
        );
    }

    assert_eq!(
        facturar(&e.app, &e.ana, e.orden).await.0,
        StatusCode::CREATED
    );
    let (codigo, cuerpo) = facturar(&e.app, &e.ana, e.orden).await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(
        cuerpo["error"],
        "La orden está facturada; solo se puede facturar una orden terminada"
    );
    assert_eq!(facturas_guardadas(&pool).await, 1);
}

#[sqlx::test]
async fn sin_imputaciones_o_sin_permiso_no_se_factura(pool: PgPool) {
    let e = escenario(&pool).await;
    let vacia = comun::crear_orden(&pool, 43, Estado::Terminada, e.luis_id).await;

    let (codigo, cuerpo) = facturar(&e.app, &e.ana, vacia).await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        cuerpo["error"],
        "La orden no tiene imputaciones; no se puede facturar"
    );
    assert_eq!(estado_de_orden(&pool, vacia).await, Estado::Terminada);

    let marta = entrar_como(&e.app, &pool, "marta", "mecanico").await;
    assert_eq!(
        facturar(&e.app, &marta, e.orden).await.0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        facturar(&e.app, &e.ana, 999_999).await.0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(facturas_guardadas(&pool).await, 0);

    let paco = entrar_como(&e.app, &pool, "paco", "jefe").await;
    assert_eq!(
        facturar(&e.app, &paco, e.orden).await.0,
        StatusCode::CREATED
    );
}

#[sqlx::test]
async fn ver_la_factura_y_su_orden(pool: PgPool) {
    let e = escenario(&pool).await;
    let (_, emitida) = facturar(&e.app, &e.ana, e.orden).await;
    let id = emitida["id"].as_i64().unwrap();

    let ruta = format!("/api/facturas/{id}");
    let (codigo, f, _) = comun::peticion(&e.app, "GET", &ruta, Some(&e.ana), None).await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(f, emitida);

    let marta = entrar_como(&e.app, &pool, "marta", "mecanico").await;
    let (codigo, _, _) = comun::peticion(&e.app, "GET", &ruta, Some(&marta), None).await;
    assert_eq!(codigo, StatusCode::FORBIDDEN);
    let (codigo, _, _) =
        comun::peticion(&e.app, "GET", "/api/facturas/999999", Some(&e.ana), None).await;
    assert_eq!(codigo, StatusCode::NOT_FOUND);

    let mut conexion = pool.acquire().await.unwrap();
    assert_eq!(
        factura_de_orden(&mut conexion, e.orden).await.unwrap(),
        Some(id)
    );
    let otra = comun::crear_orden(&pool, 43, Estado::Terminada, e.luis_id).await;
    assert_eq!(factura_de_orden(&mut conexion, otra).await.unwrap(), None);
}

// ---------------------------------------------------------------------------------------------
// US2 · Avisar al cliente (T018)
// ---------------------------------------------------------------------------------------------

async fn aviso_guardado(pool: &PgPool) -> bool {
    sqlx::query_scalar("SELECT aviso_enviado FROM factura")
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test]
async fn el_aviso_se_anota_si_sale_el_correo(pool: PgPool) {
    let e = escenario(&pool).await;

    let (codigo, f) = facturar(&e.app, &e.ana, e.orden).await;

    assert_eq!(codigo, StatusCode::CREATED);
    // Mailpit (docker compose) recibe el correo en el puerto 1025 de la configuración de prueba.
    assert_eq!(f["aviso_enviado"], true);
    assert!(aviso_guardado(&pool).await);
}

#[sqlx::test]
async fn si_el_correo_falla_la_factura_se_emite_igual(pool: PgPool) {
    let e = escenario(&pool).await;
    let sin_correo = app_sin_correo(&pool);

    let (codigo, f) = facturar(&sin_correo, &e.ana, e.orden).await;

    assert_eq!(codigo, StatusCode::CREATED, "{f}");
    assert_eq!(f["aviso_enviado"], false);
    assert!(!aviso_guardado(&pool).await);
    assert_eq!(estado_de_orden(&pool, e.orden).await, Estado::Facturada);
    assert_eq!(facturas_guardadas(&pool).await, 1);
}

// ---------------------------------------------------------------------------------------------
// US3 · Historial del vehículo por matrícula (T021, T022)
// ---------------------------------------------------------------------------------------------

/// Otra orden del mismo vehículo que `orden_id`, con su propia cita.
async fn otra_orden_del_vehiculo(pool: &PgPool, orden_id: i64, numero: i64, estado: Estado) -> i64 {
    sqlx::query_scalar(
        "WITH c AS (INSERT INTO cita (vehiculo_id, fecha, motivo)
                    SELECT vehiculo_id, current_date, 'Otra visita' FROM orden WHERE id = $1
                    RETURNING id, vehiculo_id)
         INSERT INTO orden (numero, cita_id, vehiculo_id, mecanico_id, km, danos,
                            fecha_entrega_prometida, estado, diagnostico)
         SELECT $2, c.id, c.vehiculo_id, o.mecanico_id, 25000, 'Sin daños visibles',
                current_date + 3, $3, 'Correa de distribución'
           FROM c, orden o WHERE o.id = $1
         RETURNING id",
    )
    .bind(orden_id)
    .bind(numero)
    .bind(estado)
    .fetch_one(pool)
    .await
    .unwrap()
}

#[sqlx::test]
async fn historial_con_la_reparacion_mas_reciente_primero(pool: PgPool) {
    let e = escenario(&pool).await;
    assert_eq!(
        facturar(&e.app, &e.ana, e.orden).await.0,
        StatusCode::CREATED
    );
    sqlx::query("UPDATE factura SET fecha = current_date - 10")
        .execute(&pool)
        .await
        .unwrap();
    let segunda = otra_orden_del_vehiculo(&pool, e.orden, 43, Estado::Terminada).await;
    imputar_sql(
        &pool,
        segunda,
        "mano_obra",
        "Cambio de correa",
        None,
        "3",
        "40",
    )
    .await;
    assert_eq!(
        facturar(&e.app, &e.ana, segunda).await.0,
        StatusCode::CREATED
    );
    // Una orden del vehículo sin facturar no sale.
    otra_orden_del_vehiculo(&pool, e.orden, 44, Estado::EnReparacion).await;

    let (codigo, h) = historial(&e.app, Some(&e.ana), "0042PRB").await;

    assert_eq!(codigo, StatusCode::OK, "{h}");
    assert_eq!(h["matricula"], "0042PRB");
    assert_eq!(h["marca"], "Seat");
    assert_eq!(h["modelo"], "Ibiza");
    let reparaciones = h["reparaciones"].as_array().unwrap();
    assert_eq!(reparaciones.len(), 2);
    assert_eq!(reparaciones[0]["orden_numero"], 43);
    assert_eq!(reparaciones[0]["factura_numero"], 2);
    assert_eq!(reparaciones[0]["km"], 25000);
    assert_eq!(reparaciones[0]["diagnostico"], "Correa de distribución");
    assert_eq!(reparaciones[0]["total"], "145.20");
    assert_eq!(reparaciones[0]["lineas"][0]["concepto"], "Cambio de correa");
    assert_eq!(reparaciones[1]["orden_numero"], 42);
    assert_eq!(reparaciones[1]["factura_numero"], 1);
    assert_eq!(reparaciones[1]["km"], 1000);
    assert_eq!(reparaciones[1]["total"], "157.30");
    assert_eq!(reparaciones[1]["lineas"].as_array().unwrap().len(), 2);
    let ayer = (fechas::hoy() - chrono::Days::new(10))
        .format("%Y-%m-%d")
        .to_string();
    assert_eq!(reparaciones[1]["fecha"], ayer);
}

#[sqlx::test]
async fn historial_por_matricula_escrita_de_cualquier_forma(pool: PgPool) {
    let e = escenario(&pool).await;
    facturar(&e.app, &e.ana, e.orden).await;

    for escrita in ["0042%20prb", "0042-PRB", "0042prb"] {
        let (codigo, h) = historial(&e.app, Some(&e.ana), escrita).await;
        assert_eq!(codigo, StatusCode::OK, "{escrita}");
        assert_eq!(h["reparaciones"].as_array().unwrap().len(), 1);
    }

    // Un vehículo sin facturas: lista vacía.
    let sin_facturas = comun::crear_orden(&pool, 43, Estado::EnReparacion, e.luis_id).await;
    let _ = sin_facturas;
    let (codigo, h) = historial(&e.app, Some(&e.ana), "0043PRB").await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(h["reparaciones"], json!([]));

    let (codigo, cuerpo) = historial(&e.app, Some(&e.ana), "9999ZZZ").await;
    assert_eq!(codigo, StatusCode::NOT_FOUND);
    assert_eq!(
        cuerpo["error"],
        "No hay ningún vehículo con la matrícula 9999ZZZ"
    );
    let (codigo, cuerpo) = historial(&e.app, Some(&e.ana), "--").await;
    assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(cuerpo["error"], "Matrícula no válida");

    // Lo puede consultar cualquier rol con sesión.
    for (login, rol) in [("luisa", "mecanico"), ("pepe", "almacen")] {
        let cookie = entrar_como(&e.app, &pool, login, rol).await;
        assert_eq!(
            historial(&e.app, Some(&cookie), "0042PRB").await.0,
            StatusCode::OK
        );
    }
    assert_eq!(
        historial(&e.app, None, "0042PRB").await.0,
        StatusCode::UNAUTHORIZED
    );
}

// ---------------------------------------------------------------------------------------------
// Logo del taller en la factura (FR-009, T030)
// ---------------------------------------------------------------------------------------------

/// App cuyo `TALLER_LOGO` apunta a `ruta` (o sin logo).
fn app_con_logo(pool: &PgPool, ruta: Option<std::path::PathBuf>) -> Router {
    let mut estado: EstadoApp = comun::estado_prueba(pool.clone());
    let mut config = (*estado.config).clone();
    config.taller_logo = ruta;
    estado.config = Arc::new(config);
    app(estado)
}

/// GET que devuelve el código, el `Content-Type` y los bytes tal cual (para la imagen).
async fn peticion_cruda(
    app: &Router,
    ruta: &str,
    cookie: Option<&str>,
) -> (StatusCode, Option<String>, Vec<u8>) {
    use axum::body::Body;
    use axum::http::{Request, header};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    let mut req = Request::builder().method("GET").uri(ruta);
    if let Some(cookie) = cookie {
        req = req.header(header::COOKIE, cookie);
    }
    let respuesta = app
        .clone()
        .oneshot(req.body(Body::empty()).unwrap())
        .await
        .unwrap();
    let codigo = respuesta.status();
    let tipo = respuesta
        .headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().to_string());
    let bytes = respuesta
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec();
    (codigo, tipo, bytes)
}

#[sqlx::test]
async fn con_logo_configurado_la_factura_lo_muestra(pool: PgPool) {
    let e = escenario(&pool).await;
    let ruta = std::env::temp_dir().join(format!("logo-prueba-{}.png", std::process::id()));
    let imagen = b"\x89PNG\r\n\x1a\nlogo de prueba".to_vec();
    std::fs::write(&ruta, &imagen).unwrap();
    let con_logo = app_con_logo(&pool, Some(ruta.clone()));

    let (codigo, f) = facturar(&con_logo, &e.ana, e.orden).await;
    assert_eq!(codigo, StatusCode::CREATED, "{f}");
    assert_eq!(f["taller"]["logo"], "/api/taller/logo");

    let (codigo, tipo, bytes) = peticion_cruda(&con_logo, "/api/taller/logo", Some(&e.ana)).await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(tipo.as_deref(), Some("image/png"));
    assert_eq!(bytes, imagen);

    let (codigo, _, _) = peticion_cruda(&con_logo, "/api/taller/logo", None).await;
    assert_eq!(codigo, StatusCode::UNAUTHORIZED);
    std::fs::remove_file(ruta).ok();
}

#[sqlx::test]
async fn sin_logo_o_con_un_archivo_que_no_existe_no_hay_logo(pool: PgPool) {
    let e = escenario(&pool).await;
    let (_, f) = facturar(&e.app, &e.ana, e.orden).await;
    let id = f["id"].as_i64().unwrap();
    assert_eq!(f["taller"]["logo"], Value::Null);
    let (codigo, _, cuerpo) = peticion_cruda(&e.app, "/api/taller/logo", Some(&e.ana)).await;
    assert_eq!(codigo, StatusCode::NOT_FOUND);
    let cuerpo: Value = serde_json::from_slice(&cuerpo).unwrap();
    assert_eq!(cuerpo["error"], "No hay logo configurado");

    let roto = app_con_logo(&pool, Some("/no/existe/logo.png".into()));
    let ruta = format!("/api/facturas/{id}");
    let (_, f, _) = comun::peticion(&roto, "GET", &ruta, Some(&e.ana), None).await;
    assert_eq!(f["taller"]["logo"], Value::Null);
    let (codigo, _, _) = peticion_cruda(&roto, "/api/taller/logo", Some(&e.ana)).await;
    assert_eq!(codigo, StatusCode::NOT_FOUND);
}
