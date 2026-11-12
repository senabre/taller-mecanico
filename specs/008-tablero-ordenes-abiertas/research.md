# Research: spec-008 · Tablero de órdenes abiertas

Decisiones propias de spec-008 después del clarify (sesión 2026-11-12, Q1–Q4; registro en
[decisiones-clarify.md](decisiones-clarify.md)). Todo lo común (pila, SSE y bus de eventos, fechas en
Europe/Madrid, errores, roles, transiciones de estado) está en
[000-base-comun/research.md](../000-base-comun/research.md) y no se repite aquí: el tablero en tiempo
real es **R11**, «hoy» es **R12** y la publicación después del `commit` es **R14**. Código de
referencia: rama `spec-008`, que incluye `origin/spec-004`. Fecha: 2026-11-12.

## D1. `GET /api/tablero`: una instantánea con dos consultas

- **Decisión**: `backend/src/rutas/tablero.rs`, con `exigir_rol(&[Rol::Jefe])` (D4). En una
  transacción de solo lectura `REPEATABLE READ`:
  1. **Columnas**: `ordenes::resumenes(&mut tx, None, true)` (D3) devuelve las órdenes no
     facturadas como `OrdenResumen`, con `retrasada` ya calculada. `tablero.rs` las reparte en las
     6 columnas en el orden de `Estado` (recibida → terminada), crea las 6 aunque estén vacías y
     ordena cada una por `(fecha_entrega_prometida, numero)` ascendentes (Q4).
  2. **Carga**: la consulta de D2.
- **Motivo**: con `REPEATABLE READ`, las dos consultas ven la misma foto de la base de datos. Si
  entre una y otra se abre o se factura una orden, la carga no puede dejar de cuadrar con las
  tarjetas (SC-003). El criterio de `retrasada` es el de `OrdenResumen` (I1 e I2 de spec-002): no hay
  una segunda definición. Ordenar en Rust no toca la consulta compartida, cuyo orden sirve para la
  lista de órdenes.
- **Alternativas descartadas**: una consulta propia de las tarjetas en `tablero.rs`, porque duplica
  `OrdenResumen` (I2); dos consultas sin transacción común, porque pueden ver momentos distintos;
  ordenar en el frontend, porque la prueba del orden quedaría fuera del backend.

## D2. Carga por mecánico: la suma siempre cuadra con las tarjetas

- **Decisión**:

  ```sql
  SELECT u.id, u.nombre, u.activo, count(o.id) AS ordenes_abiertas
    FROM usuario u
    LEFT JOIN orden o ON o.mecanico_id = u.id AND o.estado <> 'facturada'
   GROUP BY u.id
  HAVING (u.rol = 'mecanico' AND u.activo) OR count(o.id) > 0
   ORDER BY u.nombre, u.id
  ```

  La pantalla escribe «(de baja)» junto al nombre cuando `activo` es `false` (Q1).
- **Motivo**: salen los mecánicos activos aunque tengan 0 (US2-1) y **cualquier usuario con órdenes
  abiertas**, aunque esté inactivo (US2-2). Como la condición del `HAVING` no filtra por rol a quien
  tiene órdenes, la suma de `ordenes_abiertas` es siempre el número de tarjetas (SC-003), aunque
  algún día cambie el rol de un usuario. Ordenar por nombre da «Luis · Marta», como el quickstart.
- **Prueba**: en todas las pruebas de carga se comprueba que la suma de `ordenes_abiertas` es igual
  al número total de tarjetas.
- **Alternativas descartadas**: `WHERE u.rol = 'mecanico' AND u.activo`, que era la del plan
  anterior y deja fuera las órdenes de un inactivo (Q1-B); contar en Rust a partir de las tarjetas,
  porque no daría los mecánicos con 0.

## D3. Cambio en `ordenes.rs`: `resumenes()` puede traer solo las abiertas

- **Decisión**: `resumenes(conexion, mecanico_id, solo_abiertas: bool)`, con `AND (NOT $3 OR
  o.estado <> 'facturada')` en el `WHERE`. `listar_ordenes` pasa `false` y sigue igual; el tablero
  pasa `true`.
- **Motivo**: el tablero se vuelve a pedir con cada cambio de estado de cualquier orden, todo el día.
  Sin el filtro, cada petición traería todo el historial de órdenes facturadas para tirarlo después,
  y eso crece cada mes. Es un parámetro en una consulta que ya existe (principio I), en un fichero de
  la Persona A (spec-002).
- **Alternativas descartadas**: filtrar en `tablero.rs`, por lo dicho; una consulta nueva en
  `tablero.rs`, por I2 de spec-002.

## D4. Acceso: solo el jefe, con el mensaje de las demás pantallas

- **Decisión**: `GET /api/tablero` llama a `exigir_rol(&[Rol::Jefe])`. Recepción, mecánico y almacén
  reciben 403 «No tienes permiso para hacer esto» (`ErrorApi::Prohibido`), y `/tablero` muestra ese
  mensaje dentro de la página, sin columnas ni tarjetas, igual que `/almacen`, `/ordenes` o
  `/pedidos` (Q2, corregido en este plan). El menú ya enseña «Tablero» solo al jefe
  (`+layout.svelte`) y `INICIO_ROL` ya lleva al jefe a `/tablero`: no cambian.
  `GET /api/eventos` sigue abierto a cualquier usuario con sesión.
- **Motivo**: Q2 pedía «la misma página de error que ya muestran las otras pantallas protegidas
  (`+error.svelte`)», pero ninguna la usa para el 403: todas piden los datos al abrirse y muestran el
  mensaje del servidor. La autora eligió seguir a las demás pantallas (2026-11-12). Los eventos solo
  envían `{id, estado}`, sin matrícula ni datos del cliente (principio V).
- **Prueba**: 403 con el texto exacto para recepción, mecánico y almacén; 200 para el jefe.

## D5. Cuándo se vuelve a pedir el tablero (Q3)

- **Decisión**: una sola función `cargar()` en `/tablero/+page.svelte`, que pide
  `GET /api/tablero` entero. Se llama:
  1. al abrir la página;
  2. en cada `onopen` de `EventSource('/api/eventos')`: el primero, porque así no se pierde ningún
     evento entre la primera carga y la suscripción (y, si la primera carga falló, se recupera), y
     **cada vez que la conexión se abre de nuevo después de un error**, tanto si el navegador
     reconecta solo como si la página la vuelve a crear tras quedar cerrada, porque lo enviado
     mientras no había conexión se ha perdido (N3 de analyze-2). Un error **no** pide el tablero:
     se pide al volver a abrirse;
  3. con cada evento `orden`, sin mirar su contenido;
  4. al pasar la medianoche de Madrid (D6).

  Solo hay una petición en curso a la vez: si llega otro aviso mientras tanto, se apunta y se hace
  **una** petición más al terminar. Esa regla va en `frontend/src/lib/recarga.ts`
  (`crearRecarga(fn): () => void`, que envuelve `cargar()`), para poder probarla con Vitest sin
  montar la página: `recarga.test.ts` comprueba, con promesas que se resuelven a mano y reloj
  simulado, que tres avisos durante una petición dan **una** sola petición más, que nunca hay dos a
  la vez y que un fallo de la petición no bloquea las siguientes.

  **La conexión de eventos** (corregido por E1 y E2 de analyze-1) va en
  `frontend/src/lib/eventos.ts`: `conectarEventos({alAbrir, alEvento, alError, crear, espera})`
  devuelve la función que la cierra. `crear` es `(url) => new EventSource(url)` por defecto y
  `espera`, 5 000 ms; los dos se pueden cambiar en la prueba. Reglas:
  - `onopen` → `alAbrir()`: la página quita el aviso y pide el tablero entero (Q3). Es la **regla
    general**: el tablero se pide cuando la conexión se abre, la primera vez y cada vez que se
    vuelve a abrir tras un error.
  - cada evento `orden` → `alEvento()`: la página pide el tablero entero.
  - `onerror` → `alError(cerrada: boolean)`: la página muestra «Sin conexión, reintentando…».
    - Con `readyState` `CONNECTING` (`cerrada = false`), no se hace nada más: el navegador ya está
      reintentando tras un error de red, y el tablero se pedirá en el `onopen`.
    - Con `readyState` `CLOSED` (`cerrada = true`: la respuesta no fue un 200 con
      `text/event-stream`, por ejemplo un 401 con la sesión caducada), el navegador **no**
      reintenta: `eventos.ts` cierra esa conexión y crea otra pasada la `espera`, una sola vez por
      cierre. **Única excepción** a la regla general: la página pide el tablero **una vez** antes de
      la conexión nueva, porque si la causa es un 401 la conexión no volverá a abrirse nunca; si
      `/api/tablero` responde 401, `api.ts` lleva a `/entrar`, como en el resto de la app (E1).
  - la función que devuelve cierra la conexión y cancela la espera pendiente.

  La página llama a `conectarEventos` y a `programarMedianoche` después de la primera carga, **en
  todos los casos salvo con un 403**: con un 403 se queda con el mensaje y no abre ninguna conexión
  (E2). Si la primera carga falla por otra causa (por ejemplo, el servidor se está reiniciando), la
  conexión se abre igualmente y su primer `onopen` vuelve a pedir el tablero, así que se recupera
  solo (N2 de analyze-2). Un 401 ya lleva a `/entrar` desde `api.ts`. Al salir de la página se
  cierran la conexión y el temporizador.

  `eventos.test.ts` lo prueba con un `EventSource` simulado (una clase con `readyState`, `onopen`,
  `onerror`, `addEventListener` y `close`, que la prueba controla) y reloj simulado, sin
  dependencias nuevas: (1) `onopen` llama a `alAbrir`, también tras reconectar y en la conexión
  creada tras un cierre; (2) un evento `orden` llama a `alEvento`; (3) con `onerror` y `CONNECTING`,
  `alError(false)` se llama y no se crea otra conexión; (4) con `onerror` y `CLOSED`, `alError(true)`
  se llama, no se crea otra conexión antes de la `espera`, se crea una sola después, y la anterior
  queda cerrada; (5) la función de cierre cierra la conexión y, si había una
  espera pendiente, no se crea ninguna más.
- **Motivo**: todo lo que cambia el tablero pasa por el mismo camino. Con una sola petición en curso,
  una respuesta antigua no puede llegar después de una nueva y pisarla, y una ráfaga de eventos (un
  pedido que cambia varias órdenes, por ejemplo) no lanza una petición por cada uno. Pedir el tablero
  entero es más simple que aplicar cada evento a mano, y con unas decenas de órdenes es barato.
- **Eventos que llegan** (comprobado en la rama spec-008): los 9 movimientos que cambian el tablero
  publican `CambioEstado::publicar` después del `commit` (lista en [plan.md](plan.md)). Los eventos
  perdidos por ir lento (`Lagged`, `eventos.rs`) se recuperan con el siguiente aviso, que pide el
  tablero entero.
- **Sesión**: cada petición renueva la sesión (12 h sin actividad, `auth.rs`). Si el tablero pasa
  más de 12 horas sin ningún aviso, la petición de medianoche recibe 401 y `api.ts` lleva a
  `/entrar`, como en cualquier pantalla. El jefe vuelve a entrar por la mañana. Si la sesión caduca
  mientras se corta la red, la conexión de eventos queda cerrada (`CLOSED`) y la petición única de
  ese caso lleva a `/entrar` en ese momento, sin esperar a la medianoche (E1, N3).
- **Alternativas descartadas**: aplicar cada evento sobre la lista que ya se tiene, porque el evento
  no trae la tarjeta completa y obligaría a pedirla igualmente; consultar cada minuto (Q3-C).

## D6. La medianoche de Madrid, en `frontend/src/lib/medianoche.ts`

- **Decisión**: un módulo con dos funciones:
  - `msHastaMedianocheMadrid(ahora: Date): number`: milisegundos hasta la siguiente 00:00 de
    `Europe/Madrid`. Saca la fecha de Madrid de `ahora` con
    `Intl.DateTimeFormat('es-ES', {timeZone: 'Europe/Madrid'})` (`formatToParts`), suma un día y
    busca el instante UTC de su 00:00 con el desfase que da `Intl` para ese instante, en dos
    pasadas por los cambios de hora. Nunca usa la zona del navegador (`getHours`, `setHours`…).
  - `programarMedianoche(fn): () => void`: un `setTimeout` hasta la siguiente medianoche **más 60
    segundos**. Al saltar llama a `fn`, se vuelve a programar y devuelve la función que lo cancela.
- **Motivo**: «retrasada» la calcula el servidor con su «hoy» de Madrid (R12). Si el reloj del
  ordenador del taller va unos segundos adelantado, una petición a las 00:00:00 exactas podría llegar
  al servidor a las 23:59 y no cambiar nada hasta la noche siguiente. El minuto de margen cubre esa
  diferencia sin que nadie lo note (escenario 4 de la US4). Volver a programarse en cada salto, en
  lugar de usar un `setInterval` de 24 h, sigue siendo correcto los días de 23 y 25 horas.
- **Prueba** (`frontend/src/lib/medianoche.test.ts`, Vitest con `vi.useFakeTimers` y
  `vi.setSystemTime`, sin dependencias nuevas): con `vi.stubEnv('TZ', 'America/New_York')` (escribe en `process.env` sin necesitar los
  tipos de Node, que el proyecto no tiene; cambiado al implementar), para que
  el navegador simulado esté en otra zona. **Antes de nada**, la prueba comprueba que la zona del
  proceso es de verdad otra: si `new Date('2026-07-01T12:00:00Z').getTimezoneOffset()` no es el de
  Nueva York (240) o coincide con el de Madrid (−120), falla con un mensaje claro, en lugar de dar
  verde sin probar nada (corregido por A2 de analyze-1). Después:
  1. a las 23:59:00 de Madrid, `fn` no se llama hasta pasado 1 min + 60 s, y se llama una sola vez;
  2. después de saltar se vuelve a programar para la medianoche siguiente;
  3. empezando a las 00:01 de un día con cambio de hora (28/03/2027, de 23 h; 31/10/2027, de 25 h),
     el siguiente salto es a las 00:01 de Madrid del día siguiente, ni una hora antes ni una
     después;
  4. la función que devuelve cancela el temporizador.
- **Cambio compartido**: [R13](../000-base-comun/research.md) decía «Vitest solo para `formato.ts`».
  Ahora dice «para `formato.ts`, `medianoche.ts`, `recarga.ts` y `eventos.ts`», por Q3, D5 y E1 de
  analyze-1. Lo revisa la Persona B.
- **Alternativas descartadas**: `new Date().setHours(24, 0, 0, 0)`, que usa la zona del navegador;
  una prueba de la página con `@testing-library/svelte` y `jsdom`, que añade dependencias solo para
  esto (principio I): la página solo llama a `programarMedianoche(cargar)`, y eso se comprueba en el
  código y en la prueba de las funciones; una dependencia de zonas horarias como `date-fns-tz`, porque
  `Intl` ya lo hace.

## D7. La pantalla en la pared del taller y en el móvil del jefe

- **Decisión**:
  - **Ancho**: el `<nav>` y el `<main>` de `+layout.svelte` llevan `max-w-6xl`. En `/tablero` los
    **dos** pasan a ocupar todo el ancho, para que en una pantalla grande quepan las 6 columnas con
    tarjetas legibles y el menú quede alineado con ellas (corregido por I3 de analyze-1). Se hace
    con una sola variable (`ancho`, verdadera si la ruta empieza por `/tablero`) que quita
    `max-w-6xl` en los dos. En las demás pantallas el menú y el contenido siguen exactamente igual:
    mismas entradas, mismo orden y mismo ancho.
  - **Carga**: arriba, una fila de chips «Luis 5 · Marta 3 · Pedro (de baja) 1», que salta de línea
    si no cabe.
  - **Columnas**: con 1280 px o más (`xl`), una rejilla de 6 columnas iguales. Por debajo, una tira
    que se desplaza en horizontal **solo dentro de su contenedor** (`overflow-x-auto`, con
    `scroll-snap`), con columnas de `w-72`, o del 85 % del ancho en el móvil. Así a 390 px se ve una
    columna entera y el borde de la siguiente, y la página no tiene desplazamiento horizontal.
  - Cada columna tiene una cabecera fija con el nombre del estado (`NOMBRE_ESTADO`) y su propio
    desplazamiento vertical (`overflow-y-auto`, alto máximo de la pantalla menos la cabecera), para
    el caso límite de muchas órdenes. Una columna vacía muestra «Sin órdenes».
  - **Tarjeta**: «Nº 12», matrícula en grande, mecánico y «Entrega dd/mm/aaaa» (`fecha()` de
    `formato.ts`). Una retrasada lleva borde de 2 px `border-error-500` y la insignia «Retrasada»
    (`badge preset-filled-error-500`). Así se distingue también sin color, por el texto (SC-001).
  - Las tarjetas no son enlaces ni se arrastran (Assumptions de la spec, principio III).
- **Textos de la página**: «Tablero», «Carga por mecánico», «(de baja)», «Sin órdenes», «Entrega»,
  «Retrasada», «Sin conexión, reintentando…» y, si falla la carga, el mensaje del servidor o «No se
  pudo cargar el tablero». Todos en español (principio II).
- **Motivo**: lo pide el contexto del plan: una pantalla grande en el taller y el móvil del jefe a
  390 px, con las columnas desplazables.
- **Alternativas descartadas**: apilar las columnas en vertical en el móvil, porque obliga a bajar
  mucho para llegar a «Terminada» y se pierde la idea de tablero; romper el `max-w-6xl` desde la
  página con márgenes negativos, porque es frágil y puede crear desplazamiento horizontal.

## D8. Datos demo para ver el tablero con sentido

- **Comprobación** de `backend/seed/demo.sql` (parte 3), con los datos recién cargados:

  | Necesidad | Hay | ¿Basta? |
  |---|---|---|
  | Al menos una orden en cada columna | recibida 1, en diagnóstico 4 y 5, en reparación 2 y 3, en espera de piezas 6, pendiente de conformidad 7, terminada 8 | Sí |
  | Una facturada que no sale | 9 | Sí |
  | Alguna retrasada | 8 (entrega ayer, terminada: el caso límite de la spec) | Sí |
  | Alguna con entrega **hoy**, que no sale retrasada | ninguna: las entregas son +3, +2, +4, +5, +3, +2, +1, −1 y −15 | **No** |
  | Carga de Luis y Marta | Luis 5 (1, 2, 3, 4, 7) · Marta 3 (5, 6, 8); Pedro, de baja y sin órdenes, no sale | Sí |
  | El orden de las tarjetas se nota | en diagnóstico: 5 (+3) antes que 4 (+5), aunque la 4 se abrió después | Sí |

- **Propuesta** (fichero compartido; se hace en la primera tarea y la revisa la Persona B):
  1. Orden **3** (`3333DDD`, en reparación, Luis): entrega de `4` a `0`, es decir, hoy. Así US3-2 se
     ve en la app, y en la columna «En reparación» la 3 (hoy) queda encima de la 2 (+2), al revés
     que en la lista de órdenes, que va por fecha de apertura.
  2. En la misma parte, `fecha_entrega_prometida` pasa de `current_date + o.entrega` a
     `(now() AT TIME ZONE 'Europe/Madrid')::date + o.entrega`, como ya hace la parte 5 (R12). Con la
     base de datos en UTC, y los datos cargados entre las 00:00 y las 02:00 de Madrid,
     `current_date` aún sería ayer y la orden «de hoy» saldría retrasada.
  3. El comentario de la parte 3 añade «3 · entrega hoy: no sale retrasada (008)».
- **Qué no cambia**: no se crea ninguna orden, así que la numeración sigue (la primera real es la
  Nº 10). Tampoco cambian los estados, mecánicos, km, imputaciones ni presupuestos.
- **La orden 3 sí la usa el quickstart común**: es la orden «R» del §6 (pedidos de FA-9 y PF-123), y
  la regresión T029 de spec-004 la fija así (`evidencias/T029/scripts/t029r.mjs`: `const R = 3`).
  Comprobado el 2026-11-12 que poner su entrega a hoy no cambia nada de lo que se espera:

  | Dónde | Qué comprueba | ¿Depende de la entrega? |
  |---|---|---|
  | §6.3–§6.5 (`s6.log`) | Pedidos de FA-9 y PF-123 para la orden 3, el duplicado rechazado y el paso a «En espera de piezas» | No: el pedido solo exige que la orden esté en reparación o en espera de piezas. `almacen.rs` no lee `fecha_entrega_prometida` (ningún fichero de `backend/src` la lee, salvo `ordenes.rs`) |
  | §6.6 | Pedido para una orden en diagnóstico: usa la orden 4 | No |
  | §6.7 | `luis` ve los dos pedidos de la orden 3 y no puede imputar ni terminar | No: lo deciden el estado y los pedidos. La ficha muestra la entrega de hoy sin «Retrasada», y el script no la mira |
  | §6.8–§6.9 | Recibir FA-9 (sigue en espera) y PF-123 (vuelve sola a «En reparación») | No |
  | T029 §2.5 (`t029.mjs`) | Lista de órdenes: 11 y 10 primero, por fecha de apertura; la orden 8 con «Retrasada» | No: la lista va por `abierta_en`, que no cambia, y la 3 no sale retrasada (entrega hoy) |
  | T029 ficha (`t029.mjs`, línea 178) | Fichas de las órdenes 2, 6 y 8 | No: no incluye la 3 |
  | T029 D7 (`t029d7.mjs`) | Entrega de la orden 7, no retrasada | No: la 7 conserva su `+1`. Con el «hoy» de Madrid, la fecha solo cambiaría si los datos se cargan entre las 00:00 y las 02:00, y entonces la de antes era la incorrecta |

  Además, el §6 deja la orden 3 otra vez «En reparación», con la entrega de hoy, que es lo que
  esperan los pasos 5 y 9 del §8. **No hace falta otra orden.**
- **Lo que no se puede ver con datos demo**: el «(de baja)» de un mecánico inactivo con órdenes.
  Pedro no tiene órdenes, y dárselas obligaría a quitarle una a Luis, que la usa en §3–§6 como su
  mecánico (spec-002 no deja asignar a un inactivo). Se comprueba en las pruebas automáticas (US2-2).
- **Alternativas descartadas**: una orden nueva con entrega hoy, porque cambia la numeración del §2
  y las cuentas de la carga; cambiar la entrega de la orden 7 u 8, porque la 8 es la retrasada y la 7
  tiene fechas ya ajustadas para la ampliación caducada (spec-004 D7).

## D9. Comprobación manual de SC-001 y SC-002

- **SC-001** (retrasadas en menos de 10 s): una persona que no haya visto antes el tablero lo abre
  como `paco` mientras otra cronometra desde que se ve la página hasta que dice qué órdenes van con
  retraso. Debe nombrar `8888KKK`, y **no** `3333DDD` (entrega hoy).
- **SC-002** (menos de 5 s): dos navegadores (o una ventana normal y otra privada, porque la sesión va
  en una cookie), uno con `paco` en `/tablero` y otro con `luis`. Se cronometra desde que `luis` pulsa
  «Terminar reparación» en `2222CCC` hasta que la tarjeta cambia de columna. Lo mismo al abrir una
  orden (`ana`, `8888KKK`, que tiene cita hoy) y al facturar (`ana`, `2222CCC`).
- **Motivo**: principio IV. Lo que no se puede hacer a mano, la medianoche, queda en la prueba de D6.
