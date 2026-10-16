//! «Hoy» siempre en hora de España peninsular (research R12): caducidad de presupuestos,
//! retrasos del tablero y días de cita dependen del día en Madrid, no del día en UTC.

use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;

pub const ZONA: Tz = chrono_tz::Europe::Madrid;

pub fn fecha_en_madrid(instante: DateTime<Utc>) -> NaiveDate {
    instante.with_timezone(&ZONA).date_naive()
}

pub fn hoy() -> NaiveDate {
    fecha_en_madrid(Utc::now())
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn instante(texto: &str) -> DateTime<Utc> {
        texto.parse().unwrap()
    }

    #[test]
    fn de_noche_en_verano_ya_es_el_dia_siguiente_en_madrid() {
        // UTC+2
        assert_eq!(
            fecha_en_madrid(instante("2026-10-15T22:30:00Z")),
            NaiveDate::from_ymd_opt(2026, 10, 16).unwrap()
        );
    }

    #[test]
    fn de_noche_en_invierno_ya_es_el_ano_siguiente_en_madrid() {
        // UTC+1
        assert_eq!(
            fecha_en_madrid(instante("2026-12-31T23:30:00Z")),
            NaiveDate::from_ymd_opt(2027, 1, 1).unwrap()
        );
    }
}
