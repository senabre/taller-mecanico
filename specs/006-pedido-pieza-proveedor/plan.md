# Implementation Plan: spec-006 · Pedir al proveedor una pieza sin stock

**Branch**: `006-pedido-pieza-proveedor` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md) | **Responsable**: Persona B

**Input**: Feature specification from `/specs/006-pedido-pieza-proveedor/spec.md`

**Note**: Lo común está en el **plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md).
Decisiones propias de esta spec: [research.md](research.md). El modelo de datos, el contrato y la
guía de validación son los compartidos (ver Phase 1).

## Summary

El rol almacén da de alta referencias de piezas y registra pedidos a proveedores precargados,
vinculados a una orden por su número. Registrar un pedido para una orden `en_reparacion` la pone
`en_espera_piezas`. Al recibir un pedido se suma su cantidad al stock y, si era el último
pendiente de la orden, esta vuelve sola a `en_reparacion`. No existe ninguna forma de sacar a mano
una orden de la espera. Tampoco se permiten dos pedidos pendientes de la misma referencia para la
misma orden.

## Technical Context

**Language/Version · Dependencies · Storage · Testing · Platform**: igual que el plan maestro
(Rust + Axum + SQLx, SvelteKit, PostgreSQL 16, `#[sqlx::test]`).

**Project Type**: aplicación web; módulo `almacen.rs` dentro del backend común.

**Performance Goals**: registrar un pedido en menos de 1 minuto (SC-001); pantallas en menos de 1 s.

**Constraints**: todo en español, importes en € y fechas dd/mm/aaaa. Todas las transiciones de
estado pasan por `estados::cambiar_estado` (R14).

**Scale/Scope**: 3 tablas (`pieza`, `proveedor`, `pedido`), 6 endpoints, 2 pantallas y 1 sección
de la ficha de la orden.

- **Endpoints** ([contrato](../000-base-comun/contracts/api.md#spec-006--almacén-y-pedidos)):
  `GET|POST /api/piezas` (el `GET` se crea aquí y lo reutilizan spec-003 y spec-005),
  `GET /api/proveedores`, `GET|POST /api/pedidos`, `POST /api/pedidos/{id}/recibir`. Los pedidos de
  una orden salen en `GET /api/ordenes/{id}` (campo `pedidos`).
- **Pantallas**:
  - `/almacen`: lista de referencias con su stock y formulario de alta.
  - `/pedidos`: formulario de pedido (proveedor, referencia, cantidad y nº de orden), lista de
    pendientes de todas las órdenes y botón «Marcar recibido».
  - Sección «Pedidos» de `/ordenes/[id]`, solo de lectura: proveedor, referencia, cantidad, fecha
    y estado.

No queda ningún NEEDS CLARIFICATION. Las decisiones abiertas se cierran en [research.md](research.md).

## Constitution Check

*GATE: antes de Phase 0 y de nuevo después de Phase 1.*

| Principio | Comprobación | Estado |
|---|---|---|
| I. Simplicidad | Las reglas las garantiza la base de datos: índice parcial (D2), `FOR UPDATE` (D3), `CHECK` de stock. Sin capas: `almacen.rs` hace sus consultas SQLx. La orden se indica por número, sin abrir endpoints de otra persona (D5). | ✅ |
| II. Español y euro | Mensajes de error en español; precio en € con `formato.ts`; fechas dd/mm/aaaa. | ✅ |
| III. Cero alcance fantasma | Se **quita** `POST /api/proveedores` (FR-010, D1). No hay catálogos, envío al proveedor, edición o baja de referencias, recepción parcial, cancelación ni avisos al mecánico (D8). | ✅ |
| IV. Verificable sin código | Cada SC tiene su paso en [quickstart §6](../000-base-comun/quickstart.md). | ✅ |
| V. Datos | Solo datos de piezas y pedidos; ningún dato personal. | ✅ |

**Re-check post-diseño**: los cambios al modelo y al contrato compartidos solo quitan alcance
(proveedores) o concretan reglas de la spec (stock opcional, índice parcial, `orden_numero`). ✅

## Project Structure

### Documentation (this feature)

```text
specs/006-pedido-pieza-proveedor/
├── spec.md
├── plan.md        # este fichero
└── research.md    # decisiones D1–D8
# data-model, contrato y quickstart: los compartidos de specs/000-base-comun/ (actualizados)
```

### Source Code (repository root)

```text
backend/src/rutas/almacen.rs              # piezas, proveedores, pedidos
backend/tests/spec006_pedidos.rs          # pruebas de reglas (ver abajo)
backend/seed/demo.sql                     # + proveedores demo (compartido)
frontend/src/routes/almacen/+page.svelte
frontend/src/routes/pedidos/+page.svelte
frontend/src/routes/ordenes/[id]/Pedidos.svelte
frontend/src/lib/tipos.ts                 # tipos Pieza, Proveedor, Pedido (compartido)
```

**Structure Decision**: la del plan maestro. Esta spec solo añade su módulo de rutas, su fichero de
pruebas, dos pantallas y un componente de la ficha de la orden.

### Reglas clave (cada una en una transacción)

- **Alta de referencia** (FR-001, FR-002): validar (D6) e insertar. Si se viola el `UNIQUE` de
  `referencia` → 409 «Ya existe la referencia PF-123».
- **Crear pedido** (FR-003…FR-005, FR-004a):
  1. Validar `cantidad` (entero > 0), proveedor y pieza existentes → si no, 422.
  2. `SELECT id, estado FROM orden WHERE numero = $1 FOR UPDATE` → 422 si no existe; 409 con el
     motivo si no está `en_reparacion` ni `en_espera_piezas`.
  3. Insertar el pedido (`estado = 'pendiente'`, `pedido_en = now()`). Si choca con el índice
     parcial → 409 «Ya hay un pedido pendiente de PF-123 para la orden 42».
  4. Si la orden estaba `en_reparacion`, `cambiar_estado(&[EnReparacion], EnEsperaPiezas)`.
- **Recibir** (FR-007, FR-008):
  1. Leer `orden_id` del pedido (404 si no existe) y bloquear la orden `FOR UPDATE`.
  2. `UPDATE pedido SET estado = 'recibido', recibido_en = now() WHERE id = $1 AND estado =
     'pendiente'`. Si no se actualiza ninguna fila → 409 «Este pedido ya está recibido».
  3. `UPDATE pieza SET stock = stock + cantidad`.
  4. Si la orden ya no tiene pedidos `pendiente`,
     `cambiar_estado(&[EnEsperaPiezas], EnReparacion)`.
- **Sin cambio manual** (FR-008a, D4): no hay endpoint. Las pruebas comprueban que «terminar» e
  imputar devuelven 409 con la orden en espera.

### Pruebas (`spec006_pedidos.rs`)

1. Alta con stock 4 y alta sin stock (→ 0); referencia repetida → 409; stock negativo, precio
   negativo o descripción vacía → 422.
2. Pedido para orden `en_reparacion` → pendiente con fecha de hoy y orden `en_espera_piezas`.
3. Segundo pedido de otra referencia con la orden en espera → se admite y sigue en espera.
4. Misma referencia, misma orden, con el primero pendiente → 409; tras recibir el primero → se admite.
5. Orden `en_diagnostico` → 409 con el motivo; nº de orden inexistente → 422; cantidad 0 → 422.
6. Recibir → stock + cantidad, pedido recibido con fecha y orden `en_reparacion`.
7. Dos pedidos y se recibe solo uno → la orden sigue en espera.
8. Recibir dos veces → 409 y el stock no cambia.
9. Orden en espera: `terminar` e imputar → 409.
10. Acceso: un usuario `mecanico` en `POST /api/pedidos` → 403.

**Dependencias**: base común. Para los pedidos hacen falta órdenes `en_reparacion` (datos demo).
**Es la primera spec de la Persona B**, porque spec-003 y spec-005 necesitan referencias de piezas.

**Cambios en documentos compartidos** (los revisa también la Persona A antes de fusionar):
[data-model.md](../000-base-comun/data-model.md) (`pieza.stock` por defecto 0, proveedor
precargado, índice parcial de `pedido`, `pedido_en` por defecto `now()`),
[contracts/api.md](../000-base-comun/contracts/api.md) (sin `POST /api/proveedores`, `stock`
opcional, `orden_numero`, nuevos errores) y [quickstart.md §6](../000-base-comun/quickstart.md).

## Complexity Tracking

No hay violaciones que justificar.
