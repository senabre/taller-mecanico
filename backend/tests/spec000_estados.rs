//! Transiciones de estado de la orden (research R14, data-model «Máquina de estados»).

mod comun;

use serde_json::json;
use sqlx::PgPool;
use taller::error::ErrorApi;
use taller::estados::{CambioEstado, Estado, cambiar_estado};
use taller::eventos::Eventos;

async fn orden(pool: &PgPool, estado: Estado) -> i64 {
    let luis = comun::insertar_usuario(pool, "luis", "mecanico").await;
    comun::crear_orden(pool, 1, estado, luis).await
}

async fn estado_de(pool: &PgPool, id: i64) -> Estado {
    sqlx::query_scalar("SELECT estado FROM orden WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test]
async fn cambio_permitido(pool: PgPool) {
    let id = orden(&pool, Estado::Recibida).await;

    let mut tx = pool.begin().await.unwrap();
    let cambio = cambiar_estado(&mut tx, id, &[Estado::Recibida], Estado::EnDiagnostico)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    assert_eq!(cambio, CambioEstado { id, estado: Estado::EnDiagnostico });
    assert_eq!(estado_de(&pool, id).await, Estado::EnDiagnostico);
}

#[sqlx::test]
async fn desde_un_estado_que_no_es_el_actual_da_409(pool: PgPool) {
    let id = orden(&pool, Estado::Recibida).await;

    let mut tx = pool.begin().await.unwrap();
    let r = cambiar_estado(&mut tx, id, &[Estado::EnReparacion], Estado::Terminada).await;
    tx.commit().await.unwrap();

    match r {
        Err(ErrorApi::Conflicto(m)) => {
            assert_eq!(m, "La orden está recibida; no se puede pasar a terminada")
        }
        otro => panic!("se esperaba 409 y llegó {otro:?}"),
    }
    assert_eq!(estado_de(&pool, id).await, Estado::Recibida);
}

#[sqlx::test]
async fn transicion_que_no_esta_en_la_tabla_da_error(pool: PgPool) {
    let id = orden(&pool, Estado::EnEsperaPiezas).await;

    let mut tx = pool.begin().await.unwrap();
    let r = cambiar_estado(&mut tx, id, &[Estado::EnEsperaPiezas], Estado::Terminada).await;
    tx.commit().await.unwrap();

    assert!(matches!(r, Err(ErrorApi::Interno(_))), "{r:?}");
    assert_eq!(estado_de(&pool, id).await, Estado::EnEsperaPiezas);
}

#[sqlx::test]
async fn orden_inexistente_da_404(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    let r = cambiar_estado(&mut tx, 999_999, &[Estado::Recibida], Estado::EnDiagnostico).await;

    assert!(matches!(r, Err(ErrorApi::NoEncontrado(_))), "{r:?}");
}

#[sqlx::test]
async fn terminar_rellena_terminada_en(pool: PgPool) {
    let id = orden(&pool, Estado::EnReparacion).await;

    let mut tx = pool.begin().await.unwrap();
    let _ = cambiar_estado(&mut tx, id, &[Estado::EnReparacion], Estado::Terminada)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let terminada: bool =
        sqlx::query_scalar("SELECT terminada_en IS NOT NULL FROM orden WHERE id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(terminada);
}

#[sqlx::test]
async fn publicar_llega_al_tablero(pool: PgPool) {
    let id = orden(&pool, Estado::Recibida).await;
    let eventos = Eventos::nuevo();
    let mut tablero = eventos.suscribir();

    let mut tx = pool.begin().await.unwrap();
    let cambio = cambiar_estado(&mut tx, id, &[Estado::Recibida], Estado::EnDiagnostico)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    cambio.publicar(&eventos);

    let recibido = tablero.recv().await.unwrap();
    assert_eq!(
        serde_json::to_value(recibido).unwrap(),
        json!({ "id": id, "estado": "en_diagnostico" })
    );
}
