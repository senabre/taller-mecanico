# Converge 2 · spec-003 · Registrar diagnóstico y preparar presupuesto

**Fecha**: 2026-11-12 · **Rama**: `spec-003` (con T036–T039 hechos) · **Fuente de intención**:
[spec.md](spec.md), [plan.md](plan.md) y [tasks.md](tasks.md) (T001–T039), con la constitución
1.2.1 · **Código revisado**: el mismo que en [converge-1](converge-1.md).

**Resultado**: ✅ **Converged**: la implementación cumple la spec, el plan y las tareas. `tasks.md` no
cambia.

## Comprobación de converge-1

| ID | Comprobación | Resultado |
|----|--------------|-----------|
| F1 | El formulario de `Diagnostico.svelte` lleva `novalidate`; con el diagnóstico vacío llega el 422 «Escribe el diagnóstico» ([T039](evidencias/T039/informe.md), paso a). | Resuelto (T038) |
| F2 | Con un presupuesto emitido, el diagnóstico se ve sin formulario y con el motivo; sin presupuesto emitido sigue editable ([T039](evidencias/T039/informe.md), pasos b y c). | Resuelto (T039) |

## Convergence Findings

Ninguno.

## Summary metrics

- **Requisitos y escenarios comprobados**: 10 FR, 4 SC, 11 escenarios de aceptación y 6 casos límite.
- **Decisiones del plan comprobadas**: D1–D8 y las «Reglas clave».
- **Principios de la constitución comprobados**: los 5. Ningún formulario de spec-003 deja que el
  navegador valide ni pide confirmación con `confirm()`: los `required` que quedan están en
  formularios con `novalidate` y solo sirven a la accesibilidad.
- **Pruebas**: `cargo test --test spec003_presupuestos`, 20 de 20; `npm run check`, 0 errores y 0
  avisos; `npm test`, 21 de 21.
- **Por tipo de hallazgo**: 0 `missing`, 0 `partial`, 0 `contradicts` y 0 `unrequested`.

## Next Actions

Spec-003 cerrada: commit en `spec-003` y, al final de las cuatro specs, llevarla a `main` y a
`entrega`.
