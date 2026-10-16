//! Números correlativos sin huecos para órdenes y facturas (research R5). Se usa dentro de la
//! transacción que crea la orden o la factura: si esta falla, el número se devuelve. Nunca
//! `SEQUENCE` ni `SERIAL`, que dejan huecos con los rollback.

use sqlx::PgConnection;

use crate::error::{ErrorApi, Resultado};

pub const ORDEN: &str = "orden";
pub const FACTURA: &str = "factura";

pub async fn siguiente(tx: &mut PgConnection, nombre: &str) -> Resultado<i64> {
    sqlx::query_scalar(
        "UPDATE contador SET ultimo = ultimo + 1 WHERE nombre = $1 RETURNING ultimo",
    )
    .bind(nombre)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| ErrorApi::Interno(format!("No existe el contador «{nombre}»")))
}
