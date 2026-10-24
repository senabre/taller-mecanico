//! Citas (spec-001; contrato, sección «spec-001 · Citas»): pedir cita desde la página pública y
//! lista de citas del día para recepción. El orden de las comprobaciones del `POST` es el de la
//! tabla de validación del contrato: todos los 422 antes que cualquier 409 (research D2 y D3).

use axum::Router;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use chrono::{Datelike, Days, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use sqlx::PgExecutor;

use crate::EstadoApp;
use crate::auth::{Rol, Usuario};
use crate::error::{ErrorApi, Json, Resultado};
use crate::fechas;
use crate::matricula::normalizar_matricula;

pub fn rutas() -> Router<EstadoApp> {
    Router::new()
        .route("/publico/vehiculos/{matricula}", get(vehiculo_publico))
        .route("/publico/dias-disponibles", get(dias_disponibles))
        .route("/publico/citas", post(crear_cita))
        .route("/citas", get(citas_del_dia))
}

/// 422 con el campo afectado, para que el formulario muestre el mensaje bajo ese campo
/// (contrato, tabla de validación de spec-001).
fn error_campo(campo: &str, mensaje: &str) -> ErrorApi {
    ErrorApi::Validacion {
        mensaje: mensaje.to_string(),
        detalle: Some(serde_json::json!({ "campo": campo })),
    }
}

/// Días que se pueden pedir: de lunes a viernes, desde mañana hasta `hoy + 30` incluido (FR-006).
fn dias_ofrecidos(hoy: NaiveDate) -> Vec<NaiveDate> {
    (1..=30)
        .map(|d| hoy + Days::new(d))
        .filter(|dia| !matches!(dia.weekday(), Weekday::Sat | Weekday::Sun))
        .collect()
}

/// Id del vehículo con esa matrícula (ya normalizada), si existe.
async fn buscar_vehiculo(db: impl PgExecutor<'_>, matricula: &str) -> Resultado<Option<i64>> {
    let id = sqlx::query_scalar("SELECT id FROM vehiculo WHERE matricula = $1")
        .bind(matricula)
        .fetch_optional(db)
        .await?;
    Ok(id)
}

const CITA_PENDIENTE: &str = "Este vehículo ya tiene una cita pendiente.";

/// Lo único que la página pública sabe de un vehículo: nunca datos del cliente ni la fecha de su
/// cita pendiente (FR-003, FR-010).
#[derive(Serialize, sqlx::FromRow)]
struct VehiculoPublico {
    matricula: String,
    marca: String,
    modelo: String,
    cita_pendiente: bool,
}

async fn vehiculo_publico(
    State(estado): State<EstadoApp>,
    Path(texto): Path<String>,
) -> Resultado<Json<VehiculoPublico>> {
    let matricula = normalizar_matricula(&texto)
        .ok_or_else(|| error_campo("matricula", "La matrícula no es válida"))?;
    sqlx::query_as(
        "SELECT v.matricula, v.marca, v.modelo,
                EXISTS (SELECT 1 FROM cita c WHERE c.vehiculo_id = v.id AND c.fecha >= $2)
                    AS cita_pendiente
           FROM vehiculo v
          WHERE v.matricula = $1",
    )
    .bind(&matricula)
    .bind(fechas::hoy())
    .fetch_optional(&estado.db)
    .await?
    .map(Json)
    .ok_or_else(|| ErrorApi::NoEncontrado("No hay ningún vehículo con esa matrícula".to_string()))
}

/// De momento, todos los días del rango; el filtro por cupo lo añade T029.
async fn dias_disponibles() -> Json<Vec<NaiveDate>> {
    Json(dias_ofrecidos(fechas::hoy()))
}

#[derive(Deserialize)]
struct PeticionCita {
    matricula: Option<String>,
    fecha: Option<String>,
    motivo: Option<String>,
}

async fn crear_cita(
    State(estado): State<EstadoApp>,
    Json(peticion): Json<PeticionCita>,
) -> Resultado<Response> {
    // 1. Matrícula, fecha y motivo, en el orden de la tabla de validación.
    let matricula = peticion
        .matricula
        .as_deref()
        .and_then(normalizar_matricula)
        .ok_or_else(|| error_campo("matricula", "La matrícula no es válida"))?;
    let fecha = peticion
        .fecha
        .as_deref()
        .and_then(|texto| texto.parse::<NaiveDate>().ok())
        .filter(|fecha| dias_ofrecidos(fechas::hoy()).contains(fecha))
        .ok_or_else(|| error_campo("fecha", "Elige uno de los días disponibles"))?;
    let motivo = peticion.motivo.as_deref().map(str::trim).unwrap_or_default();
    if motivo.is_empty() {
        return Err(error_campo("motivo", "El motivo es obligatorio"));
    }

    // 2. Antes de los bloqueos: si el vehículo es nuevo, sus datos se validan aquí, para que
    //    todos los 422 lleguen antes que cualquier 409.
    if buscar_vehiculo(&estado.db, &matricula).await?.is_none() {
        // Hueco de T024 (US2): validar marca, modelo, nombre, teléfono y correo.
        return Err(ErrorApi::Validacion {
            mensaje: "Todavía no se pueden dar de alta vehículos nuevos".to_string(),
            detalle: None,
        });
    }

    // 3. Los dos bloqueos, siempre en este orden para que no haya interbloqueos (R6 del plan
    //    maestro, tabla de claves; research D3). Primero la fecha, con la forma de una clave de
    //    64 bits; después la matrícula, con la de dos claves de 32 bits. PostgreSQL no solapa
    //    los dos espacios de claves.
    let mut tx = estado.db.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(($1::date - DATE '2000-01-01')::bigint)")
        .bind(fecha)
        .execute(&mut *tx)
        .await?;
    sqlx::query("SELECT pg_advisory_xact_lock(1, hashtext($1))")
        .bind(&matricula)
        .execute(&mut *tx)
        .await?;

    // 4. Con los bloqueos tomados, se vuelve a buscar: si otra persona lo ha dado de alta entre
    //    el paso 2 y el 3, se trata como vehículo conocido (D4) y se ignoran `vehiculo` y
    //    `cliente`. Con un vehículo conocido nunca se modifica ni el vehículo ni su propietario.
    let existente = buscar_vehiculo(&mut *tx, &matricula).await?;

    // 5. Una sola cita pendiente por matrícula (FR-010), antes que el cupo (research D3).
    if let Some(id) = existente {
        let pendiente: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM cita WHERE vehiculo_id = $1 AND fecha >= $2)",
        )
        .bind(id)
        .bind(fechas::hoy())
        .fetch_one(&mut *tx)
        .await?;
        if pendiente {
            return Err(ErrorApi::Conflicto(CITA_PENDIENTE.to_string()));
        }
    }

    // 6. Hueco de T029 (US3): cupo del día, 409.

    // 7. Hueco de T024 (US2): alta del cliente y del vehículo nuevos. Inalcanzable hasta
    //    entonces, porque el paso 2 ya ha respondido con una matrícula nueva.
    let Some(vehiculo_id) = existente else {
        return Err(ErrorApi::Validacion {
            mensaje: "Todavía no se pueden dar de alta vehículos nuevos".to_string(),
            detalle: None,
        });
    };

    // 8. La cita.
    sqlx::query("INSERT INTO cita (vehiculo_id, fecha, motivo) VALUES ($1, $2, $3)")
        .bind(vehiculo_id)
        .bind(fecha)
        .bind(motivo)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    let creada = CitaCreada {
        fecha,
        motivo: motivo.to_string(),
        matricula,
    };
    Ok((StatusCode::CREATED, Json(creada)).into_response())
}

#[derive(Serialize)]
struct CitaCreada {
    fecha: NaiveDate,
    motivo: String,
    matricula: String,
}

#[derive(Deserialize)]
struct FiltroCitas {
    fecha: Option<String>,
}

#[derive(sqlx::FromRow)]
struct FilaCita {
    id: i64,
    matricula: String,
    marca: String,
    modelo: String,
    motivo: String,
    nombre: String,
    telefono: String,
    orden_id: Option<i64>,
}

#[derive(Serialize)]
struct ClienteDeCita {
    nombre: String,
    telefono: String,
}

/// Una cita de la lista de recepción: nunca lleva el correo (FR-009).
#[derive(Serialize)]
struct CitaDelDia {
    id: i64,
    matricula: String,
    marca: String,
    modelo: String,
    motivo: String,
    cliente: ClienteDeCita,
    orden_id: Option<i64>,
}

/// Citas de un día (hoy si no se indica), en el orden en que se pidieron. Recepción y jefe.
async fn citas_del_dia(
    usuario: Usuario,
    State(estado): State<EstadoApp>,
    Query(filtro): Query<FiltroCitas>,
) -> Resultado<Json<Vec<CitaDelDia>>> {
    usuario.exigir_rol(&[Rol::Recepcion])?;
    let fecha = match filtro.fecha.as_deref() {
        None => fechas::hoy(),
        Some(texto) => texto
            .parse::<NaiveDate>()
            .map_err(|_| ErrorApi::PeticionMala("La fecha no es válida".to_string()))?,
    };
    let filas: Vec<FilaCita> = sqlx::query_as(
        "SELECT c.id, v.matricula, v.marca, v.modelo, c.motivo, cl.nombre, cl.telefono,
                o.id AS orden_id
           FROM cita c
           JOIN vehiculo v ON v.id = c.vehiculo_id
           JOIN cliente cl ON cl.id = v.cliente_id
           LEFT JOIN orden o ON o.cita_id = c.id
          WHERE c.fecha = $1
          ORDER BY c.creada_en, c.id",
    )
    .bind(fecha)
    .fetch_all(&estado.db)
    .await?;
    let citas = filas
        .into_iter()
        .map(|f| CitaDelDia {
            id: f.id,
            matricula: f.matricula,
            marca: f.marca,
            modelo: f.modelo,
            motivo: f.motivo,
            cliente: ClienteDeCita {
                nombre: f.nombre,
                telefono: f.telefono,
            },
            orden_id: f.orden_id,
        })
        .collect();
    Ok(Json(citas))
}
