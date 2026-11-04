//! Pruebas de spec-004 · Aceptar o rechazar el presupuesto desde un enlace (plan.md, «Pruebas»;
//! aquí «prueba 1»…«prueba 12»). Crean sus propios datos y no usan valores demo (principio V).

mod comun;

use axum::Router;
use axum::http::StatusCode;
use chrono::{Days, NaiveDate};
use serde_json::{Value, json};
use sqlx::PgPool;
use taller::estados::{CambioEstado, Estado};
use taller::fechas;
use tokio::sync::broadcast::error::TryRecvError;

const DIAGNOSTICO: &str = "Pastillas delanteras desgastadas";
const YA_RESPONDIDO: &str = "Este presupuesto ya fue respondido";

/// Presupuesto por SQL, con una línea de mano de obra y sus totales. Escribe también el diagnóstico
/// de la orden, que `comun::crear_orden` deja a NULL.
async fn presupuesto(
    pool: &PgPool,
    orden_id: i64,
    tipo: &str,
    estado: &str,
    caduca_el: NaiveDate,
    token: &str,
) -> i64 {
    sqlx::query("UPDATE orden SET diagnostico = $2 WHERE id = $1")
        .bind(orden_id)
        .bind(DIAGNOSTICO)
        .execute(pool)
        .await
        .unwrap();
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO presupuesto (orden_id, tipo, estado, token, iva_porcentaje, base, iva, total,
                                  emitido_en, caduca_el)
         VALUES ($1, $2::tipo_presupuesto, $3::estado_presupuesto, $4, 21.00, 60.00, 12.60, 72.60,
                 now(), $5)
         RETURNING id",
    )
    .bind(orden_id)
    .bind(tipo)
    .bind(estado)
    .bind(token)
    .bind(caduca_el)
    .fetch_one(pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO linea_presupuesto (presupuesto_id, tipo, concepto, cantidad, precio_unitario,
                                        importe)
         VALUES ($1, 'mano_obra', 'Cambio de pastillas', 1.50, 40.00, 60.00)",
    )
    .bind(id)
    .execute(pool)
    .await
    .unwrap();
    id
}

/// Un presupuesto inicial emitido y vigente, en una orden nueva en diagnóstico.
async fn inicial_vigente(pool: &PgPool, numero: i64, mecanico: i64, token: &str) -> (i64, i64) {
    let orden = comun::crear_orden(pool, numero, Estado::EnDiagnostico, mecanico).await;
    let id = presupuesto(pool, orden, "inicial", "emitido", vigente(), token).await;
    (orden, id)
}

fn vigente() -> NaiveDate {
    fechas::hoy() + Days::new(15)
}

async fn estado_orden(pool: &PgPool, id: i64) -> Estado {
    sqlx::query_scalar("SELECT estado FROM orden WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

/// El estado y la hora de respuesta guardados, como texto para compararlos.
async fn guardado(pool: &PgPool, id: i64) -> (String, Option<String>) {
    sqlx::query_as("SELECT estado::text, respondido_en::text FROM presupuesto WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn ver(app: &Router, token: &str) -> (StatusCode, Value) {
    let ruta = format!("/api/publico/presupuestos/{token}");
    let (codigo, cuerpo, _) = comun::peticion(app, "GET", &ruta, None, None).await;
    (codigo, cuerpo)
}

async fn enviar(app: &Router, token: &str, cuerpo: Value) -> (StatusCode, Value) {
    let ruta = format!("/api/publico/presupuestos/{token}/respuesta");
    let (codigo, cuerpo, _) = comun::peticion(app, "POST", &ruta, None, Some(cuerpo)).await;
    (codigo, cuerpo)
}

async fn responder(app: &Router, token: &str, decision: &str) -> (StatusCode, Value) {
    enviar(app, token, json!({ "decision": decision })).await
}

/// Prueba 1 · FR-002, US1-1: el presupuesto completo, sin ningún dato del cliente.
#[sqlx::test]
async fn ver_el_presupuesto_sin_datos_del_cliente(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let luis = comun::insertar_usuario(&pool, "luis", "mecanico").await;
    let (orden, id) = inicial_vigente(&pool, 1, luis, "enlace-prueba").await;

    let (codigo, cuerpo) = ver(&app, "enlace-prueba").await;

    assert_eq!(codigo, StatusCode::OK);
    let linea: i64 =
        sqlx::query_scalar("SELECT id FROM linea_presupuesto WHERE presupuesto_id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
    let matricula: String = sqlx::query_scalar(
        "SELECT v.matricula FROM orden o JOIN vehiculo v ON v.id = o.vehiculo_id WHERE o.id = $1",
    )
    .bind(orden)
    .fetch_one(&pool)
    .await
    .unwrap();
    // El objeto entero: cualquier campo de más, como un dato del cliente, hace fallar la prueba.
    assert_eq!(
        cuerpo,
        json!({
            "taller": { "nombre": "Talleres Rueda" },
            "matricula": matricula,
            "marca": "Seat",
            "modelo": "Ibiza",
            "diagnostico": DIAGNOSTICO,
            "tipo": "inicial",
            "lineas": [{
                "id": linea,
                "tipo": "mano_obra",
                "concepto": "Cambio de pastillas",
                "pieza_id": null,
                "cantidad": "1.50",
                "precio_unitario": "40.00",
                "importe": "60.00",
            }],
            "base": "60.00",
            "iva_porcentaje": "21.00",
            "iva": "12.60",
            "total": "72.60",
            "caduca_el": vigente().to_string(),
            "caducado": false,
            "decision": null,
            "respondido_en": null,
        })
    );
    let texto = cuerpo.to_string();
    for dato in ["Cliente de prueba", "600000000", "cliente@ejemplo.test"] {
        assert!(!texto.contains(dato), "la respuesta enseña «{dato}»");
    }
}

/// Prueba 8 · FR-002: un token inventado da el mismo 404 en el `GET` y en el `POST`.
#[sqlx::test]
async fn un_token_inexistente_da_el_mismo_404(pool: PgPool) {
    let app = comun::app_prueba(pool);
    let esperado = json!({ "error": "Este enlace no es válido" });

    let (codigo, cuerpo) = ver(&app, "enlace-inventado").await;
    assert_eq!(codigo, StatusCode::NOT_FOUND);
    assert_eq!(cuerpo, esperado);

    let (codigo, cuerpo) = responder(&app, "enlace-inventado", "aceptar").await;
    assert_eq!(codigo, StatusCode::NOT_FOUND);
    assert_eq!(cuerpo, esperado);
}

/// Prueba 9 · FR-003: sin una decisión válida, 422 y no se guarda nada.
#[sqlx::test]
async fn una_decision_no_valida_da_422(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let luis = comun::insertar_usuario(&pool, "luis", "mecanico").await;
    let (orden, id) = inicial_vigente(&pool, 1, luis, "enlace-prueba").await;

    for cuerpo in [
        json!({}),
        json!({ "decision": "" }),
        json!({ "decision": "otra" }),
    ] {
        let (codigo, respuesta) = enviar(&app, "enlace-prueba", cuerpo.clone()).await;
        assert_eq!(codigo, StatusCode::UNPROCESSABLE_ENTITY, "{cuerpo}");
        assert_eq!(respuesta, json!({ "error": "Elija aceptar o rechazar" }));
    }

    assert_eq!(guardado(&pool, id).await, ("emitido".to_string(), None));
    assert_eq!(estado_orden(&pool, orden).await, Estado::EnDiagnostico);
}

/// Prueba 2 · FR-004, FR-006, US1-2, SC-002: aceptar guarda la decisión, la orden pasa a «en
/// reparación» y el tablero recibe el cambio. La ficha trae la fecha y la hora.
#[sqlx::test]
async fn aceptar_el_inicial_pasa_la_orden_a_reparacion(pool: PgPool) {
    let estado = comun::estado_prueba(pool.clone());
    let mut tablero = estado.eventos.suscribir();
    let app = taller::app(estado);
    let luis = comun::insertar_usuario(&pool, "luis", "mecanico").await;
    let (orden, id) = inicial_vigente(&pool, 1, luis, "enlace-prueba").await;

    let (codigo, cuerpo) = responder(&app, "enlace-prueba", "aceptar").await;

    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(cuerpo["decision"], "aceptado");
    assert!(cuerpo["respondido_en"].is_string());
    assert_eq!(guardado(&pool, id).await.0, "aceptado");
    assert_eq!(estado_orden(&pool, orden).await, Estado::EnReparacion);
    assert_eq!(
        tablero.try_recv().unwrap(),
        CambioEstado {
            id: orden,
            estado: Estado::EnReparacion
        }
    );

    let (_, contrasena) = comun::crear_usuario(&pool, "ana", "recepcion", true).await;
    let ana = comun::entrar(&app, "ana", &contrasena).await;
    let ruta = format!("/api/ordenes/{orden}");
    let (codigo, ficha, _) = comun::peticion(&app, "GET", &ruta, Some(&ana), None).await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(
        ficha["presupuestos"][0]["respondido_en"],
        cuerpo["respondido_en"]
    );
}

/// Prueba 3 · FR-003, US1-3: la decisión se puede volver a ver, pero no cambiar.
#[sqlx::test]
async fn solo_se_responde_una_vez(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let luis = comun::insertar_usuario(&pool, "luis", "mecanico").await;
    let (_, id) = inicial_vigente(&pool, 1, luis, "enlace-prueba").await;

    let (codigo, primera) = responder(&app, "enlace-prueba", "aceptar").await;
    assert_eq!(codigo, StatusCode::OK);
    let antes = guardado(&pool, id).await;

    let (codigo, abierta) = ver(&app, "enlace-prueba").await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(abierta["decision"], "aceptado");
    assert_eq!(abierta["respondido_en"], primera["respondido_en"]);

    let (codigo, segunda) = responder(&app, "enlace-prueba", "aceptar").await;
    assert_eq!(codigo, StatusCode::CONFLICT);
    assert_eq!(segunda["error"], YA_RESPONDIDO);
    assert_eq!(guardado(&pool, id).await, antes);
}

/// Prueba 4 · FR-003, US1-4 (Q4): de dos respuestas a la vez solo cuenta una, y la orden cambia una
/// sola vez. Se compara el texto exacto: sin `estado = 'emitido'` en el `UPDATE`, `cambiar_estado`
/// también da 409, pero con otro texto.
#[sqlx::test]
async fn dos_respuestas_a_la_vez_solo_cuenta_una(pool: PgPool) {
    let estado = comun::estado_prueba(pool.clone());
    let mut tablero = estado.eventos.suscribir();
    let app = taller::app(estado);
    let luis = comun::insertar_usuario(&pool, "luis", "mecanico").await;

    for vuelta in 1..=20 {
        let token = format!("enlace-prueba-{vuelta}");
        let (orden, id) = inicial_vigente(&pool, vuelta, luis, &token).await;

        let (a, b) = tokio::join!(
            responder(&app, &token, "aceptar"),
            responder(&app, &token, "aceptar")
        );

        let (ganadora, perdedora) = if a.0 == StatusCode::OK {
            (a, b)
        } else {
            (b, a)
        };
        assert_eq!(ganadora.0, StatusCode::OK, "vuelta {vuelta}");
        assert_eq!(perdedora.0, StatusCode::CONFLICT, "vuelta {vuelta}");
        assert_eq!(perdedora.1["error"], YA_RESPONDIDO, "vuelta {vuelta}");
        assert_eq!(guardado(&pool, id).await.0, "aceptado");
        assert_eq!(estado_orden(&pool, orden).await, Estado::EnReparacion);
        assert_eq!(
            tablero.try_recv().unwrap(),
            CambioEstado {
                id: orden,
                estado: Estado::EnReparacion
            }
        );
        assert_eq!(
            tablero.try_recv(),
            Err(TryRecvError::Empty),
            "vuelta {vuelta}"
        );
    }
}

/// Prueba 12 · FR-008, US1-5 (Q5): el enlace se oculta solo en la pantalla.
#[sqlx::test]
async fn tras_responder_la_api_sigue_enviando_el_enlace(pool: PgPool) {
    let app = comun::app_prueba(pool.clone());
    let luis = comun::insertar_usuario(&pool, "luis", "mecanico").await;
    let (_, id) = inicial_vigente(&pool, 1, luis, "enlace-prueba").await;

    let (codigo, _) = responder(&app, "enlace-prueba", "aceptar").await;
    assert_eq!(codigo, StatusCode::OK);

    let (_, contrasena) = comun::crear_usuario(&pool, "ana", "recepcion", true).await;
    let ana = comun::entrar(&app, "ana", &contrasena).await;
    let ruta = format!("/api/presupuestos/{id}");
    let (codigo, cuerpo, _) = comun::peticion(&app, "GET", &ruta, Some(&ana), None).await;
    assert_eq!(codigo, StatusCode::OK);
    assert_eq!(
        cuerpo["enlace"],
        "http://localhost:3000/presupuesto/enlace-prueba"
    );
}
