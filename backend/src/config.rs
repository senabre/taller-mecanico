//! Configuración leída de variables de entorno (research R7). En desarrollo se cargan desde
//! `.env`; ningún valor de negocio ni secreto se escribe en el código (principio V, FR-004).

use std::env;
use std::str::FromStr;

use rust_decimal::Decimal;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub smtp_url: String,
    pub correo_remitente: String,
    pub url_publica: String,
    pub cupo_diario: u32,
    pub precio_hora: Decimal,
    pub iva_porcentaje: Decimal,
    pub taller_nombre: String,
    pub taller_nif: String,
    pub taller_direccion: String,
}

impl Config {
    pub fn desde_entorno() -> Result<Config, String> {
        dotenvy::dotenv().ok();
        Ok(Config {
            database_url: texto("DATABASE_URL")?,
            smtp_url: texto("SMTP_URL")?,
            correo_remitente: texto("CORREO_REMITENTE")?,
            url_publica: texto("URL_PUBLICA")?,
            cupo_diario: valor("TALLER_CUPO_DIARIO")?,
            precio_hora: valor("TALLER_PRECIO_HORA")?,
            iva_porcentaje: valor("TALLER_IVA_PORCENTAJE")?,
            taller_nombre: texto("TALLER_NOMBRE")?,
            taller_nif: texto("TALLER_NIF")?,
            taller_direccion: texto("TALLER_DIRECCION")?,
        })
    }

    /// La cookie de sesión lleva `Secure` solo cuando la app se sirve por HTTPS (research R8).
    pub fn cookie_segura(&self) -> bool {
        self.url_publica.starts_with("https://")
    }
}

fn texto(nombre: &str) -> Result<String, String> {
    match env::var(nombre) {
        Ok(v) if !v.trim().is_empty() => Ok(v),
        _ => Err(format!("Falta la variable de entorno {nombre}")),
    }
}

fn valor<T: FromStr>(nombre: &str) -> Result<T, String> {
    texto(nombre)?
        .trim()
        .parse()
        .map_err(|_| format!("La variable de entorno {nombre} no tiene un valor válido"))
}
