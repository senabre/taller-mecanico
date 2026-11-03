# Analyze 1 · spec-002 · Recibir el vehículo y abrir la orden de reparación

**Fecha**: 2026-11-03 · **Rama**: `spec-002` · **Artefactos**: [spec.md](spec.md), [plan.md](plan.md),
[research.md](research.md) (D1–D8), [data-model.md](data-model.md),
[contracts/api.md](contracts/api.md), [quickstart.md](quickstart.md) y [tasks.md](tasks.md)
(T001–T031) · **Contexto**: [000-base-comun](../000-base-comun/) (plan, data-model, contrato y
quickstart comunes), spec-001 implementada (`backend/src/rutas/citas.rs`, `/citas`), los planes de
la Persona B (003, 005, 006, 007) y el de spec-008 · **Constitución**: 1.2.1.

Análisis de solo lectura: no se ha corregido nada. Las decisiones se registran en
[hallazgos.md](hallazgos.md).

## Specification Analysis Report

| ID | Category | Severity | Location(s) | Summary | Recommendation |
|----|----------|----------|-------------|---------|----------------|
| C1 | Constitution | CRITICAL | spec.md:169 (SC-002) · quickstart.md (fila SC-002) · plan.md:63 (Constitution Check IV) · research D8 · T009, T029 | **SC-002 no se puede comprobar usando la aplicación** (principio IV: «Cada criterio de éxito de una spec DEBE poder comprobarse usando la aplicación»). Pide «Tras abrir 10 órdenes seguidas, sus números son consecutivos», pero los datos demo de D8 solo dan dos citas de hoy que acaban en orden (`1313MMM`, `1414NNN`) y una tercera que se rechaza (`8888KKK`). El quickstart de 002 deja «las 10 seguidas» a las pruebas automáticas, y el Constitution Check del plan marca IV con ✅ sin decirlo. | Elegir entre: (a) reescribir SC-002 en un clarify para que sea comprobable en la app con las órdenes del §2 (p. ej., «las órdenes abiertas seguidas, también con errores entre medias, tienen números consecutivos sin huecos ni repetidos», que el §2 ya comprueba con la 10 y la 11 tras los errores del paso 3); o (b) añadir diez citas de hoy a la parte 5 de `demo.sql` y un paso al §2. La (a) es la más simple (principio I). Después, corregir el Constitution Check del plan. |
| I1 | Inconsistency | MEDIUM | data-model común:110 · 008/plan.md:47 · 002 T016, T019 | Dos definiciones de `retrasada`. El data-model común (y spec-002, T016 y T019) dice «estado ≠ `facturada` y `fecha_entrega_prometida` < hoy»; el plan de spec-008 dice solo «`fecha_entrega_prometida < fechas::hoy()`» y añade que «se calcula en el backend para que el criterio sea el mismo en todas las pantallas». En el tablero da igual (no salen facturadas), pero en `/ordenes` de 002 una orden facturada con entrega pasada (la demo nº 9) saldría «Retrasada» según 008 y no según el data-model. | Dejar el data-model común como fuente única y alinear el plan de spec-008 con él (los dos son de la Persona A). |
| I2 | Inconsistency | MEDIUM | 002 plan.md (Reglas clave; «spec-008 usa … `OrdenResumen`») · research D6 · T019 · 008/plan.md (Project Structure) | El plan de 002 dice que spec-008 reutiliza `OrdenResumen` y la consulta de `ordenes.rs` (T019 la hace `pub` «para que la reutilicen `GET /api/ordenes` y spec-008»), pero el plan de spec-008 solo nombra `tablero.rs` y no menciona esa reutilización. Si 008 escribe su propia consulta, `OrdenResumen` (y `retrasada`, I1) puede divergir entre pantallas. | Anotar en el plan de spec-008 que usa la consulta y el tipo `OrdenResumen` de `ordenes.rs`, o decidir explícitamente lo contrario. |
| E1 | Coverage | MEDIUM | T016, T021, T025 · plan.md:142, :195 · contrato común (`GET /api/ordenes?mias=true`) | **Tareas sin requisito propio en spec-002**: la lista `GET /api/ordenes` y la pantalla `/ordenes` («Órdenes» y «Mis órdenes») no salen de ningún FR ni escenario de spec-002; las tareas las atribuyen a US1-3 y SC-004, que se cumplen ya con la ficha y el enlace de `/citas`. Vienen del contrato común (sección spec-002) y del menú común, y las necesitan los quickstart de 003, 005 y 008 («Como `luis`, abrir la orden…»). El plan las justifica en el Constitution Check III, pero la spec no las nombra. | Decidir si se mantienen con esa justificación (y escribirla en las tareas, en lugar de US1-3) o si se trasladan a la spec con un escenario (p. ej., en un clarify), para que el principio III quede trazado. |
| I3 | Inconsistency | MEDIUM | 001/contracts/api.md:60 · 001/quickstart.md:11–12 · backend/seed/demo.sql:34 · T001, T022, T023 | **Documentos de spec-001 que quedan desfasados y sin tarea**: el contrato de 001 dice «`orden_id` no se muestra como dato», pero T023 muestra «Orden Nº N» en `/citas` y T022 añade `orden_numero`; el quickstart de 001 dice «Las citas demo son de días pasados … los vehículos demo no tienen ninguna cita pendiente», y el comentario de la parte 3 de `demo.sql` lo repite, pero la parte 5 (T001) crea citas de hoy, que cuentan como pendientes (`1313MMM`, `1414NNN`, `8888KKK`). | Añadir a T022/T023 (o a una tarea propia) la actualización del contrato y del quickstart de spec-001, y a T001 la del comentario de la parte 3 de `demo.sql`. |
| G1 | Shared files | LOW | plan maestro (000/plan.md:140–143) · 002 plan.md (Ficheros compartidos) · research D8 · T001 | El plan maestro enumera los ficheros compartidos («la migración, `estados.rs`, `dinero.rs`, `matricula.rs`, `rutas/mod.rs`, `tipos.ts`, `formato.ts`, `componentes/` y `ordenes/[id]`») y **no incluye `backend/seed/demo.sql`**, que spec-002 trata como compartido (T001) y que usan los quickstart de todas las specs. Es el mismo caso que el hallazgo I5 de spec-001 con `matricula.rs`. | Añadir `backend/seed/demo.sql` a la lista del plan maestro (revisa la Persona B; se puede incluir en T031). |
| A1 | Ambiguity | LOW | contrato común (tabla de validación de `POST /api/ordenes`) · T003 (`km: number \| string`) · T012 | El contrato no fija el tipo JSON de cada campo de la petición ni qué pasa si falta `cita_id` o `mecanico_id`. T003 tipa `km` como `number \| string` y T012 espera 422 con `"abc"` (no el 400 de «JSON mal formado»), lo que obliga a leer `km` como valor libre; un `cita_id` ausente podría dar 400 o 404. | Escribir en el contrato: `km` puede llegar como número o como texto con cifras; sin `cita_id` → 404 «No existe la cita»; sin `mecanico_id` → 422 «Elige un mecánico de la lista». Ajustar T012 y T017. |
| A2 | Ambiguity | LOW | T010 | La prueba de concurrencia deja abierto cómo se repite («borrando la orden y restando 1 al contador entre vueltas, **o** con una cita nueva por vuelta»). Restar 1 al contador a mano mezcla la preparación con lo que se comprueba (el contador). | Fijar una sola forma: una cita nueva por vuelta y comprobar que el contador avanza exactamente 1 en cada vuelta. |
| O1 | Ordering | LOW | T017, T019, T007 | T017 (POST) «responde 201 con el objeto `Orden` (T019)», pero T019 va después; la parte de la respuesta de T007 depende de T019. El orden se entiende, pero no está en «Dependencies». | Decir en T017 que, hasta T019, responde con lo mínimo (`id`, `numero`, `estado`), o mover la construcción de `Orden` antes, y anotarlo en «Within Each User Story». |
| E2 | Coverage | LOW | spec US1-6 · T011 (Cubre US1-6) · T029 | US1-6 («cliente sin cita») no tiene prueba propia en spec-002: lo que describe (no aparece en la lista de hoy y la página pública ofrece días desde mañana) lo hacen spec-001 FR-006 y su prueba 11. T011 lo declara cubierto, pero prueba citas de mañana y de ayer, no un vehículo sin cita. Solo lo comprueba el paso 1 del §2. | Quitar US1-6 del «Cubre» de T011 y anotar en la tabla de trazabilidad que US1-6 se cubre con spec-001 (prueba 11) y con el quickstart §2.1. |
| I4 | Dependency (007) | LOW | T026 · 007/plan.md:25, :43 | T026 enumera las secciones del contenedor (`Datos`, `Diagnostico`, `Presupuesto`, `Imputaciones`, `Pedidos`) y define `puedeEditar` = mecánico asignado o jefe. spec-007 añadirá `Factura.svelte`, cuyo botón es de **recepción**: con la regla de T026 no tendría un permiso adecuado sin cambiar el contenedor. | Dejar escrito en T026 que el contenedor calcula también el rol del usuario (o lo pasa) y que spec-007 añade su sección con una línea, con la revisión de la Persona A. |
| I5 | Dependency (003/004) | LOW | quickstart común §4.3, §4.5 (000/quickstart.md:117, :119) · contrato `PresupuestoResumen` | Fuera del alcance de spec-002, pero lo destapa su ficha: (a) §4.3 espera ver en la orden «Aceptado el … a las …», y `PresupuestoResumen` (lo que la ficha recibe de spec-003) no trae `respondido_en`; (b) §4.5 emite un presupuesto en «la orden de 1414NNN», pero ningún paso anterior le escribe un diagnóstico (ya pasaba con «la otra orden»). | Registrar para spec-004 (Persona A) y spec-003 (Persona B); revisarlo en T031 junto con sus secciones del quickstart. |

### Coverage Summary Table

| Requirement Key | Has Task? | Task IDs | Notes |
|-----------------|-----------|----------|-------|
| FR-001 (orden desde la cita, solo el día de la cita) | ✅ | T004, T006, T011, T017, T023, T024, T029 | |
| FR-002 (número correlativo sin huecos, también a la vez) | ✅ | T008, T009, T010, T017, T029 | |
| FR-003 (km entero ≥ 0, daños obligatorios) | ✅ | T012, T017, T024 | |
| FR-004 (km ≥ visita anterior, con el valor previo) | ✅ | T027, T028 | |
| FR-005 (vinculada a cita, vehículo y mecánico activo) | ✅ | T007, T013, T014, T017, T019 | |
| FR-006 (fecha de entrega prometida) | ✅ | T007, T012, T017 | |
| FR-007 (estado «recibida») | ✅ | T007, T017, T018 | T018 publica el evento |
| FR-008 (una orden por cita; «Orden Nº N»; 409) | ✅ | T010, T015, T017, T022, T023, T024 | |
| US1-1 … US1-5, US1-7, US1-8 | ✅ | T006–T026 | |
| US1-6 (cliente sin cita) | ⚠️ | T011, T029 | Ver E2: lo cubre spec-001 |
| US2-1, US2-2, US2-3 | ✅ | T027, T028 | |
| SC-001 (< 2 min) | ✅ | T029 (cronometrado) | |
| SC-002 (10 órdenes consecutivas) | ⚠️ | T009 (automática) | Ver C1: no comprobable en la app |
| SC-003 (km menor siempre rechazado) | ✅ | T027, T028, T029 | |
| SC-004 (la orden muestra cita, vehículo y mecánico) | ✅ | T014, T019, T026 | |

**Constitution Alignment Issues**: C1 (principio IV, SC-002). E1 roza el principio III (lista de
órdenes sin requisito en la spec), pero tiene justificación en el contrato y el menú comunes; se
deja como MEDIUM para decidir dónde se escribe.

**Unmapped Tasks**: ninguna sin «Cubre». T016, T021 y T025 están mapeadas a US1-3 y SC-004 de
forma débil (E1). T005 es infraestructura de pruebas, sin requisito propio, como T005 de spec-001.

**Encaje con la base común, spec-001 y las specs de la Persona B** (sin hallazgo):

- Las funciones que usa la ficha existen con la firma que dice D6:
  `presupuestos::presupuestos_de_orden(&mut PgConnection, i64, Decimal)`,
  `imputaciones::resumen_imputaciones(&mut PgConnection, i64)` y
  `almacen::pedidos_de_orden(&mut PgConnection, i64)`. Los planes de 003, 005 y 006 dicen que la
  ficha es de spec-002 y que sus secciones esperan los campos que T026 les pasa.
- Los componentes de B (`Diagnostico`, `Presupuesto`, `Imputaciones`, `Pedidos`) reciben
  exactamente las props que describe T026. Los tipos `OrdenResumen`, `PresupuestoResumen`, `Linea`,
  `Pedido` y `ResumenImputaciones` ya existen en `tipos.ts`.
- No hay rutas solapadas: B usa `/ordenes/{id}/diagnostico`, `/presupuestos`, `/imputaciones` y
  `/terminar`; spec-002 añade `/ordenes` y `/ordenes/{id}`.
- Los bloqueos de D2 (fila de la cita y después `contador('orden')`) no se cruzan con los de spec-001
  (consultivos, R6) ni con el `contador('factura')` de spec-007.
- `orden_numero` en `GET /api/citas` no rompe las pruebas de spec-001, que comprueban campos
  concretos y no el objeto entero.
- Los textos de error coinciden en spec, contrato común, quickstart y tareas.

### Metrics

- **Total Requirements**: 23 (8 FR, 4 SC, 11 escenarios: US1-1 … US1-8, US2-1 … US2-3).
- **Total Tasks**: 31 (Setup 3, Foundational 2, US1 21, US2 2, Polish 3).
- **Coverage %**: 100 % de los requisitos con al menos una tarea; 2 con cobertura débil (SC-002,
  C1; US1-6, E2).
- **Ambiguity Count**: 2 (A1, A2).
- **Duplication Count**: 0.
- **Critical Issues Count**: 1 (C1).
- **Hallazgos por gravedad**: CRITICAL 1 · HIGH 0 · MEDIUM 4 (I1, I2, E1, I3) · LOW 7 (G1, A1, A2,
  O1, E2, I4, I5).

## Next Actions

- **Hay un CRITICAL (C1)**: resolverlo antes de `/speckit-implement`. Lo más simple es un
  `/speckit-clarify` de spec-002 centrado en SC-002, y después corregir el Constitution Check del
  plan y la fila SC-002 del quickstart de 002.
- **MEDIUM**: I1 e I2 se resuelven editando el plan de spec-008 (Persona A); E1, decidiendo dónde
  se justifica la lista de órdenes; I3, añadiendo a las tareas la actualización de los documentos
  de spec-001 y del comentario de `demo.sql`.
- **LOW**: se pueden corregir editando `tasks.md` y el contrato común (A1, A2, O1, E2, I4) y el
  plan maestro (G1, con revisión de B); I5 se registra para spec-003 y spec-004.

¿Quieres que proponga las ediciones concretas para los hallazgos C1 e I1–I3?
