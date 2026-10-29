//! Líneas de mano de obra y de pieza: el objeto `Linea` del contrato y la validación de una línea
//! nueva, iguales para el presupuesto (spec-003) y las imputaciones (spec-005; research D1).

use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgConnection;

use crate::config::Config;
use crate::dinero::importe_linea;
use crate::error::{ErrorApi, Resultado};

/// El objeto `Linea` del contrato.
#[derive(Serialize, sqlx::FromRow)]
pub struct Linea {
    pub id: i64,
    pub tipo: String,
    pub concepto: String,
    pub pieza_id: Option<i64>,
    #[serde(with = "rust_decimal::serde::str")]
    pub cantidad: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub precio_unitario: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub importe: Decimal,
}

/// Lo que llega para añadir una línea. Los campos llegan como JSON sin tipar para responder 422
/// (y no 400) a un tipo equivocado.
#[derive(Deserialize)]
pub struct NuevaLinea {
    pub tipo: Option<String>,
    pub concepto: Option<String>,
    pub pieza_id: Option<Value>,
    pub cantidad: Option<Value>,
}

/// Una línea ya validada, con su precio e importe, lista para insertar.
pub struct LineaValidada {
    pub tipo: &'static str,
    pub concepto: String,
    pub pieza_id: Option<i64>,
    pub cantidad: Decimal,
    pub precio_unitario: Decimal,
    pub importe: Decimal,
}

fn validacion(mensaje: &str) -> ErrorApi {
    ErrorApi::Validacion {
        mensaje: mensaje.to_string(),
        detalle: None,
    }
}

/// Cantidad > 0 con 2 decimales como máximo, como texto (`"1.5"`) o número.
fn cantidad(valor: &Option<Value>) -> Option<Decimal> {
    let decimal = match valor.as_ref()? {
        Value::String(texto) => Decimal::from_str(texto.trim()).ok()?,
        Value::Number(numero) => Decimal::from_str(&numero.to_string()).ok()?,
        _ => return None,
    };
    if decimal <= Decimal::ZERO || decimal.normalize().scale() > 2 {
        return None;
    }
    let mut decimal = decimal;
    decimal.rescale(2);
    Some(decimal)
}

/// Mano de obra: horas en múltiplos de 0,25, concepto obligatorio y precio por hora del taller.
/// Pieza: unidades enteras, una referencia dada de alta, concepto «referencia descripción» y su
/// precio de venta. No mira el stock: eso lo decide quien llama.
pub async fn validar(
    conexion: &mut PgConnection,
    config: &Config,
    nueva: &NuevaLinea,
) -> Resultado<LineaValidada> {
    let (tipo, concepto, pieza_id, cantidad, precio_unitario) = match nueva.tipo.as_deref() {
        Some("mano_obra") => {
            let horas = cantidad(&nueva.cantidad)
                .ok_or_else(|| validacion("Indica las horas: un número mayor que 0"))?;
            if (horas * Decimal::from(4)).fract() != Decimal::ZERO {
                return Err(validacion(
                    "Las horas van en múltiplos de 0,25 (p. ej. 1,5)",
                ));
            }
            let concepto = nueva.concepto.as_deref().unwrap_or("").trim().to_string();
            if concepto.is_empty() {
                return Err(validacion("Escribe el concepto de la mano de obra"));
            }
            ("mano_obra", concepto, None, horas, config.precio_hora)
        }
        Some("pieza") => {
            let unidades = cantidad(&nueva.cantidad)
                .ok_or_else(|| validacion("Indica las unidades: un número mayor que 0"))?;
            if unidades.fract() != Decimal::ZERO {
                return Err(validacion(
                    "Las unidades de pieza deben ser un número entero",
                ));
            }
            let pieza: Option<(i64, String, String, Decimal)> =
                match nueva.pieza_id.as_ref().and_then(Value::as_i64) {
                    Some(pieza_id) => {
                        sqlx::query_as(
                            "SELECT id, referencia, descripcion, precio_venta
                               FROM pieza WHERE id = $1",
                        )
                        .bind(pieza_id)
                        .fetch_optional(&mut *conexion)
                        .await?
                    }
                    None => None,
                };
            let (pieza_id, referencia, descripcion, precio_venta) =
                pieza.ok_or_else(|| validacion("Elige una referencia de pieza dada de alta"))?;
            // El concepto de una pieza lo pone el servidor (spec-003 D3).
            (
                "pieza",
                format!("{referencia} {descripcion}"),
                Some(pieza_id),
                unidades,
                precio_venta,
            )
        }
        _ => return Err(validacion("El tipo de línea debe ser mano de obra o pieza")),
    };
    Ok(LineaValidada {
        tipo,
        concepto,
        pieza_id,
        cantidad,
        precio_unitario,
        importe: importe_linea(cantidad, precio_unitario),
    })
}
