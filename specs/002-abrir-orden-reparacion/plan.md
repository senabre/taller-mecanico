# Implementation Plan: spec-002 · Recibir el vehículo y abrir la orden de reparación

**Branch**: `002-abrir-orden-reparacion` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md) | **Responsable**: Persona A

**Input**: Feature specification from `/specs/002-abrir-orden-reparacion/spec.md`

**Note**: Lo común está en el **plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md).

## Summary

Desde la lista de citas del día, recepción abre la orden. En una sola transacción se toma el
siguiente número de `contador('orden')` (research R5), se valida que el km no sea menor que el de
la última orden del vehículo y se crea la orden en estado `recibida`. Esta spec también crea la
página `/ordenes/[id]` con su sección «Datos», que las demás specs amplían.

## Technical Context

Igual que el plan maestro.

- **Tablas**: `orden`, `contador` ([data-model](../000-base-comun/data-model.md)).
- **Endpoints**: `POST /api/ordenes`, `GET /api/ordenes`, `GET /api/ordenes/{id}`,
  `GET /api/mecanicos` ([contrato](../000-base-comun/contracts/api.md#spec-002--orden-de-reparación)).
- **Pantallas**: botón «Abrir orden» en `/citas`; `/ordenes` y la sección Datos de `/ordenes/[id]`.

## Constitution Check

| Principio | Comprobación | Estado |
|---|---|---|
| I | Daños en texto libre, sin fotos; correlativo con una tabla contador | ✅ |
| II | Km con separador de miles (85.000) y fechas dd/mm/aaaa | ✅ |
| III | La fecha de entrega prometida se justifica en la spec (FR-006, la necesita spec-008) | ✅ |
| IV | quickstart §2 | ✅ |
| V | No se piden datos nuevos del cliente | ✅ |

## Project Structure

```text
backend/src/rutas/ordenes.rs          # crear, listar, detalle (el detalle lo amplían 003–007)
backend/src/contador.rs               # (base común) siguiente(tx, "orden")
backend/tests/spec002_ordenes.rs      # correlativo sin huecos (incl. fallo y concurrencia), km, 1 orden/cita
frontend/src/routes/ordenes/+page.svelte
frontend/src/routes/ordenes/[id]/+page.svelte    # contenedor de secciones (compartido)
frontend/src/routes/ordenes/[id]/Datos.svelte
```

**Reglas clave**:
- Km anterior: `SELECT km FROM orden WHERE vehiculo_id = $1 ORDER BY abierta_en DESC LIMIT 1`. Si
  el nuevo es menor, se devuelve 422 con `detalle.km_anterior`. El km del mensaje se escribe con
  `formato::numero` de la base común («85.000 km»).
- Las validaciones se hacen **antes** de pedir el número, y todo en la misma transacción, así un
  fallo no consume número.
- `mecanico_id` debe ser un usuario activo con rol `mecanico` (422 si no).
- Abrir la orden no pasa por `cambiar_estado` (es un `INSERT`), pero **publica su evento** para
  que el tablero (spec-008) la vea: después del `commit`,
  `CambioEstado { id, estado: Estado::Recibida }.publicar(&estado_app.eventos)`.
- `GET /api/ordenes/{id}` devuelve el objeto `Orden` completo del contrato. Los campos de otras
  specs (presupuestos, imputaciones, pedidos, factura) se devuelven vacíos hasta que existan.

**Dependencias**: base común y spec-001 (citas). Datos demo para probar sin 001.
