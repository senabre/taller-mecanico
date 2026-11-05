//! Formato español para los mensajes de error del backend; el equivalente de `numero()` de
//! `frontend/src/lib/formato.ts`.

use rust_decimal::Decimal;

/// Entero con punto de miles: 85000 → «85.000».
pub fn numero(n: i64) -> String {
    let cifras = n.unsigned_abs().to_string();
    let mut con_puntos = String::with_capacity(cifras.len() + cifras.len() / 3);
    for (i, c) in cifras.chars().enumerate() {
        if i > 0 && (cifras.len() - i).is_multiple_of(3) {
            con_puntos.push('.');
        }
        con_puntos.push(c);
    }
    if n < 0 {
        format!("-{con_puntos}")
    } else {
        con_puntos
    }
}

/// Importe en euros con formato español: 1234.56 → «1.234,56 €» (para los correos, spec-007; en
/// pantalla se usa `euros()` de `formato.ts`).
pub fn euros(importe: Decimal) -> String {
    let mut importe = importe.round_dp(2);
    importe.rescale(2);
    let signo = if importe.is_sign_negative() && !importe.is_zero() { "-" } else { "" };
    let texto = importe.abs().to_string();
    let (enteros, centimos) = texto.split_once('.').unwrap_or((&texto, "00"));
    let enteros: i64 = enteros.parse().unwrap_or(0);
    format!("{signo}{},{centimos} €", numero(enteros))
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn euros_con_punto_de_miles_y_coma_decimal() {
        assert_eq!(euros(dec!(157.30)), "157,30 €");
        assert_eq!(euros(dec!(1234.56)), "1.234,56 €");
        assert_eq!(euros(dec!(0)), "0,00 €");
        assert_eq!(euros(dec!(12.5)), "12,50 €");
        assert_eq!(euros(dec!(1000000)), "1.000.000,00 €");
    }

    #[test]
    fn numeros_con_punto_de_miles() {
        assert_eq!(numero(85000), "85.000");
        assert_eq!(numero(1500), "1.500");
        assert_eq!(numero(0), "0");
        assert_eq!(numero(-1234), "-1.234");
    }
}
