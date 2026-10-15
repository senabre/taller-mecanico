# Implementation Plan: spec-006 · Pedir al proveedor una pieza sin stock

**Branch**: `006-pedido-pieza-proveedor` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md) | **Responsable**: Persona B

**Input**: Feature specification from `/specs/006-pedido-pieza-proveedor/spec.md`

**Note**: Lo común está en el **plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md).

## Summary

El rol almacén da de alta referencias y proveedores y registra pedidos vinculados a una orden.
Registrar un pedido pone la orden `en_espera_piezas`. Al recibirlo se suma el stock y, si era el
último pedido pendiente, la orden vuelve sola a `en_reparacion`.

## Technical Context

Igual que el plan maestro.

- **Tablas**: `pieza`, `proveedor`, `pedido`.
- **Endpoints**: `POST /api/piezas`, `GET|POST /api/proveedores`, `GET|POST /api/pedidos`,
  `POST /api/pedidos/{id}/recibir`
  ([contrato](../000-base-comun/contracts/api.md#spec-006--almacén-y-pedidos)). `GET /api/piezas`
  se crea aquí, en `almacen.rs`, y lo reutiliza spec-003.
- **Pantallas**:
  - `/almacen`: lista de piezas con stock y formulario de alta.
  - `/pedidos`: pendientes, alta y «Marcar recibido».
  - Sección «Pedidos» de `/ordenes/[id]`.

## Constitution Check

| Principio | Comprobación | Estado |
|---|---|---|
| I | Proveedor con solo el nombre; recepción del pedido completo | ✅ |
| II | Precios en € | ✅ |
| III | Sin catálogos de fabricantes, sin enviar el pedido al proveedor, sin editar ni borrar referencias | ✅ |
| IV | quickstart §6 | ✅ |
| V | — | ✅ |

## Project Structure

```text
backend/src/rutas/almacen.rs          # piezas, proveedores, pedidos
backend/tests/spec006_pedidos.rs      # referencia única, pedido → espera, recibir → stock + vuelve, 2 pedidos, doble recepción
frontend/src/routes/almacen/+page.svelte
frontend/src/routes/pedidos/+page.svelte
frontend/src/routes/ordenes/[id]/Pedidos.svelte
```

**Reglas clave** (en transacciones):
- **Crear pedido**:
  1. Comprobar que la orden está `en_reparacion` o `en_espera_piezas`; si no, 409.
  2. Insertar el pedido.
  3. Si la orden estaba `en_reparacion`, ejecutar `cambiar_estado(en_reparacion → en_espera_piezas)`.
- **Recibir**:
  1. `UPDATE pedido SET estado = 'recibido', recibido_en = now() WHERE id = $1 AND estado =
     'pendiente'`. Si no se actualiza ninguna fila, 409.
  2. `stock += cantidad`.
  3. Si la orden no tiene más pedidos pendientes, ejecutar
     `cambiar_estado(en_espera_piezas → en_reparacion)`.

**Dependencias**: base común. Para los pedidos hacen falta órdenes `en_reparacion` (datos demo).
**Es la primera spec de la Persona B**, porque spec-003 y spec-005 necesitan
referencias de piezas.
