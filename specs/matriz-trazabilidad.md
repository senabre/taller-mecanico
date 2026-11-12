# Matriz de trazabilidad del grupo

**Qué es**: la matriz que pide el apartado 5.2 del enunciado. Cada fila relaciona un criterio de una
historia (un escenario de aceptación o un criterio de éxito de su spec) con la decisión del clarify o
del plan que lo concreta, las tareas que lo cubren, la prueba prevista y el resultado obtenido, con un
enlace a la evidencia. Otro integrante revisa las filas de cada alumno.

**Versión del código**: rama `entrega`, commit `892dd2a` (12/11/2026). Las evidencias enlazadas se
obtuvieron sobre commits anteriores de cada spec, que se indican en cada informe; la regresión del
quickstart común entero (§0–§8) se repitió sobre `02e9868`, con todo el código de las cuatro
historias de la Persona A ([T030 de spec-008](008-tablero-ordenes-abiertas/evidencias/T030/regresion/verificacion.md)).

## Cómo se lee

- **Criterio**: `USn-m` es el escenario *m* de la historia de usuario *n* de la spec; `SC-nnn`, un
  criterio de éxito. Hay una fila por cada escenario y cada SC de la spec, sin saltar ninguno.
- **Decisión**: `Qn` es la pregunta *n* del clarify (`decisiones-clarify.md` de la spec; en las specs
  003, 005, 006 y 007, el apartado «Clarifications» de su `spec.md`); `Dn`, la decisión *n* de su
  `research.md`. «—» si la spec ya lo decía y no hizo falta concretarlo.
- **Tareas**: las de `tasks.md` de la spec que declaran cubrirlo en su «Cubre:».
- **Prueba**: el nombre de la prueba automática (`backend/tests/specNNN_*.rs`, o el fichero de Vitest)
  y el paso del [quickstart común](000-base-comun/quickstart.md) que lo comprueba en la aplicación.
- **Resultado y evidencia**: lo que se obtuvo, con enlace al informe. «fila *n*» es la fila de la
  tabla de resultados de ese informe. Las verificaciones en el navegador se hicieron con un navegador
  automático que sigue los pasos de una persona; los criterios que hablan de una persona y un tiempo
  (cronometrajes) los midió una persona con un reloj.
- **Autor**: quien escribió la fila. **Revisor**: quien la revisa (apartado 5.2), y si ya lo ha hecho.

**Estados** (columna «Estado», independiente de la revisión):

| Estado | Significa |
|---|---|
| **Verificado** | La prueba prevista se ejecutó y pasó, y hay evidencia. |
| **Verificado con desviación** | Se comprobó y se cumple, pero no exactamente como estaba previsto; la fila explica la diferencia. |
| **Pendiente** | Falta la prueba o la evidencia; la fila explica qué falta. |

---

## HU-01 · Pedir cita en línea (spec-001)

Documentos: [spec](001-pedir-cita-en-linea/spec.md) ·
[clarify](001-pedir-cita-en-linea/decisiones-clarify.md) ·
[research](001-pedir-cita-en-linea/research.md) · [tareas](001-pedir-cita-en-linea/tasks.md).
Pruebas: `backend/tests/spec001_citas.rs` y las unitarias de `backend/src/matricula.rs`. Verificación
en el navegador: [T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md) (quickstart §1).

| Criterio | Decisión | Tareas | Prueba | Resultado y evidencia | Autor | Revisor | Estado |
|---|---|---|---|---|---|---|---|
| US1-1 · Una matrícula registrada muestra marca y modelo, sin datos del cliente | Q5, D4 | T009, T015, T032 | `prueba_2_vehiculo_conocido_se_reconoce` · §1.4, §1.6 | Pasa; en la app, «Seat Ibiza» sin datos de contacto ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), filas 9 y 12) | Persona A | Persona B (pendiente) | Verificado |
| US1-2 · Al confirmar, se ve la fecha (dd/mm/aaaa) y el motivo | — | T009, T016, T032 | `prueba_2_vehiculo_conocido_se_reconoce` · §1.3, §1.6 | Pasa; confirmación con fecha y motivo ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), filas 8 y 13) | Persona A | Persona B (pendiente) | Verificado |
| US1-3 · Recepción ve la cita con matrícula, marca, modelo, motivo, nombre y teléfono, sin correo | Q4, D5 | T014, T017, T032 | `prueba_12_lista_de_citas` · §1.7 | Pasa; columnas sin correo ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), fila 16) | Persona A | Persona B (pendiente) | Verificado |
| US1-4 · Con una cita pendiente: aviso «Este vehículo ya tiene una cita pendiente.», sin fecha y sin poder confirmar | Q3, D3 | T012, T015, T016, T032 | `prueba_7_una_sola_cita_pendiente`, `prueba_10_concurrencia_cita_pendiente` · §1.4 | Pasa; aviso y Confirmar desactivado ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), fila 9) | Persona A | Persona B, revisado el 12/11/2026 | Verificado |
| US1-5 · Matrícula inválida («12#4», «A», 11 caracteres) y límites válidos | Q1, D1 | T001, T010, T015, T032 | `prueba_3_matricula_invalida`; unitarias `matriculas_invalidas`, `limites_de_longitud` · §1.1 | Pasa; las tres inválidas rechazadas y «AB» aceptada ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), filas 1–4) | Persona A | Persona B (pendiente) | Verificado |
| US1-6 · Si la cita anterior ya pasó, se puede pedir otra | Q3, D3 | T012, T016, T032 | `prueba_7_una_sola_cita_pendiente` · §1.6 | Pasa; nueva cita de `1111BBB` ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), fila 13) | Persona A | Persona B (pendiente) | Verificado |
| US1-7 · Vehículo conocido: solo día y motivo; el propietario no cambia | Q5, D4 | T011, T016, T032 | `prueba_6_vehiculo_conocido_no_cambia` · §1.6, §1.7 | Pasa; sin campos de contacto y datos sin cambiar en la lista ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), filas 12 y 17) | Persona A | Persona B (pendiente) | Verificado |
| US1-8 · Recepción y jefe ven la lista de hoy por defecto, en el orden en que se pidieron | Q4, D5 | T014, T017, T032 | `prueba_12_lista_de_citas` · §1.7, §1.9 (`paco`) | Pasa ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), filas 15, 16 y 22) | Persona A | Persona B (pendiente) | Verificado |
| US1-9 · Un día sin citas: «No hay citas para este día» | Q4, D5 | T014, T017, T032 | `prueba_12_lista_de_citas` · §1.8 | Pasa ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), fila 18) | Persona A | Persona B (pendiente) | Verificado |
| US1-10 · Mecánico, almacén y sin sesión no ven la lista | Q4, D5 | T002, T014, T017, T032 | `prueba_12_lista_de_citas` · §1.9 | Pasa: 401 y 403 en la API; en la app, sin menú ni tabla ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), filas 19–21) | Persona A | Persona B (pendiente) | Verificado |
| US2-1 · Una matrícula nueva pide marca, modelo, nombre, teléfono y correo | D2 | T020, T025, T032 | `prueba_1_alta_completa` · §1.2 | Pasa; los cinco campos ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), fila 5) | Persona A | Persona B (pendiente) | Verificado |
| US2-2 · Se dan de alta el cliente, el vehículo y la cita | D2 | T020, T032 | `prueba_1_alta_completa` · §1.3 | Pasa ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), fila 8) | Persona A | Persona B (pendiente) | Verificado |
| US2-3 · Teléfono o correo incorrectos dan su mensaje | Q2, D2 | T021, T025, T032 | `prueba_4_telefono_y_correo` · §1.3 | Pasa; «12345» y «ana@correo» rechazados ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), filas 6 y 7) | Persona A | Persona B (pendiente) | Verificado |
| US2-4 · Teléfonos con prefijo, espacios o guiones y correo válido se aceptan | Q2, D2 | T021, T032 | `prueba_4_telefono_y_correo` · §1.3, §1.5 | Pasa; «+34 612 345 678» aceptado ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), fila 10) | Persona A | Persona B (pendiente) | Verificado |
| US3-1 · Un día completo deja de ofrecerse | D6 | T026, T028, T029, T030, T032, T041 | `prueba_8_cupo`, `con_cupo_0_no_se_ofrece_ningun_dia` · §1.5 | Pasa; el día lleno ya no aparece (fila 11). Con cupo 0 falló al principio (hallazgo H1, fila 32) y, tras T041, se repitió y pasa ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), «Repetición tras T041») | Persona A | Persona B (pendiente) | Verificado |
| US3-2 · Dos clientes a la vez por la última plaza: solo uno la consigue | D6 | T027, T029, T030 | `prueba_9_concurrencia_cupo` · pasos (b) de T032 | Pasa; en la app, la segunda pestaña ve el aviso y elige otro día ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), filas 25 y 26) | Persona A | Persona B (pendiente) | Verificado |
| SC-001 · Un cliente conocido pide cita en menos de 1 minuto | — | T032 | Cronometraje humano · §1.10 (`2222CCC`) | **47 s**, medido por una persona en Chrome en español ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), «Sobre SC-001 y SC-002») | Persona A | Persona B (pendiente) | Verificado |
| SC-002 · Un cliente nuevo pide cita en menos de 3 minutos | — | T032 | Cronometraje humano · §1.10 (`9012DFG`) | **1 min 27 s**, medido por una persona ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), «Sobre SC-001 y SC-002») | Persona A | Persona B (pendiente) | Verificado |
| SC-003 · Nunca hay más citas que el cupo | D6 | T027, T032, T033 | `prueba_8_cupo`, `prueba_9_concurrencia_cupo` · §1.5 | Pasa ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), fila 11) | Persona A | Persona B (pendiente) | Verificado |
| SC-004 · El 100 % de las citas confirmadas sale en la lista del día | Q4, D5 | T032 | `prueba_12_lista_de_citas` · §1.7 | Pasa; `1234BCD` y `5678CDF` en la lista ([T032](001-pedir-cita-en-linea/evidencias/T032/verificacion.md), fila 16) | Persona A | Persona B (pendiente) | Verificado |

---

## HU-02 · Abrir la orden de reparación (spec-002)

Documentos: [spec](002-abrir-orden-reparacion/spec.md) ·
[clarify](002-abrir-orden-reparacion/decisiones-clarify.md) ·
[research](002-abrir-orden-reparacion/research.md) · [tareas](002-abrir-orden-reparacion/tasks.md).
Pruebas: `backend/tests/spec002_ordenes.rs`. Verificación en el navegador:
[T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md) (quickstart §2).

| Criterio | Decisión | Tareas | Prueba | Resultado y evidencia | Autor | Revisor | Estado |
|---|---|---|---|---|---|---|---|
| US1-1 · Con una cita de hoy, la orden se guarda en «Recibida» con km, daños, mecánico y entrega | D2 | T007, T017, T019, T024, T029 | `prueba_1_abrir_y_publicar_evento` · §2.4 | Pasa; ficha de la Orden Nº 10 en «Recibida» ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), fila 9) | Persona A | Persona B (pendiente) | Verificado |
| US1-2 · La orden nueva recibe el número siguiente al último | D2 | T008, T017, T029 | `prueba_2_correlativo_sin_huecos` · §2.4, §2.5 | Pasa; Nº 10 y después Nº 11 ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), filas 9 y 12) | Persona A | Persona B (pendiente) | Verificado |
| US1-3 · La orden muestra su cita, su vehículo y su mecánico | D6 | T002, T003, T014, T019, T020, T026, T029 | `prueba_10_ficha` · §2.4 | Pasa ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), fila 9) | Persona A | Persona B (pendiente) | Verificado |
| US1-4 · Una cita con orden muestra «Orden Nº N» y no deja abrir otra | Q5, D2, D7 | T001, T003, T010, T015, T017, T022, T023, T024, T029 | `prueba_4_misma_cita_a_la_vez`, `prueba_11_lista_de_citas` · §2.4, §2.5 | Pasa; enlace «Orden Nº 10» y «Esta cita ya tiene una orden (Nº 11)» en la segunda pestaña ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), filas 11 y 13) | Persona A | Persona B (pendiente) | Verificado |
| US1-5 · Una cita de mañana o de ayer no ofrece «Abrir orden» | Q2, D1 | T001, T011, T017, T023, T029 | `prueba_5_solo_el_dia_de_la_cita` · §2.1 | Pasa, también abriendo por URL una cita de ayer ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), filas 2–4) | Persona A | Persona B (pendiente) | Verificado |
| US1-6 · Un cliente sin cita no puede tener orden | Q1, D1 | T001, T029 | `prueba_11_solo_dias_laborables_de_manana_a_30_dias` de spec-001 · §2.1 | Pasa: no está en la lista de hoy y la página pública solo da días desde mañana ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), fila 1). Lo cubre spec-001 FR-006, como decidió el hallazgo E2 | Persona A | Persona B (pendiente) | Verificado |
| US1-7 · Solo se ofrecen los mecánicos activos | Q3, D4 | T001, T013, T017, T024, T029 | `prueba_8_mecanico` · §2.2 | Pasa; Luis y Marta, sin Pedro ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), fila 5) | Persona A | Persona B (pendiente) | Verificado |
| US1-8 · Sin descripción de daños no se guarda | Q4, D4 | T012, T017, T024, T029 | `prueba_7_campos` · §2.3 | Pasa; mensaje bajo el campo ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), filas 6–8) | Persona A | Persona B (pendiente) | Verificado |
| US1-9 · Con intentos rechazados entre medias, los números siguen siendo consecutivos | Q6, D2 | T008, T029 | `prueba_2_correlativo_sin_huecos` · §2.3–§2.5 | Pasa; tras tres errores, Nº 10 y Nº 11 ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), filas 6–9 y 12) | Persona A | Persona B (pendiente) | Verificado |
| US2-1 · Un km menor que el de la visita anterior se rechaza, con ese valor | D3 | T001, T027, T028, T029 | `prueba_6_kilometraje` · §2.6 (`8888KKK`) | Pasa; «…visita anterior (39.800 km)» ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), «US2 y ancho de móvil») | Persona A | Persona B, revisado el 12/11/2026 | Verificado |
| US2-2 · Un km igual al anterior se acepta | D3 | T027, T028 | `prueba_6_kilometraje` | Pasa en la prueba automática; la guía no prevé paso en la app ([quickstart de 002](002-abrir-orden-reparacion/quickstart.md)) | Persona A | Persona B (pendiente) | Verificado |
| US2-3 · Un vehículo sin visitas anteriores acepta cualquier km | D3 | T001, T027, T028, T029 | `prueba_6_kilometraje` · §2.4 (`1313MMM`), §2.5 (`1414NNN`) | Pasa ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), filas 9 y 12) | Persona A | Persona B (pendiente) | Verificado |
| SC-001 · La recepcionista abre una orden en menos de 2 minutos | — | T001, T024, T029 | Cronometraje humano · §2.4 | **31 s**, medido por una persona en Chrome en español ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), «Comprobación humana») | Persona A | Persona B (pendiente) | Verificado |
| SC-002 · Órdenes seguidas con números consecutivos, sin huecos ni repetidos, aunque haya rechazos | Q6, D2 | T008, T009, T029, T030 | `prueba_2_correlativo_sin_huecos`, `prueba_3_concurrencia_del_numero` · §2.3–§2.6 | Pasa; tras el rechazo del km se guarda la Nº 10: el rechazo no gastó número ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), filas 9 y 12 y «US2 y ancho de móvil») | Persona A | Persona B (pendiente) | Verificado |
| SC-003 · El 100 % de los km menores se rechazan | D3 | T027, T028, T029, T030 | `prueba_6_kilometraje` · §2.6 | Pasa ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), «US2 y ancho de móvil») | Persona A | Persona B (pendiente) | Verificado |
| SC-004 · Toda orden muestra su cita, su vehículo y su mecánico | D6 | T002, T014, T019, T020, T026, T029, T030, T031 | `prueba_10_ficha` · §2.4, §2.7 | Pasa, también en las fichas de las órdenes demo ([T029](002-abrir-orden-reparacion/evidencias/T029/verificacion.md), filas 9 y 17–23) | Persona A | Persona B (pendiente) | Verificado |

---

## HU-03 · Diagnóstico y presupuesto (spec-003)

Documentos: [spec](003-diagnostico-presupuesto/spec.md) (el clarify está en su apartado
«Clarifications»: Q1 cantidades, Q2 quitar líneas, Q3 cambiar el diagnóstico, Q4 concepto) ·
[research](003-diagnostico-presupuesto/research.md) · [tareas](003-diagnostico-presupuesto/tasks.md).
Pruebas: `backend/tests/spec003_presupuestos.rs`; salida de la ejecución para esta matriz:
[pruebas](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt). Verificación en el navegador
(quickstart §3): anotada en la tarea T035 de [tasks.md](003-diagnostico-presupuesto/tasks.md) (pasos
1–7 contra la API el 22/10 y en pantalla el 05/11, como `luis` y `paco`, con la Orden Nº 1), sin
informe con filas aparte.

| Criterio | Decisión | Tareas | Prueba | Resultado y evidencia | Autor | Revisor | Estado |
|---|---|---|---|---|---|---|---|
| US1-1 · Al guardar el diagnóstico de una orden «recibida», queda en la orden y pasa a «en diagnóstico» | Q3, D2 | T010, T012, T013, T031, T035 | `diagnostico_en_orden_recibida_la_pasa_a_en_diagnostico` · §3.1 | Pasa ([pruebas](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt)); en la app, anotado en T035 | Persona B | Persona A (pendiente) | Verificado |
| US1-2 · Sin presupuesto emitido, el diagnóstico se puede cambiar y la orden sigue «en diagnóstico» | Q3, D2 | T010, T012, T013, T035 | `el_diagnostico_se_puede_cambiar_mientras_no_haya_presupuesto_emitido` · §3.1 | Pasa ([pruebas](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt)); en la app, anotado en T035 | Persona B | Persona A (pendiente) | Verificado |
| US1-3 · Con un presupuesto emitido, el diagnóstico no se puede cambiar y se indica el motivo | Q3, D2, D6 | T011, T012, T028, T035 | `con_un_presupuesto_emitido_el_diagnostico_queda_fijo` · §3.7 | Pasa, también con el presupuesto rechazado ([pruebas](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt)); en la app, anotado en T035 | Persona B | Persona A (pendiente) | Verificado |
| US2-1 · Mano de obra 1,5 h a 40,00 € → importe 60,00 € | Q1, D4 | T015, T022, T024, T035 | `lineas_importes_y_totales_del_ejemplo` · §3.2 | Pasa ([pruebas](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt)); en la app, anotado en T035 | Persona B | Persona A (pendiente) | Verificado |
| US2-2 · Pieza PF-123 × 2 a 25,00 € → concepto «PF-123 Pastillas freno», importe 50,00 € | Q4, D3, D4 | T015, T022, T024, T035 | `lineas_importes_y_totales_del_ejemplo`, `el_concepto_enviado_para_una_pieza_se_ignora` · §3.2 | Pasa ([pruebas](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt)); en la app, anotado en T035 | Persona B | Persona A (pendiente) | Verificado |
| US2-3 · Con esas dos líneas: base 110,00 €, IVA (21 %) 23,10 €, total 133,10 € | D4 | T005, T009, T015, T024, T035 | `lineas_importes_y_totales_del_ejemplo` · §3.3 | Pasa ([pruebas](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt)); en la app, anotado en T035 | Persona B | Persona A (pendiente) | Verificado |
| US2-4 · Solo se pueden elegir referencias dadas de alta en el almacén | D3 | T016, T022, T024 | `lineas_no_validas_dan_422` (`pieza_id` inexistente → 422); la búsqueda la prueba spec-006 (`lista_de_piezas_ordenada_y_con_busqueda`) · §3.2 | Pasa ([pruebas de 003](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt), [pruebas de 006](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)); en la app se elige con el buscador de `/piezas?q=`, anotado en T035 | Persona B | Persona A (pendiente) | Verificado |
| US2-5 · Al quitar la mano de obra en borrador: base 50,00 €, IVA 10,50 €, total 60,50 € | Q2 | T017, T023, T024, T035 | `quitar_una_linea_recalcula_los_totales` · §3.5 | Pasa ([pruebas](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt)); en la app, anotado en T035 | Persona B | Persona A, revisado el 12/11/2026 | Verificado |
| US3-1 · Al emitir: «Válido hasta» = emisión + 15 días y las líneas ya no se pueden modificar | D6 | T025, T026, T028, T030, T035 | `emitir_fija_totales_caducidad_y_token`, `un_presupuesto_emitido_no_se_puede_modificar` · §3.6 | Pasa ([pruebas](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt)); en la app, con el correo en Mailpit, anotado en T035 | Persona B | Persona A (pendiente) | Verificado |
| US3-2 · Un presupuesto sin líneas no se puede emitir | D6 | T026, T028, T030 | `no_se_emite_sin_lineas_ni_por_otro_mecanico` | Pasa: 422 «El presupuesto no tiene líneas» ([pruebas](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt)). La guía no prevé paso en la app (el botón «Emitir» solo sale con al menos una línea, T030) | Persona B | Persona A (pendiente) | Verificado |
| SC-001 · Un mecánico prepara un presupuesto de 5 líneas en menos de 5 minutos | — | T035 | Recorrido con navegador automático · §3.2–§3.6 | 5 líneas (4 de mano de obra y PF-123 × 2, total 169,40 €) a los **1,02 s** de abrir la ficha de la Orden Nº 4 ([matriz B](recorridos/matriz-B/informe.md), captura `c03`). **Desviación**: no es un cronometraje humano; es el tiempo de máquina con el navegador automático. Una persona necesita 1 clic para crearlo y, por línea, 2 campos y 1 clic | Persona B | Persona A (pendiente) | Verificado con desviación |
| SC-002 · Base + IVA = total y cada importe es cantidad × precio | D4 | T015, T018 | `lineas_importes_y_totales_del_ejemplo`, `el_iva_se_redondea_una_vez_sobre_la_base` · §3.3 | Pasa, también los casos de redondeo del IVA (4,011 → 4,01; 0,525 → 0,53) ([pruebas](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt)) | Persona B | Persona A (pendiente) | Verificado |
| SC-003 · Todo presupuesto emitido caduca 15 días después de su emisión | D6 | T025, T027, T035 | `emitir_fija_totales_caducidad_y_token`, `presupuestos_de_una_orden_con_su_caducidad` · §3.6 | Pasa ([pruebas](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt)); en la app, anotado en T035 | Persona B | Persona A (pendiente) | Verificado |
| SC-004 · No se puede añadir una pieza que no exista en el almacén | D3 | T016, T022 | `lineas_no_validas_dan_422` | Pasa: 422 «Elige una referencia de pieza dada de alta» y no se crea la línea ([pruebas](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt)) | Persona B | Persona A (pendiente) | Verificado |

---

## HU-04 · Aceptar o rechazar el presupuesto desde un enlace (spec-004)

Documentos: [spec](004-aceptar-presupuesto-enlace/spec.md) ·
[clarify](004-aceptar-presupuesto-enlace/decisiones-clarify.md) ·
[research](004-aceptar-presupuesto-enlace/research.md) · [tareas](004-aceptar-presupuesto-enlace/tasks.md).
Pruebas: `backend/tests/spec004_respuesta.rs`. Verificación en el navegador:
[T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md) (quickstart §4). Cronometraje:
[T028](004-aceptar-presupuesto-enlace/evidencias/T028/informe.md).

| Criterio | Decisión | Tareas | Prueba | Resultado y evidencia | Autor | Revisor | Estado |
|---|---|---|---|---|---|---|---|
| US1-1 · El enlace muestra el presupuesto completo, sin datos del cliente | D1, D8 | T005, T012, T015, T027 | `ver_el_presupuesto_sin_datos_del_cliente` · §4.1 | Pasa, en escritorio y a 390 px, sin datos del cliente ([T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md), filas 2–4) | Persona A | Persona B (pendiente) | Verificado |
| US1-2 · Al aceptar, la orden lo muestra con fecha y hora y pasa a «En reparación» | D3, D5 | T008, T013, T014, T015, T016, T027 | `aceptar_el_inicial_pasa_la_orden_a_reparacion` · §4.2, §4.3 | Pasa ([T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md), filas 5–9) | Persona A | Persona B (pendiente) | Verificado |
| US1-3 · Al volver a abrir el enlace se ve la decisión, sin botones | — | T009, T013, T015, T027 | `solo_se_responde_una_vez` · §4.2 | Pasa ([T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md), fila 8) | Persona A | Persona B (pendiente) | Verificado |
| US1-4 · Con dos pestañas solo cuenta la primera respuesta | Q4, D2 | T010, T013, T015, T020, T027 | `dos_respuestas_a_la_vez_solo_cuenta_una`, `aceptar_y_rechazar_a_la_vez_solo_cuenta_una` (con mutación, T010 y T020) · §4.7 | Pasa; la segunda pestaña ve «Este presupuesto ya fue respondido» ([T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md), filas 15 y 16; [mutación](004-aceptar-presupuesto-enlace/evidencias/T010-mutacion.txt)) | Persona A | Persona B (pendiente) | Verificado |
| US1-5 · El personal ve el enlace y «Copiar enlace» solo mientras está vigente | Q5, D6 | T001, T011, T016, T027 | `tras_responder_la_api_sigue_enviando_el_enlace` · §4.3, §4.4, §4.6 | Pasa; visible y copiable en la orden 5, oculto tras responder y en la orden 7 caducada ([T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md), filas 9–11 y 14) | Persona A | Persona B (pendiente) | Verificado |
| US2-1 · Al rechazar, se guarda con fecha y hora y la orden sigue «En diagnóstico» | Q2, D3 | T017, T021, T022, T027 | `rechazar_el_inicial_no_cambia_la_orden` · §4.5 | Pasa ([T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md), fila 13) | Persona A | Persona B (pendiente) | Verificado |
| US2-2 · Al rechazar una ampliación, la orden sigue «Pendiente de conformidad» | D3 | T018, T021, T022, T027 | `la_ampliacion_se_acepta_y_se_rechaza_igual` · §4.7 | Pasa ([T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md), fila 15) | Persona A | Persona B (pendiente) | Verificado |
| US2-3 · La ficha muestra todas las respuestas, cada una con su fecha y hora | Q1, D6 | T019, T021, T027 | `la_ficha_trae_todas_las_respuestas` · §4.7 | Pasa; inicial y ampliación ([T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md), fila 15) | Persona A | Persona B (pendiente) | Verificado |
| US3-1 · Un presupuesto caducado muestra el aviso, sin botones | Q3, D7 | T001, T024, T025, T026, T027, T029 | `un_caducado_no_admite_ninguna_respuesta`, `el_get_marca_el_caducado` · §4.8 | Pasa, en escritorio y a 390 px ([T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md), filas 17 y 19) | Persona A | Persona B (pendiente) | Verificado |
| US3-2 · Si caduca con la página abierta, al responder se ve el aviso y no se guarda nada | Q3, Q4 | T024, T025, T026 | `un_caducado_no_admite_ninguna_respuesta`, `el_dia_de_la_caducidad_aun_se_puede_responder`, `un_respondido_no_caduca` | Pasa en las pruebas automáticas; la guía no prevé paso en la app, porque haría falta esperar a la medianoche ([quickstart de 004](004-aceptar-presupuesto-enlace/quickstart.md)) | Persona A | Persona B (pendiente) | Verificado |
| US4-1 · Sin presupuesto aceptado, el mecánico no puede imputar | — | T023, T027 | `sin_presupuesto_aceptado_no_se_imputa` (con mutación, T023); y en spec-005, `solo_se_imputa_en_reparacion_con_presupuesto_aceptado` · §4.4 | Pasa; `marta` no ve «Imputar» y un `POST` directo da 409 ([T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md), fila 12; [mutación](004-aceptar-presupuesto-enlace/evidencias/T023-mutacion.txt)). La prueba de 004 usa órdenes «en diagnóstico» (el 409 sale del estado); la de spec-005 cubre la otra rama, una orden «en reparación» sin presupuesto aceptado → 409 «La orden no tiene un presupuesto aceptado» ([pruebas de 005](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt)). Añadido por la sugerencia de la revisión de la Persona B | Persona A | Persona B, revisado el 12/11/2026 (ver nota) | Verificado |
| SC-001 · El cliente responde en menos de 1 minuto desde que abre el enlace | D5 | T015, T028 | Cronometraje humano · §4.2 | **13 s**, medido por una persona en Chrome en español ([T028](004-aceptar-presupuesto-enlace/evidencias/T028/informe.md)). **Desviación**: se cronometró con la orden Nº 4 (`4444FFF`) y no con la de `1313MMM` del §4.2; es la misma página y el mismo recorrido | Persona A | Persona B (pendiente) | Verificado con desviación |
| SC-002 · El 100 % de las respuestas quedan con fecha y hora visibles en la orden | Q1, D6 | T008, T014, T016, T019, T027, T030 | `aceptar_el_inicial_pasa_la_orden_a_reparacion`, `la_ficha_trae_todas_las_respuestas` · §4.3, §4.7 | Pasa; la hora coincide con `respondido_en` en hora de Madrid ([T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md), filas 9 y 15) | Persona A | Persona B (pendiente) | Verificado |
| SC-003 · Ningún presupuesto caducado se acepta ni se rechaza | Q3, D7 | T001, T024, T025, T027, T030 | `un_caducado_no_admite_ninguna_respuesta` · §4.8 | Pasa ([T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md), filas 17 y 19) | Persona A | Persona B (pendiente) | Verificado |
| SC-004 · Ninguna orden sin presupuesto aceptado permite imputar | — | T023, T027, T030 | `sin_presupuesto_aceptado_no_se_imputa` · §4.4 | Pasa ([T027](004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md), fila 12) | Persona A | Persona B (pendiente) | Verificado |

---

## HU-05 · Imputar tiempo y piezas (spec-005)

Documentos: [spec](005-imputar-tiempo-piezas/spec.md) (clarify en «Clarifications»: Q1 concepto, Q2
terminar sin imputaciones, Q3 ampliación rechazada) · [research](005-imputar-tiempo-piezas/research.md) ·
[tareas](005-imputar-tiempo-piezas/tasks.md). Pruebas: `backend/tests/spec005_imputaciones.rs`;
salida: [pruebas](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt). Verificación en el
navegador (quickstart §5): anotada en la tarea T034 de [tasks.md](005-imputar-tiempo-piezas/tasks.md)
(pasos 2–6 contra la API el 29/10; en pantalla el 05/11 como `luis` en la Orden Nº 10, `1313MMM`).

| Criterio | Decisión | Tareas | Prueba | Resultado y evidencia | Autor | Revisor | Estado |
|---|---|---|---|---|---|---|---|
| US1-1 · Imputar 2 uds de una pieza con stock 10 deja el stock en 8 en ese momento | D3 | T011, T016, T034 | `imputar_pieza_descuenta_stock_y_suma_al_acumulado` · §5.2 | Pasa ([pruebas](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt)); en la app, stock de PF-123 de 4 a 2 en `/almacen` (nota de T034) | Persona B | Persona A (pendiente) | Verificado |
| US1-2 · Con 110,00 € aceptados, 1 h a 40,00 € y 2 uds a 25,00 € → «Acumulado 90,00 € de 110,00 € presupuestados» | Q1, D3, D5 | T007, T011, T017, T030, T034 | `imputar_pieza_descuenta_stock_y_suma_al_acumulado` · §5.3 | Pasa ([pruebas](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt)); en la app, acumulado 90 € (nota de T034) | Persona B | Persona A (pendiente) | Verificado |
| US1-3 · Con stock 1, imputar 2 uds se rechaza indicando el stock y que puede pedirse al proveedor | D3 | T012, T016, T017, T034 | `sin_stock_suficiente_no_se_imputa` · §5.2 | Pasa: «Solo hay 1 unidad en stock de PF-123; el almacén puede pedirla al proveedor», sin cambiar el stock ni crear la imputación ([pruebas](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt)); en la app, anotado en T034 | Persona B | Persona A (pendiente) | Verificado |
| US2-1 · Con 100,00 € presupuestados, un acumulado de 110,00 € deja la orden «en reparación» | D4 | T018, T019, T020 | `pasar_del_110_por_ciento_bloquea_la_orden`, `el_limite_del_10_por_ciento_es_exacto_al_centimo` | Pasa: 110,00 no bloquea y 110,01 sí ([pruebas](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt)). La guía no prevé este límite exacto en la app | Persona B | Persona A, revisado el 12/11/2026 | Verificado |
| US2-2 · La imputación que supera el 110 % se guarda, la orden pasa a «pendiente de conformidad» y no admite más | D4 | T018, T020, T021, T034 | `pasar_del_110_por_ciento_bloquea_la_orden` · §5.4 | Pasa, con el evento al tablero ([pruebas](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt)); en la app, 130 € y «Pendiente de conformidad» (nota de T034) | Persona B | Persona A (pendiente) | Verificado |
| US3-1 · Al aceptar una ampliación de 50,00 €, la orden vuelve a «en reparación» con 150,00 € presupuestados | D7 | T022, T024, T034 | `aceptar_la_ampliacion_suma_lo_presupuestado_y_reanuda` · §5.5 | Pasa ([pruebas](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt)). **Desviación**: tanto la prueba como el recorrido de T034 simulan la aceptación del cliente con SQL, porque la página del enlace es de spec-004; la aceptación real por enlace la prueba spec-004 (`la_ampliacion_se_acepta_y_se_rechaza_igual`, HU-04 US2-2) | Persona B | Persona A (pendiente) | Verificado con desviación |
| US3-2 · Con la ampliación sin respuesta no se puede imputar | Q3, D7 | T022, T023, T024 | `aceptar_la_ampliacion_suma_lo_presupuestado_y_reanuda`, `con_la_ampliacion_rechazada_sigue_bloqueada` | Pasa: 409 con la ampliación emitida y también con la rechazada ([pruebas](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt)) | Persona B | Persona A (pendiente) | Verificado |
| US4-1 · Una orden «en reparación» con imputaciones pasa a «terminada» y no admite más | D6 | T025, T028, T029, T034 | `terminar_una_orden_con_imputaciones` · §5.6 | Pasa ([pruebas](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt)); en la app, anotado en T034 | Persona B | Persona A (pendiente) | Verificado |
| US4-2 · Sin imputaciones no se puede terminar y se indica el motivo | Q2, D6 | T026, T028, T029, T034 | `no_se_termina_sin_imputaciones_ni_fuera_de_reparacion` · §5.1 | Pasa: 422 «Primero hay que imputar el trabajo realizado» ([pruebas](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt)); en la app el botón «Terminar reparación» no aparece sin imputaciones (nota de T034) | Persona B | Persona A (pendiente) | Verificado |
| SC-001 · Tras imputar piezas, el stock del almacén refleja el descuento al instante | D3 | T011, T016, T034 | `imputar_pieza_descuenta_stock_y_suma_al_acumulado` · §5.2 | Pasa ([pruebas](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt)); en la app, de 4 a 2 (nota de T034) | Persona B | Persona A (pendiente) | Verificado |
| SC-002 · El 100 % de las órdenes que superan el 110 % quedan bloqueadas | D4 | T018, T019, T020 | `pasar_del_110_por_ciento_bloquea_la_orden`, `el_limite_del_10_por_ciento_es_exacto_al_centimo` · §5.4 | Pasa ([pruebas](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt)) | Persona B | Persona A (pendiente) | Verificado |
| SC-003 · Un mecánico registra una imputación en menos de 30 segundos | — | T034 | Recorrido con navegador automático · §5.3 | «Acumulado 50,00 € de 110,00 €» a los **0,24 s** de abrir la ficha de la Orden Nº 3 ([matriz B](recorridos/matriz-B/informe.md), captura `c02`). **Desviación**: no es un cronometraje humano; es el tiempo de máquina con el navegador automático. Una persona necesita 2 campos y 1 clic | Persona B | Persona A (pendiente) | Verificado con desviación |
| SC-004 · El acumulado coincide con la suma de las imputaciones | D4, D5 | T007, T011, T015 | `imputar_pieza_descuenta_stock_y_suma_al_acumulado`, `el_resumen_suma_solo_lo_aceptado_de_esa_orden` · §5.3, §5.4 | Pasa ([pruebas](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt)); en la app, 90 € y 130 € (nota de T034) | Persona B | Persona A (pendiente) | Verificado |

---

## HU-06 · Pedido de pieza al proveedor (spec-006)

Documentos: [spec](006-pedido-pieza-proveedor/spec.md) (clarify en «Clarifications»: Q1 varios pedidos,
Q2 stock inicial, Q3 sin cambio manual, Q4 misma referencia, Q5 sin aviso) ·
[research](006-pedido-pieza-proveedor/research.md) · [tareas](006-pedido-pieza-proveedor/tasks.md).
Pruebas: `backend/tests/spec006_pedidos.rs`; salida:
[pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt). Verificación en el navegador
(quickstart §6): anotada en la tarea T032 de [tasks.md](006-pedido-pieza-proveedor/tasks.md) (pasos
1–6, 8 y 9 como `pepe` el 22/10; paso 7 como `luis` en la Orden Nº 2 el 05/11).

| Criterio | Decisión | Tareas | Prueba | Resultado y evidencia | Autor | Revisor | Estado |
|---|---|---|---|---|---|---|---|
| US1-1 · Alta de «PF-123», 25,00 €, stock 4 → aparece en la lista con esos datos | Q2, D6 | T009, T010, T011, T012, T013 | `alta_con_stock_y_sin_stock`, `lista_de_piezas_ordenada_y_con_busqueda` | Pasa ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)). En la app, la guía da de alta sin stock (§6.1), no con stock 4 | Persona B | Persona A (pendiente) | Verificado |
| US1-2 · Alta sin indicar stock → stock 0 | Q2, D6 | T009, T011, T013, T032 | `alta_con_stock_y_sin_stock` · §6.1 | Pasa ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)); en la app, `FA-9` con stock 0 (nota de T032) | Persona B | Persona A (pendiente) | Verificado |
| US1-3 · Un código repetido se rechaza indicando que ya existe | D6 | T009, T011, T032 | `referencia_repetida_da_409` · §6.1 | Pasa: 409 «Ya existe la referencia PF-123» ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)); en la app, anotado en T032 | Persona B | Persona A (pendiente) | Verificado |
| US2-1 · Un pedido para una orden «en reparación» queda «pendiente» con la fecha de hoy y la orden pasa a «en espera de piezas» | D3, D5, D7 | T014, T018, T019, T032 | `pedido_para_orden_en_reparacion_la_pone_en_espera` · §6.3 | Pasa ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)); en la app, anotado en T032 | Persona B | Persona A (pendiente) | Verificado |
| US2-2 · Con la orden ya en espera, otro pedido se admite y sigue en espera | Q1, D3 | T014, T018, T032 | `otro_pedido_con_la_orden_en_espera_se_admite` · §6.5 | Pasa ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)); en la app, anotado en T032 | Persona B | Persona A (pendiente) | Verificado |
| US2-3 · Un pedido para una orden en otro estado (p. ej. «en diagnóstico») se rechaza con el motivo | D3 | T015, T018, T032 | `orden_en_diagnostico_da_409_y_no_cambia` · §6.6 | Pasa: 409 con el motivo y la orden no cambia ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)); en la app, anotado en T032 | Persona B | Persona A (pendiente) | Verificado |
| US2-4 · Con la orden «en espera de piezas», el mecánico no puede imputar | — | T016, T027 de spec-005, T032 | `orden_en_espera_no_se_puede_terminar_ni_imputar` · §6.7 | Pasa: 409 al imputar y al terminar ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)); en la app, `luis` en la Orden Nº 2 no puede imputar ni terminar (nota de T032) | Persona B | Persona A (pendiente) | Verificado |
| US2-5 · Un segundo pedido pendiente de la misma referencia para la misma orden se rechaza con el motivo | Q4, D2 | T015, T018, T032 | `misma_referencia_pendiente_para_la_misma_orden_da_409` · §6.4 | Pasa: «Ya hay un pedido pendiente de PF-123 para la orden 42» ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)); en la app, anotado en T032 | Persona B | Persona A (pendiente) | Verificado |
| US3-1 · Al recibir el pedido: «recibido», el stock sube 2 y la orden vuelve a «en reparación» | D3, D4, D7 | T020, T023, T025, T032 | `recibir_suma_el_stock_y_la_orden_vuelve_a_reparacion` · §6.8, §6.9 | Pasa ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)); en la app, anotado en T032 (la vuelta a «en reparación» se miró en la base de datos) | Persona B | Persona A (pendiente) | Verificado |
| US3-2 · Con dos pedidos pendientes, recibir uno deja la orden en espera hasta el otro | D4 | T020, T023, T032 | `con_dos_pedidos_la_orden_espera_al_ultimo` · §6.8, §6.9 | Pasa ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)); en la app, anotado en T032 | Persona B | Persona A, revisado el 12/11/2026 | Verificado |
| US3-3 · Con un pedido pendiente, nadie (tampoco el jefe) puede cambiar el estado de la orden a mano | Q3, D4 | T016, T030 | `orden_en_espera_no_se_puede_terminar_ni_imputar` (como `paco`, jefe) · revisión de T030 | Pasa: el jefe recibe 409 al terminar y al imputar, y la orden sigue en espera ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)). No existe ningún endpoint ni control para cambiar el estado a mano (T030) | Persona B | Persona A (pendiente) | Verificado |
| US4-1 · La ficha de la orden muestra sus pedidos, recibidos y pendientes, con su estado | — | T026, T027, T028 | `pedidos_de_una_orden_sin_los_de_otras`, `la_ficha_de_la_orden_trae_sus_pedidos` · §6.7 | Pasa ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)); en la app, la sección Pedidos de la Orden Nº 2 (nota de T028 y T032) | Persona B | Persona A (pendiente) | Verificado |
| US4-2 · El almacén ve los pedidos pendientes de todas las órdenes, cada uno con su orden | — | T022, T024, T025 | `lista_de_pedidos_pendientes_de_todas_las_ordenes` · §6.8 | Pasa ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)); en la app, anotado en T032 | Persona B | Persona A (pendiente) | Verificado |
| SC-001 · El responsable registra un pedido en menos de 1 minuto | — | T032 | Recorrido con navegador automático · §6.3 | «Pedido registrado: 1 × PF-123 para la orden 2» a los **0,56 s** de abrir `/pedidos` ([matriz B](recorridos/matriz-B/informe.md), captura `c01`). **Desviación**: no es un cronometraje humano; es el tiempo de máquina con el navegador automático. Una persona necesita 4 campos y 2 clics | Persona B | Persona A (pendiente) | Verificado con desviación |
| SC-002 · El 100 % de las órdenes con algún pedido pendiente están «en espera de piezas» | D4 | T014, T018, T020 | `pedido_para_orden_en_reparacion_la_pone_en_espera`, `otro_pedido_con_la_orden_en_espera_se_admite`, `con_dos_pedidos_la_orden_espera_al_ultimo` · §6.3 | Pasa ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)) | Persona B | Persona A (pendiente) | Verificado |
| SC-003 · Al recibir el último pedido, la orden vuelve a «en reparación» sin cambiarla nadie | D4 | T020, T023 | `recibir_suma_el_stock_y_la_orden_vuelve_a_reparacion`, `con_dos_pedidos_la_orden_espera_al_ultimo` · §6.9 | Pasa ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)) | Persona B | Persona A (pendiente) | Verificado |
| SC-004 · Tras recibir un pedido, el stock sube exactamente la cantidad pedida | D3 | T020, T021 | `recibir_suma_el_stock_y_la_orden_vuelve_a_reparacion`, `recibir_dos_veces_da_409_y_no_suma_otra_vez` · §6.8 | Pasa, y recibir dos veces no suma otra vez ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)); en la app, stock de FA-9 = 1 (nota de T032) | Persona B | Persona A (pendiente) | Verificado |
| SC-005 · El 100 % de los pedidos sobre órdenes en un estado no permitido se rechazan con el motivo | D3 | T015, T018 | `orden_en_diagnostico_da_409_y_no_cambia` · §6.6 | Pasa ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)). La prueba usa una orden «en diagnóstico»; el código rechaza todo estado distinto de «en reparación» y «en espera de piezas» (T018) | Persona B | Persona A (pendiente) | Verificado |
| SC-006 · No puede haber dos referencias con el mismo código | D6 | T002, T009, T011 | `referencia_repetida_da_409` · §6.1 | Pasa; además, `referencia` es `UNIQUE` en la base de datos (T002) ([pruebas](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt)) | Persona B | Persona A (pendiente) | Verificado |

---

## HU-07 · Facturar y avisar al cliente (spec-007)

Documentos: [spec](007-facturar-avisar-cliente/spec.md) (clarify en «Clarifications»: Q1 imprimir la
factura, Q2 numeración, Q3 logo) · [research](007-facturar-avisar-cliente/research.md) ·
[tareas](007-facturar-avisar-cliente/tasks.md). Pruebas: `backend/tests/spec007_facturas.rs`; salida:
[pruebas](007-facturar-avisar-cliente/evidencias/matriz/pruebas.txt). Verificación en el navegador
(quickstart §7): anotada en la tarea T028 de [tasks.md](007-facturar-avisar-cliente/tasks.md) (por la
API el 29/10; en el navegador el 05/11 como `ana` y `luis`, Orden Nº 10, `1313MMM`).

| Criterio | Decisión | Tareas | Prueba | Resultado y evidencia | Autor | Revisor | Estado |
|---|---|---|---|---|---|---|---|
| US1-1 · Con 2 h a 40,00 € y PF-123 × 2 a 25,00 €, la factura muestra esas líneas, base 130,00 €, IVA 27,30 € y total 157,30 € | D1, D6 | T009, T013, T015, T016, T017, T028 | `facturar_copia_las_imputaciones_y_cierra_la_orden` · §7.2 | Pasa ([pruebas](007-facturar-avisar-cliente/evidencias/matriz/pruebas.txt)); en la app, Factura nº 2 con 130,00 / 27,30 / 157,30 € (nota de T028) | Persona B | Persona A, revisado el 12/11/2026 | Verificado |
| US1-2 · La factura tiene número, fecha, datos del cliente y matrícula | Q2, D4 | T009, T010, T012, T015 | `facturar_copia_las_imputaciones_y_cierra_la_orden`, `los_numeros_de_factura_son_correlativos_y_sin_repetir`, `ver_la_factura_y_su_orden` · §7.2 | Pasa, también con dos facturas a la vez ([pruebas](007-facturar-avisar-cliente/evidencias/matriz/pruebas.txt)); en la app, anotado en T028 | Persona B | Persona A (pendiente) | Verificado |
| US1-3 · «Imprimir» abre la impresión del navegador con la factura sola, sin menú ni botones | Q1, D3 | T015, T028 | Sin prueba automática (es la hoja de estilos de impresión) · §7.3 | En la app, vista previa de impresión sin menú (nota de T028) | Persona B | Persona A (pendiente) | Verificado |
| US1-4 · Con logo configurado sale en la cabecera; sin él, la factura sale sin logo | Q3, D8 | T029, T030, T031, T032 | `con_logo_configurado_la_factura_lo_muestra`, `sin_logo_o_con_un_archivo_que_no_existe_no_hay_logo` · §7.3 · comprobación añadida en la app | La API pasa ([pruebas](007-facturar-avisar-cliente/evidencias/matriz/pruebas.txt)). En la app: con `TALLER_LOGO` apuntando a una imagen, el logo sale a la izquierda de los datos del taller, también en la vista de impresión; con `TALLER_LOGO` apuntando a un archivo que no existe, la factura sale sin logo y sin hueco ([matriz B](recorridos/matriz-B/informe.md), capturas `b01`–`b04`) | Persona B | Persona A (pendiente) | Verificado |
| US1-5 · En una orden que no está «terminada», «Facturar» no está disponible | D1, D7 | T011, T016, T028 | `solo_se_factura_una_vez_y_solo_si_esta_terminada` · §7.1 | Pasa: 409 en «en reparación», «pendiente de conformidad» y «en diagnóstico» ([pruebas](007-facturar-avisar-cliente/evidencias/matriz/pruebas.txt)); en la app, sin «Facturar» en una orden en reparación (nota de T028) | Persona B | Persona A (pendiente) | Verificado |
| US2-1 · Al emitir la factura, el cliente recibe «Su vehículo 1234BCD está listo para recoger» con el total | D2 | T018, T019, T020, T028 | `el_aviso_se_anota_si_sale_el_correo`, `si_el_correo_falla_la_factura_se_emite_igual` · §7.4 | Pasa ([pruebas](007-facturar-avisar-cliente/evidencias/matriz/pruebas.txt)); en la app, el correo llega a Mailpit (nota de T028, con `1313MMM`) | Persona B | Persona A (pendiente) | Verificado |
| US3-1 · Con dos reparaciones facturadas, el historial las muestra con la más reciente primero, con fecha, km, diagnóstico y líneas | D5 | T021, T023, T024, T028 | `historial_con_la_reparacion_mas_reciente_primero` · §7.5 | Pasa ([pruebas](007-facturar-avisar-cliente/evidencias/matriz/pruebas.txt)); en la app, historial de `1313 mmm` y `9999LLL` (nota de T028) | Persona B | Persona A (pendiente) | Verificado |
| US3-2 · Una matrícula sin reparaciones facturadas indica que no hay historial | D5 | T022, T023, T024, T028 | `historial_por_matricula_escrita_de_cualquier_forma` · §7.5 | Pasa: un vehículo sin facturas da `reparaciones: []` y una matrícula que no existe, 404 ([pruebas](007-facturar-avisar-cliente/evidencias/matriz/pruebas.txt)); en la app, «No hay historial para esa matrícula» con una matrícula inexistente (nota de T028) | Persona B | Persona A (pendiente) | Verificado |
| SC-001 · La recepcionista emite una factura en menos de 30 segundos sin escribir nada | — | T028 | Recorrido con navegador automático · §7.2 | «Factura nº 2 · 12/11/2026 · 133,10 €» a los **0,24 s** de abrir la ficha de la Orden Nº 8 ([matriz B](recorridos/matriz-B/informe.md), captura `c04`). **Desviación**: no es un cronometraje humano; es el tiempo de máquina con el navegador automático. Una persona necesita 2 clics y ningún campo | Persona B | Persona A (pendiente) | Verificado con desviación |
| SC-002 · El 100 % de las líneas imputadas aparecen en la factura con el importe correcto | D1, D6 | T009, T013 | `facturar_copia_las_imputaciones_y_cierra_la_orden` · §7.2 | Pasa: las dos líneas, en el orden en que se imputaron ([pruebas](007-facturar-avisar-cliente/evidencias/matriz/pruebas.txt)) | Persona B | Persona A (pendiente) | Verificado |
| SC-003 · Ninguna orden que no esté «terminada» puede facturarse | D1 | T011, T013 | `solo_se_factura_una_vez_y_solo_si_esta_terminada`, `sin_imputaciones_o_sin_permiso_no_se_factura` · §7.1 | Pasa ([pruebas](007-facturar-avisar-cliente/evidencias/matriz/pruebas.txt)) | Persona B | Persona A (pendiente) | Verificado |
| SC-004 · El historial se consulta en menos de 10 segundos escribiendo la matrícula | — | T028 | Recorrido con navegador automático · §7.5 | La reparación de `9999LLL` a los **0,22 s** de abrir `/historial` ([matriz B](recorridos/matriz-B/informe.md), captura `c05`). **Desviación**: no es un cronometraje humano; es el tiempo de máquina con el navegador automático. Una persona necesita 1 campo y 1 clic | Persona B | Persona A (pendiente) | Verificado con desviación |

---

## HU-08 · Tablero de órdenes abiertas (spec-008)

Documentos: [spec](008-tablero-ordenes-abiertas/spec.md) ·
[clarify](008-tablero-ordenes-abiertas/decisiones-clarify.md) ·
[research](008-tablero-ordenes-abiertas/research.md) · [tareas](008-tablero-ordenes-abiertas/tasks.md).
Pruebas: `backend/tests/spec008_tablero.rs` (la 9, con `#[ignore]`: `cargo test --test spec008_tablero
-- --ignored`) y los ficheros de Vitest de `frontend/src/lib/`. Verificación en el navegador:
[T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md) y su
[complemento](008-tablero-ordenes-abiertas/evidencias/T028/complemento.md) (quickstart §8);
cronometraje: [T029](008-tablero-ordenes-abiertas/evidencias/T029/informe.md); recorrido completo:
[T039](008-tablero-ordenes-abiertas/evidencias/T039/informe.md) (quickstart §9); comprobaciones añadidas al construir
esta matriz: [matriz](008-tablero-ordenes-abiertas/evidencias/matriz/informe.md).

| Criterio | Decisión | Tareas | Prueba | Resultado y evidencia | Autor | Revisor | Estado |
|---|---|---|---|---|---|---|---|
| US1-1 · Seis columnas y cada orden en la de su estado | D1 | T006, T010, T012, T028 | `prueba1_seis_columnas_cada_orden_en_la_suya_y_sin_facturadas` · §8.2 | Pasa (rojo antes del verde: [salida](008-tablero-ordenes-abiertas/evidencias/T006-T010-US1.txt)); en la app, 6 columnas en orden y cada tarjeta en la suya ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), fila 4) | Persona A | Persona B (pendiente) | Verificado |
| US1-2 · Una orden facturada no aparece | D1, D3 | T006, T010, T012, T028 | `prueba1_seis_columnas_cada_orden_en_la_suya_y_sin_facturadas` · §8.2, §8.8 | Pasa; `9999LLL` no sale y `2222CCC` desaparece al facturar ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), filas 4 y 10) | Persona A | Persona B, revisado el 12/11/2026 | Verificado |
| US1-3 · Recepción, mecánico y almacén: sin menú, «No tienes permiso para hacer esto» y 403 | Q2, D4 | T008, T010, T012, T025, T028 | `prueba3_solo_el_jefe` · §8.1 | Pasa; además, con un 403 no se abre `/api/eventos` ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), filas 1–3; [complemento](008-tablero-ordenes-abiertas/evidencias/T028/complemento.md)) | Persona A | Persona B (pendiente) | Verificado |
| US1-4 · Tarjetas por fecha de entrega y, a igual fecha, por número | Q4, D1, D8 | T001, T009, T010, T012, T028 | `prueba4_tarjetas_por_entrega_y_numero` · §8.5 | Pasa; `3333DDD` (hoy) encima de `2222CCC` ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), fila 7) | Persona A | Persona B (pendiente) | Verificado |
| US2-1 · «Luis 3 · Marta 1», y los mecánicos activos sin órdenes con 0 | Q1, D2 | T013, T015, T016, T028, T034 | `prueba5_carga_por_mecanico_con_los_activos_a_cero`, `prueba2_sin_ordenes_seis_columnas_vacias` · §8.3 | Pasa ([salida](008-tablero-ordenes-abiertas/evidencias/T013-T015-US2.txt)); en la app, «Luis 5 · Marta 3», igual que las tarjetas ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), fila 5) | Persona A | Persona B (pendiente) | Verificado |
| US2-2 · Un mecánico de baja con órdenes sale con «(de baja)»; uno sin órdenes, no | Q1, D2, D8 | T014, T015, T016, T034 | `prueba6_inactivos_solo_con_ordenes_abiertas` (con mutación, T014 y T034) · comprobación añadida en la app | La API pasa, también con la mutación ([T014](008-tablero-ordenes-abiertas/evidencias/T014-mutacion-carga.txt), [T034](008-tablero-ordenes-abiertas/evidencias/T034-mutacion-carga.txt)). En la app: «Luis 4 · Marta 3 · Pedro (de baja) 1», la tarjeta de `1111BBB` con «Pedro», y la carga suma 8, como las tarjetas ([matriz](008-tablero-ordenes-abiertas/evidencias/matriz/informe.md), captura `c01`). El caso se preparó con SQL (Pedro como mecánico de la orden demo 1, restaurado después), porque los datos demo no lo tienen (research D8) | Persona A | Persona B (pendiente) | Verificado |
| US3-1 · Una orden con la entrega de ayer sale destacada con «Retrasada» | D1 | T017, T018, T028, T029 | `prueba7_retrasada_ayer_si_hoy_no_y_terminada_sin_facturar_si` (con mutación, T017) · §8.4 | Pasa ([mutación](008-tablero-ordenes-abiertas/evidencias/T017-mutacion-retrasada.txt)); en la app, `8888KKK` con borde e insignia ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), fila 6) | Persona A | Persona B (pendiente) | Verificado |
| US3-2 · Una orden con la entrega de hoy no sale retrasada | D8 | T001, T017, T018, T028 | `prueba7_retrasada_ayer_si_hoy_no_y_terminada_sin_facturar_si` · §8.4 | Pasa; `3333DDD` sin «Retrasada» ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), fila 6) | Persona A | Persona B (pendiente) | Verificado |
| US4-1 · Una orden que pasa a «Terminada» cambia de columna en menos de 5 s | Q3, D5 | T020, T023, T025, T026, T028, T029, T039 | `recarga.test.ts`, `eventos.test.ts` · §8.6 | Pasa: 109 ms de máquina ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), fila 8) y 4 s cronometrados por una persona ([T029](008-tablero-ordenes-abiertas/evidencias/T029/informe.md)) | Persona A | Persona B (pendiente) | Verificado |
| US4-2 · Una orden nueva aparece en «Recibida» y su mecánico suma 1 | Q3, D5 | T025, T026, T028, T029, T039 | `eventos.test.ts` · §8.7, §9.3 | Pasa ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), fila 9; [T039](008-tablero-ordenes-abiertas/evidencias/T039/informe.md), §9.3) | Persona A | Persona B (pendiente) | Verificado |
| US4-3 · Al facturar, la tarjeta desaparece y su mecánico resta 1 | Q3, D5 | T025, T026, T028, T029, T039 | `eventos.test.ts` · §8.8, §9.7 | Pasa ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), fila 10; [T039](008-tablero-ordenes-abiertas/evidencias/T039/informe.md), §9.7) | Persona A | Persona B (pendiente) | Verificado |
| US4-4 · A medianoche de Madrid (00:01) se vuelve a pedir y la orden de hoy pasa a «Retrasada» | Q3, D6 | T019, T022, T025, T031 | `medianoche.test.ts` (reloj simulado, proceso en Nueva York; incluye los días de cambio de hora) · §8.11 | Pasa, con rojo antes del verde ([salida](008-tablero-ordenes-abiertas/evidencias/T019-T024-vitest.txt)). A mano no se comprueba, como prevén el plan y el §8.11 | Persona A | Persona B (pendiente) | Verificado |
| US4-5 · Al volver la conexión, el tablero se pide entero, con lo cambiado mientras tanto | Q3, D5 | T020, T021, T023, T024, T025, T028, T037 | `eventos.test.ts` · §8.9 · comprobación añadida en la app | Con el servidor parado y arrancado, el aviso sale y el tablero se recupera solo ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), fila 11). Lo cambiado mientras tanto también aparece: con el servidor parado, `3333DDD` pasa a «terminada» y, al arrancarlo, sale en «Terminada» a los 1,8 s, sin recargar, y el aviso desaparece ([matriz](008-tablero-ordenes-abiertas/evidencias/matriz/informe.md), capturas `c02` y `c03`). El cambio durante el corte se hizo directamente en la base, porque con el servidor parado no se puede hacer desde la app (hallazgo V1) | Persona A | Persona B (pendiente) | Verificado |
| SC-001 · Las retrasadas se identifican en menos de 10 s | Q4, D9 | T009, T018, T029 | Cronometraje humano · §8.4 | **5 s**, por una persona que no había visto el tablero, que nombra solo `8888KKK` ([T029](008-tablero-ordenes-abiertas/evidencias/T029/informe.md)) | Persona A | Persona B (pendiente) | Verificado |
| SC-002 · Un cambio de estado aparece en menos de 5 s sin recargar | Q3, D5, D9 | T020, T023, T025, T029, T030, T039 | Cronometraje humano · §8.6 (y §8.7, §8.8) | **4 s** al terminar `2222CCC`, medido por una persona ([T029](008-tablero-ordenes-abiertas/evidencias/T029/informe.md)). **Desviación**: T029 pedía cronometrar también abrir y facturar (§8.7 y §8.8); esos dos solo tienen tiempo de máquina, 108 y 111 ms ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md)) | Persona A | Persona B (pendiente) | Verificado con desviación |
| SC-003 · La carga coincide con el recuento de las tarjetas | Q1, D1, D2 | T013, T014, T015, T028, T031, T033, T034, T035, T039 | Comprobación de SC-003 en las pruebas 1, 2, 4, 5 y 6; `prueba9_peticiones_cortadas_no_dejan_conexiones_en_transaccion` · §8.3, §9.2 | Pasa; en la app, la carga coincide con las tarjetas ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), fila 5; regresión, «Luis 6 · Marta 3»: [T030](008-tablero-ordenes-abiertas/evidencias/T030/regresion/verificacion.md)). La prueba 9 (hallazgo V3) falló 3 de 3 con la transacción anterior y pasa sin ella ([salida](008-tablero-ordenes-abiertas/evidencias/T033-T034-prueba9.txt), [con `--ignored`](008-tablero-ordenes-abiertas/evidencias/T033-prueba9-ignored.txt)) | Persona A | Persona B (pendiente) | Verificado |
| SC-004 · Todas las órdenes abiertas aparecen y ninguna facturada | D1, D3 | T005, T006, T010, T028, T031 | `prueba1_seis_columnas_cada_orden_en_la_suya_y_sin_facturadas`, `prueba8_la_lista_de_ordenes_sigue_trayendo_las_facturadas_en_el_mismo_orden` · §8.2 | Pasa; 8 de 8 abiertas y ninguna facturada ([T028](008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), fila 4) | Persona A | Persona B (pendiente) | Verificado |

---

## Resumen

Recuento de las filas, para el balance de la memoria.

| Historia | Criterios | Verificados | Verificados con desviación | Pendientes |
|---|---|---|---|---|
| HU-01 · Pedir cita (spec-001) | 20 (16 escenarios + 4 SC) | 20 | 0 | 0 |
| HU-02 · Abrir la orden (spec-002) | 16 (12 + 4) | 16 | 0 | 0 |
| HU-04 · Responder al presupuesto (spec-004) | 15 (11 + 4) | 14 | 1 | 0 |
| HU-08 · Tablero (spec-008) | 17 (13 + 4) | 16 | 1 | 0 |
| **Total de la Persona A** | **68** | **66** | **2** | **0** |
| HU-03 · Diagnóstico y presupuesto (spec-003) | 14 (10 + 4) | 13 | 1 | 0 |
| HU-05 · Imputar (spec-005) | 13 (9 + 4) | 11 | 2 | 0 |
| HU-06 · Pedidos (spec-006) | 19 (13 + 6) | 18 | 1 | 0 |
| HU-07 · Factura e historial (spec-007) | 12 (8 + 4) | 10 | 2 | 0 |
| **Total de la Persona B** | **58** | **52** | **6** | **0** |
| **Total del grupo** | **126** | **118** | **8** | **0** |

Desviaciones de la Persona B: cinco criterios de tiempo (003 SC-001, 005 SC-003, 006 SC-001, 007
SC-001 y SC-004) se midieron con un navegador automático (tiempo de máquina, con los campos y clics que
necesita una persona) y no con una persona cronometrada ([matriz B](recorridos/matriz-B/informe.md)), y
la aceptación de la ampliación de 005 US3-1 se simuló con SQL. La salida de las pruebas automáticas de
sus cuatro specs (66 pruebas, todas en verde) se obtuvo sobre la rama `entrega`, commit `48d884b`, el
12/11/2026.

**Revisión (apartado 5.2)**: la Persona B revisó el 12/11/2026 un criterio de cada historia de la
Persona A (ver abajo). Las demás filas de la Persona A siguen pendientes de revisión. La Persona A
revisó el 12/11/2026, de la misma forma, un criterio de cada historia de la Persona B (ver abajo); sus
otras 54 filas siguen pendientes de revisión.

### Revisión de la Persona B sobre las filas de la Persona A (12/11/2026)

Para cada historia se eligió un criterio, se leyó la prueba citada, se comprobó que sus
aserciones cubren lo que dice el criterio y se ejecutó sobre `48d884b` (las cuatro pasan).

| Historia · criterio | Qué se comprobó | Resultado |
|---|---|---|
| HU-01 · US1-4 (cita pendiente) | `prueba_7_una_sola_cita_pendiente` comprueba, con una cita de hoy y otra futura, que el `GET` público da `cita_pendiente: true` sin la fecha y que pedir otra da 409 con el texto exacto «Este vehículo ya tiene una cita pendiente.» La parte de pantalla (Confirmar desactivado) la cubre T032, fila 9 | Correcta |
| HU-02 · US2-1 (km menor) | `prueba_6_kilometraje` comprueba el 422 con «…visita anterior (85.000 km)», que no se crea la orden ni se gasta número, y que la «visita anterior» es la más reciente y no la de más km | Correcta |
| HU-04 · US4-1 (sin presupuesto aceptado no se imputa) | `sin_presupuesto_aceptado_no_se_imputa` usa órdenes «en diagnóstico», así que el 409 sale de la comprobación del estado de spec-005 (lo confirma su [mutación](004-aceptar-presupuesto-enlace/evidencias/T023-mutacion.txt)). La otra rama, una orden «en reparación» sin presupuesto aceptado → 409 «La orden no tiene un presupuesto aceptado», la cubre spec-005 (`solo_se_imputa_en_reparacion_con_presupuesto_aceptado`, T014), que también pasa | Correcta; se podría citar también la prueba de spec-005 en la fila (**aplicado** el 12/11/2026) |
| HU-08 · US1-2 (la facturada no aparece) | `prueba1_seis_columnas_cada_orden_en_la_suya_y_sin_facturadas` crea una orden por estado más una facturada y comprueba que hay 6 tarjetas y que la facturada no está; la desaparición al facturar en directo la cubre T028, fila 10 | Correcta |

### Revisión de la Persona A sobre las filas de la Persona B (12/11/2026)

Para cada historia se eligió un criterio, se leyó la prueba citada, se comprobó que sus aserciones
cubren lo que dice el criterio y que la evidencia enlazada la da por superada. El detalle, con la
revisión de los cambios compartidos de la Persona B, está en
[revision-cruzada.md](revision-cruzada.md).

| Historia · criterio | Qué comprueba la prueba | Evidencia | Resultado |
|---|---|---|---|
| HU-03 · US2-5 (quitar una línea recalcula los totales) | `quitar_una_linea_recalcula_los_totales`: parte del presupuesto del ejemplo (1,5 h + PF-123 × 2), quita la mano de obra y comprueba que queda solo la pieza con base 50,00, IVA 10,50 y total 60,50; quitarla otra vez da 404 y otro mecánico recibe 403 | [pruebas de 003](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt): `ok` (20 de 20) | Correcta |
| HU-05 · US2-1 (el 110 % justo no bloquea) | `el_limite_del_10_por_ciento_es_exacto_al_centimo`: con 100,00 € presupuestados, una pieza de 110,00 € deja la orden «en reparación» y otra de 110,01 €, en otra orden, la pasa a «pendiente de conformidad» | [pruebas de 005](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt): `ok` (12 de 12) | Correcta |
| HU-06 · US3-2 (con dos pedidos, la orden espera al último) | `con_dos_pedidos_la_orden_espera_al_ultimo`: al recibir el primero la orden sigue «en espera de piezas» y el tablero no recibe ningún evento; al recibir el segundo vuelve a «en reparación» y se publica `{id, estado: "en_reparacion"}` | [pruebas de 006](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt): `ok` (23 de 23) | Correcta; además comprueba el evento que usa spec-008 |
| HU-07 · US1-1 (factura de 157,30 €) | `facturar_copia_las_imputaciones_y_cierra_la_orden`: las dos líneas (2 h × 40,00 = 80,00 y PF-123 × 2 × 25,00 = 50,00), base 130,00, IVA 21 % 27,30 y total 157,30; número 1, fecha de hoy, cliente, matrícula y taller; la orden pasa a «facturada» y se publica el evento | [pruebas de 007](007-facturar-avisar-cliente/evidencias/matriz/pruebas.txt): `ok` (11 de 11) | Correcta |
