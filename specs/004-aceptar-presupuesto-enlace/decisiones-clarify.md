# Registro de decisiones del clarify · spec-004 · Aceptar o rechazar el presupuesto desde un enlace (HU-04)

**Sesión**: 2026-11-04 · **Spec**: [spec.md](spec.md) (sección «Clarifications», sesión 2026-11-04)
· **Constitución**: 1.2.1

Cada pregunta se hizo de una en una, con una opción recomendada. La respuesta de cada una está
integrada en la spec, en los requisitos y escenarios que se indican. Q1 cierra el apartado a del
hallazgo I5 de [spec-002](../002-abrir-orden-reparacion/hallazgos.md); Q2 y Q4 cierran I5 y D1 de la
[base común](../000-base-comun/hallazgos.md).

| ID | Historia y criterio afectados | Pregunta | Opciones consideradas | Respuesta | Justificación | Autora | Revisor |
|----|-------------------------------|----------|-----------------------|-----------|---------------|--------|---------|
| Q1 | HU-04 · FR-004, SC-002, escenario 3 de la US2 (nuevo) · hallazgo I5 de spec-002, apartado a | ¿Qué respuestas del cliente debe enseñar la ficha de la orden, con su fecha y hora, y de dónde sale ese dato? | A) añadir `respondido_en` a `PresupuestoResumen` y listar todas las respuestas de la orden, indicando si es inicial o ampliación; B) usar el `respondido_en` del `Presupuesto` completo que carga la sección: solo se ve la respuesta del último presupuesto; C) pedir el `Presupuesto` completo de cada presupuesto de la orden | **A** (la recomendada). Además se actualiza `PresupuestoResumen` en el contrato común y en `tipos.ts` | SC-002 pide que «El 100 % de las respuestas quedan registradas con fecha y hora visibles en la orden», y una orden puede tener varios presupuestos: uno rechazado y otro nuevo, o uno aceptado y una ampliación. **B se descarta** porque, en cuanto el mecánico crea otro presupuesto, la respuesta anterior deja de verse y SC-002 no se cumpliría. **C se descarta** porque hace una petición por presupuesto para un dato que cabe en el resumen (principio I). La ficha es el sitio donde el taller consulta la respuesta (US1, US2), y cada respuesta se comprueba en pantalla (principio IV) | Persona A | Persona B (pendiente) |
| Q2 | HU-04 · escenario 1 de la US2 · máquina de estados del data-model común · hallazgo I5 de la base común | Cuando el cliente rechaza el presupuesto inicial, la orden sigue «en diagnóstico». ¿Debe quedar en la máquina de estados una transición de «en diagnóstico» a «en diagnóstico», o se elimina? | A) eliminarla de `TRANSICIONES` y del data-model (quedan 8) y corregir «vuelve» por «sigue»; B) mantenerla y usarla: el rechazo publica un evento aunque el estado no cambie; C) mantenerla sin usarla | **A** (la recomendada). `tasks.md` y los analyze de la base común no se reescriben: son registro histórico | El rechazo no cambia el estado de la orden, así que no hay transición que registrar: la orden nunca salió de «en diagnóstico». **B se descarta** porque publicaría en el tablero un evento sin ningún cambio: la tarjeta no se mueve, y la decisión se ve en la ficha (Q1). **C se descarta** porque deja en un fichero compartido una regla que nadie usa (principios I y III). La redacción «vuelve» de US2-1 y del quickstart §4.5 inducía a pensar que había un cambio de estado | Persona A | Persona B (pendiente) |
| Q3 | HU-04 · FR-003, FR-005, FR-009, US3, casos límite | Cuando un presupuesto ya ha caducado, ¿puede el cliente rechazarlo desde el enlace, o un presupuesto caducado no admite ninguna respuesta? | A) ninguna respuesta: sin botones, y cualquier intento da 409 con el aviso de caducidad; B) se puede rechazar, pero no aceptar: la página enseña solo «Rechazar» | **A** (la recomendada). La regla vale igual para el presupuesto inicial y para las ampliaciones (FR-009) | Un presupuesto caducado ya no obliga a nada: rechazarlo no cambia el estado de la orden y el mecánico puede emitir otro igualmente. **B se descarta** porque añade un botón y una condición distinta para cada decisión para registrar algo sin efecto (principios I y III). El aviso «Contacte con el taller» ya da la salida al cliente. Que el texto del 409 sea el de la página evita dos mensajes para lo mismo | Persona A | Persona B (pendiente) |
| Q4 | HU-04 · FR-003, FR-005, escenario 4 de la US1 (nuevo), casos límite · hallazgo D1 de la base común | Para que «solo cuenta la primera respuesta» se cumpla también con dos pestañas a la vez, ¿qué condición debe llevar el `UPDATE` que guarda la respuesta, y dónde queda escrita? | A) una sentencia con la condición completa (`token`, `estado = 'emitido'`, `caduca_el >= $hoy`), escrita en research R9, y el plan de 004 remite a ella; B) la de R9 (`respondido_en IS NULL`) y comprobar la caducidad aparte, en el código | **A** (la recomendada). Cuando no se actualiza ninguna fila, el motivo se averigua con el mismo `$hoy`: token inexistente → 404; ya tiene respuesta → 409 «ya respondido»; si no → 409 «caducó el …». El cambio de estado de la orden va en la misma transacción. Prueba de concurrencia con dos respuestas a la vez, comprobada con mutación: debe fallar si se quita `estado = 'emitido'` | Una sola sentencia comprueba y guarda a la vez, así que entre las dos operaciones no hay hueco por el que colarse: de dos respuestas simultáneas, la base de datos deja pasar a una y a la otra no le actualiza ninguna fila. **B se descarta** porque separa la comprobación de la caducidad de la escritura y son dos pasos donde basta uno (principio I). `caduca_el >= $hoy` es lo contrario de `caducado()` de spec-003, así que el criterio es el mismo en la ficha y en la respuesta, con «hoy» en Europe/Madrid (R12). Guardar la respuesta y cambiar la orden en la misma transacción impide una respuesta aceptada con la orden sin cambiar. Una sola versión en R9 cierra D1 | Persona A | Persona B (pendiente) |
| Q5 | HU-04 · FR-008, escenario 5 de la US1 (nuevo), caso límite «El correo no llega» | ¿Quién del personal puede ver y copiar el enlace del presupuesto en la orden, y durante cuánto tiempo? | A) todo el personal con sesión, solo mientras esté emitido y vigente; B) solo recepción, el jefe y el mecánico asignado, mientras esté vigente; C) todo el personal, siempre que se haya emitido | **A** (la recomendada). Se oculta **solo en la pantalla**: `GET /api/presupuestos/{id}` sigue enviando `enlace`, porque tras la respuesta o la caducidad ya no permite cambiar nada (la respuesta da 409). El enlace sigue funcionando para el cliente, que ve su decisión (US1-3) | FR-008 dice «el personal», y el caso límite lo pide para cuando «El correo no llega»: el enlace solo tiene utilidad mientras se puede responder. **B se descarta** porque restringe lo que FR-008 no restringe y añade una regla por rol en la API (principios I y III). **C se descarta** porque mostraría un enlace que ya no sirve para responder. Ocultarlo solo en pantalla no abre ningún riesgo: quien tenga el enlace después de la respuesta o la caducidad no puede cambiar nada. El enlace nunca da datos de contacto del cliente (principio V) | Persona A | Persona B (pendiente) |

## Criterios de aceptación resultantes

Cada criterio está escrito igual que el escenario de [spec.md](spec.md) que se indica entre
corchetes; los marcados como «nuevo» se añadieron a la spec en esta sesión.

### Q1 · Todas las respuestas en la ficha (FR-004, SC-002)

- **Dado** un presupuesto inicial rechazado y después otro aceptado en la misma orden, **cuando** el
  personal consulta la orden, **entonces** ve las dos respuestas, cada una con su fecha y hora y
  diciendo si es inicial o ampliación, aunque ya exista un presupuesto más reciente. [US2, escenario
  3, nuevo]

### Q2 · El rechazo no cambia el estado (máquina de estados)

- **Dado** un presupuesto vigente, **cuando** el cliente lo rechaza, **entonces** se guarda
  «Rechazado» con fecha y hora exactas y la orden sigue «en diagnóstico» para que el mecánico pueda
  preparar otro presupuesto. [US2, escenario 1, con «sigue» en lugar de «vuelve a»]

### Q3 · Un presupuesto caducado no admite respuesta (FR-005, FR-009)

- **Dado** un presupuesto caducado ayer, inicial o ampliación, **cuando** el cliente abre el enlace,
  **entonces** ve «Este presupuesto caducó el dd/mm/aaaa. Contacte con el taller», sin botones, y no
  puede aceptarlo ni rechazarlo. [US3, escenario 1, ampliado]
- **Dado** un presupuesto que caduca mientras el cliente lo tiene abierto, **cuando** pulsa
  «Aceptar» o «Rechazar» y confirma, **entonces** ve el mismo aviso y no se guarda ninguna respuesta.
  [US3, escenario 2, nuevo]

### Q4 · Solo cuenta la primera respuesta (FR-003)

- **Dado** un presupuesto vigente abierto en dos pestañas, **cuando** el cliente confirma en las dos
  a la vez, **entonces** solo se guarda una respuesta, la orden cambia una sola vez, y la otra
  pestaña ve «Este presupuesto ya fue respondido». [US1, escenario 4, nuevo]

### Q5 · El enlace en la orden (FR-008)

- **Dado** un presupuesto emitido y vigente, **cuando** cualquier persona del personal con sesión
  abre la orden, **entonces** ve el enlace y «Copiar enlace»; y cuando el presupuesto ya tiene
  respuesta o ha caducado, la orden deja de mostrarlos, mientras el cliente sigue pudiendo abrir el
  enlace y ver la decisión tomada o el aviso de caducidad. [US1, escenario 5, nuevo]

## Cambios en ficheros compartidos (los revisa la Persona B)

Igual que T031 en spec-002: se revisan antes de fusionar, con una tarea propia en el `tasks.md` de
spec-004.

| Fichero | Cambio | Decisión | Estado |
|---------|--------|----------|--------|
| `backend/src/rutas/presupuestos.rs` (spec-003) | `respondido_en` en `PresupuestoResumen` | Q1 | **Pendiente**: se hace al implementar, con su prueba antes |
| `frontend/src/lib/tipos.ts` | `respondido_en` en `PresupuestoResumen` | Q1 | Hecho. Hasta el cambio anterior, el tipo declara un campo que el servidor aún no envía; nada lo usa todavía |
| `specs/000-base-comun/contracts/api.md` | `PresupuestoResumen` con `respondido_en`; texto completo del 409 de caducidad, para las dos decisiones | Q1, Q3 | Hecho |
| `backend/src/estados.rs` y `backend/tests/spec000_estados.rs` | Sin la autotransición: 8 transiciones, con la prueba `en_diagnostico_no_tiene_transicion_a_si_mismo` | Q2 | Hecho |
| `specs/000-base-comun/data-model.md` | Sin la fila de la autotransición; regla transversal sobre el rechazo | Q2 | Hecho |
| `specs/000-base-comun/quickstart.md` | §4.5: «sigue» en lugar de «vuelve a»; lo revisa B por estar junto a su sección de spec-003 | Q2 | Hecho |
| `specs/003-diagnostico-presupuesto/research.md` | La nota sobre la autotransición dice que se quitó | Q2 | Hecho |
| `specs/000-base-comun/research.md` | R9: única versión del `UPDATE` y orden de los motivos | Q4 | Hecho |
| `specs/000-base-comun/hallazgos.md` | I5 y D1 cerrados | Q2, Q4 | Hecho |

## Comando y versión

- **Comando**: `/speckit-clarify` con este argumento:

  > Revisa las ambigüedades de HU-04 en specs/004-aceptar-presupuesto-enlace/spec.md. Además de lo
  > que detectes, resuelve estos pendientes asignados a spec-004: (1) I5 de spec-002 (hallazgos.md,
  > apartado a): el quickstart común §4.3 espera ver en la orden «Aceptado el … a las …», pero
  > PresupuestoResumen (presupuestos.rs, el campo presupuestos de GET /api/ordenes/{id}) no trae
  > respondido_en; el objeto Presupuesto completo sí; hay que decidir de dónde sale ese dato en la
  > ficha de la orden. (2) I5 de la base común (specs/000-base-comun/hallazgos.md): la autotransición
  > en_diagnostico → en_diagnostico está en TRANSICIONES de estados.rs, pero el plan de 004 dice que
  > el rechazo del presupuesto inicial es «sin transición»; hay que decidir si sobra. (3) D1 de la
  > base común: hay dos versiones del UPDATE de «solo cuenta la primera respuesta» (research R9 y el
  > plan de 004); debe quedar una sola. Ten en cuenta lo que ya hace spec-003 (en main): el correo
  > con el enlace se envía al emitir, caduca_el = hoy + 15 días, la función caducado() de
  > presupuestos.rs y el comentario de Presupuesto.svelte que reserva «Copiar enlace» para spec-004
  > (FR-008). Haz las preguntas de una en una y espera la respuesta. Guarda el registro de decisiones
  > en specs/004-aceptar-presupuesto-enlace/decisiones-clarify.md, como en 001 y 002. No hagas commit.

  Q3 y Q5 las añadió el asistente al revisar la spec. Las respuestas de Q1, Q2, Q3 y Q4 llevaron
  además indicaciones de la autora, recogidas en la columna «Respuesta». Durante la sesión se
  hicieron también, a petición de la autora, los cambios de la tabla anterior; el de `estados.rs`,
  con una prueba que falló en rojo antes de quitar la transición.
- **Spec Kit**: plantillas del proyecto inicializadas con la versión 1.0.6
  (`.specify/init-options.json`); CLI `specify` instalada: 1.0.8.dev0.
