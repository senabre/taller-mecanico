# Analyze 1 · spec-003 · Registrar diagnóstico y preparar presupuesto

**Fecha**: 2026-11-12 · **Rama**: `spec-003` (commit `fc16c4f`, al día con `main`) · **Artefactos**:
[spec.md](spec.md), [plan.md](plan.md), [research.md](research.md) (D1–D8) y [tasks.md](tasks.md)
(T001–T035) · **Contexto**: [000-base-comun](../000-base-comun/) (spec, contrato, data-model,
quickstart §3 y research R4, R9, R10, R12–R14) y el código de la rama · **Constitución**: 1.2.1.

Análisis de solo lectura: no se ha corregido nada. Las decisiones se registran en
[hallazgos.md](hallazgos.md).

**Sobre el momento**: spec-003 está implementada y fusionada en `main` desde el 05/11/2026 (las 35
tareas marcadas). Este análisis se hace al final, así que, además de la coherencia entre spec, plan y
tareas, compara los documentos con lo que acabó haciendo el código y con lo que cambiaron después
spec-004 y spec-008.

**Alcance de las comprobaciones**: todo sale de leer los documentos y el código. No se ha ejecutado
nada; las pruebas de spec-003 (20 de 20) se ejecutaron en la revisión T031 de spec-004 del mismo
día, sobre el mismo código.

## Specification Analysis Report

| ID | Category | Severity | Location(s) | Summary | Recommendation |
|----|----------|----------|-------------|---------|----------------|
| C1 | Constitution (II) | CRITICAL | tasks.md:130 (T030) · `frontend/src/routes/ordenes/[id]/Presupuesto.svelte:122` · `frontend/src/lib/componentes/FormularioLinea.svelte:145–147` · quickstart común §3.4 · [regresión T030 de spec-008](../008-tablero-ordenes-abiertas/evidencias/T030/regresion/verificacion.md) (§3.4) · V2 de [hallazgos de spec-008](../008-tablero-ordenes-abiertas/hallazgos.md) | **Dos textos los pone el navegador, no la aplicación.** (a) «Emitir presupuesto» pide confirmación con `confirm()`: la pregunta está en español, pero los botones salen en el idioma del navegador («OK»/«Cancel» en uno en inglés). (b) Las unidades de pieza usan `type="number"` con `step="1"`: con 2,5 el navegador bloquea el envío con su propio mensaje, y en la regresión T030 salió en inglés («Please enter a valid value. The two nearest valid values are 2 and 3.»). Las horas no tienen este problema: van en un campo de texto y el mensaje es el del servidor. El principio II pide que todo lo visible esté en español; spec-004 descartó `confirm()` por eso mismo (su research D5). | Confirmación dentro de la página, como en spec-004, y que las unidades no las valide el navegador (`novalidate` o campo de texto como las horas), para que llegue el 422 del servidor «Las unidades de pieza deben ser un número entero». Si se deja para después, justificarlo en hallazgos y cerrar el V2 de spec-008 con la misma decisión. |
| G1 | Coverage (IV) | MEDIUM | spec.md:152 (SC-001) · plan.md:60 (fila IV del Constitution Check) · quickstart común §3 · tasks.md:142 (T035) · [matriz](../matriz-trazabilidad.md) (003 SC-001) | **SC-001 (5 líneas en menos de 5 minutos) no tiene un paso en la guía ni una tarea que lo mida.** El plan dice «Cada SC tiene su paso en quickstart §3», pero el §3 no cronometra nada, y T035 solo pide «anotar que se cumplen SC-001…SC-004». En la matriz quedó «Verificado con desviación»: se midió con un navegador automático (1,02 s), no con una persona. Spec-004 sí tiene un paso de cronometraje (§4.2) y su tarea (T028). | Añadir al §3 un paso de cronometraje (preparar 5 líneas en una orden, con el reloj desde que se abre la ficha) y una tarea para hacerlo con una persona, o dejar la medición automática como desviación aceptada en hallazgos y corregir la frase del plan. |
| I1 | Inconsistency | LOW | research.md:99–102 (D8) · tasks.md:49 (T006), 129 (T029) · plan.md:129–130 | **`PresupuestoResumen` y la frontera con spec-004 están desfasados.** D8, T006 y T029 lo describen sin `respondido_en`, que añadió spec-004 (T014, `4b6e62c`) y que ya están en el contrato, en `tipos.ts` y en `presupuestos.rs`. El plan dice «A no modifica `presupuestos.rs`», pero spec-004 lo modificó con el acuerdo de la Persona B (I1 de spec-004, 05/11/2026): `respondido_en` y `caducado()` en `pub(crate)`. | Añadir a D8 una nota con `respondido_en` (spec-004 T014) y matizar la frase del plan («salvo los dos cambios acordados en spec-004 T014»). Las tareas ya hechas no se reescriben: basta con la nota. |
| I2 | Inconsistency | LOW | tasks.md:124 (T027) · tasks.md:129 (T029) · research.md:99 (D8) · `backend/src/rutas/presupuestos.rs:200–204` | **Firma de `presupuestos_de_orden`.** T027 la llama con dos argumentos (`conexion, orden_id`); T029, D8 y el código llevan tres (`iva_porcentaje`, para el total de un borrador). La prueba usa tres. | Corregir T027 para que nombre los tres argumentos. |
| I3 | Terminology | LOW | spec.md:145 (Key Entities) | **«Código» de pieza.** La entidad «Referencia de pieza» dice «código, descripción, precio de venta y stock»; el resto de la spec, el plan, spec-006 y la pantalla lo llaman «referencia» (p. ej. «PF-123»). | Cambiar «código» por «referencia». |
| G2 | Coverage | LOW | spec.md:77–79 (US2-5), 98 (US3-2) · quickstart común §3.5 | **Dos escenarios sin paso exacto en la guía.** US2-5 quita la mano de obra y espera 50,00 / 10,50 / 60,50; el §3.5 añade y quita una línea «Prueba» y vuelve a 110,00 / 23,10 / 133,10. US3-2 (no se emite sin líneas) no tiene paso, porque el botón «Emitir» solo sale con alguna línea. Los dos los cubren las pruebas (`quitar_una_linea_recalcula_los_totales`, `no_se_emite_sin_lineas_ni_por_otro_mecanico`). | Opcional: dejar constancia en hallazgos de que el §3.5 comprueba la misma regla con otra línea, y de que US3-2 se ve en pantalla como «sin líneas no hay botón Emitir». |

### Coverage Summary Table

| Requirement Key | Has Task? | Task IDs | Notes |
|-----------------|-----------|----------|-------|
| FR-001 | Sí | T010, T012, T013 | |
| FR-001a | Sí | T010, T011, T012, T028 | T028 bloquea también la orden al emitir |
| FR-002 | Sí | T002, T015, T016, T022, T024 | |
| FR-002a | Sí | T017, T023, T024 | |
| FR-003 | Sí | T015, T022, T008 | |
| FR-004 | Sí | T015, T022 | |
| FR-005 | Sí | T016, T022, T024 | |
| FR-006 | Sí | T009, T015, T032 | |
| FR-007 | Sí | T025, T026, T028, T030 | |
| FR-008 | Sí | T014, T020 | |
| SC-001 | Parcial | T035 | Sin paso de cronometraje (G1) |
| SC-002 | Sí | T015, T018 | |
| SC-003 | Sí | T025, T027, T030 | |
| SC-004 | Sí | T016, T022 | |

### Constitution Alignment Issues

- **Principio II** (C1): la confirmación de «Emitir presupuesto» y la validación de unidades del
  navegador pueden salir en otro idioma.
- **Principio IV** (G1): SC-001 no tiene un paso en la guía para comprobarlo una persona.

### Unmapped Tasks

Ninguna. T001–T009 son de preparación (base común, migración, tipos y componentes) y T032–T035 de
cierre (revisión de idioma y alcance, pruebas y recorrido).

### Metrics

- **Total Requirements**: 14 (10 FR + 4 SC)
- **Total Tasks**: 35 (todas hechas)
- **Coverage**: 100 % de los requisitos con al menos una tarea (SC-001, solo parcial)
- **Ambiguity Count**: 0
- **Duplication Count**: 0
- **Critical Issues Count**: 1 (C1)

## Next Actions

- **C1 es CRITICAL**: hay que corregirlo o justificar por escrito por qué se deja, antes de dar
  spec-003 por cerrada. La corrección es pequeña y solo toca pantallas de spec-003 (de la Persona B).
- G1: decidir entre cronometrar SC-001 con una persona o aceptar la medición automática como
  desviación.
- I1, I2, I3 y G2: correcciones de documentación, sin código.
- Después, repetir el análisis (`analyze-2.md`) y lanzar `/speckit-converge`.
