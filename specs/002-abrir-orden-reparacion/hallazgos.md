# Seguimiento de hallazgos · spec-002 Recibir el vehículo y abrir la orden de reparación (HU-02)

**Origen**: [analyze-1.md](analyze-1.md) (2026-11-03) · **Revisión**: [analyze-2.md](analyze-2.md)
(2026-11-03). Decisiones del 2026-11-03.

**Decisión**: *pendiente* = sin decidir; *corregido* = se ha cambiado el documento o la tarea;
*justificado* = no se cambia, con un motivo explícito; *falso positivo* = el hallazgo no se
sostiene; *registrado* = se resuelve fuera de esta spec.

**Estado**: *Abierto* = sin decisión; *Cerrado (corregido; pendiente de verificar en analyze-2)* =
cambio aplicado y todavía sin comprobar en un nuevo análisis; *Cerrado (corregido; revisión de B en
T031)* = además toca documentos comunes y la Persona B lo revisa en T031; *Cerrado (corregido en
las tareas; …)* = la corrección está escrita en las tareas y se aplica al hacerlas; *Cerrado
(justificado)* = no se cambia, con su motivo; *Asignado a spec-NNN* = lo resuelve esa spec;
*Cerrado (verificado en analyze-2)* = analyze-2 ha comprobado el cambio en todos los documentos;
*Cerrado (verificado en analyze-2; revisión de B en T031)* = comprobado, pero toca documentos
comunes y la revisión de la Persona B se da por hecha solo cuando se marque T031.

## Hallazgos de analyze-1

| ID | Historia o criterio | Gravedad | Evidencia | Decisión | Responsable | Estado |
|----|---------------------|----------|-----------|----------|-------------|--------|
| C1 | SC-002 · principio IV de la constitución · plan (Constitution Check IV) · research D8 · quickstart de 002 · T009, T029 | CRITICAL | SC-002 pide comprobar que, tras abrir 10 órdenes seguidas, los números son consecutivos, pero los datos demo solo permiten abrir dos órdenes en la app (`1313MMM`, `1414NNN`); el quickstart de 002 deja «las 10 seguidas» a las pruebas automáticas y el plan marca el principio IV con ✅ sin decirlo. | Corregido (2026-11-03): SC-002 reescrito en el clarify (Q6 de [decisiones-clarify](decisiones-clarify.md)) para comprobarse en la app con las órdenes 10 y 11 del quickstart §2 y los intentos rechazados entre medias; escenario 9 de la US1 nuevo. Actualizados el Constitution Check IV del plan, la lista de pruebas del plan, el quickstart de 002 (SC-002 y US1-9), T008, T009, T029 y T030. La apertura simultánea queda en FR-002 y en las pruebas automáticas. | Persona A | Cerrado (verificado en analyze-2) |
| I1 | US1-3, SC-004 · data-model común (`retrasada`) · plan de spec-008 · T016, T019 | MEDIUM | El data-model común define `retrasada` = «estado ≠ `facturada` y `fecha_entrega_prometida` < hoy»; el plan de spec-008 la define solo como «`fecha_entrega_prometida < fechas::hoy()`». En `/ordenes`, una orden facturada con entrega pasada saldría distinta según quién la calcule. | Corregido (2026-11-03): la definición es la del data-model común (orden no facturada con `fecha_entrega_prometida` anterior a hoy), que es la fuente única. Corregido el plan de spec-008 (reglas clave). | Persona A (plan de spec-008) | Cerrado (verificado en analyze-2) |
| I2 | SC-004 · research D6 · T019 · plan de spec-008 | MEDIUM | El plan de 002 dice que spec-008 reutiliza `OrdenResumen` y la consulta de `ordenes.rs`; el plan de spec-008 no lo menciona y solo nombra `tablero.rs`. | Corregido (2026-11-03): el plan de spec-008 dice ahora que las tarjetas usan `OrdenResumen` y su consulta de `ordenes.rs` (spec-002), sin consulta propia, y añade spec-002 a sus dependencias. | Persona A (plan de spec-008) | Cerrado (verificado en analyze-2) |
| E1 | US1-3, SC-004 · contrato común (`GET /api/ordenes?mias=true`) · menú común · T016, T021, T025 | MEDIUM | La lista de órdenes (`GET /api/ordenes`, `/ordenes`, «Mis órdenes») no sale de ningún FR ni escenario de spec-002; las tareas la atribuyen a US1-3 y SC-004, que ya cumplen la ficha y el enlace de `/citas`. Viene del contrato y el menú comunes y la usan los quickstart de 003, 005 y 008. | Justificado (2026-11-03): la lista de órdenes la exigen el menú común (escenario 1 de la US1 de la base común, «Mis órdenes»; quickstart §0.4, «Órdenes»; el jefe, por FR-002 común) y el quickstart §3, y se implementa en spec-002 porque es la spec que crea las órdenes. Anotado en el plan (Constitution Check III y regla de `GET /api/ordenes`) y en T016, T021 y T025, cuyo «Cubre» ya no atribuye la lista a US1-3 ni SC-004. | Persona A | Cerrado (justificado; verificado en analyze-2) |
| I3 | FR-008, US1-4 · contrato y quickstart de spec-001 · comentario de la parte 3 de `demo.sql` · T001, T022, T023 | MEDIUM | El contrato de 001 dice «`orden_id` no se muestra como dato» y su quickstart dice que las citas demo son de días pasados y que ningún vehículo demo tiene cita pendiente; con T001, T022 y T023 deja de ser cierto. Ninguna tarea los actualiza. | Corregido en las tareas (2026-11-03): T001 actualiza el comentario de la parte 3 de `demo.sql` y el quickstart de spec-001 («Antes de empezar»); T022 y T023, el contrato de spec-001 (`orden_id`/`orden_numero` y la columna «Orden»). Los documentos se cambian al hacer esas tareas. | Persona A | Cerrado (verificado en analyze-2 en las tareas; se aplica con T001, T022 y T023) |
| G1 | Plan maestro (ficheros compartidos) · research D8 · T001 | LOW | El plan maestro no incluye `backend/seed/demo.sql` en su lista de ficheros compartidos, aunque spec-002 lo trata como compartido y lo usan los quickstart de todas las specs (mismo caso que I5 de spec-001). | Corregido (2026-11-03): `backend/seed/demo.sql` añadido a la lista de ficheros compartidos del plan maestro; añadido a la tabla de ficheros compartidos del plan de 002 y a T031. | Persona A; revisa Persona B | Cerrado (verificado en analyze-2; revisión de B en T031) |
| A1 | FR-003, FR-005 · contrato común (tabla de validación) · T003, T012, T017 | LOW | El contrato no fija el tipo JSON de cada campo ni qué pasa si faltan `cita_id` o `mecanico_id`; T003 tipa `km` como `number \| string` y T012 espera 422 con `"abc"`. | Corregido (2026-11-03): el contrato común dice ahora que `km` se acepta como número o como texto con cifras y que un campo ausente o de otro tipo da el error de su fila (sin `cita_id` → 404; sin los demás → su 422), nunca el 400. Ajustados T012 (casos nuevos) y T017 (lectura de los campos como valores libres); T031 lo incluye en la revisión. | Persona A; revisa Persona B (contrato común) | Cerrado (verificado en analyze-2; revisión de B en T031) |
| A2 | FR-008 · T010 | LOW | T010 deja dos formas de repetir la prueba de concurrencia, y una de ellas resta 1 al contador a mano, que es justo lo que se comprueba. | Corregido (2026-11-03): T010 usa una cita de hoy nueva en cada vuelta, sin borrar órdenes ni tocar el contador, y comprueba que el contador avanza exactamente 1 por vuelta. | Persona A | Cerrado (verificado en analyze-2) |
| O1 | US1-1 · T007, T017, T019 | LOW | T017 responde «con el objeto `Orden` (T019)», que va después; la respuesta de T007 depende de T019 sin que lo diga «Dependencies». | Corregido (2026-11-03): T017 responde provisionalmente `{id, numero, estado}` hasta T019, que pone el objeto `Orden` completo; anotado en «Within Each User Story». | Persona A | Cerrado (verificado en analyze-2) |
| E2 | US1-6 · T011 · spec-001 FR-006 (prueba 11) · T029 | LOW | US1-6 («cliente sin cita») no tiene prueba en spec-002: lo cumplen spec-001 FR-006 y su prueba 11. T011 lo declara cubierto, pero prueba citas de mañana y de ayer. | Corregido (2026-11-03): US1-6 sale del «Cubre» de T011; la nota de trazabilidad de `tasks.md` y T029 dicen que lo cubren spec-001 FR-006 (su prueba 11) y el paso 1 del quickstart común §2. | Persona A | Cerrado (verificado en analyze-2) |
| I4 | US1-3 · T026 · plan de spec-007 (`Factura.svelte`) | LOW | El contenedor de T026 solo calcula `puedeEditar` = mecánico asignado o jefe; la sección «Factura» de spec-007 es para recepción y necesitará otro permiso. | Corregido (2026-11-03): T026 y research D6 dicen que el contenedor tiene también el rol del usuario y que la sección «Factura» de spec-007 se activa para recepción y el jefe; spec-007 la añade con una línea, con la revisión de la Persona A. | Persona A; revisa Persona B | Cerrado (verificado en analyze-2) |
| I5 | Quickstart común §4.3 y §4.5 · contrato `PresupuestoResumen` | LOW | (a) §4.3 espera ver en la orden «Aceptado el … a las …», pero `PresupuestoResumen` no trae `respondido_en`; (b) §4.5 emite un presupuesto en la orden de `1414NNN` sin que ningún paso le escriba antes un diagnóstico. Fuera del alcance de spec-002. | Registrado (2026-11-03), fuera del alcance de spec-002: (a) §4.3 («Aceptado el … a las …» en la orden, sin `respondido_en` en `PresupuestoResumen`) se asigna a spec-004 para su clarify; (b) §4.5 (presupuesto en la orden de `1414NNN` sin diagnóstico previo) se asigna a spec-003. | (a) Persona A (clarify de spec-004); (b) Persona B (spec-003) | Asignado a spec-004 (§4.3) y a spec-003 (§4.5) (comprobado en analyze-2; ver N3) |

## Hallazgos nuevos de analyze-2

Todos son LOW y no se corrigen (decisión del 2026-11-03: los hallazgos bajos nuevos se justifican
o se asignan).

| ID | Historia o criterio | Gravedad | Evidencia | Decisión | Responsable | Estado |
|----|---------------------|----------|-----------|----------|-------------|--------|
| N1 | Plan de spec-008 (dependencias) · I2 | LOW | El plan de spec-008 añade spec-002 a sus dependencias, pero sigue diciendo «Se puede hacer desde el principio con las órdenes demo en varios estados», que ya no es cierto porque usa `OrdenResumen` de `ordenes.rs`. El orden del plan maestro (001 → 002 → 004 → 008) ya pone 008 después de 002. | Asignado: quitar la frase al generar las tareas de spec-008. | Persona A (spec-008) | Asignado a spec-008 |
| N2 | Plan de 002 («Cambios en documentos compartidos») · G1 | LOW | El párrafo nombra «el contrato, el data-model y el quickstart comunes» y no el plan maestro, que la tabla de ficheros compartidos sí incluye desde G1. | Justificado: el párrafo remite a «la tabla de arriba», que es la que manda, y T031 revisa el plan maestro. | Persona A | Cerrado (justificado) |
| N3 | I5 · specs 003 y 004 | LOW | La asignación de I5 solo consta en el registro de spec-002: spec-003 no tiene `hallazgos.md` y spec-004 aún no ha pasado por el clarify. | Asignado: la Persona A lo lleva al clarify de spec-004 (§4.3); la Persona B lo ve en T031, que revisa el §4 del quickstart (§4.5). | Persona A (spec-004) y Persona B (T031) | Asignado a spec-004 y a T031 |
| N4 | SC-002 · US1-9 · quickstart común §2 (pasos 3–6) · T008 | LOW | En la app, solo el rechazo del paso 3 va seguido de una orden nueva (la Nº 10); los de la segunda pestaña y del km menor van después de la última orden, así que en pantalla no se ve que tampoco gastan número. | Justificado: el paso 3 ya muestra en la app que un rechazo no gasta número, SC-002 no exige más, y T008 (prueba 2) lo comprueba con todos los tipos de rechazo. | Persona A | Cerrado (justificado) |

## Hallazgos encontrados durante la implementación

| ID | Historia o criterio | Gravedad | Evidencia | Decisión | Responsable | Estado |
|----|---------------------|----------|-----------|----------|-------------|--------|
| V1 | Lista de órdenes (`GET /api/ordenes`, `/ordenes`) · datos demo · [verificación de US1](evidencias/T029/verificacion.md), fila 14 | LOW | La lista se ordena por fecha de apertura, como dice el contrato, y en los datos demo los números no siguen el orden de las fechas (la orden 1 es de ayer y la 9 de hace 20 días): el navegador vio 11, 10, 1, 4, 5, 2, 3, 6, 8, 7, 9. | Justificado (2026-11-04): no afecta a ningún criterio. Es una propiedad de los datos demo, cuyas fechas se eligieron por otros motivos; con datos reales, número y fecha avanzan juntos. No se cambian ni la lista ni los datos demo. | Persona A | Cerrado (justificado) |

## Resumen

- **Verificados en analyze-2**: 8 (C1, I1, I2, A2, O1, E2, I4; E1, justificado)
- **Verificados en analyze-2, con revisión de la Persona B en T031** (documentos comunes): 2 (G1, A1)
- **Verificado en las tareas** (se aplica al hacerlas): 1 (I3)
- **Asignados**: I5 (spec-004 y spec-003), N1 (spec-008), N3 (spec-004 y T031)
- **Nuevos de analyze-2, justificados**: 2 (N2, N4)
- **Encontrados durante la implementación, justificados**: 1 (V1)
- Ninguno queda abierto ni bloquea `/speckit-implement` de spec-002. Antes de fusionar hay que
  completar T031 (revisión de la Persona B de lo compartido, incluidos G1 y A1).
