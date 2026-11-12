# Analyze 1 · spec-007 · Facturar la orden y avisar al cliente

**Fecha**: 2026-11-12 · **Rama**: `spec-007` (commit `076f626`: `main` más el cierre de spec-003,
spec-005 y spec-006) · **Artefactos**: [spec.md](spec.md), [plan.md](plan.md),
[research.md](research.md) (D1–D8) y [tasks.md](tasks.md) (T001–T032) · **Contexto**:
[000-base-comun](../000-base-comun/) (contrato, data-model, quickstart §7 y research R4, R5, R10,
R12–R14) y el código de la rama · **Constitución**: 1.2.1.

Análisis de solo lectura: no se ha corregido nada. Las decisiones se registran en
[hallazgos.md](hallazgos.md).

**Sobre el momento**: spec-007 está implementada y fusionada en `main` desde el 05/11/2026 (las 32
tareas marcadas). Como en las otras specs de la Persona B, el análisis compara también los documentos
con el código.

## Specification Analysis Report

| ID | Category | Severity | Location(s) | Summary | Recommendation |
|----|----------|----------|-------------|---------|----------------|
| C1 | Constitution (II) | CRITICAL | tasks.md:77 (T016) · research.md:77 (D7) · `frontend/src/routes/ordenes/[id]/Factura.svelte:37` · `frontend/src/routes/historial/+page.svelte:41–49` · V2 de los [hallazgos de spec-008](../008-tablero-ordenes-abiertas/hallazgos.md) | **Dos textos los pone el navegador.** (a) «Facturar» confirma con `confirm()`: los botones salen en el idioma del navegador. (b) El buscador de `/historial` tiene la matrícula `required` sin `novalidate`: buscar sin escribir nada saca el mensaje del navegador («Completa este campo», en su idioma). Es el mismo caso que C1 de spec-003, spec-005 y spec-006. | (a) Confirmación en la página («¿Emitir la factura? Ya no se podrá modificar.», con «Sí, facturar» y «Volver»). (b) `novalidate` y, con la matrícula vacía, el mensaje de la aplicación «Escribe la matrícula» (sin llamar al servidor, que con una matrícula vacía no llegaría a la ruta del historial). |
| G1 | Coverage (IV) | MEDIUM | spec.md:138 (SC-001), 142–143 (SC-004) · plan.md:60 (fila IV) · quickstart común §7 · T028 · [matriz](../matriz-trazabilidad.md) (007 SC-001 y SC-004) | **SC-001 (facturar en menos de 30 s) y SC-004 (historial en menos de 10 s) no tienen paso de cronometraje**, y el plan dice «Cada SC tiene su paso». En la matriz quedaron «Verificado con desviación» (0,24 s y 0,22 s con un navegador automático). Mismo caso que G1 de las otras tres specs. | La misma decisión que en spec-003, spec-005 y spec-006. |
| I1 | Inconsistency | LOW | research.md:80–82 (D7) · tasks.md:78 (T017) · `backend/src/rutas/facturas.rs:183` | **`factura_de_orden` no la usa spec-002.** D7 dice que spec-002 la llama para llenar `factura_id`, pero T017 anota que `GET /api/ordenes/{id}` usa su propia subconsulta, equivalente, y no se tocó `ordenes.rs`. La función solo la usan las pruebas (T012). Lo detectó también la revisión T031 de spec-002. | Nota en D7 con lo que se hizo. No se quita la función: la pide T014 y la prueba T012. |
| I2 | Inconsistency | LOW | plan.md:75 (Project Structure) | **El plan dice «research.md # decisiones D1–D7»**, pero hay D8 (el logo, añadido el 29/10). | «D1–D8». |

### Coverage Summary Table

| Requirement Key | Has Task? | Task IDs | Notes |
|-----------------|-----------|----------|-------|
| FR-001 | Sí | T011, T013, T016 | |
| FR-002 | Sí | T009, T013 | |
| FR-003 | Sí | T009, T015, T007 | Confirmación de «Facturar» del navegador (C1) |
| FR-004 | Sí | T010, T013 | |
| FR-005 | Sí | T009, T013 | |
| FR-006 | Sí | T018, T019, T020 | |
| FR-007 | Sí | T021, T022, T023, T024 | Buscador con validación del navegador (C1) |
| FR-008 | Sí | T011, T013 | |
| FR-009 | Sí | T029–T032 | |
| SC-001 | Parcial | T028 | Sin paso de cronometraje (G1) |
| SC-002 | Sí | T009 | |
| SC-003 | Sí | T011, T016 | |
| SC-004 | Parcial | T028 | Sin paso de cronometraje (G1) |

### Constitution Alignment Issues

- **Principio II** (C1): la confirmación de «Facturar» y el mensaje de la matrícula vacía los pone el
  navegador.
- **Principio IV** (G1): SC-001 y SC-004 sin paso de cronometraje.

### Unmapped Tasks

Ninguna.

### Metrics

- **Total Requirements**: 13 (9 FR + 4 SC)
- **Total Tasks**: 32 (todas hechas)
- **Coverage**: 100 % de los requisitos con al menos una tarea (SC-001 y SC-004, solo parcial)
- **Ambiguity Count**: 0
- **Duplication Count**: 0
- **Critical Issues Count**: 1 (C1)

## Next Actions

- **C1 es CRITICAL**: corregirlo como en spec-003 (decisión de la Persona B del 12/11/2026, que
  incluía el `confirm()` de «Facturar»). Con esto queda cerrado el V2 de spec-008.
- G1: misma decisión que en las otras specs. I1 e I2: documentación.
- Después, `analyze-2.md` y `/speckit-converge`.
