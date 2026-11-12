# Converge 1 · spec-006 · Pedir al proveedor una pieza sin stock

**Fecha**: 2026-11-12 · **Rama**: `spec-006` (sobre `7bf2648`, con T033 y T034) · **Fuente de
intención**: [spec.md](spec.md), [plan.md](plan.md) y [tasks.md](tasks.md) (T001–T034), con la
constitución 1.2.1 · **Código revisado**: `backend/src/rutas/almacen.rs`,
`backend/tests/spec006_pedidos.rs`, `frontend/src/routes/almacen/+page.svelte`,
`frontend/src/routes/pedidos/+page.svelte` y `frontend/src/routes/ordenes/[id]/Pedidos.svelte`.

**Resultado**: ✅ **Converged**: la implementación cumple la spec, el plan y las tareas. `tasks.md` no
cambia.

## Convergence Findings

Ninguno.

## Summary metrics

- **Requisitos y escenarios comprobados**: 13 FR, 6 SC, 12 escenarios de aceptación (US1-1…3,
  US2-1…5, US3-1…3, US4-1…2) y 8 casos límite.
- **Decisiones del plan comprobadas**: D1–D8 y las «Reglas clave»: sin alta de proveedores; el índice
  parcial de pedidos pendientes con su 409; la orden bloqueada `FOR UPDATE` al crear y al recibir; la
  orden indicada por su número; recibir una sola vez (`WHERE estado = 'pendiente'`); la vuelta a «en
  reparación» solo con el último pedido; el evento después del `commit` y ningún aviso más; ningún
  endpoint ni botón para cambiar el estado de una orden a mano; la sección de la ficha, solo de
  lectura.
- **Principios de la constitución comprobados**: los 5. Tras T033, ningún formulario de spec-006 deja
  que valide el navegador.
- **Pruebas**: `cargo test --test spec006_pedidos`, 23 de 23; `npm run check`, 0 errores y 0 avisos.
- **Por tipo de hallazgo**: 0 `missing`, 0 `partial`, 0 `contradicts` y 0 `unrequested`.

## Next Actions

Spec-006 cerrada: commit en `spec-006`.
