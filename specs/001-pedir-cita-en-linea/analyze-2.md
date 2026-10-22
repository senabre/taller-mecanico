# Análisis de coherencia 2 · spec-001 Pedir cita en línea (HU-01)

**Fecha**: 2026-10-22 · **Alcance**: el mismo que [analyze-1.md](analyze-1.md): `spec.md`,
`plan.md`, `research.md`, `data-model.md`, `contracts/api.md`, `quickstart.md` y `tasks.md` de
spec-001, contrastados con la constitución 1.2.1, con `specs/000-base-comun/` y con lo que esperan
spec-002 y spec-007. Estado revisado: commits `889e431` (correcciones de 001) y `b46f545`
(documentos comunes).
**Modo**: solo lectura sobre los documentos de la spec. Además de este informe, solo se actualiza
el estado de [hallazgos.md](hallazgos.md).

## Verificación de los hallazgos de analyze-1

| ID | Gravedad | Resultado | Evidencia |
|----|----------|-----------|-----------|
| I1 | HIGH | **Verificado** | T014, T026 y T027 crean sus vehículos con `insertar_vehiculo`, que aparece 4 veces en tasks. El checkpoint de US1 («pruebas 2, 3, 6, 7, 10 y 12») ya se puede cumplir sin US2, y «Phase Dependencies» dice que US3 no depende de US2. El plan (pruebas 8, 9 y 12) coincide. |
| I2 | HIGH | Sigue abierto (spec-002) | No hay cambios en spec-002; se decide en su clarify. |
| I3 | MEDIUM | **Verificado** | Contrato común: «Todos los 422 llegan antes que cualquier 409». Plan, reglas clave, pasos 1–8; research D2 («Cuándo») y D3 («Segunda búsqueda»); T008 pasos 2 y 4; T024 con dos huecos; T021 con el caso del día lleno. Todos coinciden. |
| I4 | MEDIUM | Sigue asignado (spec-007) | La fila del historial en el contrato común sigue sin el 422; le corresponde a la Persona B. |
| A1 | MEDIUM | **Verificado** | Quickstart común §1, paso 10 (`2222CCC` < 1 min, `9012DFG` < 3 min). Correspondencia de SC-001…SC-004 en el quickstart de 001; T032 «pasos 1 a 10». |
| I5 | LOW | **Verificado** | Plan maestro: la lista de ficheros compartidos incluye `matricula.rs` y `rutas/mod.rs`. Plan de 001: 3 ficheros compartidos. |
| I6 | LOW | Sigue abierto (spec-002) | Sin cambios; va junto con I2. |
| I7 | LOW | Cerrado (falso positivo) | Sin cambios. Observación: el mensaje de `b46f545` también dice «revisión Persona B». Con el mismo criterio que en I7, la revisión se da por hecha solo cuando se marque T031. |
| A2 | LOW | **Verificado** | El texto fijo «No quedan fechas libres en los próximos 30 días. Llama al taller para pedir cita.» está en la spec (casos límite), el plan, research D6, el contrato de 001 y T030. |
| A3 | LOW | **Verificado** | T006 (prueba 11) incluye el caso positivo «último día de lunes a viernes ≤ `hoy + 30`»; el plan, en su prueba 11, coincide. |
| T1 | LOW | **Verificado** | El contrato común tiene el 404 «No hay ningún vehículo con esa matrícula»; T015 y el plan remiten a él. |
| T2 | LOW | **Verificado** | T032 enumera todos los FR (con FR-004a), los escenarios y SC-001…SC-004; T033 relaciona cada FR con sus pruebas; T005 queda como infraestructura. |
| G1 | LOW | **Verificado** | R6 del plan maestro tiene la tabla de claves (una clave = fecha desde 2000-01-01; dos claves `(1, hashtext(matrícula))`) y la regla de registrar ahí cualquier bloqueo nuevo. T008 remite a R6. |
| D1 | LOW | Cerrado (justificado), con una precisión | El data-model común enlaza la tabla de validación (2 enlaces). Research D2 la llama «fuente única», pero sigue repitiendo las reglas: ver el hallazgo nuevo D2. |

## Hallazgos nuevos

| ID | Categoría | Gravedad | Ubicación | Resumen | Recomendación |
|----|-----------|----------|-----------|---------|---------------|
| D2 | Duplicación | LOW | research D2 (viñetas «Teléfono», «Correo» y «Matrícula, fecha, motivo…: no vacíos»); hallazgos D1 («research D2 enlazan ahora la tabla en lugar de repetir la regla») | D2 dice que la tabla del contrato es la fuente única, pero sigue copiando las reglas, y para la fecha dice «no vacía», cuando la tabla exige «uno de los días que ofrece `dias-disponibles`». La decisión de D1 en hallazgos.md describe un cambio que en D2 no se hizo. | Bajo, no se corrige: D2 es el registro de la decisión y, si hay diferencias, manda la tabla del contrato, que D2 nombra como fuente única. Se corrige el texto de D1 en hallazgos.md para que diga lo que se hizo de verdad. |
| I8 | Inconsistencia | LOW | research D2, viñeta «Frontend» («Los mensajes salen siempre del servidor»); contrato de 001 («los mensajes de error de los campos salen del servidor» y dos textos propios del formulario); T030 | Después de A2, el formulario tiene dos textos propios (no quedan fechas libres, y el día se acaba de completar). El contrato de 001 ya los recoge como excepción, pero research D2 sigue diciendo «siempre». | Bajo, no se corrige: D2 trata de la validación de campos, y para esos mensajes «siempre» es correcto. El contrato de 001 es el documento que lista los textos de la pantalla. |
| G2 | Hueco de pruebas | LOW | research D3 («evita que dos altas simultáneas de la misma matrícula nueva choquen en el `UNIQUE`», «Segunda búsqueda»); T008 paso 4; T013 (prueba 10, con un vehículo ya existente) | La carrera entre dos `POST` simultáneos con la **misma matrícula nueva** recorre la segunda búsqueda de T008 (paso 4) y la rama D4, pero ninguna prueba la provoca: la prueba 10 parte de un vehículo existente. | Bajo, no se corrige: usa el mismo bloqueo por matrícula que la prueba 10, el resultado esperado está escrito en D3, D4 y T008 (una sola cita y un solo vehículo; el segundo `POST` recibe el 409 de cita pendiente), y el `UNIQUE` de `vehiculo` impide duplicados aunque el código fallara. Se puede añadir como caso de la prueba 10 al implementar, si se ve conveniente. |
| A4 | Ambigüedad | LOW | T016 («…insertar la cita y responder 201») y T029 («entre la comprobación de cita pendiente (T016) y el alta del vehículo nuevo (T024)») | En la rama del vehículo conocido, T016 inserta la cita justo después de comprobar la cita pendiente. T029 sitúa el cupo «antes del alta del vehículo nuevo», sin decir de forma explícita que también va antes de esa inserción. | Bajo, no se corrige: la prueba 8 usa ahora vehículos conocidos (I1), así que si el cupo se coloca mal en esa rama, la prueba falla. El orden completo está en «Within Each User Story» y en el plan (pasos 5 → 6 → 8). |

## Cobertura

| Requisito | ¿Tiene tarea? | Tareas | Notas |
|-----------|---------------|--------|-------|
| FR-001 | Sí | T002, T003, T018, T032 | |
| FR-002 | Sí | T001, T008, T009, T010, T015, T018, T031, T032, T033 | |
| FR-003 | Sí | T004, T009, T011, T015, T016, T018, T032, T033 | |
| FR-004 | Sí | T020, T022, T024, T025, T032, T033 | |
| FR-004a | Sí | T021, T023, T024, T025, T031, T032, T033 | Ahora también en T032 (T2) |
| FR-005 | Sí | T008, T022, T024, T032, T033 | |
| FR-006 | Sí | T006, T007, T008, T030, T032, T033 | Límite positivo comprobado (A3) |
| FR-007 | Sí | T008, T026, T027, T028, T029, T030, T032, T033 | |
| FR-008 | Sí | T009, T016, T018, T032, T033 | |
| FR-009 | Sí | T002, T004, T014, T017, T019, T031, T032, T033 | |
| FR-010 | Sí | T008, T012, T013, T015, T016, T018, T031, T032, T033, T034 | Carrera con una matrícula nueva sin prueba (G2) |
| SC-001 | Sí | T032 | Quickstart §1.10 (A1) |
| SC-002 | Sí | T032 | Quickstart §1.10 (A1) |
| SC-003 | Sí | T026, T027, T032, T033 | |
| SC-004 | Sí | T014, T032 | |

## Constitución (1.2.1)

No hay conflictos. Los cambios no añaden alcance (III): el 404 con texto y los mensajes fijos
salen de la spec o del contrato. SC-001 y SC-002 ya tienen paso de comprobación en la aplicación
(IV). Las pruebas siguen sin valores demo (V). El quickstart usa `2222CCC` y `9012DFG`, que son
matrículas de demostración y no secretos.

## Encaje con la base común y otras specs

- **Documentos comunes cambiados** en `b46f545` (contrato, data-model, quickstart, plan maestro y
  R6): son coherentes entre sí y con 001. La revisión de la Persona B sigue en T031, junto con T001
  y T004.
- **spec-002**: I2 e I6 siguen abiertos; ningún cambio de esta ronda los agrava. El paso 10
  añadido al quickstart no afecta al §2.
- **spec-007**: I4 sigue pendiente de la Persona B. Nada más de esta ronda afecta a spec-007.

## Métricas

- Requisitos: 15 (11 FR y 4 SC) · Tareas: 34 (sin cambios)
- Cobertura: 15/15 (100 %), ya sin coberturas parciales
- Hallazgos de analyze-1: 10 verificados o cerrados; 3 siguen fuera de esta spec (I2 e I6 en
  spec-002, I4 en spec-007)
- Hallazgos nuevos: 4, todos LOW (D2, I8, G2, A4) y justificados
- Críticos: 0 · Altos abiertos en 001: 0

## Próximos pasos

- Nada de spec-001 bloquea `/speckit-implement`. Antes de fusionar hay que completar T031
  (revisión de la Persona B).
- Pendiente fuera de esta spec: clarify de spec-002 (I2, I6) y contrato y spec de spec-007 (I4).
