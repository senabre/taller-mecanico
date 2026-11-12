# Analyze 1 · spec-006 · Pedir al proveedor una pieza sin stock

**Fecha**: 2026-11-12 · **Rama**: `spec-006` (commit `7bf2648`: `main` más el cierre de spec-003 y
spec-005) · **Artefactos**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md)
(D1–D8) y [tasks.md](tasks.md) (T001–T032) · **Contexto**: [000-base-comun](../000-base-comun/)
(contrato, data-model, quickstart §6 y research R11–R14) y el código de la rama · **Constitución**:
1.2.1.

Análisis de solo lectura: no se ha corregido nada. Las decisiones se registran en
[hallazgos.md](hallazgos.md).

**Sobre el momento**: spec-006 está implementada y fusionada en `main` desde el 22/10/2026 (las 32
tareas marcadas). Como en spec-003 y spec-005, el análisis compara también los documentos con el
código.

## Specification Analysis Report

| ID | Category | Severity | Location(s) | Summary | Recommendation |
|----|----------|----------|-------------|---------|----------------|
| C1 | Constitution (II) | CRITICAL | `frontend/src/routes/almacen/+page.svelte:67–94` · `frontend/src/routes/pedidos/+page.svelte:125–192` · tasks.md:76 (T013), 100 (T019) · spec.md:131–137 (casos límite) | **Los dos formularios dejan que valide el navegador.** `/almacen` y `/pedidos` tienen campos `required` y números con `type="number"`, `min` y `step`, sin `novalidate`. Los casos límite de la spec (cantidad 0, negativa o vacía; stock negativo; pedido sin proveedor, sin referencia o sin orden) no llegan al servidor: los para el navegador con su propio mensaje y en su idioma («Completa este campo», «El valor debe ser superior o igual a 1»…). El servidor ya tiene todos los mensajes en español («La cantidad debe ser un número entero mayor que 0», «El stock debe ser un número entero de 0 o más», «Indica el número de la orden», «Elige un proveedor de la lista»…). Es el mismo caso que C1 de spec-003. | `novalidate` en los dos formularios y los números como campos de texto (`inputmode="numeric"`), enviando el número si es un entero y, si no, el texto tal cual para que el servidor responda con su 422. |
| G1 | Coverage (IV) | MEDIUM | spec.md:189 (SC-001) · plan.md:57 (fila IV) · quickstart común §6 · T032 · [matriz](../matriz-trazabilidad.md) (006 SC-001) | **SC-001 (un pedido en menos de 1 minuto) no tiene paso de cronometraje**, y el plan dice «Cada SC tiene su paso». En la matriz quedó «Verificado con desviación» (0,56 s con un navegador automático). Mismo caso que G1 de spec-003 y spec-005. | La misma decisión que en spec-003 y spec-005. |
| G2 | Coverage (IV) | LOW | tasks.md:156 (T032) · spec.md:101–103 (US3-1) · SC-003 | **T032 dice que la vuelta de la orden a «en reparación» se comprobó en la base de datos**, no en la aplicación (principio IV). En la regresión T030 de spec-008 (§6.9) ya se vio en la ficha de la orden. | Nota en T032 con las capturas de la regresión (s6-05 y s6-06). |
| I1 | Terminology | LOW | spec.md:39, 54–55, 148–150 (US1, US1-3, FR-001, FR-002), 177 (Key Entities), 196 (SC-006) | **«Código» frente a «referencia».** La spec habla del «código» de la pieza («ese código ya existe»); el plan, el contrato, la pantalla y el mensaje dicen «referencia» («Ya existe la referencia PF-123»). Es el mismo caso que I3 de spec-003. | Usar «referencia» en la spec. |

### Coverage Summary Table

| Requirement Key | Has Task? | Task IDs | Notes |
|-----------------|-----------|----------|-------|
| FR-001 | Sí | T009, T011, T013 | Validación en pantalla del navegador (C1) |
| FR-002 | Sí | T009, T011 | |
| FR-003 | Sí | T014, T015, T018, T019 | Validación en pantalla del navegador (C1) |
| FR-004 | Sí | T015, T018 | |
| FR-004a | Sí | T015, T018 | |
| FR-005 | Sí | T014, T018 | |
| FR-006 | Sí | T016 | Lo hace spec-005 |
| FR-007 | Sí | T020, T021, T023, T025 | |
| FR-008 | Sí | T020, T023 | |
| FR-008a | Sí | T016, T030 | No hay endpoint para cambiarlo a mano (D4) |
| FR-009 | Sí | T022, T024, T025, T026, T027, T028 | |
| FR-010 | Sí | T017, T030 | |
| FR-011 | Sí | T030 | |
| SC-001 | Parcial | T032 | Sin paso de cronometraje (G1) |
| SC-002 | Sí | T014, T020 | |
| SC-003 | Sí | T020, T032 | En la app, vía la regresión (G2) |
| SC-004 | Sí | T020 | |
| SC-005 | Sí | T015 | |
| SC-006 | Sí | T009 | |

### Constitution Alignment Issues

- **Principio II** (C1): los mensajes de validación de `/almacen` y `/pedidos` los pone el navegador.
- **Principio IV** (G1, G2): SC-001 sin paso de cronometraje; la vuelta a «en reparación» se anotó
  como comprobada en la base de datos.

### Unmapped Tasks

Ninguna.

### Metrics

- **Total Requirements**: 19 (13 FR + 6 SC)
- **Total Tasks**: 32 (todas hechas)
- **Coverage**: 100 % de los requisitos con al menos una tarea (SC-001, solo parcial)
- **Ambiguity Count**: 0
- **Duplication Count**: 0
- **Critical Issues Count**: 1 (C1)

## Next Actions

- **C1 es CRITICAL**: corregirlo como en spec-003 (decisión de la Persona B del 12/11/2026).
- G1: misma decisión que en spec-003 y spec-005. G2 e I1: documentación.
- Después, `analyze-2.md` y `/speckit-converge`.
