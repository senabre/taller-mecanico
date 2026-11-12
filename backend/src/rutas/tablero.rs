//! spec-008 · Tablero de órdenes abiertas (HU-08): `GET /api/tablero`, solo para el jefe
//! (research D1, D2 y D4 de spec-008). Solo lee: no cambia ningún estado.

use std::collections::HashMap;

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

#[derive(Serialize)]
struct CargaMecanico {
    id: i64,
    nombre: String,
    activo: bool,
    ordenes_abiertas: i64,
}

/// Un usuario que puede salir en la carga: los mecánicos y quien tenga órdenes abiertas.
#[derive(sqlx::FromRow)]
struct UsuarioCarga {
    id: i64,
    nombre: String,
    activo: bool,
    rol: String,
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

    // Sin transacción (hallazgo V3): si el navegador corta la petición a mitad de un `BEGIN`, la
    // conexión podía volver al pool dentro de la transacción y hacer fallar a toda la aplicación.
    // La carga se cuenta en Rust a partir de la misma lista de órdenes que las columnas, así que
    // cuadra con las tarjetas por construcción (D1, SC-003).
    let mut conexion = estado.db.acquire().await?;

    // Las tarjetas son `OrdenResumen` de spec-002, con `retrasada` ya calculada (I1 e I2).
    let mut abiertas = ordenes::resumenes(&mut conexion, None, true).await?;
    abiertas.sort_by_key(|o| (o.fecha_entrega_prometida, o.numero));

    // Órdenes abiertas por mecánico, de esa misma lista.
    let mut por_mecanico: HashMap<i64, i64> = HashMap::new();
    for orden in &abiertas {
        *por_mecanico.entry(orden.mecanico.id).or_default() += 1;
    }

    // A la base, solo los usuarios de la carga: los mecánicos y quien tenga órdenes abiertas, por
    // nombre (D2).
    let con_ordenes: Vec<i64> = por_mecanico.keys().copied().collect();
    let usuarios: Vec<UsuarioCarga> = sqlx::query_as(
        "SELECT id, nombre, activo, rol::text AS rol FROM usuario
          WHERE rol = 'mecanico' OR id = ANY($1)
          ORDER BY nombre, id",
    )
    .bind(&con_ordenes)
    .fetch_all(&mut *conexion)
    .await?;
    // Los mecánicos activos aunque tengan 0, y cualquier usuario con órdenes abiertas, aunque esté
    // de baja (Q1).
    let carga_mecanicos: Vec<CargaMecanico> = usuarios
        .into_iter()
        .filter_map(|u| {
            let ordenes_abiertas = por_mecanico.get(&u.id).copied().unwrap_or(0);
            ((u.rol == "mecanico" && u.activo) || ordenes_abiertas > 0).then_some(CargaMecanico {
                id: u.id,
                nombre: u.nombre,
                activo: u.activo,
                ordenes_abiertas,
            })
        })
        .collect();

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

    Ok(Json(Tablero {
        columnas,
        carga_mecanicos,
    }))
}
