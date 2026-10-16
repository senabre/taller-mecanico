//! Importes, IVA y redondeo (research R4). Nunca se usan floats: las cuentas cuadran con una
//! calculadora (spec-003 SC-002, spec-007 SC-002).

use rust_decimal::{Decimal, RoundingStrategy};
use serde::Serialize;

/// Redondea a 2 decimales (mitad hacia arriba) y deja siempre escala 2, para que se serialice
/// `"60.00"` y no `"60"`.
pub fn redondear(x: Decimal) -> Decimal {
    let mut r = x.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero);
    r.rescale(2);
    r
}

pub fn importe_linea(cantidad: Decimal, precio_unitario: Decimal) -> Decimal {
    redondear(cantidad * precio_unitario)
}

/// Totales de un presupuesto o una factura. Se serializan como strings con 2 decimales y punto
/// (`"133.10"`), según el contrato.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Totales {
    #[serde(with = "rust_decimal::serde::str")]
    pub base: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub iva_porcentaje: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub iva: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub total: Decimal,
}

/// Base = suma de los importes; el IVA se calcula **una vez** sobre la base; total = base + IVA.
pub fn totales(importes: impl IntoIterator<Item = Decimal>, iva_porcentaje: Decimal) -> Totales {
    let base = redondear(importes.into_iter().sum());
    let iva = redondear(base * iva_porcentaje / Decimal::ONE_HUNDRED);
    Totales {
        base,
        iva_porcentaje: redondear(iva_porcentaje),
        iva,
        total: redondear(base + iva),
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use rust_decimal_macros::dec;
    use serde_json::json;

    #[test]
    fn importe_de_mano_de_obra() {
        assert_eq!(importe_linea(dec!(1.5), dec!(40.00)).to_string(), "60.00");
    }

    #[test]
    fn importe_de_pieza() {
        assert_eq!(importe_linea(dec!(2), dec!(25.00)).to_string(), "50.00");
    }

    #[test]
    fn totales_del_quickstart() {
        let t = totales([dec!(60.00), dec!(50.00)], dec!(21));
        assert_eq!(t.base.to_string(), "110.00");
        assert_eq!(t.iva.to_string(), "23.10");
        assert_eq!(t.total.to_string(), "133.10");
    }

    #[test]
    fn la_mitad_redondea_hacia_arriba() {
        assert_eq!(redondear(dec!(0.005)), dec!(0.01));
    }

    #[test]
    fn totales_se_serializan_como_strings_con_dos_decimales() {
        let t = totales([dec!(60), dec!(50)], dec!(21));
        assert_eq!(
            serde_json::to_value(&t).unwrap(),
            json!({ "base": "110.00", "iva_porcentaje": "21.00", "iva": "23.10", "total": "133.10" })
        );
    }
}
