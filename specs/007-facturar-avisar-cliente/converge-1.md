# Converge 1 · spec-007 · Facturar la orden y avisar al cliente

**Fecha**: 2026-11-12 · **Rama**: `spec-007` (sobre `076f626`, con T033 y T034) · **Fuente de
intención**: [spec.md](spec.md), [plan.md](plan.md) y [tasks.md](tasks.md) (T001–T034), con la
constitución 1.2.1 · **Código revisado**: `backend/src/rutas/facturas.rs`, `backend/src/formato.rs`,
`backend/tests/spec007_facturas.rs`, `frontend/src/routes/ordenes/[id]/Factura.svelte`,
`frontend/src/routes/facturas/[id]/+page.svelte`, `frontend/src/routes/historial/+page.svelte` y
`frontend/src/lib/componentes/TablaLineas.svelte`.

**Resultado**: ✅ **Converged**: la implementación cumple la spec, el plan y las tareas. `tasks.md` no
cambia.

## Convergence Findings

Ninguno.

## Summary metrics

- **Requisitos y escenarios comprobados**: 9 FR, 4 SC, 8 escenarios de aceptación (US1-1…5, US2-1,
  US3-1…2) y 5 casos límite.
- **Decisiones del plan comprobadas**: D1–D8 y las «Reglas clave»: facturar con la orden bloqueada
  (`FOR UPDATE OF o`), solo `terminada`, con imputaciones, número del contador dentro de la
  transacción, líneas en el orden en que se imputaron y `terminada → facturada`; el correo y
  `aviso_enviado` después del `commit`; ver la factura solo recepción y jefe; el historial con la
  matrícula normalizada, de la más reciente a la más antigua; el logo opcional por
  `GET /api/taller/logo`.
- **Principios de la constitución comprobados**: los 5. Tras T033 no queda ningún `confirm()` en la
  aplicación ni ninguna validación del navegador en las pantallas de spec-007.
- **Pruebas**: `cargo test --test spec007_facturas`, 11 de 11; `npm run check`, 0 errores y 0 avisos.
- **Por tipo de hallazgo**: 0 `missing`, 0 `partial`, 0 `contradicts` y 0 `unrequested`.

## Next Actions

Spec-007 cerrada: commit en `spec-007`.
