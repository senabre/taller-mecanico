//! Bus de eventos del tablero (research R11): cada cambio de estado de una orden se envía a los
//! navegadores que escuchan `GET /api/eventos` (Server-Sent Events).

use std::convert::Infallible;

use axum::Router;
use axum::extract::State;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::routing::get;
use futures_util::{Stream, StreamExt};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

use crate::EstadoApp;
use crate::auth::Usuario;
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

pub fn rutas() -> Router<EstadoApp> {
    Router::new().route("/eventos", get(escuchar))
}

/// `GET /api/eventos`: un evento SSE `orden` con `{id, estado}` por cada cambio de estado. El
/// navegador (`EventSource`) se reconecta solo; si se pierden avisos por ir lento (`Lagged`), se
/// ignoran, porque el tablero vuelve a pedir `GET /api/tablero` en el siguiente.
async fn escuchar(
    _usuario: Usuario,
    State(estado): State<EstadoApp>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let flujo = BroadcastStream::new(estado.eventos.suscribir()).filter_map(|recibido| async move {
        let cambio = recibido.ok()?;
        let evento = Event::default().event("orden").json_data(cambio).ok()?;
        Some(Ok(evento))
    });
    Sse::new(flujo).keep_alive(KeepAlive::default())
}
