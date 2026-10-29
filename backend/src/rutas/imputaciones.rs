//! Imputaciones de horas y piezas, bloqueo del 10 % y terminar la reparación (spec-005; contrato,
//! sección «spec-005 · Imputaciones»). Las ampliaciones usan los endpoints de spec-003.

use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::post;
use rust_decimal::Decimal;
use serde::Serialize;
use sqlx::PgConnection;

use crate::EstadoApp;
use crate::auth::Usuario;
use crate::error::{ErrorApi, Json, Resultado};
use crate::estados::{Estado, cambiar_estado};
use crate::lineas::{Linea, NuevaLinea, validar};

pub fn rutas() -> Router<EstadoApp> {
    Router::new()
        .route("/ordenes/{id}/imputaciones", post(imputar))
        .route("/ordenes/{id}/terminar", post(terminar))
}

/// El objeto `ResumenImputaciones` del contrato.
#[derive(Serialize)]
pub struct ResumenImputaciones {
    orden_id: i64,
    estado: Estado,
    #[serde(with = "rust_decimal::serde::str")]
    coste_acumulado: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    total_presupuestado: Decimal,
    imputaciones: Vec<Linea>,
}

/// Lo imputado y lo presupuestado de una orden, calculados al vuelo (research D4).
async fn acumulado_y_presupuestado(
    conexion: &mut PgConnection,
    orden_id: i64,
) -> Resultado<(Decimal, Decimal)> {
    Ok(sqlx::query_as(
        "SELECT (SELECT coalesce(sum(importe), 0.00) FROM imputacion WHERE orden_id = $1),
                (SELECT coalesce(sum(base), 0.00) FROM presupuesto
                  WHERE orden_id = $1 AND estado = 'aceptado')",
    )
    .bind(orden_id)
    .fetch_one(conexion)
    .await?)
}

/// Las imputaciones de una orden con su acumulado y lo presupuestado (la suma de los presupuestos
/// aceptados: inicial y ampliaciones). Spec-002 la llama desde `ordenes.rs` para llenar esos campos
/// de `GET /api/ordenes/{id}` (research D5).
pub async fn resumen_imputaciones(
    conexion: &mut PgConnection,
    orden_id: i64,
) -> Resultado<ResumenImputaciones> {
    let estado: Estado = sqlx::query_scalar("SELECT estado FROM orden WHERE id = $1")
        .bind(orden_id)
        .fetch_optional(&mut *conexion)
        .await?
        .ok_or_else(|| ErrorApi::NoEncontrado("No existe la orden".to_string()))?;
    let (mut coste_acumulado, mut total_presupuestado) =
        acumulado_y_presupuestado(conexion, orden_id).await?;
    coste_acumulado.rescale(2);
    total_presupuestado.rescale(2);
    let imputaciones = sqlx::query_as(
        "SELECT id, tipo::text AS tipo, concepto, pieza_id, cantidad, precio_unitario, importe
           FROM imputacion WHERE orden_id = $1 ORDER BY creada_en, id",
    )
    .bind(orden_id)
    .fetch_all(&mut *conexion)
    .await?;
    Ok(ResumenImputaciones {
        orden_id,
        estado,
        coste_acumulado,
        total_presupuestado,
        imputaciones,
    })
}

/// Bloquea la orden hasta el `commit`, exige que la imputa (o termina) su mecánico o el jefe y
/// devuelve su estado.
async fn orden_del_mecanico(
    tx: &mut PgConnection,
    orden_id: i64,
    usuario: &Usuario,
) -> Resultado<Estado> {
    let (estado, mecanico_id): (Estado, i64) =
        sqlx::query_as("SELECT estado, mecanico_id FROM orden WHERE id = $1 FOR UPDATE")
            .bind(orden_id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| ErrorApi::NoEncontrado("No existe la orden".to_string()))?;
    usuario.exigir_mecanico_de(mecanico_id)?;
    Ok(estado)
}

/// FR-001…FR-005 (research D3 y D4).
async fn imputar(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
    Path(id): Path<i64>,
    Json(nueva): Json<NuevaLinea>,
) -> Resultado<(StatusCode, Json<ResumenImputaciones>)> {
    let mut tx = estado.db.begin().await?;
    let estado_orden = orden_del_mecanico(&mut tx, id, &usuario).await?;
    if estado_orden != Estado::EnReparacion {
        return Err(ErrorApi::Conflicto(format!(
            "La orden está {}; solo se puede imputar en órdenes en reparación",
            estado_orden.texto()
        )));
    }
    let aceptado: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM presupuesto WHERE orden_id = $1 AND estado = 'aceptado')",
    )
    .bind(id)
    .fetch_one(&mut *tx)
    .await?;
    if !aceptado {
        return Err(ErrorApi::Conflicto(
            "La orden no tiene un presupuesto aceptado".to_string(),
        ));
    }

    // Mismas reglas que una línea de presupuesto (lineas.rs).
    let linea = validar(&mut tx, &estado.config, &nueva).await?;

    if let Some(pieza_id) = linea.pieza_id {
        // Solo se descuenta si hay bastante stock: nunca queda negativo.
        let descontada =
            sqlx::query("UPDATE pieza SET stock = stock - $2 WHERE id = $1 AND stock >= $2")
                .bind(pieza_id)
                .bind(linea.cantidad)
                .execute(&mut *tx)
                .await?
                .rows_affected();
        if descontada == 0 {
            let (stock, referencia): (i32, String) =
                sqlx::query_as("SELECT stock, referencia FROM pieza WHERE id = $1")
                    .bind(pieza_id)
                    .fetch_one(&mut *tx)
                    .await?;
            let unidades = if stock == 1 { "unidad" } else { "unidades" };
            return Err(ErrorApi::Validacion {
                mensaje: format!(
                    "Solo hay {stock} {unidades} en stock de {referencia}; el almacén puede \
                     pedirla al proveedor"
                ),
                detalle: None,
            });
        }
    }

    sqlx::query(
        "INSERT INTO imputacion (orden_id, tipo, concepto, pieza_id, cantidad, precio_unitario,
                                 importe, usuario_id)
         VALUES ($1, $2::tipo_linea, $3, $4, $5, $6, $7, $8)",
    )
    .bind(id)
    .bind(linea.tipo)
    .bind(linea.concepto)
    .bind(linea.pieza_id)
    .bind(linea.cantidad)
    .bind(linea.precio_unitario)
    .bind(linea.importe)
    .bind(usuario.id)
    .execute(&mut *tx)
    .await?;

    // La imputación que cruza el 110 % se guarda; lo que se bloquea son las siguientes (FR-005).
    let (acumulado, presupuestado) = acumulado_y_presupuestado(&mut tx, id).await?;
    let cambio = if acumulado > presupuestado * Decimal::new(110, 2) {
        Some(
            cambiar_estado(
                &mut tx,
                id,
                &[Estado::EnReparacion],
                Estado::PendienteConformidad,
            )
            .await?,
        )
    } else {
        None
    };
    let resumen = resumen_imputaciones(&mut tx, id).await?;
    tx.commit().await?;

    // Después del commit, para que el tablero nunca lea un estado sin confirmar (R11, R14).
    if let Some(cambio) = cambio {
        cambio.publicar(&estado.eventos);
    }
    Ok((StatusCode::CREATED, Json(resumen)))
}

/// FR-008 (research D6).
async fn terminar(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
    Path(id): Path<i64>,
) -> Resultado<Json<ResumenImputaciones>> {
    let mut tx = estado.db.begin().await?;
    let estado_orden = orden_del_mecanico(&mut tx, id, &usuario).await?;
    if estado_orden != Estado::EnReparacion {
        return Err(ErrorApi::Conflicto(format!(
            "La orden está {}; solo se puede terminar una orden en reparación",
            estado_orden.texto()
        )));
    }
    let con_imputaciones: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM imputacion WHERE orden_id = $1)")
            .bind(id)
            .fetch_one(&mut *tx)
            .await?;
    if !con_imputaciones {
        return Err(ErrorApi::Validacion {
            mensaje: "Primero hay que imputar el trabajo realizado".to_string(),
            detalle: None,
        });
    }

    let cambio = cambiar_estado(&mut tx, id, &[Estado::EnReparacion], Estado::Terminada).await?;
    let resumen = resumen_imputaciones(&mut tx, id).await?;
    tx.commit().await?;
    cambio.publicar(&estado.eventos);
    Ok(Json(resumen))
}
