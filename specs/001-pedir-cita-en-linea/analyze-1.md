# Análisis de coherencia 1 · spec-001 Pedir cita en línea (HU-01)

**Fecha**: 2026-10-22 · **Alcance**: `spec.md`, `plan.md`, `research.md` (D1–D7), `data-model.md`,
`contracts/api.md`, `quickstart.md` y `tasks.md` (T001–T034) de `specs/001-pedir-cita-en-linea/`.
Se contrastan con la constitución 1.2.1, con los documentos comunes de `specs/000-base-comun/`
(plan maestro, research, data-model, contrato y quickstart) y con lo que esperan spec-002 y
spec-007.
**Modo**: solo lectura. Este informe no corrige ningún otro fichero.

## Hallazgos

| ID | Categoría | Gravedad | Ubicación | Resumen | Recomendación |
|----|-----------|----------|-----------|---------|---------------|
| I1 | Inconsistencia (orden de tareas) | **HIGH** | tasks T014 (prueba 12), T026 (prueba 8), T027 (prueba 9); tasks «Checkpoint» de US1 y «Phase Dependencies» | T014 crea por `POST` dos citas, «primero `1234BCD` y después `5678CDF`». `5678CDF` es una matrícula nueva, y el alta de vehículos nuevos no existe hasta T024 (US2). Por tanto, la prueba 12 no puede pasar al final de US1, aunque el checkpoint de US1 lo afirma. T026 y T027 (US3) no dicen si sus matrículas están ya insertadas; si son nuevas, US3 también depende de US2, aunque «Phase Dependencies» dice que US3 solo depende de la Phase 2. | En T014, T026 y T027, crear antes los vehículos con `insertar_vehiculo` (T005) y pedir las citas por `POST` como vehículos conocidos. Así cada historia se prueba sola, como indica la plantilla. Si se prefiere usar matrículas nuevas, mover la prueba 12 a US2 y declarar que US3 depende de US2. |
| I2 | Inconsistencia entre specs | **HIGH** | spec-002 `spec.md:112-113` (Assumptions: «Si un cliente llega sin cita, la recepción le registra primero la cita (spec-001) y después abre la orden»); spec-001 FR-006 (solo días a partir de mañana) y FR-010; 001 research D7; 001 plan «Pendiente fuera de esta spec» | Con FR-006, recepción no puede registrar una cita para hoy, así que un cliente sin cita no puede entrar en el taller el mismo día. Si el vehículo ya tiene otra cita pendiente, FR-010 también lo impide. La contradicción solo está anotada en documentos de 001; no hay ningún hallazgo abierto ni nada en spec-002 que la recoja. | No bloquea la implementación de 001, pero hay que decidirla en el clarify de spec-002 (Persona A). Las opciones son permitir que recepción pida citas para hoy (un cambio en FR-006 de 001) o quitar esa suposición de spec-002. Mientras tanto, anotarla en un hallazgo con su responsable para que no se pierda. |
| I3 | Inconsistencia (regla de validación) | MEDIUM | 000 `contracts/api.md`, párrafo «Validación de spec-001 (422)» («El servidor comprueba en este orden y responde el primer error»); 001 research D2 («Se valida en orden fijo»); 001 plan «Reglas clave» (pasos 1, 3, 4 y 5); tasks T008, T016, T029 y T024 | El contrato da a entender que todos los 422 se comprueban antes que cualquier otra cosa. Pero el plan y las tareas comprueban los campos del vehículo nuevo (marca, modelo, nombre, teléfono, correo) **después** de los 409 de cita pendiente y de cupo. Una matrícula nueva con un teléfono incorrecto para un día lleno recibe 409 «Ese día ya está completo» y no el 422 del teléfono. Ningún escenario lo contradice, pero el contrato y el código dirían cosas distintas. | Escribir en el contrato común el orden completo: validación de matrícula, fecha y motivo (422) → cita pendiente (409) → cupo (409) → datos del vehículo nuevo (422). Otra opción es validar los campos del vehículo nuevo antes de los bloqueos (se sabe si es nuevo con una consulta previa) y dejar el contrato como está. Decidir una y alinear D2, el plan y T024. |
| I4 | Dependencia con spec-007 | MEDIUM | 000 `contracts/api.md:134` (`GET /api/vehiculos/{matricula}/historial`, sin 422); 001 research D1 («buscar … 1 carácter o más de 10 da 422»); spec-007 (sin escenario para una matrícula no válida); tasks T001 y T031 | Tras T001, `normalizar_matricula` rechaza matrículas de 1 carácter o de más de 10, y según D1 el historial de spec-007 responde 422. El contrato común de spec-007 no recoge ese 422 (tampoco el que ya existía para `12#4`), y spec-007 no dice qué ve el usuario. La Persona B depende de un cambio que su contrato no recoge. | Añadir al contrato de spec-007 «422 «La matrícula no es válida»» y, en spec-007 (Persona B), un caso límite con el mensaje que se muestra en `/historial`. Incluirlo en la revisión de T031. |
| A1 | Ambigüedad / cobertura de criterios | MEDIUM | spec SC-001 (< 1 min) y SC-002 (< 3 min); tasks T032 (única tarea que los cita); 000 `quickstart.md` §1 | Ningún paso del quickstart dice que se mida el tiempo, así que T032 «cubre» SC-001 y SC-002 sin decir cómo se comprueban. Según el principio IV, cada criterio de éxito tiene que poder comprobarse usando la aplicación. | Añadir al quickstart §1 que se cronometre el paso 6 (vehículo conocido, < 1 min) y el paso 3 sin los errores provocados (vehículo nuevo, < 3 min), y citar esos pasos en T032. |
| I5 | Inconsistencia (ficheros compartidos) | LOW | 000 `plan.md:140-141` («Los ficheros compartidos son la migración, `estados.rs`, `dinero.rs`, `tipos.ts`, `formato.ts`, `componentes/` y `ordenes/[id]`»); 001 plan («1 cambio en un fichero compartido (`matricula.rs`)» y Source Code con `tipos.ts (compartido)`) | El plan maestro no incluye `matricula.rs` entre los ficheros compartidos, aunque 001 lo trata como tal y lo usa también 007. Además, el plan de 001 habla de «1» fichero compartido, pero T004 también cambia `tipos.ts`. | Añadir `matricula.rs` (y `rutas/mod.rs`, que también toca T003) a la lista del plan maestro, y en 001 escribir «2 ficheros compartidos (`matricula.rs`, `tipos.ts`)». |
| I6 | Dependencia con spec-002 | LOW | 000 `quickstart.md` §2.1–§2.3; spec-002 US1 («El día de la cita…», «Given una cita para hoy») | Después del §1, todas las citas son de mañana en adelante (FR-006), pero el §2 abre órdenes enseguida, sobre citas de días futuros. Pasa lo mismo con la cita de `9999LLL` que usa el §2.3 y T034. spec-002 describe que la orden se abre el día de la cita. Esto ya ocurría antes del clarify; el cambio del §2.3 no lo arregla ni lo empeora. | En el clarify de spec-002, decidir si se puede abrir una orden sobre una cita de otro día (lo que asume hoy el quickstart) o solo el mismo día; ajustar el §2 en consecuencia. Va junto con I2. |
| I7 | Inconsistencia (estado de tareas) | LOW | tasks T031; commit `2494cfd` («contrato, data-model y quickstart §1-§2.3 … (revisión Persona B)») | T031 pide a la Persona B revisar documentos comunes que, según el historial, ya revisó. | Indicar en T031 que la revisión de los documentos ya está hecha (citando el commit) y que solo queda la del código compartido (T001 y T004) y el 422 de spec-007 (I4). |
| A2 | Ambigüedad | LOW | spec «Edge Cases» («se muestra un mensaje indicando que no hay fechas libres y que llame al taller»); tasks T030 | No hay un texto fijado para este mensaje; T030 deja la redacción a quien lo implemente. Se puede comprobar la idea, pero no el texto. | Fijar el texto en la spec o en el contrato (p. ej., «No quedan fechas libres en los próximos 30 días. Llama al taller para pedir cita.»). |
| A3 | Ambigüedad (límite) | LOW | spec FR-006 («a partir del día siguiente y durante los próximos 30 días naturales»); 000 data-model `cita.fecha`; tasks T006 (prueba 11) y T007 | T007 interpreta el rango como de `hoy + 1` a `hoy + 30`, y la prueba 11 solo comprueba que `hoy + 31` se rechaza. No hay ningún caso que compruebe que `hoy + 30` (si es laborable) se acepta, así que un error de uno en el límite pasaría las pruebas. | Añadir a la prueba 11 el caso positivo: el último laborable ≤ `hoy + 30` → 201 o presente en `dias-disponibles`. |
| T1 | Tarea sin requisito | LOW | tasks T015 (404 «No hay ningún vehículo con esa matrícula»); 000 contrato (404 sin texto) | T015 fija un mensaje que no está ni en la spec ni en el contrato. El formulario no lo muestra (con el 404 abre los campos del vehículo nuevo), así que no afecta a lo que ve el usuario. | Añadir el texto al contrato o quitarlo de T015 y usar el mensaje genérico del 404 (principio III). |
| T2 | Trazabilidad | LOW | tasks T032 («Cubre: FR-001…FR-010, US1-1…US1-10, …»), T033 («todas las pruebas»), T005 («—») | En T032 se escribe un rango, y no queda claro si incluye FR-004a. T033 y T005 no tienen ningún requisito concreto. Con eso, la matriz de trazabilidad no se puede construir de forma automática. | En T032, escribir la lista completa (incluido FR-004a, que comprueba el paso 3 del quickstart). En T033, poner la lista de las pruebas 1–12 y sus FR. T005 puede quedarse como infraestructura, indicándolo así. |
| G1 | Hueco (convención compartida) | LOW | 000 research R6 («con la fecha como clave»); tasks T008 (clave = días desde 2000-01-01, forma de una clave; matrícula = `(1, hashtext)`, forma de dos claves); 001 research D3 | Hoy no hay ningún otro `pg_advisory_xact_lock` en el código ni en los documentos, así que no hay colisión. Pero la codificación de las claves solo está en una tarea de 001. Si otra spec usa en el futuro bloqueos de una clave con valores pequeños, podría coincidir con las fechas sin que nadie lo sepa. | Documentar en R6 del plan maestro las dos claves y su espacio («una clave = fecha de cita, días desde 2000-01-01; dos claves con primer valor 1 = matrícula») y pedir que cualquier bloqueo nuevo se registre ahí. |
| D1 | Duplicación | LOW | Reglas de teléfono y correo en spec FR-004a, 000 data-model `cliente`, 000 contrato (tabla de validación), 001 research D2, tasks T021 y T023 | La misma regla está escrita en cinco sitios. Si cambia, es fácil que alguno se quede desactualizado. | Dejar la regla completa en la spec (fuente funcional) y en la tabla del contrato (fuente técnica), y que data-model y research remitan a la tabla. |

## Cobertura

| Requisito | ¿Tiene tarea? | Tareas | Notas |
|-----------|---------------|--------|-------|
| FR-001 | Sí | T002, T003, T018, T032 | |
| FR-002 | Sí | T001, T008, T009, T010, T015, T018, T031, T032 | Cambio compartido en T001 (I4, I5) |
| FR-003 | Sí | T004, T009, T011, T015, T016, T018, T032 | |
| FR-004 | Sí | T020, T022, T024, T025, T032 | |
| FR-004a | Sí | T021, T023, T024, T025, T031 | No aparece de forma explícita en T032 (T2) |
| FR-005 | Sí | T008, T022, T024, T032 | |
| FR-006 | Sí | T006, T007, T008, T030, T032 | Sin caso positivo en el límite (A3) |
| FR-007 | Sí | T008, T026, T027, T028, T029, T030, T032 | Datos de las pruebas sin precisar (I1) |
| FR-008 | Sí | T009, T016, T018, T032 | |
| FR-009 | Sí | T002, T004, T014, T017, T019, T031, T032 | La prueba 12 depende de US2 (I1) |
| FR-010 | Sí | T008, T012, T013, T015, T016, T018, T031, T032, T034 | |
| SC-001 | Parcial | T032 | Sin paso de medida (A1) |
| SC-002 | Parcial | T032 | Sin paso de medida (A1) |
| SC-003 | Sí | T026, T027, T032 | |
| SC-004 | Sí | T014, T032 | |

Escenarios: US1-1…US1-10, US2-1…US2-4, US3-1 y US3-2 tienen al menos dos tareas cada uno, y el
[quickstart.md](quickstart.md) de 001 relaciona cada escenario con un paso del quickstart común.

## Constitución (1.2.1)

No hay conflictos con ningún principio. Observaciones:

- **III (cero alcance fantasma)**: solo T1, un mensaje sin respaldo que el usuario no llega a ver.
  `cita_pendiente`, `cliente {nombre, telefono}` y el 422 de fecha se apoyan en FR-010, FR-009 y
  FR-006.
- **IV (verificable sin código)**: SC-001 y SC-002 no tienen paso de medida (A1). La concurrencia
  (US3-2 y FR-010 con dos peticiones a la vez) solo se comprueba con pruebas automáticas, pero su
  efecto visible (SC-003) sí se comprueba en la app (quickstart §1.5).
- **V (datos)**: las pruebas usan datos propios (`Ana Pérez`, `ana@ejemplo.test`) y ningún valor
  demo ni secreto. La lista no expone el correo y la API pública no devuelve la fecha de la cita.

## Tareas sin requisito concreto

- T005 (utilidades de prueba): infraestructura, aceptable.
- T033 (comprobaciones automáticas): sin un FR concreto (T2).
- T015, solo en el texto del 404 (T1).

## Métricas

- Requisitos: 15 (11 FR, incluido FR-004a, y 4 SC)
- Tareas: 34
- Cobertura: 15/15 requisitos con al menos una tarea (100 %); 2 SC solo de forma parcial (A1)
- Ambigüedades: 3 (A1, A2, A3)
- Duplicaciones: 1 (D1)
- Críticos: 0 · Altos: 2 (I1, I2) · Medios: 3 (I3, I4, A1) · Bajos: 9

## Próximos pasos

- No hay hallazgos CRITICAL. Antes de `/speckit-implement` conviene resolver **I1** (datos de las
  pruebas 8, 9 y 12 en tasks.md) e **I3** (orden de validación en el contrato común y en T024),
  porque cambian lo que comprueban las pruebas.
- **I2** e **I6** se resuelven en el clarify de spec-002 (Persona A). Hasta entonces, se registran
  como hallazgo abierto.
- **I4** lo cierra la Persona B en el contrato y la spec de spec-007, dentro de la revisión de
  T031.
- **A1** y los LOW se pueden corregir con una edición manual de `tasks.md`, del quickstart común y
  de R6, sin volver a ejecutar `/speckit-plan`.
