# Analyze 2 · spec-008 · Tablero de órdenes abiertas

**Fecha**: 2026-11-12 · **Rama**: `spec-008` (commit `fbd54a8` más los cambios sin commit de las
decisiones de analyze-1) · **Artefactos**: [spec.md](spec.md), [plan.md](plan.md),
[research.md](research.md) (D1–D9), [data-model.md](data-model.md),
[contracts/api.md](contracts/api.md), [quickstart.md](quickstart.md),
[decisiones-clarify.md](decisiones-clarify.md) y [tasks.md](tasks.md) (T001–T032, renumeradas) ·
**Contexto**: [000-base-comun](../000-base-comun/) (contrato, quickstart §8 y §9, research R13) ·
**Constitución**: 1.2.1.

Análisis de solo lectura: comprueba las correcciones de los 10 hallazgos de
[analyze-1](analyze-1.md) y busca problemas nuevos. Solo se han escrito este informe y los estados
de [hallazgos.md](hallazgos.md).

## Verificación de los hallazgos de analyze-1

| ID | Comprobación | Resultado |
|----|--------------|-----------|
| E1 | research D5 (`lib/eventos.ts`, reconexión tras `CLOSED` con 5 s de espera, 401 → `/entrar`, tablero entero al reconectar); T021 (5 casos con `EventSource` simulado) y T024 antes de T025; plan (estructura y pruebas); contrato de la spec (filas `CONNECTING` y `CLOSED`); R13 común con `eventos.ts` | Verificado. Queda una diferencia de redacción entre D5 y las tareas: N3 |
| G1 | quickstart común §9: 8 pasos con `3579KLM`; día A después del §8, con el primer día que ofrezca la página y el aviso de que puede no ser el siguiente; día B sin recargar los datos demo, con el aviso de «Retrasada» en el paso; Nº 13 tras el §8.7; lo que se ve en el tablero en cada paso. T030 sigue esos pasos. `3579KLM` es válida (`matricula.rs`) y no se usa en ningún otro sitio | Verificado. Revisión de B en T032 |
| I1 | spec: ninguna «00:00» queda en la aclaración de Q3, en US4-4 ni en FR-005 («a medianoche (00:01 de Madrid, con un minuto de margen)»). El plan, D6, el contrato de la spec y T019 dicen lo mismo | Verificado |
| E2 | D4/D5, plan, contrato de la spec y T025: con un 403 no se abre la conexión ni la medianoche | Verificado, pero escrito como «solo tras una primera carga con 200», que va más allá de lo decidido: N2 |
| A1 | T010 nombra `estado.db.begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY")` y `&mut *tx` | Verificado |
| A2 | T019, D6 y el plan: la prueba comprueba primero el desfase (240, no −120) y falla si no es otro | Verificado |
| I2 | T026 y el plan: «9 movimientos en 8 llamadas» | Verificado |
| C1 | quickstart común §8.9 (parar y arrancar el servidor; la red cortada, opcional) y §8.10 (móvil en la red; el servidor escucha en `0.0.0.0:3000`); T028 hace las dos | Verificado. Revisión de B en T032 |
| I3 | D7, el plan (estructura y tabla) y T011: `<nav>` y `<main>` sin `max-w-6xl` solo en `/tablero`, con una variable; el resto igual | Verificado. Revisión de B en T032 |
| I4 | decisiones-clarify ya no tiene tabla propia y remite a la del plan | Verificado |

**Renumeración**: las 32 tareas siguen el formato y van en orden (T019–T021 pruebas, T022–T024
módulos, T025 página, T026 eventos). Las referencias internas (dependencias, «no se marca hasta T028»,
«en el mismo recorrido de T028», carpetas `evidencias/T028/`, `T029/` y `T030/`) están al día, salvo
una referencia a spec-004 que se cambió por error: N1.

## Specification Analysis Report

| ID | Category | Severity | Location(s) | Summary | Recommendation |
|----|----------|----------|-------------|---------|----------------|
| N1 | Inconsistency | MEDIUM | tasks.md:173 (T030) | **La renumeración cambió una ruta de spec-004.** T030 dice `specs/004-aceptar-presupuesto-enlace/evidencias/T031/scripts/`, que no existe: los scripts están en `evidencias/T029/scripts/`. Las dos menciones de texto («T029 de spec-004») sí se arreglaron, pero la ruta no. Error introducido al aplicar las decisiones de analyze-1. | Volver a poner `evidencias/T029/scripts/` en T030. |
| N2 | Coverage | MEDIUM | research.md:110–111 (D5) · plan.md:136–137 · contracts/api.md:46 · tasks.md:161 (T025) | **Con un fallo de red en la primera carga, el tablero no se recupera nunca.** La decisión de E2 fue «con un 403 no se abre la conexión de eventos», pero los documentos dicen «solo tras una primera carga con 200». Si el jefe abre el tablero mientras el servidor se reinicia, la primera carga falla («No se pudo cargar el tablero»), no se abre la conexión y no hay `onopen` que vuelva a pedirlo: se queda así hasta recargar a mano. Choca con el caso límite «se recupera solo al volver la conexión». | Abrir la conexión y la medianoche en todos los casos **salvo con un 403** (un 401 ya lleva a `/entrar`). Así, tras un fallo de red, el `onopen` de la conexión vuelve a pedir el tablero. Corregir D5, el plan, el contrato de la spec y T025. |
| N3 | Inconsistency | LOW | research.md:102, :106 (D5) · tasks.md:160 (T024), :161 (T025) · contracts/api.md (filas `onerror`) | D5 escribe `alError()` sin argumento y dice que «`alError` pide el tablero» en cualquier error. T024 lo define como `alError(cerrada)`, y T025 y el contrato solo piden el tablero con la conexión cerrada (`CLOSED`). | Dejar en D5 `alError(cerrada: boolean)`, y que la página pida el tablero solo si `cerrada`. |
| N4 | Inconsistency | LOW | quickstart.md (de la spec): «Pruebas automáticas» y tabla de correspondencia | La guía de la spec solo nombra `medianoche.test.ts` (faltan `recarga.test.ts` y `eventos.test.ts`) y no incluye el §9 reescrito, que comprueba US4-1 a US4-3 y SC-002 con la orden nueva. Tampoco dice que el paso 9 ya no necesita herramientas. | Añadir los dos ficheros de Vitest, una fila para el §9 (pasos 3–7) y la nota del paso 9. |

**Coverage Summary Table:**

| Requirement Key | Has Task? | Task IDs | Notes |
|-----------------|-----------|----------|-------|
| FR-001 | Sí | T005–T007, T009, T010, T012, T028, T030 | |
| FR-002 | Sí | T006, T012, T028 | |
| FR-003 | Sí | T013–T016, T028 | «(de baja)» solo en pruebas automáticas (D8) |
| FR-004 | Sí | T017, T018, T028, T029 | |
| FR-005 | Sí | T019–T026, T028–T030 | Ver N2 y N3 |
| FR-006 | Sí | T008, T010, T012, T025, T028 | |
| SC-001 | Sí | T029 (cronometrado), T009, T018 | |
| SC-002 | Sí | T029 (cronometrado), T030 (§9, día B), T020, T023, T025 | |
| SC-003 | Sí | T013–T015, T028 | |
| SC-004 | Sí | T006, T010, T028 | |

**Constitution Alignment Issues:** ninguno. Tras C1, los pasos del §8 se pueden hacer sin
herramientas técnicas, y las herramientas de desarrollo quedan como opción.

**Unmapped Tasks:** ninguna.

**Metrics:**

- Total Requirements: 10 (FR-001–FR-006, SC-001–SC-004)
- Total Tasks: 32
- Coverage %: 100 %
- Ambiguity Count: 0
- Duplication Count: 0
- Critical Issues Count: 0
- Hallazgos de analyze-1: 10 de 10 verificados (G1, C1, I3 y la parte de R13 de E1, pendientes de la
  revisión de B en T032)
- Hallazgos nuevos: 4 (0 CRITICAL, 0 HIGH, 2 MEDIUM: N1 y N2; 2 LOW: N3 y N4)

## Next Actions

- Ningún CRITICAL ni HIGH. N1 y N2 conviene corregirlos antes de `/speckit-implement`: N1 es una
  ruta rota que T030 seguiría al pie de la letra, y N2 dejaría sin cubrir el caso límite de la
  conexión al abrir el tablero.
- N3 y N4 son cambios de una línea en D5 y en la guía de la spec.
- Después de corregirlos, se puede pasar a `/speckit-implement` sin otro análisis completo, como
  en 004, anotando la corrección en hallazgos.md.
