# Analyze 3 · spec-008 · Tablero de órdenes abiertas

**Fecha**: 2026-11-12 · **Rama**: `spec-008` (commit `fbd54a8` más los cambios sin commit de las
decisiones de analyze-1 y analyze-2) · **Artefactos**: [spec.md](spec.md), [plan.md](plan.md),
[research.md](research.md) (D1–D9), [data-model.md](data-model.md),
[contracts/api.md](contracts/api.md), [quickstart.md](quickstart.md),
[decisiones-clarify.md](decisiones-clarify.md) y [tasks.md](tasks.md) (T001–T032) · **Contexto**:
[000-base-comun](../000-base-comun/) (contrato, quickstart §8 y §9, research R13) · **Constitución**:
1.2.1.

Análisis de solo lectura: comprueba las correcciones de los 4 hallazgos de [analyze-2](analyze-2.md)
y busca problemas nuevos. Solo se han escrito este informe y los estados de
[hallazgos.md](hallazgos.md).

## Verificación de los hallazgos de analyze-2

| ID | Comprobación | Resultado |
|----|--------------|-----------|
| N1 | T030 remite a `specs/004-aceptar-presupuesto-enlace/evidencias/T029/scripts/`, que existe. No queda ninguna ruta `evidencias/T031/` | Verificado |
| N2 | D5 (research.md:119), plan.md:137, contracts/api.md:46 y T025 dicen lo mismo: la conexión de eventos y la medianoche se abren después de la primera carga **en todos los casos salvo con un 403**, y si la primera carga falló por la red, el primer `onopen` la repite. Ya no queda en ningún documento «solo tras una primera carga con 200» (solo en el registro histórico de E2 en hallazgos.md, que remite a N2) | Verificado |
| N3 | La regla es la misma en D5, el plan, el contrato de la spec, T021, T024 y T025: el tablero se pide en cada `onopen` (la primera vez y cada vez que la conexión se abre de nuevo tras un error, reconecte el navegador solo o la cree la página tras un cierre); un error no lo pide; **única excepción**, con la conexión cerrada (`CLOSED`) se pide una vez antes de crear la nueva, para que un 401 lleve a `/entrar`. La firma es `alError(cerrada: boolean)` en todos, y las pruebas usan `alError(false)` y `alError(true)`. No queda «en cada error» ni «en cualquier error» | Verificado |
| N4 | La guía de la spec remite también al §9 y explica sus dos sesiones; tiene tres filas del §9 (US4-1 a US4-3, SC-002 y SC-003), la nota del paso 9 sin herramientas y las tres pruebas de Vitest (`medianoche`, `recarga` y `eventos`). La fila del caso límite de la conexión dice que la primera carga fallida (N2) se comprueba en el código de la página, porque a mano no se puede provocar | Verificado |

## Specification Analysis Report

No se han encontrado hallazgos nuevos.

Además de los cuatro puntos anteriores, se ha vuelto a comprobar:

- La spec no tiene ninguna «00:00» (I1) y sus escenarios de la US4 y el caso límite «Se pierde la
  conexión» coinciden con D5 tras N2 y N3.
- Las 32 tareas siguen el formato, van en orden y todas terminan con «**Cubre:**». Las
  dependencias, «no se marca hasta T028», «en el mismo recorrido de T028», las carpetas
  `evidencias/T028/` a `T030/` y las referencias a T029 de spec-004 están al día.
- El contrato común, el quickstart común (§8 y §9) y R13 coinciden con el plan y las tareas.

**Coverage Summary Table:**

| Requirement Key | Has Task? | Task IDs | Notes |
|-----------------|-----------|----------|-------|
| FR-001 | Sí | T005–T007, T009, T010, T012, T028, T030 | |
| FR-002 | Sí | T006, T012, T028 | |
| FR-003 | Sí | T013–T016, T028 | «(de baja)» solo en pruebas automáticas (D8) |
| FR-004 | Sí | T017, T018, T028, T029 | |
| FR-005 | Sí | T019–T026, T028–T030 | |
| FR-006 | Sí | T008, T010, T012, T025, T028 | |
| SC-001 | Sí | T029 (cronometrado), T009, T018 | |
| SC-002 | Sí | T029 (cronometrado), T030 (§9, día B), T020, T023, T025 | |
| SC-003 | Sí | T013–T015, T028, T030 (§9, paso 2) | |
| SC-004 | Sí | T006, T010, T028 | |

**Constitution Alignment Issues:** ninguno.

**Unmapped Tasks:** ninguna.

**Metrics:**

- Total Requirements: 10 (FR-001–FR-006, SC-001–SC-004)
- Total Tasks: 32
- Coverage %: 100 %
- Ambiguity Count: 0
- Duplication Count: 0
- Critical Issues Count: 0
- Hallazgos de analyze-2: 4 de 4 verificados
- Hallazgos nuevos: 0

## Next Actions

- Sin hallazgos abiertos: se puede pasar a `/speckit-implement`, empezando por T001 (`demo.sql`) y
  T002 (`tipos.ts`), cada uno en su commit.
- Siguen pendientes de la revisión de la Persona B en T032 los cambios en documentos comunes: G1
  (§9), C1 (§8.9 y §8.10), I3 (`+layout.svelte`, al implementar) y R13 (E1).
