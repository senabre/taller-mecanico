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

/// Teléfono (contrato, tabla de validación): tras quitar espacios y guiones, un «+» opcional
/// solo al principio y de 9 a 15 cifras.
fn telefono_valido(texto: &str) -> bool {
    let limpio: String = texto.chars().filter(|c| *c != ' ' && *c != '-').collect();
    let cifras = limpio.strip_prefix('+').unwrap_or(&limpio);
    (9..=15).contains(&cifras.len()) && cifras.chars().all(|c| c.is_ascii_digit())
}

/// Correo (contrato, tabla de validación): una sola «@», texto delante y, detrás, un dominio con
/// un «.» que tiene texto a ambos lados. Solo se comprueban caracteres (research D2).
fn correo_valido(texto: &str) -> bool {
    let Some((usuario, dominio)) = texto.split_once('@') else {
        return false;
    };
    !usuario.is_empty()
        && !dominio.contains('@')
        && dominio.contains('.')
        && !dominio.starts_with('.')
        && !dominio.ends_with('.')
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
const DIA_COMPLETO: &str = "Ese día ya está completo";

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

/// Los días del rango que aún tienen plazas (FR-007), con una sola consulta.
async fn dias_disponibles(State(estado): State<EstadoApp>) -> Resultado<Json<Vec<NaiveDate>>> {
    let dias = dias_ofrecidos(fechas::hoy());
    let (Some(&primero), Some(&ultimo)) = (dias.first(), dias.last()) else {
        return Ok(Json(dias));
    };
    let ocupacion: Vec<(NaiveDate, i64)> = sqlx::query_as(
        "SELECT fecha, count(*) FROM cita WHERE fecha BETWEEN $1 AND $2 GROUP BY fecha",
    )
    .bind(primero)
    .bind(ultimo)
    .fetch_all(&estado.db)
    .await?;
    let cupo = i64::from(estado.config.cupo_diario);
    let completos: Vec<NaiveDate> = ocupacion
        .into_iter()
        .filter(|(_, citas)| *citas >= cupo)
        .map(|(fecha, _)| fecha)
        .collect();
    Ok(Json(dias.into_iter().filter(|dia| !completos.contains(dia)).collect()))
}

#[derive(Deserialize)]
struct PeticionCita {
    matricula: Option<String>,
    fecha: Option<String>,
    motivo: Option<String>,
    /// Solo cuentan si la matrícula es nueva; con un vehículo conocido se ignoran (D4).
    vehiculo: Option<DatosVehiculo>,
    cliente: Option<DatosCliente>,
}

#[derive(Deserialize)]
struct DatosVehiculo {
    marca: Option<String>,
    modelo: Option<String>,
}

#[derive(Deserialize)]
struct DatosCliente {
    nombre: Option<String>,
    telefono: Option<String>,
    email: Option<String>,
}

/// Datos de un vehículo nuevo y de su propietario, ya validados y sin espacios al principio ni
/// al final; por lo demás, tal como se escribieron (research D2).
struct AltaNueva<'a> {
    marca: &'a str,
    modelo: &'a str,
    nombre: &'a str,
    telefono: &'a str,
    email: &'a str,
}

/// Valida los datos del vehículo nuevo en el orden de la tabla de validación. Si falta el objeto
/// `vehiculo` o `cliente`, sus campos cuentan como vacíos.
fn validar_alta(peticion: &PeticionCita) -> Resultado<AltaNueva<'_>> {
    fn texto(valor: Option<&String>) -> &str {
        valor.map(|v| v.trim()).unwrap_or_default()
    }
    fn obligatorio<'a>(valor: &'a str, campo: &str, mensaje: &str) -> Resultado<&'a str> {
        if valor.is_empty() {
            Err(error_campo(campo, mensaje))
        } else {
            Ok(valor)
        }
    }
    let vehiculo = peticion.vehiculo.as_ref();
    let cliente = peticion.cliente.as_ref();
    let alta = AltaNueva {
        marca: obligatorio(
            texto(vehiculo.and_then(|v| v.marca.as_ref())),
            "vehiculo.marca",
            "La marca es obligatoria",
        )?,
        modelo: obligatorio(
            texto(vehiculo.and_then(|v| v.modelo.as_ref())),
            "vehiculo.modelo",
            "El modelo es obligatorio",
        )?,
        nombre: obligatorio(
            texto(cliente.and_then(|c| c.nombre.as_ref())),
            "cliente.nombre",
            "El nombre es obligatorio",
        )?,
        telefono: texto(cliente.and_then(|c| c.telefono.as_ref())),
        email: texto(cliente.and_then(|c| c.email.as_ref())),
    };
    if !telefono_valido(alta.telefono) {
        return Err(error_campo("cliente.telefono", "El teléfono no es válido"));
    }
    if !correo_valido(alta.email) {
        return Err(error_campo("cliente.email", "El correo no es válido"));
    }
    Ok(alta)
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
    let alta = match buscar_vehiculo(&estado.db, &matricula).await? {
        Some(_) => None,
        None => Some(validar_alta(&peticion)?),
    };

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

    // 6. Cupo del día (FR-007). El bloqueo por fecha garantiza que nadie más cuenta ni inserta
    //    citas de ese día hasta el `commit`.
    let citas_del_dia: i64 = sqlx::query_scalar("SELECT count(*) FROM cita WHERE fecha = $1")
        .bind(fecha)
        .fetch_one(&mut *tx)
        .await?;
    if citas_del_dia >= i64::from(estado.config.cupo_diario) {
        return Err(ErrorApi::Conflicto(DIA_COMPLETO.to_string()));
    }

    // 7. Alta del cliente y del vehículo nuevos, con los datos validados en el paso 2.
    let vehiculo_id = match (existente, alta) {
        (Some(id), _) => id,
        (None, Some(alta)) => {
            sqlx::query_scalar(
                "WITH nuevo AS (
                     INSERT INTO cliente (nombre, telefono, email) VALUES ($4, $5, $6) RETURNING id
                 )
                 INSERT INTO vehiculo (matricula, marca, modelo, cliente_id)
                 SELECT $1, $2, $3, id FROM nuevo RETURNING id",
            )
            .bind(&matricula)
            .bind(alta.marca)
            .bind(alta.modelo)
            .bind(alta.nombre)
            .bind(alta.telefono)
            .bind(alta.email)
            .fetch_one(&mut *tx)
            .await?
        }
        // Existía en el paso 2 y ya no: no hay ninguna operación que borre vehículos.
        (None, None) => {
            return Err(ErrorApi::Interno(format!("el vehículo {matricula} ha desaparecido")));
        }
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

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn telefonos_validos() {
        for texto in ["612345678", "+34 612 345 678", "612-345-678", "612345678901234"] {
            assert!(telefono_valido(texto), "{texto:?}");
        }
    }

    #[test]
    fn telefonos_no_validos() {
        for texto in ["", "12345", "61234567", "6123456789012345", "6123+45678", "+", "61234567a"] {
            assert!(!telefono_valido(texto), "{texto:?}");
        }
    }

    #[test]
    fn correos_validos() {
        for texto in ["ana@correo.es", "a@b.c", "ana.perez@mi.correo.es"] {
            assert!(correo_valido(texto), "{texto:?}");
        }
    }

    #[test]
    fn correos_no_validos() {
        for texto in ["", "ana@correo", "a@@b.es", "@b.es", "a@.es", "a@b.", "a@b@c.es", "ana.es"] {
            assert!(!correo_valido(texto), "{texto:?}");
        }
    }
}
