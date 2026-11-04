# Research: spec-004 · Aceptar o rechazar el presupuesto desde un enlace

Decisiones propias de spec-004 después del clarify (sesión 2026-11-04, Q1–Q5; registro en
[decisiones-clarify.md](decisiones-clarify.md)). Todo lo común (pila, fechas en Europe/Madrid,
errores, correo, eventos del tablero) está en
[000-base-comun/research.md](../000-base-comun/research.md) y no se repite aquí; en particular, el
`UPDATE` de la respuesta está en **R9**, que es su única versión. Fecha: 2026-11-04.

## D1. La página pública muestra solo lo que el cliente necesita

- **Decisión**: `GET /api/publico/presupuestos/{token}`, en `backend/src/rutas/publico_presupuesto.rs`,
  sin sesión, devuelve `{taller: {nombre}, matricula, marca, modelo, diagnostico, tipo, lineas,
  …Totales, caduca_el, caducado, decision, respondido_en}`. `decision` sale del estado del
  presupuesto: `null` si está emitido, `"aceptado"` o `"rechazado"` si no. `caducado` llama a
  `caducado()` de spec-003, que pasa a `pub(crate)`: una sola definición (principio I).
- **Motivo**: FR-002 y principio V: ningún dato del cliente (ni nombre, ni teléfono, ni correo) y
  ningún otro dato del taller. La consulta no une con `cliente`, así que no puede devolverlos.
- **Alternativas descartadas**: reutilizar `GET /api/presupuestos/{id}`, porque exige sesión y
  trae `enlace` y datos internos.

## D2. Guardar la respuesta: una sentencia y una transacción

- **Decisión**: `POST /api/publico/presupuestos/{token}/respuesta` valida `decision` (422) y, en una
  transacción, ejecuta el `UPDATE` de R9 con `$hoy` = `fechas::hoy()`. Si no actualiza ninguna fila,
  averigua el motivo con ese mismo `$hoy`, como dice R9: 404, 409 «ya respondido» o 409 «caducó el
  …». Si la actualiza, aplica D3 en la misma transacción, hace `commit` y publica el evento.
- **Motivo**: Q4. Dos respuestas a la vez no pueden pasar las dos: la segunda espera al bloqueo de
  la fila y, al seguir, `estado` ya no es `emitido`. Y una respuesta nunca queda guardada sin su
  efecto en la orden.
- **Prueba**: dos respuestas a la vez, con mutación en las dos variantes (aceptar y aceptar; aceptar y
  rechazar): debe fallar si se quita `estado = 'emitido'`.

## D3. Efecto en la orden

- **Decisión**:

  | Respuesta | Presupuesto inicial | Ampliación |
  |---|---|---|
  | Aceptar | `cambiar_estado`: `en_diagnostico → en_reparacion` | `cambiar_estado`: `pendiente_conformidad → en_reparacion` |
  | Rechazar | sin cambio de estado (Q2) | sin cambio de estado |

  Después del `commit`, aceptar publica el `CambioEstado` que devuelve `cambiar_estado`. Rechazar no
  publica nada: el estado no cambia y el tablero no tiene nada que actualizar.
- **Motivo**: FR-006, FR-009 y Q2. El mismo código sirve para los dos tipos: solo cambia el estado
  de origen (FR-009). `cambiar_estado` da 409 si la orden no está en ese estado y la transacción se
  deshace, así que tampoco se guarda la respuesta.

## D4. Texto del 404: «Este enlace no es válido»

- **Decisión**: un token que no existe responde 404 «Este enlace no es válido», igual en el `GET` y
  en el `POST`, y la página muestra ese mismo texto, sin más datos.
- **Motivo**: caso límite «Alguien modifica el enlace a mano» y principio V. El texto habla del
  enlace y no del presupuesto, así que no da pistas de si existe alguno: la respuesta es idéntica
  para un token inventado, uno mal copiado o uno con otro formato. Un borrador no tiene token, así
  que tampoco se puede llegar a él.
- **Alternativas descartadas**: «No existe ese presupuesto», porque habla de presupuestos; «Este
  enlace ha caducado», porque es falso y se confunde con el aviso de caducidad.

## D5. Paso de confirmación de FR-003

- **Decisión**: la confirmación se hace **dentro de la página**, sin `confirm()` del navegador. Al
  pulsar «Aceptar» o «Rechazar» los dos botones se sustituyen por:

  | Decisión | Texto | Botones |
  |---|---|---|
  | Aceptar | «¿Acepta este presupuesto por 133,10 €? Una vez confirmado, no se podrá cambiar.» | «Sí, aceptar» · «Volver» |
  | Rechazar | «¿Rechaza este presupuesto por 133,10 €? Una vez confirmado, no se podrá cambiar.» | «Sí, rechazar» · «Volver» |

  El importe es el **total con IVA**, con `euros()` de `formato.ts`. Se trata de usted, como el
  correo de spec-003 y el aviso de caducidad. «Volver» deja la página como estaba, sin llamar al
  servidor.
- **Motivo**: FR-003 pide un paso de confirmación. `confirm()` muestra sus botones en el idioma del
  navegador (principio II), y en el móvil es incómodo. Poner el total en la pregunta deja claro qué
  se acepta, y «no se podrá cambiar» avisa de que solo hay una respuesta.

## D6. La respuesta y el enlace en la ficha de la orden

- **Decisión**: en `Presupuesto.svelte` (spec-003):
  - **Respuestas (Q1)**: una línea por cada presupuesto con `respondido_en`, de `presupuestos`
    (`PresupuestoResumen`): «Inicial · Aceptado el 16/10/2026 a las 10:32», «Ampliación · Rechazado
    el …», con `fechaDe` y `hora` de `formato.ts` (hora de Madrid).
  - **Enlace (Q5)**: con el `presupuesto` que la sección ya carga en estado `emitido` y sin
    `caducado`: el enlace visible en un campo de solo lectura y un botón «Copiar enlace». Con
    cualquier otro estado no se muestra. Se oculta solo en la pantalla: `GET /api/presupuestos/{id}`
    sigue enviando `enlace`.
- **Motivo**: FR-004, FR-008 y SC-002. El enlace se deja visible porque `navigator.clipboard` no está
  disponible fuera de HTTPS ni de `localhost`: si falla la copia, se puede seleccionar a mano.
  Tras copiar, el botón dice «Copiado» un momento.

## D7. Datos demo para comprobar el caducado en la aplicación

- **Problema**: US3 y SC-003 hay que poder comprobarlos en la aplicación (principio IV), y los
  datos demo no tienen ningún presupuesto caducado. Esperar 15 días no es una opción.
- **Decisión (propuesta; revisa la Persona B)**: en `backend/seed/demo.sql`, una **parte 6** al final
  con una **ampliación caducada** en la orden demo 7 (`pendiente_conformidad`, luis), más un ajuste
  de fecha en la parte 3:
  - **Parte 6**: `tipo = 'ampliacion'`, `estado = 'emitido'`, token ficticio
    `demo-presupuesto-caducado`, `emitido_en` = hace 16 días, `caduca_el` = ayer (15 días después,
    como dice el data-model), con las fechas en Europe/Madrid. Importes de 1 h de mano de obra:
    base 40,00, IVA 8,40, total 48,40.
  - **Sus líneas**: el `INSERT` de `linea_presupuesto` de la parte 4 hace `CROSS JOIN` con todos los
    presupuestos que existen **en ese momento** y les pone las dos líneas de 110,00 €. Por eso la
    parte 6 va **después** de ese `INSERT`, no lo alcanza, y lleva el suyo propio, filtrado por el
    token: una línea `mano_obra` «Sustitución de discos», 1 h × 40,00 = 40,00. Si la ampliación se
    insertara antes, recibiría además las dos líneas de la parte 4 y sus totales no cuadrarían.
  - **Ajuste en la parte 3**: la cita de `7777JJJ` pasa de hace 8 a hace **18 días**. Si no, la
    ampliación se habría emitido 8 días antes de abrir la orden, y la ficha lo enseñaría. La fecha
    de apertura, la del presupuesto inicial y la de las imputaciones se calculan a partir de la cita
    y se mueven con ella: orden abierta hace 18 días, imputaciones hace 17, ampliación emitida hace
    16. La entrega prometida se calcula desde hoy y no cambia.
- **Comprobación** (2026-11-04, en una transacción deshecha con `ROLLBACK`, y contra el código):

  | Lo que depende de la orden 7 | Resultado |
  |---|---|
  | Quickstart §5 y §8 | No usan la orden 7: el §5 trabaja con la de `1313MMM` y el §8 no nombra ninguna. Solo la usan el §4.6 y el §4.7, de esta spec |
  | Lo presupuestado | Sigue en **110,00 €**: `acumulado_y_presupuestado` solo suma los `aceptado` |
  | El 110 % | Imputado 130,00 € > 121,00 €: sigue `pendiente_conformidad`, sin cambio de estado |
  | Emitir otra ampliación | Se puede: spec-003 solo bloquea con `estado = 'emitido' AND caduca_el >= hoy`; la consulta da `false`. `Presupuesto.svelte` no la cuenta como abierta |
  | Tarjeta del tablero | No cambia: `OrdenResumen` no lee `presupuesto`, y la entrega sigue igual, así que no pasa a «Retrasada» |
  | Numeración | Sin cambios: no se crea ninguna orden, y la primera real sigue siendo la Nº 10 |
  | Lista de órdenes | Mismo orden: la 7 sigue siendo la segunda más antigua, después de la 9 (hallazgo V1 de spec-002) |
  | Respuestas en la ficha (§4.7) | La ampliación caducada no tiene `respondido_en`, así que no sale en la lista de respuestas |

  La ficha de la orden 7 enseña esa ampliación como el presupuesto más reciente, con «Caducado el …»,
  hasta que se crea otra. Es lo esperado, y por eso el §4.6, que comprueba que no se ve el enlace, va
  antes del §4.7, que crea otra.
- **Por qué no otra orden**: la 1 aún no tiene diagnóstico; la 2 y la 3 están `en_reparacion` y la 6,
  `en_espera_piezas`, y una ampliación solo existe en `pendiente_conformidad`; la 4 quedaría con el
  diagnóstico bloqueado y la usa spec-003; la 5 ya tiene un presupuesto vigente; la 8 y la 9 están
  terminadas. Una orden nueva cambiaría la numeración del §2 y las cuentas del tablero del §8. Además,
  solo la 9 es anterior a hace 16 días, así que cualquier otra necesitaría el mismo ajuste de fecha.
- **El token** es ficticio y va documentado, como el de la parte 4 (principio V).
- **Alternativas descartadas**: cambiar la fecha a mano en la base de datos, porque el principio IV
  no lo permite; poner `caduca_el` a menos de 15 días de `emitido_en`, porque rompe la regla del
  data-model.

## D8. La página pública en el móvil

- **Decisión**: `/presupuesto/[token]` reutiliza `TablaLineas` y `Totales`, sin el menú del personal
  (el layout no lo pinta en rutas públicas, y `esRutaPublica` ya incluye `/presupuesto/`). A 390 px
  no debe haber desplazamiento horizontal de la página; la tabla de líneas va dentro de `table-wrap`,
  que desplaza solo la tabla si no cabe.
- **Motivo**: el cliente abre el enlace desde el correo, normalmente en el móvil.
