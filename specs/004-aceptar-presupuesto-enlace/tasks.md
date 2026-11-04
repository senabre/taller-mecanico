---

description: "Tareas de spec-004 · Aceptar o rechazar el presupuesto desde un enlace (HU-04)"
---

# Tasks: spec-004 · Aceptar o rechazar el presupuesto desde un enlace (HU-04)

**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md) (D1–D8),
[data-model.md](data-model.md), [contracts/api.md](contracts/api.md) y
[quickstart.md](quickstart.md), que remiten a los documentos comunes de
[000-base-comun](../000-base-comun/).

**Responsable**: Persona A · **Revisor**: Persona B

**Tests**: se incluyen. El plan maestro (research R13) exige pruebas `#[sqlx::test]` para cada regla
de negocio, y el plan de esta spec enumera 12 en `backend/tests/spec004_respuesta.rs` (aquí «prueba
1»…«prueba 12»). Cada prueba se escribe **antes** de la tarea que la hace pasar y debe fallar hasta
entonces. Si una prueba protege una regla que ya existe y no puede fallar, se demuestra con una
**mutación**: se rompe la regla un momento, la prueba falla, se restaura y se comprueba con `cmp`
que el fichero queda igual.

**Trazabilidad**: cada tarea termina con «**Cubre:**», que indica los requisitos (FR-xxx), los
escenarios (US1-n, US2-n, US3-n, US4-n) y los criterios de éxito (SC-xxx) de la spec que atiende.
FR-001 (el correo con el enlace) ya lo cumple spec-003 al emitir y no tiene tarea aquí.

**Cambios compartidos**: las tareas marcadas «**cambio compartido · revisa Persona B**» tocan
ficheros de la base común o de la Persona B. Cada una va en **su propio commit**, sin mezclarse con
el resto de spec-004, y la Persona B la revisa en la tarea final (T031). T014 y T016 tocan ficheros suyos:
se le avisa antes de tocarlos. T014, además, lo revisa en cuanto esté, sin esperar a T031.

**Fuera de estas tareas**: el §4.5 del quickstart común emite un presupuesto en la orden de `1414NNN`
sin un paso previo de diagnóstico. Es el apartado b del hallazgo I5 de spec-002, asignado a la
Persona B (spec-003; G1 de [hallazgos.md](hallazgos.md)).

**Organización**: una fase por historia de usuario, por prioridad: US1, US2 y US4 (P1) y US3 (P2).
Todo texto visible, en español de España y con trato de usted; importes con `euros()`; fechas
dd/mm/aaaa y horas de Madrid; «hoy» siempre con `fechas::hoy()` (R12). Las pruebas crean sus propios
datos y no usan valores demo (principio V).

## Format: `[ID] [P?] [Story] Description`

- **[P]**: se puede hacer en paralelo (otro fichero, sin depender de tareas pendientes)
- **[Story]**: historia de usuario (US1, US2, US3, US4)

## Path Conventions

Aplicación web: `backend/src/`, `backend/tests/`, `backend/seed/`, `frontend/src/`. No hay
migraciones nuevas: `presupuesto` ya tiene `estado`, `token`, `caduca_el` y `respondido_en`.

---

## Phase 1: Setup (cambios compartidos)

**Purpose**: los datos demo y los tipos que necesitan las historias.

- [X] T001 **Cambio compartido · revisa Persona B · commit propio**. En `backend/seed/demo.sql`, según research D7: (a) en la parte 3, cambiar la cita de `7777JJJ` de `8` a `18` días; (b) añadir una **parte 6** al final, antes del `COMMIT` y **después** de la parte 5, con un `presupuesto` de la orden demo 7: `tipo = 'ampliacion'`, `estado = 'emitido'`, `token = 'demo-presupuesto-caducado'`, `iva_porcentaje` 21,00, `base` 40,00, `iva` 8,40, `total` 48,40, `emitido_en` = hace 16 días a las 10:00 y `caduca_el` = ayer, con las fechas en Europe/Madrid (`(now() AT TIME ZONE 'Europe/Madrid')::date`, como la parte 5); (c) en la parte 6, su **propio** `INSERT` de `linea_presupuesto`, filtrado por ese token: `mano_obra`, «Sustitución de discos», 1,00 × 40,00 = 40,00. No debe recibir las dos líneas de la parte 4, cuyo `CROSS JOIN` solo alcanza a los presupuestos que existen en ese momento. (d) Documentar el token como ficticio en la cabecera de la parte 6 (principio V). Comprobar con consultas, sobre una base recién migrada y cargada: la orden 7 sigue `pendiente_conformidad`, con 110,00 € presupuestados y 130,00 € imputados; la ampliación tiene **una** línea y `emitido_en` es posterior a sus imputaciones; `SELECT EXISTS (… estado = 'emitido' AND caduca_el >= hoy)` da `false` para esa orden; el contador de órdenes sigue en 9. **Cubre:** US1-5, US3-1, SC-003 (dato para verificarlos en el quickstart común §4.6 y §4.8). *(2026-11-04: rojo con el seed anterior: sin ampliación y cita sin los 18 días. Tras el cambio, base reiniciada y seed cargado sin errores: orden 7 `pendiente_conformidad`, 110,00 € presupuestados y 130,00 € imputados; ampliación con una línea, caduca ayer, 15 días después de emitirse y posterior a las imputaciones; no bloquea otra; contador de órdenes en 9.)*
- [X] T002 [P] **Cambio compartido · revisa Persona B · commit propio**. En `frontend/src/lib/tipos.ts`, añadir en una sección «spec-004 · Respuesta del cliente», sin tocar las demás: `PresupuestoPublico extends Totales` con `{taller: {nombre: string}, matricula: string, marca: string, modelo: string, diagnostico: string | null, tipo: TipoPresupuesto, lineas: Linea[], caduca_el: Fecha, caducado: boolean, decision: null | 'aceptado' | 'rechazado', respondido_en: Instante | null}`, **sin ningún dato del cliente**. `respondido_en` en `PresupuestoResumen` ya está hecho en el clarify. `npm run check`: 0 errores. **Cubre:** FR-002, FR-004. *(2026-11-04: rojo con un fichero temporal que importaba el tipo, ya borrado; verde con `npm run check`, 0 errores.)*

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: el módulo de rutas y la base de las pruebas, que necesitan todas las historias.

**⚠️ CRITICAL**: ninguna historia puede empezar hasta que esta fase esté terminada.

- [X] T003 Crear `backend/src/rutas/publico_presupuesto.rs` con `rutas()`, que registra `GET /publico/presupuestos/{token}` y `POST /publico/presupuestos/{token}/respuesta`, los dos respondiendo provisionalmente 501. Registrarlo en `backend/src/rutas/mod.rs` con `pub mod publico_presupuesto;` y `.merge(publico_presupuesto::rutas())`, después de las que ya hay, sin tocar las demás (**cambio compartido · revisa Persona B · commit propio**: esas dos líneas solas, en un commit posterior al del módulo). **Cubre:** FR-002 (infraestructura). *(2026-11-04: compila y las dos rutas responden 501. Sin commit todavía: `rutas/mod.rs` va en el suyo, después del del módulo.)*
- [X] T004 Crear `backend/tests/spec004_respuesta.rs` con `mod comun;` y utilidades propias: `presupuesto(pool, orden_id, tipo, estado, caduca_el, token) -> i64` (por SQL, con una línea de mano de obra y sus totales; `emitido_en` = ahora; escribe también un `diagnostico` en la orden, que `comun::crear_orden` deja a NULL); `estado_orden(pool, id) -> Estado`; `ver(app, token)` y `responder(app, token, decision)`, que devuelven `(StatusCode, Value)`. Reutilizar `comun::app_prueba`, `estado_prueba`, `crear_orden`, `insertar_usuario`, `crear_usuario`, `entrar` y `peticion`. **Cubre:** infraestructura de pruebas, sin requisito propio. *(2026-11-04: `presupuesto`, `estado_orden`, `ver` y `responder`, más `inicial_vigente`, `vigente`, `guardado` y `enviar`. Sin cambios en `tests/comun/mod.rs`.)*

**Checkpoint**: el backend compila con el módulo registrado y el fichero de pruebas existe.

---

## Phase 3: User Story 1 - El cliente acepta el presupuesto (Priority: P1) 🎯 MVP

**Goal**: el cliente abre el enlace, ve el presupuesto completo y lo acepta con un paso de
confirmación; la orden pasa a «en reparación» y el personal ve la respuesta y, mientras está vigente,
el enlace.

**Independent Test**: emitir un presupuesto, abrir el enlace, aceptar y comprobar en la orden el
estado «En reparación» y «Aceptado el … a las …» (quickstart común §4, pasos 1 a 4).

### Tests for User Story 1 ⚠️

- [ ] T005 [US1] **Prueba 1** (ver): un presupuesto emitido y vigente → `GET` 200 con `taller.nombre`, `matricula`, `marca`, `modelo`, `diagnostico`, `tipo`, `lineas`, `base`, `iva_porcentaje`, `iva`, `total`, `caduca_el`, `caducado: false`, `decision: null` y `respondido_en: null`. Comparar el objeto entero y comprobar que no aparece ningún dato del cliente: ni nombre, ni teléfono, ni correo. **Cubre:** FR-002, US1-1. *(2026-11-04: en rojo por el 501 que el `GET` mantiene con un presupuesto sin responder; espera a T012. Comprobado una vez quitando ese 501 a mano: la prueba pasa, y el fichero se restauró con `cmp`.)*
- [X] T006 [US1] **Prueba 8** (token inexistente): `GET` y `POST` con un token inventado → 404 «Este enlace no es válido», con el mismo cuerpo en los dos. **Cubre:** FR-002, caso límite «Alguien modifica el enlace a mano». *(2026-11-04: rojo con 501, verde con T013.)*
- [X] T007 [US1] **Prueba 9** (decisión): cuerpo `{}`, `decision` vacía o `"otra"` → 422 «Elija aceptar o rechazar», sin `detalle` (el contrato no lo pide) y sin guardar nada. Un cuerpo que no es JSON, o una `decision` que no es texto, sigue dando el 400 común de `error.rs`: no se prueba aquí. **Cubre:** FR-003. *(2026-11-04: rojo con 501, verde con T013.)*
- [ ] T008 [US1] **Prueba 2** (aceptar el inicial): con `estado_prueba` y `eventos.suscribir()`, una orden `en_diagnostico` y su presupuesto emitido, `POST` `aceptar` → 200 con `decision: "aceptado"` y `respondido_en`; en la tabla, `estado = 'aceptado'`; la orden pasa a `en_reparacion`; `tablero.try_recv()` da `CambioEstado { id, estado: EnReparacion }`; y `GET /api/ordenes/{id}` trae ese `respondido_en` en `presupuestos[0]`. **Cubre:** FR-004, FR-006, US1-2, SC-002. *(2026-11-04: rojo con 501. Con T013 ya pasan la decisión, el estado de la orden y el evento; falla solo en la ficha, `respondido_en` null, por la razón esperada. Espera a T014.)*
- [X] T009 [US1] **Prueba 3** (una sola vez): tras aceptar, `GET` da la decisión y su fecha; un segundo `POST` → 409 «Este presupuesto ya fue respondido» y `respondido_en` no cambia. **Cubre:** FR-003, US1-3. *(2026-11-04: rojo con 501, verde con T013.)*
- [X] T010 [US1] **Prueba 4** (dos respuestas a la vez): con una orden y un presupuesto nuevos en cada vuelta, lanzar con `tokio::join!` dos `POST` `aceptar` → un 200 y un 409 «Este presupuesto ya fue respondido», y la orden cambia una sola vez (un único evento). Repetir 20 veces. **Comprobación obligatoria por mutación**, después de T013: copiar `backend/src/rutas/publico_presupuesto.rs`, quitar `estado = 'emitido'` del `UPDATE`, comprobar que la prueba **falla**, restaurar y comprobar con `cmp` que el fichero queda igual; anotar el resultado en esta tarea. El 409 se compara con el texto exacto: con la mutación, `cambiar_estado` también da 409, pero con otro texto, y solo así se detecta. **Cubre:** FR-003, US1-4 (Q4). *(2026-11-04: rojo con 501, verde con T013 y estable en 5 ejecuciones más. Mutación ejecutada: sin `estado = 'emitido'` falla en la vuelta 1 con «La orden está en reparación; no se puede pasar a en reparación», restaurado y `cmp` correcto. Salida en [evidencias/T010-mutacion.txt](evidencias/T010-mutacion.txt).)*
- [X] T011 [US1] **Prueba 12** (enlace): tras responder, `GET /api/presupuestos/{id}` con sesión sigue trayendo `enlace` (Q5: se oculta solo en la pantalla). **Cubre:** FR-008, US1-5. *(2026-11-04: rojo con 501, verde con T013.)*

### Implementation for User Story 1

- [ ] T012 [US1] En `backend/src/rutas/publico_presupuesto.rs`, implementar `GET /api/publico/presupuestos/{token}` (research D1): una consulta por `token` que une `presupuesto`, `orden` y `vehiculo` y **no une con `cliente`**, más las líneas; `taller.nombre` de la configuración; `decision` sale de `estado`; `caducado` llamando a `presupuestos::caducado()`, que T014 hace `pub(crate)`, sin copiar la regla (una sola definición, principio I). Token inexistente → 404 «Este enlace no es válido». Hace pasar T005 y la parte `GET` de T006. **Cubre:** FR-002, US1-1. *(2026-11-04: **pendiente del acuerdo de la Persona B (I1)**. Adelantado con T013: la lectura pública y el 404, y el `GET` de un presupuesto ya respondido. Falta llamar a `caducado()` en lugar del `caducado: false` provisional y quitar el 501 con un presupuesto sin responder.)*
- [X] T013 [US1] En `backend/src/rutas/publico_presupuesto.rs`, implementar `POST /api/publico/presupuestos/{token}/respuesta` (research D2): (1) `decision` (`Option<String>`, para que `{}` llegue aquí) es `aceptar` o `rechazar`, si no 422 «Elija aceptar o rechazar»; (2) transacción con el `UPDATE` de [research R9](../000-base-comun/research.md), con todas sus condiciones (`token`, `estado = 'emitido'`, `caduca_el >= $hoy`) y `$hoy` = `fechas::hoy()`; (3) si no actualiza ninguna fila: token inexistente → 404; si no → 409 «Este presupuesto ya fue respondido»; (4) `aceptar` en un presupuesto inicial: `cambiar_estado(en_diagnostico → en_reparacion)` en la misma transacción; (5) `commit`, y después publicar el `CambioEstado`; responder 200 con lo mismo que el `GET`. **Provisional**: `rechazar` responde 501 hasta T021, y aceptar una ampliación también; el 409 de caducidad llega con T025, y hasta entonces un caducado da «ya fue respondido». Hace pasar T006, T007, T009 y T010, y la respuesta de T008. **Cubre:** FR-003, FR-004, FR-006, US1-2, US1-3, US1-4. *(2026-11-04: **hecha antes que T012**, porque T012 espera el acuerdo de la Persona B (I1). Quedan los 501 previstos: `rechazar` y aceptar una ampliación hasta T021. El 409 de caducidad llega con T025.)*
- [ ] T014 [US1] **Cambio compartido · revisa Persona B · commit propio**. **Avisar a la Persona B antes de tocar el fichero** (I1). En `backend/src/rutas/presupuestos.rs` (de la Persona B, spec-003), dos cambios en un mismo commit: (a) **antes de T012**, `caducado()` pasa a `pub(crate)`, sin cambiar su cuerpo; (b) **después de T013**, cuando T008 llega a la ficha y falla por ese campo, añadir `respondido_en: Option<DateTime<Utc>>` a `PresupuestoResumen` y rellenarlo en `presupuestos_de_orden`; `SELECT_PRESUPUESTO` ya lo lee. El commit va antes que el de T012 y T013, para que cada commit compile, y la Persona B lo revisa en cuanto esté, sin esperar a T031. `cargo test --test spec003_presupuestos` debe seguir pasando. Hace pasar la parte de la ficha de T008. **Cubre:** FR-004, FR-005, US1-2, SC-002 (Q1).
- [ ] T015 [P] [US1] Crear `frontend/src/routes/presupuesto/[token]/+page.svelte` (pública; D5 y D8): nombre del taller, si es presupuesto inicial o ampliación, vehículo, diagnóstico, `TablaLineas` dentro de `table-wrap`, `Totales` y caducidad. Con un presupuesto vigente, «Aceptar»; al pulsarlo, **dentro de la página y sin `confirm()`**, «¿Acepta este presupuesto por {total con `euros()`}? Una vez confirmado, no se podrá cambiar.» con «Sí, aceptar» y «Volver», que deja la página como estaba sin llamar al servidor. Con `decision`, «Aceptado el dd/mm/aaaa a las HH:mm» (`fechaDe` y `hora`), sin botones. Con un 409, muestra el mensaje y vuelve a pedir el presupuesto. Con un 404, solo «Este enlace no es válido». Si falla la conexión al cargar, «No se pudo cargar el presupuesto»; al responder, «Ha ocurrido un error inesperado». A 390 px, sin desplazamiento horizontal de la página. **Sin nombre, teléfono ni correo del cliente.** **Cubre:** FR-002, FR-003, US1-1, US1-2, US1-3, US1-4, SC-001.
- [ ] T016 [US1] **Cambio compartido · revisa Persona B · commit propio**. **Avisar a la Persona B antes de tocar el fichero** (I1, N1). En `frontend/src/routes/ordenes/[id]/Presupuesto.svelte` (de la Persona B, spec-003; research D6): (a) una línea por cada elemento de `presupuestos` con `respondido_en`: «Inicial · Aceptado el 16/10/2026 a las 10:32» o «Ampliación · Rechazado el …»; (b) con `presupuesto` en estado `emitido` y sin `caducado`, `enlace` en un campo de solo lectura y un botón «Copiar enlace» (`navigator.clipboard`) que pasa a «Copiado» un momento; con otro estado no se muestra. Quitar del comentario de cabecera que lo añade spec-004. **Cubre:** FR-004, FR-008, US1-2, US1-5, SC-002, caso límite «El correo no llega».

**Checkpoint**: las pruebas 1, 2, 3, 4, 8, 9 y 12 pasan; un cliente acepta desde el enlace y el
personal lo ve en la orden.

---

## Phase 4: User Story 2 - El cliente rechaza el presupuesto (Priority: P1)

**Goal**: el cliente puede rechazar con la misma comodidad; el taller lo ve en la orden, que no cambia
de estado, y el mismo mecanismo sirve para las ampliaciones.

**Independent Test**: rechazar un presupuesto desde el enlace y comprobar en la orden el rechazo con
su fecha y hora, y que sigue «En diagnóstico» (quickstart común §4, pasos 5 y 7).

### Tests for User Story 2 ⚠️

- [ ] T017 [US2] **Prueba 5** (rechazar el inicial): `POST` `rechazar` → 200 con `decision: "rechazado"` y `respondido_en`; en la tabla, `estado = 'rechazado'`; la orden sigue `en_diagnostico`; `tablero.try_recv()` está vacío. **Cubre:** FR-003, FR-004, US2-1 (Q2).
- [ ] T018 [US2] **Prueba 6** (ampliación): con una orden `pendiente_conformidad` y una ampliación emitida, `aceptar` → `en_reparacion` con su evento; en otra, `rechazar` → sigue `pendiente_conformidad`, sin evento. **Cubre:** FR-006, FR-009, US2-2.
- [ ] T019 [US2] **Prueba 10** (todas las respuestas en la ficha): una orden con un inicial rechazado y otro aceptado → `GET /api/ordenes/{id}` trae los dos en `presupuestos`, cada uno con su `estado` y su `respondido_en`. **Cubre:** FR-004, US2-3, SC-002 (Q1).
- [ ] T020 [US2] Ampliar la **prueba 4** con la variante `aceptar` y `rechazar` a la vez: un 200 y un 409 «Este presupuesto ya fue respondido» (texto exacto), y el presupuesto queda con la decisión del 200: si gana aceptar, `aceptado` y la orden cambia una sola vez; si gana rechazar, `rechazado` y la orden no cambia. Repetir 20 veces. **Comprobación obligatoria por mutación**, después de T021: copiar `backend/src/rutas/publico_presupuesto.rs`, quitar `estado = 'emitido'` del `UPDATE`, comprobar que esta variante **falla**, restaurar y comprobar con `cmp` que el fichero queda igual; anotar el resultado en esta tarea. Es el caso grave: un rechazo no puede sobrescribir una aceptación. **Cubre:** FR-003, US1-4 (Q4).

### Implementation for User Story 2

- [ ] T021 [US2] En `POST` de `backend/src/rutas/publico_presupuesto.rs`, quitar los 501 provisionales de T013 (research D3): `rechazar` solo guarda la respuesta, sin `cambiar_estado` y sin evento, tanto en el inicial como en una ampliación (Q2); aceptar una ampliación hace `cambiar_estado(pendiente_conformidad → en_reparacion)`. El mismo código para los dos tipos: solo cambia el estado de origen. Hace pasar T017, T018, T019 y T020. **Cubre:** FR-003, FR-004, FR-006, FR-009, US2-1, US2-2, US2-3.
- [ ] T022 [US2] En `frontend/src/routes/presupuesto/[token]/+page.svelte`, añadir «Rechazar» junto a «Aceptar», con «¿Rechaza este presupuesto por {total}? Una vez confirmado, no se podrá cambiar.», «Sí, rechazar» y «Volver», y mostrar «Rechazado el dd/mm/aaaa a las HH:mm» sin botones. **Cubre:** FR-003, US2-1, US2-2.

**Checkpoint**: las pruebas 5, 6 y 10 pasan; aceptar y rechazar funcionan en presupuestos iniciales y
en ampliaciones.

---

## Phase 5: User Story 4 - Sin presupuesto aceptado no se trabaja (Priority: P1)

**Goal**: comprobar la regla de oro del taller. No hay nada que implementar: `imputar` (spec-005) tiene
dos barreras, el estado (solo `en_reparacion`) y que haya un presupuesto aceptado.

**Independent Test**: en una orden con presupuesto emitido sin respuesta, comprobar que no aparece la
opción de imputar (quickstart común §4, paso 4).

### Tests for User Story 4 ⚠️

- [ ] T023 [US4] **Prueba 11** (sin presupuesto aceptado no se imputa): con un presupuesto emitido, uno rechazado y uno caducado, cada uno en su orden `en_diagnostico`, `POST /api/ordenes/{id}/imputaciones` como su mecánico → 409 con el texto exacto «La orden está en diagnóstico; solo se puede imputar en órdenes en reparación», sin crear imputaciones. La regla ya existe, así que la prueba pasa al escribirla: **demostrarlo por mutación**, quitando temporalmente en la función `imputar` de `backend/src/rutas/imputaciones.rs` la condición `estado_orden != Estado::EnReparacion` (no la de `terminar`). La segunda barrera sigue dando 409, con «La orden no tiene un presupuesto aceptado», así que la prueba debe fallar **por el texto**. Restaurar y comprobar con `cmp` que el fichero queda igual. **Cubre:** FR-007, US4-1, SC-004.

**Checkpoint**: la prueba 11 pasa y detecta que falte la regla.

---

## Phase 6: User Story 3 - Presupuesto caducado (Priority: P2)

**Goal**: un presupuesto caducado no admite ninguna respuesta, ni inicial ni ampliación, y el cliente
ve el aviso sin botones.

**Independent Test**: abrir `/presupuesto/demo-presupuesto-caducado` y comprobar el aviso sin botones
(quickstart común §4, paso 8).

### Tests for User Story 3 ⚠️

- [ ] T024 [US3] **Prueba 7** (caducidad), en un presupuesto inicial y en una ampliación: con `caduca_el` ayer, `GET` → `caducado: true`, y `aceptar` y `rechazar` → 409 «Este presupuesto caducó el dd/mm/aaaa. Contacte con el taller», sin guardar nada y sin cambiar la orden; con `caduca_el` hoy, se puede responder (el límite de `caduca_el >= $hoy`); y un presupuesto ya **respondido** con `caduca_el` pasado sigue enseñando su decisión, con `caducado: false`, y un nuevo `POST` da 409 «Este presupuesto ya fue respondido», no el de caducidad. Falla en rojo porque T013 responde «ya fue respondido». **Cubre:** FR-005, FR-009, US3-1, US3-2, SC-003 (Q3).

### Implementation for User Story 3

- [ ] T025 [US3] En `POST` de `backend/src/rutas/publico_presupuesto.rs`, completar el motivo de R9 cuando el `UPDATE` no actualiza ninguna fila, con el mismo `$hoy`: token inexistente → 404; ya tiene respuesta → 409 «Este presupuesto ya fue respondido»; si no → 409 «Este presupuesto caducó el dd/mm/aaaa. Contacte con el taller». Hace pasar T024. **Cubre:** FR-005, US3-1, US3-2, SC-003.
- [ ] T026 [US3] En `frontend/src/routes/presupuesto/[token]/+page.svelte`, con `caducado`, mostrar «Este presupuesto caducó el dd/mm/aaaa. Contacte con el taller» y ningún botón. Si caduca con la página abierta, el 409 de T015 vuelve a pedir el presupuesto y el aviso se ve **una sola vez**. **Cubre:** FR-005, US3-1, US3-2.

**Checkpoint**: las 12 pruebas del plan pasan; las cuatro historias funcionan juntas.

---

## Phase 7: Polish & Cross-Cutting Concerns

- [ ] T027 **Verificación en el navegador** del [quickstart común §4](../000-base-comun/quickstart.md), pasos 1 a 9, con la base de datos recién creada y cargada ese mismo día con `backend/seed/demo.sql`, después del §3. En **escritorio** y en **móvil a 390 px**, comprobando en móvil `/presupuesto/[token]` en sus tres estados (vigente, respondido y caducado), sin desplazamiento horizontal de la página. Comprobar en cada pantalla pública que no aparecen el nombre, el teléfono ni el correo del cliente. En el paso 6, captura de la orden 7 con «Caducado el …», sin el enlace ni «Copiar enlace». Guardar el informe y las capturas en `specs/004-aceptar-presupuesto-enlace/evidencias/`, usando la correspondencia de [quickstart.md](quickstart.md). Para el paso 5, escribir antes el diagnóstico de la orden de `1414NNN` como `luis` y anotarlo en el informe (I5(b), asignado a la Persona B). Si un paso falla, la spec no está terminada (principio IV): se registra el hallazgo y una tarea correctora. **Cubre:** FR-002, FR-003, FR-004, FR-005, FR-006, FR-007, FR-008, FR-009, US1-1, US1-2, US1-3, US1-4, US1-5, US2-1, US2-2, US2-3, US3-1, US4-1, SC-002, SC-003, SC-004.
- [ ] T028 **Cronometraje humano de SC-001**: una persona, en un navegador en español, cronometra el paso 2 del quickstart común §4 desde que abre el enlace en la primera pestaña hasta ver el mensaje con la fecha y la hora (menos de 1 minuto); la segunda pestaña no cuenta, porque se abre después, en el paso 7. Se hace **en el mismo recorrido de T027**, porque cada presupuesto solo se responde una vez; si ese paso ya está hecho, se recarga la base de datos y se repiten el §2 y el §3. Anota el tiempo en el informe de `specs/004-aceptar-presupuesto-enlace/evidencias/`. Los tiempos de un navegador automático no valen. **Cubre:** SC-001.
- [ ] T029 **Regresión del [quickstart común](../000-base-comun/quickstart.md) entero** con los datos demo nuevos de T001, sobre una base de datos recién cargada: §0 a §6 de principio a fin, incluidos los pasos que usan la orden 7 y la numeración de la Nº 10; §7, §8 y §9 cuando existan spec-007 y spec-008, dejándolo anotado como pendiente si aún no están. Confirmar lo que research D7 da por comprobado: la orden 7 sigue con 110,00 € presupuestados, se puede emitir otra ampliación y su tarjeta no cambia. Anotar el resultado en el informe de `evidencias/`. **Cubre:** US3-1 y la coherencia de los datos demo con las demás specs.
- [ ] T030 Ejecutar y dejar sin errores ni avisos: `cd backend && cargo test` (incluye `spec004_respuesta`, `spec000_estados`, `spec003_presupuestos` y `spec005_imputaciones`), `cargo clippy --all-targets -- -D warnings`, `cd frontend && npm run check` y `npm test`. Anotar los resultados en esta tarea. **Cubre:** las pruebas 1–12, es decir: FR-002 (1 y 8), FR-003 (3, 4, 5 y 9), FR-004 (2, 5 y 10), FR-005 (7), FR-006 (2 y 6), FR-007 (11), FR-008 (12), FR-009 (6 y 7), SC-002 (2 y 10), SC-003 (7), SC-004 (11).
- [ ] T031 **Revisión de la Persona B** de todo lo compartido antes de fusionar. **Código**: `backend/src/rutas/presupuestos.rs` (T014, con `caducado()` en `pub(crate)`; se revisa antes, en cuanto esté), `frontend/src/routes/ordenes/[id]/Presupuesto.svelte` (T016), `backend/src/rutas/mod.rs` (T003), `frontend/src/lib/tipos.ts` (T002 y `respondido_en` del clarify), `backend/seed/demo.sql` (T001), y `backend/src/estados.rs` con `backend/tests/spec000_estados.rs` (autotransición quitada en el clarify, Q2). **Documentos**: `specs/000-base-comun/contracts/api.md` (`PresupuestoResumen`, los textos del 404, del 409 y del 422 de spec-004 y la fila de `GET /api/piezas`, que vuelve a su tabla), `specs/000-base-comun/data-model.md` (sin la autotransición, regla sobre el rechazo), `specs/000-base-comun/quickstart.md` (§0 y §4, incluidos el §4.2 con el cronometraje de SC-001, el §4.5, junto a su sección de spec-003, el nuevo §4.6 y el §4.7 con la segunda pestaña), `specs/000-base-comun/research.md` (R9, única versión del `UPDATE`, y Q1 de su tabla inicial), `specs/000-base-comun/hallazgos.md` (I5 y D1) y `specs/003-diagnostico-presupuesto/research.md` (nota de la autotransición). Confirmar también que sus pruebas de spec-003, 005 y 006 siguen pasando y que la sección Presupuesto se ve bien en la ficha. Marcar «Persona B (revisado)» en [decisiones-clarify.md](decisiones-clarify.md). **Cubre:** FR-004, FR-008, FR-009 (coherencia de los documentos y ficheros compartidos).

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: T001 y T002 en paralelo, cada uno en su commit. T001 solo bloquea T027 y T029;
  T002 bloquea el frontend (T015, T022 y T026).
- **Foundational (Phase 2)**: T003 y T004. Bloquea todas las historias.
- **US1 (Phase 3)**: depende de la Phase 2. Es el MVP.
- **US2 (Phase 4)**: depende de T013, cuyos 501 quita, y de T015, cuya página amplía.
- **US4 (Phase 5)**: depende de T004. Se puede hacer en cualquier momento después.
- **US3 (Phase 6)**: depende de T013 y T015.
- **Polish (Phase 7)**: después de las historias, en el orden T027 → T028 → T029 → T030 → T031.
- **Fusión**: spec-002 no está en `main` y `spec-004` la lleva dentro, así que el orden es 001 → 002
  → 004. spec-007, sin fusionar, también toca `rutas/mod.rs` y `tipos.ts`: quien fusione después
  resuelve ese conflicto.

### Within Each User Story

- Las pruebas (T005–T011, T017–T020, T024) se escriben y fallan **antes** de su implementación.
- T023 protege una regla de spec-005 que ya existe: se demuestra por mutación, no en rojo.
- La mutación de T010 se hace después de T013 y la de T020, después de T021, cada una con el código
  restaurado y comprobado con `cmp`.
- T013 se hizo antes que T012, porque T012 necesita `caducado()` y espera el acuerdo de la Persona B
  (I1). Por eso el `GET` responde 501 con un presupuesto sin responder hasta T012.
- T014 se hace en dos pasos: `caducado()` antes de T012 y `respondido_en` después de T013, con un solo
  commit, que va delante del de T012 y T013.
- T013 deja dos huecos a propósito, para que las pruebas de US2 y US3 fallen de verdad: `rechazar` y
  aceptar una ampliación dan 501 hasta T021, y un caducado da «ya fue respondido» hasta T025.
- Backend antes que la pantalla que lo usa. El orden del `POST` es siempre: `decision` (422) →
  `UPDATE` de R9 → motivo si no actualiza (404 o 409) → efecto en la orden → `commit` → evento.

### Parallel Opportunities

- T001 y T002 entre sí.
- T015 (página pública) en paralelo con T012–T014, cuando T002 esté hecha.
- T023 (US4) en paralelo con cualquier fase posterior a la 2.
- Las pruebas van todas en `spec004_respuesta.rs`, así que se escriben una detrás de otra.

---

## Parallel Example: User Story 1

```bash
Task: "T013 POST en backend/src/rutas/publico_presupuesto.rs"
Task: "T015 Página pública en frontend/src/routes/presupuesto/[token]/+page.svelte"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. T001 y T002, cada uno en su commit y con la revisión de B.
2. Phase 2.
3. US1: el cliente acepta desde el enlace y la orden pasa a «En reparación», que es lo que desbloquea
   spec-005 en la aplicación. Se valida con el quickstart común §4, pasos 1 a 4.

### Incremental Delivery

1. US1 → aceptar (P1).
2. US2 → rechazar y ampliaciones (P1).
3. US4 → la prueba de la regla de oro (P1).
4. US3 → presupuesto caducado (P2).
5. Polish → navegador, cronometraje, regresión, comprobaciones y revisión de B.

---

## Notes

- Cada tarea se marca `[X]` al terminarla, con una nota de lo hecho y de las desviaciones.
- Las tareas de pantalla (T015, T016, T022 y T026) no se marcan hasta verificarlas en el navegador.
- Cada «cambio compartido · revisa Persona B» va en su propio commit.
- Si al implementar aparece algo que no está en la spec, se anota y se propone; no se construye
  (principio III).
