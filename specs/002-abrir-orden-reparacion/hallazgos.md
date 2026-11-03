# Seguimiento de hallazgos · spec-002 Recibir el vehículo y abrir la orden de reparación (HU-02)

**Origen**: [analyze-1.md](analyze-1.md) (2026-11-03). Todavía no se ha decidido ni corregido
ninguno.

**Decisión**: *pendiente* = sin decidir; *corregido* = se ha cambiado el documento o la tarea;
*justificado* = no se cambia, con un motivo explícito; *falso positivo* = el hallazgo no se
sostiene; *registrado* = se resuelve fuera de esta spec.

**Estado**: *Abierto* = sin decisión; *Cerrado (…)* = con la decisión aplicada, indicando cómo se
verificó; *Asignado a spec-NNN* = lo resuelve esa spec.

## Hallazgos de analyze-1

| ID | Historia o criterio | Gravedad | Evidencia | Decisión | Responsable | Estado |
|----|---------------------|----------|-----------|----------|-------------|--------|
| C1 | SC-002 · principio IV de la constitución · plan (Constitution Check IV) · research D8 · quickstart de 002 · T009, T029 | CRITICAL | SC-002 pide comprobar que, tras abrir 10 órdenes seguidas, los números son consecutivos, pero los datos demo solo permiten abrir dos órdenes en la app (`1313MMM`, `1414NNN`); el quickstart de 002 deja «las 10 seguidas» a las pruebas automáticas y el plan marca el principio IV con ✅ sin decirlo. | Pendiente. Opciones: (a) reescribir SC-002 en un clarify para que se compruebe con las órdenes del §2 (con errores entre medias); (b) diez citas de hoy más en la parte 5 de `demo.sql` y un paso al §2. | Persona A (clarify de spec-002) | Abierto |
| I1 | US1-3, SC-004 · data-model común (`retrasada`) · plan de spec-008 · T016, T019 | MEDIUM | El data-model común define `retrasada` = «estado ≠ `facturada` y `fecha_entrega_prometida` < hoy»; el plan de spec-008 la define solo como «`fecha_entrega_prometida < fechas::hoy()`». En `/ordenes`, una orden facturada con entrega pasada saldría distinta según quién la calcule. | Pendiente. Propuesta: el data-model común es la fuente única; alinear el plan de spec-008. | Persona A (plan de spec-008) | Abierto |
| I2 | SC-004 · research D6 · T019 · plan de spec-008 | MEDIUM | El plan de 002 dice que spec-008 reutiliza `OrdenResumen` y la consulta de `ordenes.rs`; el plan de spec-008 no lo menciona y solo nombra `tablero.rs`. | Pendiente. Propuesta: anotarlo en el plan de spec-008. | Persona A (plan de spec-008) | Abierto |
| E1 | US1-3, SC-004 · contrato común (`GET /api/ordenes?mias=true`) · menú común · T016, T021, T025 | MEDIUM | La lista de órdenes (`GET /api/ordenes`, `/ordenes`, «Mis órdenes») no sale de ningún FR ni escenario de spec-002; las tareas la atribuyen a US1-3 y SC-004, que ya cumplen la ficha y el enlace de `/citas`. Viene del contrato y el menú comunes y la usan los quickstart de 003, 005 y 008. | Pendiente. Opciones: justificarla en las tareas con esa procedencia, o llevarla a la spec con un escenario. | Persona A | Abierto |
| I3 | FR-008, US1-4 · contrato y quickstart de spec-001 · comentario de la parte 3 de `demo.sql` · T001, T022, T023 | MEDIUM | El contrato de 001 dice «`orden_id` no se muestra como dato» y su quickstart dice que las citas demo son de días pasados y que ningún vehículo demo tiene cita pendiente; con T001, T022 y T023 deja de ser cierto. Ninguna tarea los actualiza. | Pendiente. Propuesta: añadirlo a T022/T023 y a T001. | Persona A | Abierto |
| G1 | Plan maestro (ficheros compartidos) · research D8 · T001 | LOW | El plan maestro no incluye `backend/seed/demo.sql` en su lista de ficheros compartidos, aunque spec-002 lo trata como compartido y lo usan los quickstart de todas las specs (mismo caso que I5 de spec-001). | Pendiente. Propuesta: añadirlo a la lista del plan maestro, con la revisión de B en T031. | Persona A; revisa Persona B | Abierto |
| A1 | FR-003, FR-005 · contrato común (tabla de validación) · T003, T012, T017 | LOW | El contrato no fija el tipo JSON de cada campo ni qué pasa si faltan `cita_id` o `mecanico_id`; T003 tipa `km` como `number \| string` y T012 espera 422 con `"abc"`. | Pendiente. Propuesta: escribir en el contrato los tipos aceptados y el error de cada campo ausente. | Persona A; revisa Persona B (contrato común) | Abierto |
| A2 | FR-008 · T010 | LOW | T010 deja dos formas de repetir la prueba de concurrencia, y una de ellas resta 1 al contador a mano, que es justo lo que se comprueba. | Pendiente. Propuesta: una cita nueva por vuelta y comprobar que el contador avanza 1 en cada una. | Persona A | Abierto |
| O1 | US1-1 · T007, T017, T019 | LOW | T017 responde «con el objeto `Orden` (T019)», que va después; la respuesta de T007 depende de T019 sin que lo diga «Dependencies». | Pendiente. Propuesta: respuesta mínima hasta T019, anotada en las dependencias. | Persona A | Abierto |
| E2 | US1-6 · T011 · spec-001 FR-006 (prueba 11) · T029 | LOW | US1-6 («cliente sin cita») no tiene prueba en spec-002: lo cumplen spec-001 FR-006 y su prueba 11. T011 lo declara cubierto, pero prueba citas de mañana y de ayer. | Pendiente. Propuesta: quitar US1-6 del «Cubre» de T011 y anotar que lo cubren spec-001 y el quickstart §2.1. | Persona A | Abierto |
| I4 | US1-3 · T026 · plan de spec-007 (`Factura.svelte`) | LOW | El contenedor de T026 solo calcula `puedeEditar` = mecánico asignado o jefe; la sección «Factura» de spec-007 es para recepción y necesitará otro permiso. | Pendiente. Propuesta: que el contenedor pase también el rol y que spec-007 añada su sección con una línea. | Persona A; revisa Persona B | Abierto |
| I5 | Quickstart común §4.3 y §4.5 · contrato `PresupuestoResumen` | LOW | (a) §4.3 espera ver en la orden «Aceptado el … a las …», pero `PresupuestoResumen` no trae `respondido_en`; (b) §4.5 emite un presupuesto en la orden de `1414NNN` sin que ningún paso le escriba antes un diagnóstico. Fuera del alcance de spec-002. | Pendiente. Propuesta: registrar para spec-004 (A) y spec-003 (B) y revisarlo en T031. | Persona A (spec-004) y Persona B (spec-003) | Abierto |

## Resumen

- **Abiertos**: 12 (CRITICAL 1: C1 · MEDIUM 4: I1, I2, E1, I3 · LOW 7: G1, A1, A2, O1, E2, I4, I5).
- C1 bloquea `/speckit-implement` de spec-002 (principio IV). Los demás no lo bloquean, pero I3 y
  E1 conviene decidirlos antes de empezar las tareas que tocan.
