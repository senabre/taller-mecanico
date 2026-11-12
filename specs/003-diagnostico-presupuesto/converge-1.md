# Converge 1 · spec-003 · Registrar diagnóstico y preparar presupuesto

**Fecha**: 2026-11-12 · **Rama**: `spec-003` (sobre `fc16c4f`, con las correcciones de analyze-1 y
analyze-2) · **Fuente de intención**: [spec.md](spec.md), [plan.md](plan.md) y [tasks.md](tasks.md)
(T001–T037), con la constitución 1.2.1 · **Código revisado**: `backend/src/rutas/presupuestos.rs`,
`backend/src/lineas.rs`, `backend/migrations/0002_spec003_lineas.sql`,
`backend/tests/spec003_presupuestos.rs`, `frontend/src/routes/ordenes/[id]/Diagnostico.svelte`,
`Presupuesto.svelte`, `frontend/src/lib/componentes/FormularioLinea.svelte`, `TablaLineas.svelte` y
`Totales.svelte`.

**Resultado**: `tasks_appended`: 2 tareas en la fase 8 de [tasks.md](tasks.md) (T038 y T039).

## Convergence Findings

| ID | Gap Type | Severity | Source | Evidence | Remaining Work |
|----|----------|----------|--------|----------|----------------|
| F1 | contradicts | CRITICAL | Constitution II · US1 (diagnóstico vacío) | `Diagnostico.svelte`: el `textarea` lleva `required` y el formulario no tiene `novalidate`. Con el diagnóstico vacío, el navegador bloquea el envío con su propio mensaje («Completa este campo», en el idioma del navegador) y no llega el 422 «Escribe el diagnóstico». Es el mismo caso que C1 de analyze-1, en otro formulario. | T038: `novalidate` en el formulario. |
| F2 | partial | MEDIUM | US1/AC3 · FR-001a | `Diagnostico.svelte`: con un presupuesto ya emitido sigue enseñando el cuadro y «Guardar diagnóstico»; el servidor responde bien (409), pero el cuadro conserva el texto rechazado hasta recargar y puede hacer creer que se guardó. Es el V2 que la Persona A registró en los [hallazgos de spec-004](../004-aceptar-presupuesto-enlace/hallazgos.md) para la Persona B. | T039: con un presupuesto emitido, el diagnóstico sin formulario y el motivo a la vista. |

## Summary metrics

- **Requisitos y escenarios comprobados**: 10 FR, 4 SC y 11 escenarios de aceptación (US1-1…3,
  US2-1…5, US3-1…2), más los 6 casos límite.
- **Decisiones del plan comprobadas**: D1–D8 y las «Reglas clave» del plan (transacciones, bloqueos,
  permisos y evento tras el `commit`). Todas se cumplen en el backend, y sus 20 pruebas pasan.
- **Principios de la constitución comprobados**: los 5.
- **Por tipo de hallazgo**: 0 `missing`, 1 `partial`, 1 `contradicts` y 0 `unrequested`.
- **Por gravedad**: 1 CRITICAL, 1 MEDIUM.

## Next Actions

Hacer T038 y T039 (`/speckit-implement`), con la comprobación en el navegador primero en rojo, y
volver a lanzar converge (`converge-2.md`).
