//! Normalización de matrículas, común a todo el sistema: «1234-bcd» y «1234 BCD» son la misma
//! matrícula (spec-001 FR-002; el contrato dice que el servidor la normaliza siempre).

/// Quita espacios y guiones y pasa a mayúsculas. Devuelve `None` si no quedan de 2 a 10
/// caracteres o si alguno está fuera de `[A-Z0-9]` (spec-001 FR-002); cada spec lo convierte en
/// un 422. El `CHECK` de `vehiculo` solo comprueba los caracteres: la longitud se exige aquí.
pub fn normalizar_matricula(texto: &str) -> Option<String> {
    let matricula: String = texto
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .collect::<String>()
        .to_uppercase();
    let valida = (2..=10).contains(&matricula.len())
        && matricula
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit());
    valida.then_some(matricula)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn misma_matricula_escrita_de_varias_formas() {
        for texto in ["1234-bcd", " 1234 BCD ", "1234BCD"] {
            assert_eq!(normalizar_matricula(texto).as_deref(), Some("1234BCD"), "{texto:?}");
        }
    }

    #[test]
    fn matriculas_invalidas() {
        assert_eq!(normalizar_matricula(""), None);
        assert_eq!(normalizar_matricula("12#4"), None);
        assert_eq!(normalizar_matricula("A"), None);
        assert_eq!(normalizar_matricula("ABCDE123456"), None);
    }

    #[test]
    fn limites_de_longitud() {
        assert_eq!(normalizar_matricula("AB").as_deref(), Some("AB"));
        assert_eq!(normalizar_matricula("ABCDE12345").as_deref(), Some("ABCDE12345"));
    }
}
