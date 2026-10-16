//! Estados de la orden y única función que los cambia (research R14, data-model «Máquina de
//! estados»). No se añaden transiciones sin actualizar antes data-model.md.

use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

use crate::error::{ErrorApi, Resultado};

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[sqlx(type_name = "estado_orden", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Estado {
    Recibida,
    EnDiagnostico,
    EnReparacion,
    EnEsperaPiezas,
    PendienteConformidad,
    Terminada,
    Facturada,
}

impl Estado {
    /// Texto en minúsculas para los mensajes de error («La orden está en reparación…»).
    pub fn texto(self) -> &'static str {
        match self {
            Estado::Recibida => "recibida",
            Estado::EnDiagnostico => "en diagnóstico",
            Estado::EnReparacion => "en reparación",
            Estado::EnEsperaPiezas => "en espera de piezas",
            Estado::PendienteConformidad => "pendiente de conformidad",
            Estado::Terminada => "terminada",
            Estado::Facturada => "facturada",
        }
    }
}

/// Las 9 transiciones de data-model. Abrir una orden (`— → recibida`) es un `INSERT` de spec-002
/// y no pasa por aquí.
pub const TRANSICIONES: &[(Estado, Estado)] = &[
    (Estado::Recibida, Estado::EnDiagnostico),
    (Estado::EnDiagnostico, Estado::EnReparacion),
    (Estado::EnDiagnostico, Estado::EnDiagnostico),
    (Estado::EnReparacion, Estado::PendienteConformidad),
    (Estado::PendienteConformidad, Estado::EnReparacion),
    (Estado::EnReparacion, Estado::EnEsperaPiezas),
    (Estado::EnEsperaPiezas, Estado::EnReparacion),
    (Estado::EnReparacion, Estado::Terminada),
    (Estado::Terminada, Estado::Facturada),
];

/// Un cambio de estado ya hecho; también es el contenido del evento del tablero. Quien lo
/// recibe lo publica con `CambioEstado::publicar` (en `eventos.rs`) **después del `commit`**.
#[must_use = "publica el cambio con `publicar` después del commit"]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct CambioEstado {
    pub id: i64,
    pub estado: Estado,
}

pub async fn cambiar_estado(
    tx: &mut PgConnection,
    orden_id: i64,
    desde: &[Estado],
    hacia: Estado,
) -> Resultado<CambioEstado> {
    if desde.is_empty() || desde.iter().any(|d| !TRANSICIONES.contains(&(*d, hacia))) {
        return Err(ErrorApi::Interno(format!(
            "Transición de estado no permitida: {desde:?} → {hacia:?}"
        )));
    }

    let filas = sqlx::query(
        "UPDATE orden
            SET estado = $2,
                terminada_en = CASE WHEN $2 = 'terminada'::estado_orden THEN now() ELSE terminada_en END
          WHERE id = $1 AND estado = ANY($3)",
    )
    .bind(orden_id)
    .bind(hacia)
    .bind(desde)
    .execute(&mut *tx)
    .await?
    .rows_affected();

    if filas == 0 {
        let actual: Option<Estado> = sqlx::query_scalar("SELECT estado FROM orden WHERE id = $1")
            .bind(orden_id)
            .fetch_optional(&mut *tx)
            .await?;
        return Err(match actual {
            None => ErrorApi::NoEncontrado("No existe la orden".to_string()),
            Some(actual) => ErrorApi::Conflicto(format!(
                "La orden está {}; no se puede pasar a {}",
                actual.texto(),
                hacia.texto()
            )),
        });
    }

    Ok(CambioEstado {
        id: orden_id,
        estado: hacia,
    })
}
