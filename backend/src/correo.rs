//! Envío de correo por SMTP (research R10). En desarrollo va a Mailpit
//! (bandeja en http://localhost:8025) y no sale ningún correo real.
//!
//! Se llama **después del `commit`** de la transacción. Si el envío falla no se deshace nada: el
//! llamador guarda que el aviso no se envió (`aviso_enviado = false`) y la pantalla lo muestra.

use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::config::Config;

#[derive(Clone)]
pub struct Correo {
    transporte: AsyncSmtpTransport<Tokio1Executor>,
    remitente: Mailbox,
}

impl Correo {
    pub fn desde_config(config: &Config) -> Result<Correo, String> {
        let transporte = AsyncSmtpTransport::<Tokio1Executor>::from_url(&config.smtp_url)
            .map_err(|e| format!("La variable de entorno SMTP_URL no es válida: {e}"))?
            .build();
        let remitente = config.correo_remitente.parse().map_err(|_| {
            "La variable de entorno CORREO_REMITENTE no es una dirección válida".to_string()
        })?;
        Ok(Correo {
            transporte,
            remitente,
        })
    }

    /// Envía un correo de texto plano. Devuelve `false` (y lo registra) si no se pudo enviar;
    /// nunca entra en pánico.
    pub async fn enviar(&self, para: &str, asunto: &str, cuerpo: &str) -> bool {
        let destino: Mailbox = match para.parse() {
            Ok(destino) => destino,
            Err(e) => {
                tracing::warn!("Dirección de correo no válida «{para}»: {e}");
                return false;
            }
        };
        let mensaje = match Message::builder()
            .from(self.remitente.clone())
            .to(destino)
            .subject(asunto)
            .header(ContentType::TEXT_PLAIN)
            .body(cuerpo.to_string())
        {
            Ok(mensaje) => mensaje,
            Err(e) => {
                tracing::warn!("No se pudo preparar el correo a «{para}»: {e}");
                return false;
            }
        };
        match self.transporte.send(mensaje).await {
            Ok(_) => true,
            Err(e) => {
                tracing::warn!("No se pudo enviar el correo a «{para}»: {e}");
                false
            }
        }
    }
}
