//! spec-008 · Tablero de órdenes abiertas (HU-08): `GET /api/tablero`, solo para el jefe
//! (research D1, D2 y D4 de spec-008). Solo lee: no cambia ningún estado.

use axum::Router;
use axum::extract::State;
use axum::routing::get;
use serde::Serialize;

use crate::EstadoApp;
use crate::auth::{Rol, Usuario};
use crate::error::{Json, Resultado};
use crate::estados::Estado;
use crate::rutas::ordenes::{self, OrdenResumen};

pub fn rutas() -> Router<EstadoApp> {
    Router::new().route("/tablero", get(ver_tablero))
}

/// Las 6 columnas, en este orden (FR-001). `facturada` es el único estado que no sale.
const ESTADOS_ABIERTOS: [Estado; 6] = [
    Estado::Recibida,
    Estado::EnDiagnostico,
    Estado::EnReparacion,
    Estado::EnEsperaPiezas,
    Estado::PendienteConformidad,
    Estado::Terminada,
];

#[derive(Serialize)]
struct Columna {
    estado: Estado,
    ordenes: Vec<OrdenResumen>,
}

#[derive(Serialize, sqlx::FromRow)]
struct CargaMecanico {
    id: i64,
    nombre: String,
    activo: bool,
    ordenes_abiertas: i64,
}

/// El objeto de `GET /api/tablero` del contrato.
#[derive(Serialize)]
struct Tablero {
    columnas: Vec<Columna>,
    carga_mecanicos: Vec<CargaMecanico>,
}

async fn ver_tablero(
    usuario: Usuario,
    State(estado): State<EstadoApp>,
) -> Resultado<Json<Tablero>> {
    usuario.exigir_rol(&[Rol::Jefe])?;

    // Una sola foto para las tarjetas y la carga: si entre las dos consultas se abre o se factura
    // una orden, la carga sigue cuadrando con las tarjetas (D1, SC-003).
    let mut tx = estado
        .db
        .begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .await?;

    // Las tarjetas son `OrdenResumen` de spec-002, con `retrasada` ya calculada (I1 e I2).
    let mut abiertas = ordenes::resumenes(&mut tx, None, true).await?;
    abiertas.sort_by_key(|o| (o.fecha_entrega_prometida, o.numero));
    let mut columnas: Vec<Columna> = ESTADOS_ABIERTOS
        .into_iter()
        .map(|estado| Columna {
            estado,
            ordenes: Vec::new(),
        })
        .collect();
    for orden in abiertas {
        if let Some(columna) = columnas.iter_mut().find(|c| c.estado == orden.estado) {
            columna.ordenes.push(orden);
        }
    }

    // Carga (D2): los mecánicos activos aunque tengan 0 y cualquier usuario con órdenes abiertas,
    // aunque esté de baja. El `HAVING` no filtra por rol a quien tiene órdenes, así que la suma es
    // siempre el número de tarjetas (SC-003).
    let carga_mecanicos: Vec<CargaMecanico> = sqlx::query_as(
        "SELECT u.id, u.nombre, u.activo, count(o.id) AS ordenes_abiertas
           FROM usuario u
           LEFT JOIN orden o ON o.mecanico_id = u.id AND o.estado <> 'facturada'
          GROUP BY u.id
         HAVING (u.rol = 'mecanico' AND u.activo) OR count(o.id) > 0
          ORDER BY u.nombre, u.id",
    )
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(Json(Tablero {
        columnas,
        carga_mecanicos,
    }))
}
