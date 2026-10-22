//! Almacén y pedidos a proveedor (spec-006; contrato, sección «spec-006 · Almacén y pedidos»).
//! Las reglas las garantiza la base de datos (research D2, D3); aquí solo se validan los datos y
//! se traduce cada error al mensaje del contrato.

use std::str::FromStr;

use axum::Router;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgConnection;

use crate::EstadoApp;
use crate::auth::{Rol, Usuario};
use crate::error::{ErrorApi, Json, Resultado};
use crate::estados::{Estado, cambiar_estado};

pub fn rutas() -> Router<EstadoApp> {
    Router::new()
        .route("/piezas", get(listar_piezas).post(crear_pieza))
        .route("/proveedores", get(listar_proveedores))
        .route("/pedidos", get(listar_pedidos).post(crear_pedido))
        .route("/pedidos/{id}/recibir", post(recibir_pedido))
}

fn validacion(mensaje: impl Into<String>) -> ErrorApi {
    ErrorApi::Validacion {
        mensaje: mensaje.into(),
        detalle: None,
    }
}

/// `true` si el error es la violación de esa restricción `UNIQUE` (o índice único).
fn viola(error: &sqlx::Error, restriccion: &str) -> bool {
    matches!(error, sqlx::Error::Database(e) if e.constraint() == Some(restriccion))
}

/// Entero de un JSON que no admite decimales ni texto (`1.5` o `"2"` no valen).
fn entero(valor: &Option<Value>) -> Option<i64> {
    valor.as_ref().and_then(Value::as_i64)
}

// ---------------------------------------------------------------------------------------------
// Piezas (US1)
// ---------------------------------------------------------------------------------------------

#[derive(Serialize, sqlx::FromRow)]
struct Pieza {
    id: i64,
    referencia: String,
    descripcion: String,
    #[serde(with = "rust_decimal::serde::str")]
    precio_venta: Decimal,
    stock: i32,
}

/// Los campos llegan como JSON sin tipar para responder 422 (y no 400) a un tipo equivocado.
#[derive(Deserialize)]
struct NuevaPieza {
    referencia: Option<String>,
    descripcion: Option<String>,
    precio_venta: Option<Value>,
    stock: Option<Value>,
}

/// Importe ≥ 0 con 2 decimales como máximo, como texto (`"25.00"`) o número (research D6).
fn precio(valor: &Option<Value>) -> Option<Decimal> {
    let decimal = match valor.as_ref()? {
        Value::String(texto) => Decimal::from_str(texto.trim()).ok()?,
        Value::Number(numero) => Decimal::from_str(&numero.to_string()).ok()?,
        _ => return None,
    };
    if decimal.is_sign_negative() || decimal.normalize().scale() > 2 {
        return None;
    }
    let mut decimal = decimal;
    decimal.rescale(2);
    Some(decimal)
}

async fn crear_pieza(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
    Json(nueva): Json<NuevaPieza>,
) -> Resultado<(StatusCode, Json<Pieza>)> {
    usuario.exigir_rol(&[Rol::Almacen])?;

    let referencia = nueva.referencia.as_deref().unwrap_or("").trim();
    if referencia.is_empty() {
        return Err(validacion("Indica la referencia de la pieza"));
    }
    let descripcion = nueva.descripcion.as_deref().unwrap_or("").trim();
    if descripcion.is_empty() {
        return Err(validacion("Indica la descripción de la pieza"));
    }
    let precio_venta = precio(&nueva.precio_venta).ok_or_else(|| {
        validacion("El precio de venta debe ser un importe de 0 o más, con 2 decimales como máximo")
    })?;
    let stock = match &nueva.stock {
        None | Some(Value::Null) => 0,
        otro => entero(otro)
            .filter(|s| *s >= 0)
            .and_then(|s| i32::try_from(s).ok())
            .ok_or_else(|| validacion("El stock debe ser un número entero de 0 o más"))?,
    };

    let pieza = sqlx::query_as(
        "INSERT INTO pieza (referencia, descripcion, precio_venta, stock)
         VALUES ($1, $2, $3, $4)
         RETURNING id, referencia, descripcion, precio_venta, stock",
    )
    .bind(referencia)
    .bind(descripcion)
    .bind(precio_venta)
    .bind(stock)
    .fetch_one(&estado.db)
    .await
    .map_err(|e| {
        if viola(&e, "pieza_referencia_key") {
            ErrorApi::Conflicto(format!("Ya existe la referencia {referencia}"))
        } else {
            e.into()
        }
    })?;
    Ok((StatusCode::CREATED, Json(pieza)))
}

#[derive(Deserialize)]
struct BusquedaPiezas {
    q: Option<String>,
}

/// Todos los roles con sesión: la usan también spec-003 y spec-005 para elegir piezas.
async fn listar_piezas(
    State(estado): State<EstadoApp>,
    _usuario: Usuario,
    Query(busqueda): Query<BusquedaPiezas>,
) -> Resultado<Json<Vec<Pieza>>> {
    let texto = busqueda.q.as_deref().map(str::trim).unwrap_or("");
    // Se escapan los comodines de ILIKE para buscar el texto tal cual.
    let patron = format!(
        "%{}%",
        texto
            .replace('\\', "\\\\")
            .replace('%', "\\%")
            .replace('_', "\\_")
    );
    let piezas = sqlx::query_as(
        "SELECT id, referencia, descripcion, precio_venta, stock
           FROM pieza
          WHERE referencia ILIKE $1 OR descripcion ILIKE $1
          ORDER BY referencia",
    )
    .bind(patron)
    .fetch_all(&estado.db)
    .await?;
    Ok(Json(piezas))
}

// ---------------------------------------------------------------------------------------------
// Proveedores (precargados: solo lectura, research D1)
// ---------------------------------------------------------------------------------------------

#[derive(Serialize, sqlx::FromRow)]
struct Proveedor {
    id: i64,
    nombre: String,
}

async fn listar_proveedores(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
) -> Resultado<Json<Vec<Proveedor>>> {
    usuario.exigir_rol(&[Rol::Almacen])?;
    let proveedores = sqlx::query_as("SELECT id, nombre FROM proveedor ORDER BY nombre")
        .fetch_all(&estado.db)
        .await?;
    Ok(Json(proveedores))
}

// ---------------------------------------------------------------------------------------------
// Pedidos (US2, US3, US4)
// ---------------------------------------------------------------------------------------------

#[derive(Serialize)]
struct ProveedorDePedido {
    id: i64,
    nombre: String,
}

#[derive(Serialize)]
struct PiezaDePedido {
    id: i64,
    referencia: String,
    descripcion: String,
}

#[derive(Serialize)]
struct OrdenDePedido {
    id: i64,
    numero: i64,
}

/// El objeto `Pedido` del contrato.
#[derive(Serialize)]
pub struct Pedido {
    id: i64,
    proveedor: ProveedorDePedido,
    pieza: PiezaDePedido,
    cantidad: i32,
    orden: OrdenDePedido,
    estado: String,
    pedido_en: DateTime<Utc>,
    recibido_en: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
struct FilaPedido {
    id: i64,
    proveedor_id: i64,
    proveedor_nombre: String,
    pieza_id: i64,
    referencia: String,
    descripcion: String,
    cantidad: i32,
    orden_id: i64,
    orden_numero: i64,
    estado: String,
    pedido_en: DateTime<Utc>,
    recibido_en: Option<DateTime<Utc>>,
}

impl From<FilaPedido> for Pedido {
    fn from(f: FilaPedido) -> Self {
        Pedido {
            id: f.id,
            proveedor: ProveedorDePedido {
                id: f.proveedor_id,
                nombre: f.proveedor_nombre,
            },
            pieza: PiezaDePedido {
                id: f.pieza_id,
                referencia: f.referencia,
                descripcion: f.descripcion,
            },
            cantidad: f.cantidad,
            orden: OrdenDePedido {
                id: f.orden_id,
                numero: f.orden_numero,
            },
            estado: f.estado,
            pedido_en: f.pedido_en,
            recibido_en: f.recibido_en,
        }
    }
}

/// Las condiciones (`WHERE …`) y el orden se añaden detrás.
const SELECT_PEDIDO: &str = "
    SELECT p.id, pr.id AS proveedor_id, pr.nombre AS proveedor_nombre,
           pz.id AS pieza_id, pz.referencia, pz.descripcion, p.cantidad,
           o.id AS orden_id, o.numero AS orden_numero, p.estado::text AS estado,
           p.pedido_en, p.recibido_en
      FROM pedido p
      JOIN proveedor pr ON pr.id = p.proveedor_id
      JOIN pieza pz ON pz.id = p.pieza_id
      JOIN orden o ON o.id = p.orden_id";

async fn pedido_por_id(conexion: &mut PgConnection, id: i64) -> Resultado<Pedido> {
    let fila: FilaPedido = sqlx::query_as(&format!("{SELECT_PEDIDO} WHERE p.id = $1"))
        .bind(id)
        .fetch_one(conexion)
        .await?;
    Ok(fila.into())
}

/// Todos los pedidos de una orden, del más antiguo al más reciente. Es el campo `pedidos` de
/// `GET /api/ordenes/{id}` (spec-002 lo llama desde `ordenes.rs`; T027).
pub async fn pedidos_de_orden(
    conexion: &mut PgConnection,
    orden_id: i64,
) -> Resultado<Vec<Pedido>> {
    let filas: Vec<FilaPedido> = sqlx::query_as(&format!(
        "{SELECT_PEDIDO} WHERE p.orden_id = $1 ORDER BY p.pedido_en, p.id"
    ))
    .bind(orden_id)
    .fetch_all(conexion)
    .await?;
    Ok(filas.into_iter().map(Pedido::from).collect())
}

#[derive(Deserialize)]
struct FiltroPedidos {
    estado: Option<String>,
}

async fn listar_pedidos(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
    Query(filtro): Query<FiltroPedidos>,
) -> Resultado<Json<Vec<Pedido>>> {
    usuario.exigir_rol(&[Rol::Almacen])?;
    let estado_pedido = match filtro.estado.as_deref() {
        None | Some("") => None,
        Some(e @ ("pendiente" | "recibido")) => Some(e),
        Some(_) => {
            return Err(validacion(
                "El estado del pedido debe ser pendiente o recibido",
            ));
        }
    };
    let filas: Vec<FilaPedido> = sqlx::query_as(&format!(
        "{SELECT_PEDIDO}
          WHERE $1::estado_pedido IS NULL OR p.estado = $1::estado_pedido
          ORDER BY p.pedido_en, p.id"
    ))
    .bind(estado_pedido)
    .fetch_all(&estado.db)
    .await?;
    Ok(Json(filas.into_iter().map(Pedido::from).collect()))
}

#[derive(Deserialize)]
struct NuevoPedido {
    proveedor_id: Option<Value>,
    pieza_id: Option<Value>,
    cantidad: Option<Value>,
    orden_numero: Option<Value>,
}

/// FR-003…FR-005 y FR-004a (plan, «Crear pedido»; research D2, D3 y D5).
async fn crear_pedido(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
    Json(nuevo): Json<NuevoPedido>,
) -> Resultado<(StatusCode, Json<Pedido>)> {
    usuario.exigir_rol(&[Rol::Almacen])?;

    let cantidad = entero(&nuevo.cantidad)
        .filter(|c| *c > 0)
        .and_then(|c| i32::try_from(c).ok())
        .ok_or_else(|| validacion("La cantidad debe ser un número entero mayor que 0"))?;
    let orden_numero =
        entero(&nuevo.orden_numero).ok_or_else(|| validacion("Indica el número de la orden"))?;

    let mut tx = estado.db.begin().await?;

    let proveedor_existe: bool = match entero(&nuevo.proveedor_id) {
        Some(id) => {
            sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM proveedor WHERE id = $1)")
                .bind(id)
                .fetch_one(&mut *tx)
                .await?
        }
        None => false,
    };
    if !proveedor_existe {
        return Err(validacion("Elige un proveedor de la lista"));
    }
    let pieza: Option<(i64, String)> = match entero(&nuevo.pieza_id) {
        Some(id) => {
            sqlx::query_as("SELECT id, referencia FROM pieza WHERE id = $1")
                .bind(id)
                .fetch_optional(&mut *tx)
                .await?
        }
        None => None,
    };
    let (pieza_id, referencia) =
        pieza.ok_or_else(|| validacion("Elige una referencia de pieza dada de alta"))?;

    // Bloquea la orden hasta el commit (research D3).
    let orden: Option<(i64, Estado)> =
        sqlx::query_as("SELECT id, estado FROM orden WHERE numero = $1 FOR UPDATE")
            .bind(orden_numero)
            .fetch_optional(&mut *tx)
            .await?;
    let (orden_id, estado_orden) =
        orden.ok_or_else(|| validacion(format!("No existe la orden {orden_numero}")))?;
    if !matches!(estado_orden, Estado::EnReparacion | Estado::EnEsperaPiezas) {
        return Err(ErrorApi::Conflicto(format!(
            "La orden {orden_numero} está {}; solo se puede pedir para órdenes en reparación o \
             en espera de piezas",
            estado_orden.texto()
        )));
    }

    let pedido_id: i64 = sqlx::query_scalar(
        "INSERT INTO pedido (proveedor_id, pieza_id, cantidad, orden_id)
         VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(entero(&nuevo.proveedor_id))
    .bind(pieza_id)
    .bind(cantidad)
    .bind(orden_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        if viola(&e, "pedido_pendiente_unico") {
            ErrorApi::Conflicto(format!(
                "Ya hay un pedido pendiente de {referencia} para la orden {orden_numero}"
            ))
        } else {
            e.into()
        }
    })?;

    let cambio = if estado_orden == Estado::EnReparacion {
        Some(
            cambiar_estado(
                &mut tx,
                orden_id,
                &[Estado::EnReparacion],
                Estado::EnEsperaPiezas,
            )
            .await?,
        )
    } else {
        None
    };
    let pedido = pedido_por_id(&mut tx, pedido_id).await?;
    tx.commit().await?;

    // Después del commit, para que el tablero nunca lea un estado sin confirmar (R11, R14).
    if let Some(cambio) = cambio {
        cambio.publicar(&estado.eventos);
    }
    Ok((StatusCode::CREATED, Json(pedido)))
}

/// FR-007 y FR-008 (plan, «Recibir»; research D3, D4 y D8).
async fn recibir_pedido(
    State(estado): State<EstadoApp>,
    usuario: Usuario,
    Path(id): Path<i64>,
) -> Resultado<Json<Pedido>> {
    usuario.exigir_rol(&[Rol::Almacen])?;

    let mut tx = estado.db.begin().await?;

    let pedido: Option<(i64, i64)> =
        sqlx::query_as("SELECT orden_id, pieza_id FROM pedido WHERE id = $1")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?;
    let (orden_id, pieza_id) =
        pedido.ok_or_else(|| ErrorApi::NoEncontrado("No existe el pedido".to_string()))?;

    // Bloquea la orden hasta el commit (research D3).
    let estado_orden: Estado =
        sqlx::query_scalar("SELECT estado FROM orden WHERE id = $1 FOR UPDATE")
            .bind(orden_id)
            .fetch_one(&mut *tx)
            .await?;

    let cantidad: Option<i32> = sqlx::query_scalar(
        "UPDATE pedido SET estado = 'recibido', recibido_en = now()
          WHERE id = $1 AND estado = 'pendiente'
          RETURNING cantidad",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;
    let cantidad =
        cantidad.ok_or_else(|| ErrorApi::Conflicto("Este pedido ya está recibido".to_string()))?;

    sqlx::query("UPDATE pieza SET stock = stock + $2 WHERE id = $1")
        .bind(pieza_id)
        .bind(cantidad)
        .execute(&mut *tx)
        .await?;

    let quedan_pendientes: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM pedido WHERE orden_id = $1 AND estado = 'pendiente')",
    )
    .bind(orden_id)
    .fetch_one(&mut *tx)
    .await?;
    let cambio = if !quedan_pendientes && estado_orden == Estado::EnEsperaPiezas {
        Some(
            cambiar_estado(
                &mut tx,
                orden_id,
                &[Estado::EnEsperaPiezas],
                Estado::EnReparacion,
            )
            .await?,
        )
    } else {
        None
    };
    let pedido = pedido_por_id(&mut tx, id).await?;
    tx.commit().await?;

    // Solo el evento del tablero: sin correo ni aviso al mecánico (research D8).
    if let Some(cambio) = cambio {
        cambio.publicar(&estado.eventos);
    }
    Ok(Json(pedido))
}
