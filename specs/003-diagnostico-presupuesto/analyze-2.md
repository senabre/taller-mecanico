# Analyze 2 · spec-003 · Registrar diagnóstico y preparar presupuesto

**Fecha**: 2026-11-12 · **Rama**: `spec-003` (sobre `fc16c4f`, con las correcciones de analyze-1 sin
commit todavía: T036 y T037) · **Artefactos**: [spec.md](spec.md), [plan.md](plan.md),
[research.md](research.md) y [tasks.md](tasks.md) (T001–T037) · **Contexto**: el mismo que
[analyze-1](analyze-1.md) · **Constitución**: 1.2.1.

Análisis de solo lectura. Comprueba las correcciones de [analyze-1](analyze-1.md) y repite los mismos
controles sobre los documentos cambiados. Las decisiones se registran en [hallazgos.md](hallazgos.md).

## Verificación de analyze-1

| ID | Comprobación | Resultado |
|----|--------------|-----------|
| C1 | `Presupuesto.svelte` ya no llama a `confirm()`: «Emitir presupuesto» abre una pregunta en la página con «Sí, emitir» y «Volver». `FormularioLinea.svelte` lleva `novalidate` y las unidades son un campo de texto (`inputmode="numeric"`). En el navegador: rojo 0 de 3 y verde 4 de 4, también a 390 px ([T036](evidencias/T036/informe.md)). `npm run check` sin errores ni avisos y `npm test` 21 de 21. | Verificado |
| G1 | La fila IV del Constitution Check del plan ya no dice que todos los SC tienen paso: SC-001 queda como desviación aceptada, enlazando a hallazgos. | Verificado (justificado) |
| I1 | Research D8 tiene la nota de `respondido_en` (spec-004 T014), y las dependencias del plan, la excepción acordada. | Verificado |
| I2 | T027 nombra los tres argumentos de `presupuestos_de_orden`, como T029, D8 y el código. | Verificado |
| I3 | Las entidades de la spec dicen «referencia (p. ej. «PF-123»)». | Verificado |
| G2 | Justificado en hallazgos; no hay cambio que comprobar. | Verificado (justificado) |

## Specification Analysis Report (hallazgos nuevos)

| ID | Category | Severity | Location(s) | Summary | Recommendation |
|----|----------|----------|-------------|---------|----------------|
| N1 | Inconsistency | LOW | tasks.md:106 (T024) · `FormularioLinea.svelte` | T024 describe las horas con `step="0.25"` y las unidades con `step="1"`. Las horas ya eran un campo de texto (`inputmode="decimal"`), y desde T036 las unidades también. | Nota en T024: horas y unidades son campos de texto y las valida el servidor (T036). |
| N2 | Quickstart | LOW | quickstart común §3.6 | El paso 6 dice «Emitir →» sin la confirmación que ahora sale en la página. Quien siga la guía ve una pregunta que el paso no menciona. | «Emitir y confirmar con «Sí, emitir» →». Es la sección de spec-003 dentro de un documento común: la revisa también la Persona A. |

### Metrics

- **Total Requirements**: 14 (10 FR + 4 SC) · **Total Tasks**: 37
- **Coverage**: 100 % (SC-001 con desviación aceptada, G1)
- **Ambiguity Count**: 0 · **Duplication Count**: 0 · **Critical Issues Count**: 0

## Next Actions

- Corregir N1 y N2 (documentación, sin código).
- Lanzar `/speckit-converge`.
