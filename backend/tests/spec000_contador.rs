//! Números correlativos sin huecos (research R5; spec-002 FR-002, spec-007 FR-004).

use sqlx::PgPool;
use taller::contador::{self, FACTURA, ORDEN};

async fn siguiente_confirmado(pool: &PgPool, nombre: &str) -> i64 {
    let mut tx = pool.begin().await.unwrap();
    let n = contador::siguiente(&mut tx, nombre).await.unwrap();
    tx.commit().await.unwrap();
    n
}

#[sqlx::test]
async fn numeros_seguidos(pool: PgPool) {
    assert_eq!(siguiente_confirmado(&pool, ORDEN).await, 1);
    assert_eq!(siguiente_confirmado(&pool, ORDEN).await, 2);
    assert_eq!(siguiente_confirmado(&pool, ORDEN).await, 3);
}

#[sqlx::test]
async fn un_rollback_no_deja_hueco(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    assert_eq!(contador::siguiente(&mut tx, ORDEN).await.unwrap(), 1);
    tx.rollback().await.unwrap();

    assert_eq!(siguiente_confirmado(&pool, ORDEN).await, 1);
}

#[sqlx::test]
async fn orden_y_factura_llevan_cuentas_independientes(pool: PgPool) {
    assert_eq!(siguiente_confirmado(&pool, ORDEN).await, 1);
    assert_eq!(siguiente_confirmado(&pool, FACTURA).await, 1);
    assert_eq!(siguiente_confirmado(&pool, ORDEN).await, 2);
    assert_eq!(siguiente_confirmado(&pool, FACTURA).await, 2);
}
