# Implementation Plan: spec-005 · Imputar tiempo y piezas durante la reparación

**Branch**: `005-imputar-tiempo-piezas` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md) | **Responsable**: Persona B

**Input**: Feature specification from `/specs/005-imputar-tiempo-piezas/spec.md`

**Note**: Lo común está en el **plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md).

## Summary

El mecánico asignado imputa horas y piezas en órdenes `en_reparacion`. Cada pieza descuenta el
stock en la misma transacción. Tras cada imputación se compara el coste acumulado con el total
presupuestado; si supera el 110 %, la orden pasa a `pendiente_conformidad`. Las ampliaciones
reutilizan los endpoints de spec-003 y la respuesta de spec-004. El mecánico marca la orden como
`terminada`.

## Technical Context

Igual que el plan maestro. Configuración usada: `TALLER_PRECIO_HORA`.

- **Tablas**: `imputacion`, `pieza` (stock) y `presupuesto` (lectura de los aceptados).
- **Endpoints**: `POST /api/ordenes/{id}/imputaciones`, `POST /api/ordenes/{id}/terminar`
  ([contrato](../000-base-comun/contracts/api.md#spec-005--imputaciones)).
- **Pantallas**: sección «Imputaciones» de `/ordenes/[id]`, con el indicador «Acumulado X € de
  Y €» y el botón «Terminar reparación».

## Constitution Check

| Principio | Comprobación | Estado |
|---|---|---|
| I | Umbral calculado al vuelo, sin guardar acumulados; sin anular imputaciones | ✅ |
| II | Importes en € y horas con coma (1,25 h) | ✅ |
| III | Sin corrección de imputaciones ni control horario | ✅ |
| IV | quickstart §5 | ✅ |
| V | — | ✅ |

## Project Structure

```text
backend/src/rutas/imputaciones.rs
backend/tests/spec005_imputaciones.rs   # descuento de stock, stock insuficiente, 110 % exacto (no bloquea) / >110 % (bloquea), ampliación reanuda, terminar, solo en en_reparacion
frontend/src/routes/ordenes/[id]/Imputaciones.svelte
```

**Reglas clave** (`POST imputaciones`, en una transacción):

1. Comprobar que el usuario es el mecánico asignado (o jefe) y que la orden está `en_reparacion`
   (`SELECT … FOR UPDATE`). Si no, 409.
2. Si es pieza: `UPDATE pieza SET stock = stock - $n WHERE id = $1 AND stock >= $n`. Si no se
   actualiza ninguna fila, 422 «Solo hay N unidades en stock».
3. Si es mano de obra, comprobar que la cantidad es múltiplo de 0,25 y mayor que 0.
4. Insertar la imputación con `dinero::importe_linea`.
5. Si `coste_acumulado > total_presupuestado × 1,10`, ejecutar
   `cambiar_estado(en_reparacion → pendiente_conformidad)`.

- La comparación se hace con `Decimal`, nunca con floats, y sobre importes sin IVA.

**Dependencias**: spec-004 (orden `en_reparacion`, presupuestos aceptados) y spec-006 (stock).
Para empezar sin ellas: crear con `seed/demo.sql` una orden `en_reparacion` con un presupuesto
aceptado.
