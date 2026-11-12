# Analyze 1 · spec-005 · Imputar tiempo y piezas durante la reparación

**Fecha**: 2026-11-12 · **Rama**: `spec-005` (commit `2c207bd`: `main` más el cierre de spec-003) ·
**Artefactos**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md) (D1–D8) y
[tasks.md](tasks.md) (T001–T034) · **Contexto**: [000-base-comun](../000-base-comun/) (contrato,
data-model, quickstart §5 y research R4, R12–R14) y el código de la rama · **Constitución**: 1.2.1.

Análisis de solo lectura: no se ha corregido nada. Las decisiones se registran en
[hallazgos.md](hallazgos.md).

**Sobre el momento**: spec-005 está implementada y fusionada en `main` desde el 05/11/2026 (las 34
tareas marcadas). Como en spec-003, el análisis compara también los documentos con el código y con
lo que vino después (spec-004, spec-008).

**Sobre el código de referencia**: `spec-005` contiene el cierre de spec-003 (`2c207bd`), porque el
formulario de líneas (`FormularioLinea.svelte`) es compartido y ya se corrigió allí (T036 de spec-003:
`novalidate` y unidades como texto). Por eso ese formulario no aparece aquí como hallazgo.

## Specification Analysis Report

| ID | Category | Severity | Location(s) | Summary | Recommendation |
|----|----------|----------|-------------|---------|----------------|
| C1 | Constitution (II) | CRITICAL | tasks.md:150 (T029) · research.md:92 (D8) · `frontend/src/routes/ordenes/[id]/Imputaciones.svelte:40` · V2 de los [hallazgos de spec-008](../008-tablero-ordenes-abiertas/hallazgos.md) | **«Terminar reparación» confirma con `confirm()`.** La pregunta está en español, pero los botones salen en el idioma del navegador. Es el mismo caso que C1 de spec-003, que ya se corrigió allí con una confirmación en la página. | Confirmación en la página, como en spec-003 T036 y spec-004 D5: «¿Marcar la reparación como terminada? Ya no se podrán imputar más horas ni piezas.», con «Sí, terminar» y «Volver». |
| U1 | Underspecification | MEDIUM | spec.md:107–109 (US4-2), 142–143 (FR-008) · tasks.md:150 (T029) · quickstart común §5.1 · [regresión T030 de spec-008](../008-tablero-ordenes-abiertas/evidencias/T030/regresion/verificacion.md) (§5.1) | **Sin imputaciones, la pantalla no dice por qué no se puede terminar.** FR-008 pide impedirlo «e indicar el motivo», y el §5.1 de la guía espera ver «Primero hay que imputar el trabajo realizado». Pero T029 solo enseña el botón si hay alguna imputación, así que en pantalla no hay nada que pulsar ni ningún motivo; la regresión T030 tuvo que comprobar el mensaje por la API («botón en pantalla=0; API=422»), lo que choca con el principio IV. | Sin imputaciones (orden en reparación), enseñar el motivo «Primero hay que imputar el trabajo realizado.» donde iría el botón, y ajustar el §5.1 de la guía a lo que se ve. |
| G1 | Coverage (IV) | MEDIUM | spec.md:161 (SC-003) · plan.md:57 (fila IV del Constitution Check) · quickstart común §5 · T034 · [matriz](../matriz-trazabilidad.md) (005 SC-003) | **SC-003 (una imputación en menos de 30 s) no tiene paso de cronometraje** en la guía ni tarea que lo mida, y el plan dice «Cada SC tiene su paso». En la matriz quedó «Verificado con desviación» (0,24 s con un navegador automático). Es el mismo caso que G1 de spec-003. | Como en spec-003: aceptar la medición automática como desviación y corregir la fila IV del plan, o añadir un paso de cronometraje. |
| G2 | Coverage | LOW | tasks.md:162 (T034) · spec.md:86–88 (US3-1) | **T034 dice que la aceptación de la ampliación se simuló por SQL**, porque la página del enlace era de spec-004 y aún no existía. Ya existe: en la regresión T030 de spec-008 (§5.5) la ampliación se aceptó desde el enlace del correo, y la matriz ya la da por verificada. | Nota en T034 que enlace la evidencia de la regresión (s5-06 y s5-07). |
| I1 | Terminology | LOW | spec.md:147 (Key Entities) · spec.md:31, 127–129 | **«Horas» frente a «mano de obra».** La entidad «Imputación» tiene «tipo (horas o pieza)»; el plan, el contrato, el código y la pantalla usan «mano de obra» (`mano_obra`), igual que spec-003. | «tipo (mano de obra, en horas, o pieza)». |

### Coverage Summary Table

| Requirement Key | Has Task? | Task IDs | Notes |
|-----------------|-----------|----------|-------|
| FR-001 | Sí | T011, T013, T014, T016, T017 | |
| FR-002 | Sí | T011, T012, T016 | |
| FR-003 | Sí | T007, T011, T015, T017 | |
| FR-004 | Sí | T004, T011 | |
| FR-005 | Sí | T018, T019, T020, T021 | |
| FR-006 | Sí | T022, T024 | Endpoints de spec-003 |
| FR-007 | Sí | T015, T022, T024 | Aceptación simulada en la prueba; real en la regresión (G2) |
| FR-008 | Sí | T025, T026, T028, T029 | Sin motivo en pantalla (U1) |
| SC-001 | Sí | T011, T034 | |
| SC-002 | Sí | T018, T019 | |
| SC-003 | Parcial | T034 | Sin paso de cronometraje (G1) |
| SC-004 | Sí | T011, T015 | |

### Constitution Alignment Issues

- **Principio II** (C1): la confirmación de «Terminar reparación» puede salir en otro idioma.
- **Principio IV** (U1, G1): el §5.1 no se puede comprobar en la aplicación sin usar la API, y SC-003
  no tiene un paso para comprobarlo una persona.

### Unmapped Tasks

Ninguna. T001–T010 son de preparación (base común, migración, módulos compartidos con spec-003) y
T031–T034 de cierre.

### Metrics

- **Total Requirements**: 12 (8 FR + 4 SC)
- **Total Tasks**: 34 (todas hechas)
- **Coverage**: 100 % de los requisitos con al menos una tarea (SC-003, solo parcial)
- **Ambiguity Count**: 0
- **Duplication Count**: 0
- **Critical Issues Count**: 1 (C1)

## Next Actions

- **C1 es CRITICAL**: corregirlo (decisión de la Persona B del 12/11/2026 para C1 de spec-003, que
  incluía los `confirm()` de spec-005 y spec-007).
- U1: enseñar el motivo en pantalla. G1: misma decisión que en spec-003.
- G2 e I1: documentación.
- Después, `analyze-2.md` y `/speckit-converge`.
