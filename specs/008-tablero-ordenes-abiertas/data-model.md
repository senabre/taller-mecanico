# Data Model: spec-008 · Tablero de órdenes abiertas

El modelo de datos es **único** y está en
[000-base-comun/data-model.md](../000-base-comun/data-model.md). Esta spec **no crea tablas,
columnas ni migraciones** y **no escribe nada**: el tablero es solo de consulta (Assumptions de la
spec, principio III). Aquí solo se explica qué lee la historia y qué reglas comprueba.

## Qué hace spec-008 con cada tabla

| Tabla | Lee | Escribe | Nunca |
|---|---|---|---|
| `orden` | número, estado, mecánico, `fecha_entrega_prometida`, solo de las no facturadas (`resumenes(…, true)`, research D3) | — | cambiar estados: el tablero no tiene botones ni arrastre |
| `vehiculo` | matrícula | — | — |
| `usuario` | `id`, `nombre`, `rol`, `activo` de los mecánicos y de quien tenga órdenes abiertas, para la carga (research D2), y el nombre del mecánico de cada tarjeta | — | — |
| `cliente` | — | — | leerla: las tarjetas solo muestran la matrícula (principio V) |

## Reglas que comprueba esta spec

| Regla | Requisito | Dónde se comprueba |
|---|---|---|
| Abierta = estado distinto de `facturada` | FR-001, SC-004 | `resumenes(…, solo_abiertas = true)` (D3); la carga se cuenta sobre esa misma lista (D2) |
| Las 6 columnas, en el orden de `Estado`, aunque estén vacías | FR-001 | `tablero.rs` (D1) |
| Tarjetas por `fecha_entrega_prometida` y `numero` ascendentes | FR-001 (Q4) | `tablero.rs` (D1) |
| `retrasada` = no facturada y `fecha_entrega_prometida < hoy` (Europe/Madrid) | FR-004 | la regla común del data-model, calculada en `OrdenResumen` (`ordenes.rs`); no hay otra definición |
| Carga: activos siempre; inactivos solo con órdenes abiertas; la suma = número de tarjetas | FR-003, SC-003 (Q1) | la carga se cuenta en Rust sobre la misma lista de órdenes que las tarjetas (D1, D2), sin transacción (V3) |
| Solo el jefe | FR-006 (Q2) | `exigir_rol(&[Rol::Jefe])` (D4) |

## Estados

Las 6 columnas son los estados abiertos de la máquina de estados común, en este orden:
`recibida`, `en_diagnostico`, `en_reparacion`, `en_espera_piezas`, `pendiente_conformidad`,
`terminada` (`ESTADOS_ABIERTOS` en `formato.ts`). `facturada` es el único estado que no se
muestra. Cada movimiento de la máquina de estados publica un evento que hace que el tablero se
vuelva a pedir (research D5).

## Valores calculados

Nada de lo que devuelve `GET /api/tablero` se guarda: `retrasada` sale de `estado` y de
`fecha_entrega_prometida`, y `ordenes_abiertas` es un `count` de las órdenes no facturadas.
