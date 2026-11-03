# Analyze 2 · spec-002 · Recibir el vehículo y abrir la orden de reparación

**Fecha**: 2026-11-03 · **Rama**: `spec-002` (último commit `3e1eeec`) · **Artefactos**: [spec.md](spec.md)
(con Q6), [plan.md](plan.md), [research.md](research.md) (D1–D8), [data-model.md](data-model.md),
[contracts/api.md](contracts/api.md), [quickstart.md](quickstart.md) y [tasks.md](tasks.md)
(T001–T031) · **Contexto**: [000-base-comun](../000-base-comun/), spec-001 implementada, planes de
003, 005, 006, 007 y 008 · **Constitución**: 1.2.1.

Mismos criterios que [analyze-1](analyze-1.md). Análisis de solo lectura salvo este informe y el
estado de los hallazgos en [hallazgos.md](hallazgos.md).

## Verificación de los hallazgos de analyze-1

| ID | Decisión | Verificación | Resultado |
|----|----------|--------------|-----------|
| C1 | Corregido (Q6) | spec.md: SC-002 reescrito y escenario 9 de la US1 nuevo; plan: Constitution Check IV y prueba 2 (US1-9, SC-002) / prueba 3 (solo FR-002); quickstart de 002: filas US1-9 y SC-002 (pasos 3–6); tasks: T008 (órdenes 42 y 43 con segunda pestaña), T009 (solo FR-002), T029 (US1-9, SC-002 pasos 3–6), T030 (SC-002 → prueba 2). El quickstart común §2 ya cubre los pasos 3–5 sin cambios. SC-002 se comprueba en la app y la simultaneidad queda en FR-002 y las pruebas automáticas, sin excepción al principio IV | ✅ Verificado |
| I1 | Corregido | Plan de spec-008 (línea 47): `retrasada` = orden no facturada con entrega anterior a hoy, remitiendo al data-model común como fuente única. Coincide con el data-model común, el plan de 002 y T016/T019. La spec de 008 («hoy es posterior a la fecha prometida») no contradice, porque el tablero no muestra facturadas | ✅ Verificado |
| I2 | Corregido | Plan de spec-008 (líneas 51–53 y 59): usa `OrdenResumen` y su consulta de `ordenes.rs`, sin consulta propia, y depende de spec-002 | ✅ Verificado (ver N1) |
| E1 | Justificado | Plan de 002 (Constitution Check III y regla de `GET /api/ordenes`) y T016, T021, T025 citan el menú común (escenario 1 de la US1 de la base común, quickstart §0.4, FR-002 común para el jefe) y el quickstart §3; su «Cubre» ya no atribuye la lista a US1-3 ni SC-004 | ✅ Justificación escrita |
| I3 | Corregido en las tareas | T001 (comentario de la parte 3 de `demo.sql` y quickstart de 001), T022 (contrato de 001: `orden_id`/`orden_numero`) y T023 (contrato de 001: columna «Orden») recogen los cambios. Los documentos de spec-001 siguen con el texto antiguo, como se decidió: se cambian al hacer esas tareas | ✅ Verificado en las tareas |
| G1 | Corregido | Plan maestro (línea 143): `backend/seed/demo.sql` en la lista de ficheros compartidos; tabla de compartidos del plan de 002 y T031 | ✅ Verificado; revisión de B en T031 |
| A1 | Corregido | Contrato común (líneas 92 y 99): `km` como número o texto con cifras; campo ausente u otro tipo → el error de su fila, nunca 400; sin `cita_id` → 404. T012 tiene los casos nuevos (`"85000"`, `""`, `danos` y `mecanico_id` ausentes, sin `cita_id`) y T017 lee los campos como valores libres | ✅ Verificado; revisión de B en T031 |
| A2 | Corregido | T010: una cita nueva por vuelta y el contador avanza 1 en cada una | ✅ Verificado |
| O1 | Corregido | T017 responde `{id, numero, estado}` hasta T019; anotado en «Within Each User Story» | ✅ Verificado |
| E2 | Corregido | T011 ya no cubre US1-6; la nota de trazabilidad y T029 dicen que US1-6 lo cubren spec-001 FR-006 (prueba 11) y el paso 1 del §2 | ✅ Verificado |
| I4 | Corregido | T026 y research D6: el contenedor tiene el rol del usuario y la sección «Factura» de spec-007 se activa para recepción y el jefe | ✅ Verificado |
| I5 | Registrado | Asignado a spec-004 (§4.3) y spec-003 (§4.5) en hallazgos.md | ✅ Asignado (ver N3) |

## Specification Analysis Report (hallazgos nuevos)

| ID | Category | Severity | Location(s) | Summary | Recommendation |
|----|----------|----------|-------------|---------|----------------|
| N1 | Inconsistency | LOW | 008/plan.md:59–60 | El párrafo de dependencias de spec-008 añade spec-002 (I2), pero sigue diciendo «Se puede hacer desde el principio con las órdenes demo en varios estados», que ya no es cierto: el tablero usa la consulta `OrdenResumen` de `ordenes.rs`. El orden del plan maestro (A: 001 → 002 → 004 → 008) ya pone 008 después de 002, así que no bloquea nada. | Quitar esa frase al generar las tareas de spec-008. |
| N2 | Inconsistency | LOW | 002/plan.md:207–210 | El párrafo «Cambios en documentos compartidos» nombra «el contrato, el data-model y el quickstart comunes», pero la tabla de ficheros compartidos incluye ahora también el plan maestro (G1). El párrafo remite a «la tabla de arriba», que sí lo tiene, y T031 lo revisa. | Ninguna: la tabla manda. |
| N3 | Coverage | LOW | hallazgos.md (I5) · specs 003 y 004 | La asignación de I5 solo consta en el registro de spec-002: spec-003 no tiene `hallazgos.md` y spec-004 aún no tiene clarify, así que quien trabaje en ellas puede no verlo. | La Persona A lo lleva al clarify de spec-004; la Persona B lo ve en T031 (que ya revisa §4). |
| N4 | Coverage | LOW | quickstart común §2 (pasos 3–6) · spec SC-002 · T008 | En la app, el único intento rechazado que va seguido de una orden nueva es el del paso 3 (antes de la Nº 10); los de la segunda pestaña (paso 5) y del km menor (paso 6) van después de la última orden, así que en pantalla no se ve que tampoco gastan número. Lo comprueba T008 (prueba 2) con todos los tipos de rechazo. | Ninguna: el paso 3 basta para ver en la app que un rechazo no gasta número, y SC-002 no exige más. |

### Coverage Summary Table

| Requirement Key | Has Task? | Task IDs | Notes |
|-----------------|-----------|----------|-------|
| FR-001 | ✅ | T004, T006, T011, T017, T023, T024, T029 | |
| FR-002 | ✅ | T008, T009, T010, T017, T029 | |
| FR-003 | ✅ | T012, T017, T024 | |
| FR-004 | ✅ | T027, T028 | |
| FR-005 | ✅ | T007, T013, T014, T017, T019 | |
| FR-006 | ✅ | T007, T012, T017 | |
| FR-007 | ✅ | T007, T017, T018 | |
| FR-008 | ✅ | T010, T015, T017, T022, T023, T024 | |
| US1-1 … US1-5, US1-7, US1-8 | ✅ | T006–T026 | |
| US1-6 | ✅ | T029 (paso 1) | Lo cumple spec-001 FR-006 (prueba 11); E2 |
| US1-9 (nuevo, Q6) | ✅ | T008, T029 | |
| US2-1, US2-2, US2-3 | ✅ | T027, T028 | |
| SC-001 | ✅ | T029 (cronometrado) | |
| SC-002 | ✅ | T008, T029 | Comprobable en la app (Q6) |
| SC-003 | ✅ | T027, T028, T029 | |
| SC-004 | ✅ | T014, T019, T026 | |

**Constitution Alignment Issues**: ninguno. C1 (principio IV) queda resuelto con Q6; la lista de
órdenes (E1) tiene su justificación escrita para el principio III.

**Unmapped Tasks**: ninguna. T016, T021 y T025 se atribuyen al menú común y al quickstart §3
(E1); T005 es infraestructura de pruebas.

### Metrics

- **Total Requirements**: 24 (8 FR, 4 SC, 12 escenarios: US1-1 … US1-9, US2-1 … US2-3).
- **Total Tasks**: 31.
- **Coverage %**: 100 %.
- **Ambiguity Count**: 0.
- **Duplication Count**: 0.
- **Critical Issues Count**: 0.
- **Hallazgos de analyze-1**: 12 verificados (10 corregidos o justificados y comprobados; I3,
  escrito en las tareas; I5, asignado).
- **Hallazgos nuevos**: 4, todos LOW (N1, N2, N3, N4); ninguno se corrige: N2 y N4 justificados,
  N1 y N3 asignados.

## Next Actions

- Sin CRITICAL, HIGH ni MEDIUM: se puede pasar a `/speckit-implement`, empezando por los cambios
  compartidos T001, T002 y T003, cada uno en su commit.
- Pendiente de la Persona B en T031: G1 y A1 (documentos comunes ya cambiados) y el resto de lo
  compartido.
- N1: quitar la frase obsoleta del plan de spec-008 cuando se generen sus tareas. N3: llevar I5 al
  clarify de spec-004.
