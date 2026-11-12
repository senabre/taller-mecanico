# Converge 1 · spec-005 · Imputar tiempo y piezas durante la reparación

**Fecha**: 2026-11-12 · **Rama**: `spec-005` (sobre `2c207bd`, con T035 y T036) · **Fuente de
intención**: [spec.md](spec.md), [plan.md](plan.md) y [tasks.md](tasks.md) (T001–T036), con la
constitución 1.2.1 · **Código revisado**: `backend/src/rutas/imputaciones.rs`, `backend/src/lineas.rs`,
`backend/migrations/0003_spec005_imputaciones.sql`, `backend/tests/spec005_imputaciones.rs`,
`frontend/src/routes/ordenes/[id]/Imputaciones.svelte` y
`frontend/src/lib/componentes/FormularioLinea.svelte`.

**Resultado**: ✅ **Converged**: la implementación cumple la spec, el plan y las tareas. `tasks.md` no
cambia.

## Convergence Findings

Ninguno.

## Summary metrics

- **Requisitos y escenarios comprobados**: 8 FR, 4 SC, 10 escenarios de aceptación (US1-1…3, US2-1…2,
  US3-1…2, US4-1…2) y 6 casos límite.
- **Decisiones del plan comprobadas**: D1–D8 y las «Reglas clave»: una transacción con la orden
  bloqueada; el stock descontado con `WHERE stock >= n` y el mensaje en singular con 1; la regla del
  10 % con `Decimal` (exactamente el 110 % no bloquea); el evento después del `commit`; terminar con
  al menos una imputación; lo presupuestado es la suma de los aceptados.
- **Principios de la constitución comprobados**: los 5. Tras T035 no queda ningún `confirm()` ni
  ninguna validación del navegador en las pantallas de spec-005, y el motivo para no poder terminar se
  ve en la pantalla.
- **Pruebas**: `cargo test --test spec005_imputaciones`, 12 de 12; `spec006_pedidos` (que usa
  terminar), 23 de 23; `npm run check`, 0 errores y 0 avisos; `npm test`, 21 de 21.
- **Por tipo de hallazgo**: 0 `missing`, 0 `partial`, 0 `contradicts` y 0 `unrequested`.

## Next Actions

Spec-005 cerrada: commit en `spec-005`.
