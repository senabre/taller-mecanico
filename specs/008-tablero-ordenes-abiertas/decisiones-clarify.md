# Registro de decisiones del clarify · spec-008 · Tablero de órdenes abiertas (HU-08)

**Sesión**: 2026-11-12 · **Spec**: [spec.md](spec.md) (sección «Clarifications», sesión 2026-11-12)
· **Constitución**: 1.2.1 · **Código de referencia**: rama `spec-008`, que incluye `origin/spec-004`
(spec-004 aún no está en `main`)

Cada pregunta se hizo de una en una, con una opción recomendada. La respuesta de cada una está
integrada en la spec, en los requisitos y escenarios que se indican, y en el [plan](plan.md). Además,
en esta sesión se cierra N1 de [spec-002](../002-abrir-orden-reparacion/hallazgos.md) y se comprueba
la regla de «retrasada» (ver «Comprobaciones sin pregunta»).

| ID | Historia y criterio afectados | Pregunta | Opciones consideradas | Respuesta | Justificación | Autora | Revisor |
|----|-------------------------------|----------|-----------------------|-----------|---------------|--------|---------|
| Q1 | HU-08 · FR-003, SC-003, escenario 2 de la US2 (nuevo) | Si un mecánico inactivo (como Pedro) todavía tiene órdenes abiertas, ¿cómo debe aparecer en la carga por mecánico? | A) los activos siempre, también con 0, y los inactivos solo si tienen órdenes abiertas, marcados «(de baja)»; B) solo los activos, como decía el plan; C) todos, activos e inactivos, siempre | **A** (la recomendada). Cada fila de `carga_mecanicos` lleva `activo` (contrato común) | Las tarjetas de un inactivo salen en las columnas. Si él no sale en la carga, la suma de la carga deja de coincidir con las tarjetas y **SC-003** no se cumple. **B se descarta** justo por eso. **C se descarta** porque llenaría la barra con mecánicos de baja sin trabajo, información que no sirve para repartir el trabajo (principio I) | Persona A | Persona B (pendiente) |
| Q2 | HU-08 · FR-006, escenario 3 de la US1 (nuevo) | ¿Quién puede ver el tablero, y qué le pasa a otro rol que escriba `/tablero` a mano? | A) solo el jefe: los demás no lo ven en el menú, reciben 403 y ven la página de error; B) el jefe y recepción; C) todo el personal con sesión | **A** (la recomendada). `/tablero` muestra a los demás, dentro de la página y sin tarjetas, el mensaje del servidor «No tienes permiso para hacer esto», como las demás pantallas protegidas. *Corregido en el plan (2026-11-12)*: la respuesta pedía «la misma página de error que ya muestran las otras pantallas protegidas (`+error.svelte`)», pero ninguna la usa para el 403; la autora eligió seguir a las demás pantallas. Prueba del 403 para recepción, mecánico y almacén. `GET /api/eventos` sigue abierto a cualquiera con sesión | El contrato (`GET /api/tablero`: jefe), el menú de `+layout.svelte` y el quickstart §0.3–0.4 ya daban el tablero solo al jefe; la spec decía «disponible para el jefe» sin cerrar a los demás. A sigue el patrón de las demás pantallas (`exigir_rol` → 403). **B y C se descartan** porque HU-08 es del jefe de taller y abrirlo a otros roles contradice el contrato y el quickstart ya escritos. `/api/eventos` solo envía `{id, estado}`, sin datos de ninguna orden (principio V) | Persona A | Persona B (pendiente) |
| Q3 | HU-08 · FR-005, SC-002, US4 (texto y escenarios 2–5, nuevos), casos límite «Se pierde la conexión» y «Pasa la medianoche» (nuevo) | ¿Qué debe hacer que el tablero se actualice solo, sin recargar? | A) abrir una orden, cualquier cambio de estado (también facturar) y el cambio de día a las 00:00 Europe/Madrid; B) solo los eventos, y las retrasadas nuevas se ven con el siguiente evento o al recargar; C) los eventos y, además, pedir el tablero cada minuto | **A** (la recomendada), con dos añadidos de la autora: (1) al volver la conexión (`onopen` después de un `onerror`) también se pide el tablero entero, porque los eventos enviados durante la desconexión se han perdido; (2) la medianoche se calcula en Europe/Madrid con `Intl`, no con la zona del navegador, y se prueba en el frontend con reloj simulado (`vi.useFakeTimers`), porque en el quickstart no se puede comprobar a mano | FR-005 solo hablaba de «cambios de estado», pero el tablero también cambia al abrir una orden (aparece en «recibida») y al facturar (desaparece). «Retrasada» depende de la fecha: a las 00:00 una orden puede pasar a retrasada sin que nadie la toque, y el tablero se deja abierto todo el día (US4). **B se descarta** porque dejaría sin destacar una orden retrasada hasta el siguiente cambio de cualquier orden (SC-001). **C se descarta** porque hace 1440 peticiones al día para recoger un cambio que ocurre una sola vez (principio I). Todos los casos usan el mismo mecanismo: volver a pedir `GET /api/tablero` | Persona A | Persona B (pendiente) |
| Q4 | HU-08 · FR-001, SC-001, escenario 4 de la US1 (nuevo) | ¿En qué orden deben ir las tarjetas dentro de cada columna? | A) por fecha de entrega prometida ascendente y, a igual fecha, por número ascendente; B) como devuelve `resumenes()`: de la abierta más recientemente a la más antigua; C) por número ascendente | **A** (la recomendada). Se ordena en `tablero.rs` al agrupar, sin tocar la consulta compartida de `ordenes.rs` | Lo más urgente queda arriba, y las retrasadas, que son las de entrega más antigua, salen las primeras sin una regla aparte. Así se ven en menos de 10 s aunque la columna sea larga (SC-001). **B se descarta** porque deja una orden retrasada y antigua al fondo de la columna, la parte que hay que desplazar. **C se descarta** porque el número dice cuándo llegó la orden, no cuándo hay que entregarla. Desempatar por número hace que el orden sea siempre el mismo y se pueda probar | Persona A | Persona B (pendiente) |

## Comprobaciones sin pregunta

| Punto | Comprobación | Resultado |
|-------|--------------|-----------|
| Ejemplo de la US2 | «Luis 3 · Ana 1», pero Ana es de recepción. En los datos demo (`backend/seed/demo.sql`) los mecánicos son Luis y Marta, y Pedro está inactivo; el quickstart §8.1 dice «Luis N · Marta N» | Corregido en la US2: «Luis 3 · Marta 1» |
| Eventos que mueven el tablero (Q3) | `CambioEstado::publicar` en la rama spec-008: `ordenes.rs:399` (abrir, spec-002); `presupuestos.rs:310` (recibida→en diagnóstico, 003); `publico_presupuesto.rs:204` (aceptar el inicial: en diagnóstico→en reparación; aceptar la ampliación: pendiente de conformidad→en reparación, 004); `imputaciones.rs:193` (→pendiente de conformidad al pasar del 110 %) e `imputaciones.rs:227` (→terminada, 005); `almacen.rs:430` (→en espera de piezas) y `almacen.rs:501` (→en reparación al recibir la última pieza, 006); `facturas.rs:291` (→facturada, 007) | Los 9 movimientos (la apertura y las 8 transiciones de `TRANSICIONES`) se publican después del `commit`. No falta ninguno. Una primera revisión sobre `origin/main` dio por pendientes los dos de spec-004, pero era porque 004 aún no está fusionada; la autora lo corrigió durante la sesión |
| N1 de spec-002 | El plan de 008 decía «Se puede hacer desde el principio con las órdenes demo en varios estados» | Frase quitada. Las dependencias del plan ahora nombran spec-002 y los eventos de las specs 003 a 007. N1 queda cerrado en [hallazgos de spec-002](../002-abrir-orden-reparacion/hallazgos.md) |
| Regla de «retrasada» | Data-model común: estado ≠ `facturada` y `fecha_entrega_prometida` < hoy (Europe/Madrid). `OrdenResumen` (`ordenes.rs`): `o.estado <> 'facturada' AND o.fecha_entrega_prometida < $1` con `$1 = fechas::hoy()`. Spec, caso límite: terminada sin facturar y con la entrega pasada → retrasada | Las tres coinciden. Se precisa en Assumptions «no facturada» y «Europe/Madrid», con referencia al data-model y a I1 de spec-002. Nota para el plan: `resumenes()` también devuelve las facturadas, así que `tablero.rs` las descarta al agrupar |

## Criterios de aceptación resultantes

Cada criterio está escrito igual que el escenario de [spec.md](spec.md) que se indica entre
corchetes. Todos se añadieron a la spec en esta sesión.

### Q1 · Mecánicos inactivos en la carga (FR-003, SC-003)

- **Dado** Luis con 3 órdenes abiertas y Marta con 1, **cuando** se abre el tablero, **entonces** se
  ve «Luis 3 · Marta 1», y los mecánicos activos sin órdenes aparecen con 0. [US2, escenario 1,
  corregido]
- **Dado** un mecánico inactivo con 1 orden abierta y otro inactivo sin ninguna, **cuando** se abre
  el tablero, **entonces** el primero aparece en la carga con 1 y marcado «(de baja)», el segundo no
  aparece, y la suma de la carga coincide con el número de tarjetas. [US2, escenario 2]

### Q2 · Solo el jefe (FR-006)

- **Dado** un usuario de recepción, mecánico o almacén con sesión, **cuando** escribe a mano la
  dirección del tablero, **entonces** no tiene «Tablero» en el menú, ve el mensaje «No tienes
  permiso para hacer esto» sin ninguna tarjeta, y el sistema le niega los datos del tablero. [US1,
  escenario 3; en la prueba: `GET /api/tablero` → 403 para cada uno de los tres roles]

### Q3 · Qué actualiza el tablero (FR-005, SC-002)

- **Dado** el tablero abierto, **cuando** recepción abre una orden nueva, **entonces** en menos de
  5 segundos su tarjeta aparece en «recibida» y la carga de su mecánico sube en 1. [US4, escenario 2]
- **Dado** el tablero abierto, **cuando** se factura una orden terminada, **entonces** en menos de
  5 segundos su tarjeta desaparece y la carga de su mecánico baja en 1. [US4, escenario 3]
- **Dado** el tablero abierto con una orden cuya entrega prometida es hoy, **cuando** llegan las
  00:00 en Europe/Madrid (sea cual sea la zona horaria del ordenador), **entonces** el tablero se
  vuelve a pedir y la tarjeta aparece como «Retrasada» sin que cambie ninguna orden. [US4,
  escenario 4; prueba de frontend con `vi.useFakeTimers`]
- **Dado** el tablero sin conexión y una orden que cambia de estado mientras tanto, **cuando**
  vuelve la conexión, **entonces** el aviso de desconexión desaparece y el tablero se vuelve a pedir
  entero, con esa tarjeta ya en su columna nueva. [US4, escenario 5]

### Q4 · Orden de las tarjetas (FR-001, SC-001)

- **Dado** en una misma columna una orden con entrega pasado mañana, otra con entrega ayer y dos con
  entrega hoy (números 12 y 7), **cuando** se abre el tablero, **entonces** aparecen en este orden:
  la de ayer, la nº 7, la nº 12 y la de pasado mañana. [US1, escenario 4]

## Cambios en ficheros compartidos (los revisa la Persona B)

La lista única y al día es la tabla «Ficheros compartidos» del [plan](plan.md),
que es la que usa la tarea final de revisión de la Persona B en [tasks.md](tasks.md). Ahí están
también los cambios que salieron de esta sesión: el contrato común (Q1, Q2, Q4), `tipos.ts` (Q1) y
N1 en los hallazgos de spec-002. Esta sección ya no tiene tabla propia, para que no haya dos listas
distintas (I4 de [analyze-1](analyze-1.md)).

## Comando y versión

- **Comando**: `/speckit-clarify`, después de apuntar `.specify/feature.json` a
  `specs/008-tablero-ordenes-abiertas`, con estas indicaciones de la autora:

  > Además de lo que detectes tú, revisa estos puntos: (1) US2 usa como ejemplo «Luis 3 · Ana 1»,
  > pero Ana es de recepción, no mecánica; en los datos demo los mecánicos son Luis y Marta (y
  > Pedro, inactivo), y el quickstart §8.1 dice «Luis N · Marta N». (2) Mecánicos inactivos: el plan
  > solo cuenta los activos; ¿qué pasa si un inactivo tiene órdenes abiertas? (3) Quién puede ver el
  > tablero: FR-006 dice «disponible para el jefe»; ¿solo el jefe, y el resto recibe 403 y no ve el
  > enlace en el menú? (4) Qué eventos lo actualizan: además de los cambios de estado, abrir una
  > orden y facturarla; comprobar en el código qué publica cada spec con CambioEstado::publicar.
  > (5) Orden de las tarjetas dentro de cada columna. (6) N1 de spec-002: quitar del plan la frase
  > «Se puede hacer desde el principio con las órdenes demo». (7) Comprobar que la orden terminada
  > sin facturar y con la entrega pasada como retrasada coincide con la regla común (data-model, I1
  > de spec-002) y con OrdenResumen. Hazme las preguntas de una en una y espera mi respuesta. Guarda
  > el registro de decisiones en specs/008-tablero-ordenes-abiertas/decisiones-clarify.md, como en
  > 001, 002 y 004. No hagas commit.

  Los puntos 2 a 5 son Q1 a Q4. El paso de medianoche dentro de Q3 lo añadió el asistente. Los
  puntos 1, 6 y 7 no necesitaban decisión y están en «Comprobaciones sin pregunta». Durante la
  sesión, la autora indicó que el código de referencia es la rama spec-008 y no `origin/main`
  (punto 4). Las respuestas de Q2 y Q3 llevaron indicaciones de la autora, recogidas en la columna
  «Respuesta».
- **Spec Kit**: plantillas del proyecto inicializadas con la versión 1.0.6
  (`.specify/init-options.json`); CLI `specify` instalada: 1.0.8.dev0.
