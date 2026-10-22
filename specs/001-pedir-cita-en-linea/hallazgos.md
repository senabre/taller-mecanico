# Seguimiento de hallazgos · spec-001 Pedir cita en línea (HU-01)

**Origen**: [analyze-1.md](analyze-1.md) (2026-10-22). Las decisiones son del 2026-10-22.

**Decisión**: *corregido* = se ha cambiado el documento o la tarea; *justificado* = no se cambia,
con un motivo explícito; *falso positivo* = el hallazgo no se sostiene; *registrado* = se
resuelve fuera de esta spec.

**Estado**: *Cerrado (pendiente de verificar en analyze-2)* = cambio aplicado y todavía sin
comprobar en un nuevo análisis; *Cerrado (pendiente de revisión de B)* = además toca documentos
comunes y debe revisarlo la Persona B antes de fusionar; *Abierto (spec-002)* = se decide en el
clarify de spec-002; *Asignado a spec-007* = lo resuelve esa spec; *Cerrado (justificado)* y
*Cerrado (falso positivo)* = no hay nada más que hacer.

## Hallazgos de analyze-1

| ID | Historia o criterio | Gravedad | Evidencia | Decisión | Responsable | Estado |
|----|---------------------|----------|-----------|----------|-------------|--------|
| I1 | FR-007, FR-009 · US1-8, US3-1, US3-2 · pruebas 8, 9 y 12 (T014, T026, T027) | HIGH | La prueba 12 creaba por `POST` una cita para `5678CDF` (matrícula nueva), algo que no existe hasta T024 (US2), aunque el checkpoint de US1 decía que pasaba. Las pruebas 8 y 9 no decían de dónde salían sus vehículos. | Corregido: las tres pruebas crean sus vehículos por SQL con `insertar_vehiculo` (T005) y solo piden las citas por `POST`. US3 deja escrito que no depende de US2 (tasks T014, T026, T027, «Phase Dependencies»; plan, pruebas 8, 9 y 12). | Persona A | Cerrado (pendiente de verificar en analyze-2) |
| I2 | spec-002 Assumptions («si un cliente llega sin cita…») frente a spec-001 FR-006 y FR-010 | HIGH | Con FR-006, recepción no puede registrar una cita para hoy, así que un cliente sin cita no puede entrar el mismo día; FR-010 también lo impide si el vehículo ya tiene otra cita pendiente. | Registrado: se decide en el clarify de spec-002, que puede proponer cambiar FR-006 de spec-001 (citas para hoy desde recepción) o quitar esa suposición. No bloquea la implementación de spec-001. | Persona A (clarify de spec-002) | Abierto (spec-002) |
| I3 | FR-004a, FR-007, FR-010 · contrato común, tabla de validación · research D2 y D3 · plan · T008, T016, T024 | MEDIUM | El contrato decía «comprueba en este orden y responde el primer error», pero el plan y las tareas validaban los datos del vehículo nuevo después de los 409: con el día lleno y un teléfono incorrecto se respondía 409. | Corregido: antes de los bloqueos se busca el vehículo y, si es nuevo, se validan sus datos (422). Después se toman los bloqueos y se vuelve a buscar; si ya existe, se aplica D4. El contrato común dice ahora «Todos los 422 llegan antes que cualquier 409». Actualizados el plan (reglas clave y prueba 4), research D2, D3 y D4, el contrato común y el de 001, y las tareas T008, T016, T021 (caso del día lleno), T024 y «Within Each User Story». | Persona A; revisa Persona B (contrato común) | Cerrado (pendiente de revisión de B) |
| I4 | spec-007 · `GET /api/vehiculos/{matricula}/historial` · contrato común · research D1 · T001 | MEDIUM | Tras T001, el historial responde 422 con matrículas de 1 carácter o de más de 10, y ni el contrato de spec-007 ni su spec lo recogen (tampoco el 422 que ya existía para `12#4`). | Registrado: la Persona B añade al contrato de spec-007 el 422 «La matrícula no es válida» y a su spec un caso límite con lo que ve el usuario en `/historial`. spec-001 no toca documentos de spec-007. | Persona B | Asignado a spec-007 |
| A1 | SC-001, SC-002 · quickstart común §1 · T032 | MEDIUM | Ningún paso medía los tiempos de menos de 1 y menos de 3 minutos; T032 los citaba sin decir cómo comprobarlos (principio IV). | Corregido: nuevo paso 10 en el quickstart común §1, que cronometra con `2222CCC` (< 1 min, SC-001) y con `9012DFG` (< 3 min, SC-002). Está en la correspondencia de [quickstart.md](quickstart.md) y en T032 («pasos 1 a 10», anotar los tiempos). | Persona A; revisa Persona B (quickstart común) | Cerrado (pendiente de revisión de B) |
| I5 | Plan maestro, ficheros compartidos · plan de 001, Scale/Scope | LOW | El plan maestro no incluía `matricula.rs` entre los ficheros compartidos, y el plan de 001 decía «1» fichero compartido aunque T004 cambia `tipos.ts`. | Corregido: el plan maestro añade `matricula.rs` y `rutas/mod.rs` a la lista de ficheros compartidos. El plan de 001 habla de 3 ficheros compartidos (`matricula.rs`, `tipos.ts`, `rutas/mod.rs`). | Persona A; revisa Persona B (plan maestro) | Cerrado (pendiente de revisión de B) |
| I6 | spec-002 US1 («el día de la cita») · quickstart común §2.1–§2.3 · T034 | LOW | Después del §1, todas las citas son de días futuros, pero el §2 abre órdenes sobre ellas enseguida. Ya pasaba antes del clarify. | Registrado: en el clarify de spec-002, decidir si una orden se puede abrir sobre una cita de otro día o solo el mismo día, y ajustar el §2. Va junto con I2. | Persona A (clarify de spec-002) | Abierto (spec-002) |
| I7 | T031 · commit `2494cfd` | LOW | T031 pedía revisar documentos comunes que el mensaje del commit da por revisados. | Falso positivo: la Persona B todavía no ha revisado `2494cfd`; el mensaje del commit anticipa una revisión pendiente. T031 se mantiene tal cual. | Persona A | Cerrado (falso positivo) |
| A2 | Caso límite «no hay ningún día disponible» · FR-006, FR-007 · T030 | LOW | El mensaje no tenía texto fijo; T030 dejaba la redacción a quien lo implemente. | Corregido: texto fijo «No quedan fechas libres en los próximos 30 días. Llama al taller para pedir cita.» en la spec (casos límite), el plan, research D6, el contrato de 001 y T030. | Persona A | Cerrado (pendiente de verificar en analyze-2) |
| A3 | FR-006 · prueba 11 (T006) | LOW | La prueba 11 solo rechazaba `hoy + 31`; un error de uno en el límite pasaría las pruebas. | Corregido: la prueba 11 comprueba también que el último día de lunes a viernes ≤ `hoy + 30` se ofrece y no da el 422 de fecha (T006 y plan, prueba 11). | Persona A | Cerrado (pendiente de verificar en analyze-2) |
| T1 | FR-002, FR-004 · T015 · contrato común | LOW | T015 fijaba el 404 «No hay ningún vehículo con esa matrícula», que no estaba en la spec ni en el contrato (principio III). | Corregido: el texto pasa al contrato común (`GET /api/publico/vehiculos/{matricula}`), y T015 y el plan remiten a él. El formulario no lo muestra. | Persona A; revisa Persona B (contrato común) | Cerrado (pendiente de revisión de B) |
| T2 | Trazabilidad · T005, T032, T033 | LOW | T032 usaba el rango «FR-001…FR-010» (sin dejar claro si incluía FR-004a); T033 y T005 no tenían ningún requisito concreto. | Corregido: T032 enumera todos los FR (con FR-004a), los escenarios y SC-001…SC-004; T033 relaciona cada FR con sus pruebas; T005 queda como infraestructura de pruebas sin requisito propio. | Persona A | Cerrado (pendiente de verificar en analyze-2) |
| G1 | FR-007, FR-010 · research R6 del plan maestro · research D3 · T008 | LOW | La forma de las claves de `pg_advisory_xact_lock` solo estaba en T008; un bloqueo futuro de otra spec podría coincidir sin que nadie lo supiera. | Corregido: R6 del plan maestro tiene ahora una tabla, que es el registro único de claves (una clave = fecha de la cita, en días desde 2000-01-01; dos claves `(1, hashtext(matrícula))` = cita pendiente), y la regla de añadir allí cualquier bloqueo nuevo. T008 remite a R6. | Persona A; revisa Persona B (research común) | Cerrado (pendiente de revisión de B) |
| D1 | FR-004a · spec, data-model común, contrato común, research D2, T021 y T023 | LOW | Las reglas de teléfono y correo estaban escritas en cinco sitios. | Justificado: la tabla de validación del contrato común es la fuente única de las reglas de los campos y los demás documentos remiten a ella. La spec conserva la regla como criterio funcional, y T021 y T023 la necesitan para escribir los casos de prueba. Para que la remisión sea real, data-model común y research D2 enlazan ahora la tabla en lugar de repetir la regla. | Persona A | Cerrado (justificado) |

## Resumen

- **Cerrados, pendientes de verificar en analyze-2**: 4 (I1, A2, A3, T2)
- **Cerrados, pendientes de revisión de la Persona B** (tocan documentos comunes): 5 (I3, A1, I5,
  T1, G1)
- **Cerrados sin más trabajo**: 2 (I7 falso positivo, D1 justificado)
- **Abiertos en spec-002** (Persona A, clarify): 2 (I2, I6)
- **Asignados a spec-007** (Persona B): 1 (I4)
- Ninguno bloquea `/speckit-implement` de spec-001. Los cambios en documentos comunes se revisan
  en T031, junto con T001 y T004.
