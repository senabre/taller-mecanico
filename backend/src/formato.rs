//! Formato español para los mensajes de error del backend; el equivalente de `numero()` de
//! `frontend/src/lib/formato.ts`.

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

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn numeros_con_punto_de_miles() {
        assert_eq!(numero(85000), "85.000");
        assert_eq!(numero(1500), "1.500");
        assert_eq!(numero(0), "0");
        assert_eq!(numero(-1234), "-1.234");
    }
}
