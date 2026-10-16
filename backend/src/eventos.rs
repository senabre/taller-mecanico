//! Bus de eventos del tablero (research R11): cada cambio de estado de una orden se envía a los
//! navegadores que escuchan `GET /api/eventos` (el endpoint SSE llega con la fase 3).

use tokio::sync::broadcast;

use crate::estados::CambioEstado;

#[derive(Clone)]
pub struct Eventos(broadcast::Sender<CambioEstado>);

impl Eventos {
    pub fn nuevo() -> Self {
        let (emisor, _) = broadcast::channel(64);
        Eventos(emisor)
    }

    pub fn suscribir(&self) -> broadcast::Receiver<CambioEstado> {
        self.0.subscribe()
    }
}

impl CambioEstado {
    /// Se llama **después del `commit`**, así el tablero nunca relee un estado sin confirmar.
    /// Spec-002 también lo usa al abrir una orden: `CambioEstado { id, estado: Estado::Recibida }`.
    pub fn publicar(self, eventos: &Eventos) {
        // Si no hay ningún tablero abierto, no hay nadie a quien avisar: no es un error.
        let _ = eventos.0.send(self);
    }
}
