# Data Model: spec-001 · Pedir cita en línea

El modelo de datos es **único** y está en
[000-base-comun/data-model.md](../000-base-comun/data-model.md): las tablas `cliente`, `vehiculo` y
`cita` ya existen en la migración `0001_esquema_inicial.sql`. Esta spec **no crea tablas ni
migraciones**. Aquí solo se explica cómo las usa la historia y qué reglas, actualizadas en el
documento común tras el clarify, comprueba spec-001.

## Qué hace spec-001 con cada tabla

| Tabla | Lee | Escribe | Nunca |
|---|---|---|---|
| `cliente` | `nombre`, `telefono` para la lista de recepción | `INSERT` solo si la matrícula es nueva | `UPDATE` (FR-003, Q5); devolver `email` en ninguna respuesta |
| `vehiculo` | `marca`, `modelo` por matrícula | `INSERT` solo si la matrícula es nueva | `UPDATE` de marca, modelo o `cliente_id` (Q5) |
| `cita` | citas de una fecha; si el vehículo tiene una pendiente | `INSERT` | modificar o borrar (fuera de alcance) |

## Reglas que comprueba esta spec

Las reglas están en el documento común; la tabla dice dónde se comprueba cada una.

| Regla (data-model común) | Requisito | Dónde se comprueba |
|---|---|---|
| `vehiculo.matricula`: de 2 a 10 letras o cifras tras normalizar | FR-002 (Q1) | `matricula::normalizar_matricula` (cambio compartido, research D1) |
| `cliente.telefono` y `cliente.email` con formato válido | FR-004a (Q2) | `citas.rs`, en el servidor (research D2) |
| Como mucho una cita pendiente (fecha ≥ hoy) por vehículo | FR-010 (Q3) | transacción del `POST`, con bloqueo por matrícula (research D3) |
| Nº de citas de la fecha < `TALLER_CUPO_DIARIO` | FR-007 | transacción del `POST`, con bloqueo por fecha (R6 común) |
| Un vehículo existente conserva su propietario | FR-003 (Q5) | el `POST` ignora `vehiculo` y `cliente` (research D4) |

## Estados

Una cita no tiene campo de estado. «Pendiente» se calcula: `fecha >= hoy` en Europe/Madrid (R12).
Cuando su fecha pasa, deja de contar para FR-010 sin que nadie la toque.
