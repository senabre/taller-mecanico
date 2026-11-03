# Data Model: spec-002 · Recibir el vehículo y abrir la orden de reparación

El modelo de datos es **único** y está en
[000-base-comun/data-model.md](../000-base-comun/data-model.md): las tablas `orden` y `contador`
ya existen en la migración `0001_esquema_inicial.sql`. Esta spec **no crea tablas ni
migraciones**. Aquí solo se explica cómo las usa la historia y qué reglas, actualizadas en el
documento común tras el clarify, comprueba spec-002.

## Qué hace spec-002 con cada tabla

| Tabla | Lee | Escribe | Nunca |
|---|---|---|---|
| `cita` | la cita de la orden (fecha, vehículo, motivo); la bloquea con `FOR UPDATE` al abrir la orden | — | cambiarla o borrarla (fuera de alcance) |
| `orden` | la visita anterior del vehículo; la orden de la cita, si ya existe; la ficha y las listas | `INSERT` en estado `recibida` | cambiar su estado (lo hacen 003–007 con `estados::cambiar_estado`); editar km, daños, mecánico o entrega (fuera de alcance) |
| `contador` | — | `contador::siguiente(tx, "orden")` dentro de la transacción (R5) | pedir número antes de todas las comprobaciones |
| `usuario` | si el mecánico existe, tiene rol `mecanico` y está activo | — | — |
| `vehiculo` | matrícula, marca y modelo para la ficha | — | — |
| `factura` | `factura_id` de la ficha | — | — |

`presupuesto`, `imputacion` y `pedido` no los lee spec-002 directamente: la ficha llama a las
funciones de spec-003, 005 y 006 ([research](research.md) D6).

## Reglas que comprueba esta spec

Las reglas están en el documento común (fila `orden`) y los textos de error en la tabla de
validación del [contrato común](../000-base-comun/contracts/api.md#spec-002--orden-de-reparación).
La tabla dice dónde se comprueba cada una.

| Regla (data-model común) | Requisito | Dónde se comprueba |
|---|---|---|
| La orden se abre el mismo día de la cita (fecha de la cita = hoy en Europe/Madrid) | FR-001 (Q2) | `POST`, con `fechas::hoy()` (research D1) |
| `cita_id` único: como mucho una orden por cita | FR-008 (Q5) | `POST`, con la cita bloqueada (D2); el `UNIQUE` de la tabla es la última barrera |
| `numero` sale de `contador('orden')`, sin huecos ni repetidos | FR-002 | `POST`, solo después de todas las comprobaciones (D2, R5) |
| `km` ≥ 0 y ≥ km de la visita anterior (la orden más reciente del vehículo para otra cita) | FR-003, FR-004 | `POST` (D3) |
| `danos` obligatorio («Sin daños visibles» si no hay) | FR-003 (Q4) | `POST`, tras `trim` (D4) |
| `mecanico_id`: usuario con rol `mecanico` y activo | FR-005 (Q3) | `POST`, con la misma condición que `GET /api/mecanicos` (D4) |
| `fecha_entrega_prometida` ≥ fecha de apertura (hoy) | FR-006 | `POST` (D4) |
| `vehiculo_id` = el de la cita | FR-005 | `POST`: se toma de la cita, no de la petición |
| Estado inicial `recibida` | FR-007 | `POST` |

## Estados

Spec-002 solo crea órdenes en `recibida`, con un `INSERT` que no pasa por la tabla de transiciones
de `estados.rs`. Después del `commit` publica `CambioEstado { id, Recibida }` para el tablero
(research D5; hallazgo I4 de la base común). Las transiciones posteriores son de 003–007.

## Valores calculados que muestra la ficha

`retrasada`, `total_presupuestado` y `coste_acumulado` se calculan como dice el data-model común,
no se guardan. `retrasada` la calcula `ordenes.rs`; los dos importes vienen de
`imputaciones::resumen_imputaciones` (spec-005).
