---

description: "Tareas de spec-008 · Tablero de órdenes abiertas (HU-08)"
---

# Tasks: spec-008 · Tablero de órdenes abiertas (HU-08)

**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md) (D1–D9),
[data-model.md](data-model.md), [contracts/api.md](contracts/api.md) y
[quickstart.md](quickstart.md), que remiten a los documentos comunes de
[000-base-comun](../000-base-comun/).

**Responsable**: Persona A · **Revisor**: Persona B

**Tests**: se incluyen. El plan maestro (research R13) exige pruebas `#[sqlx::test]` para cada regla
de negocio, y el plan de esta spec enumera 8 en `backend/tests/spec008_tablero.rs` (aquí «prueba
1»…«prueba 8»), más tres ficheros de Vitest con reloj simulado: `medianoche.test.ts` (D6), `recarga.test.ts` (D5)
y `eventos.test.ts` (D5, con un `EventSource` simulado; E1 de analyze-1). Cada prueba se escribe **antes** de la tarea que la hace pasar y debe fallar
hasta entonces (rojo → verde). Si una prueba protege una regla que ya existe y no puede fallar, se
demuestra con una **mutación**: se rompe la regla un momento, la prueba falla, se restaura y se
comprueba con `cmp` que el fichero queda igual. El rojo y el verde se anotan en la tarea.

**Trazabilidad**: cada tarea termina con «**Cubre:**», que indica los requisitos (FR-xxx), los
escenarios (US1-n, US2-n, US3-n, US4-n), los casos límite y los criterios de éxito (SC-xxx) de la
spec que atiende.

**Cambios compartidos**: las tareas marcadas «**cambio compartido · revisa Persona B**» tocan
ficheros de la base común o que usan las pantallas de la Persona B. Cada una va en **su propio
commit**, sin mezclarse con el resto de spec-008, y la Persona B la revisa en la tarea final (T032).

**Organización**: una fase por historia de usuario, por prioridad: US1, US2 y US3 (P1) y US4 (P2).
Todo texto visible, en español de España; fechas dd/mm/aaaa con `fecha()` de `formato.ts`; «hoy»
siempre con `fechas::hoy()` en el backend (R12) y con `Intl` en Europe/Madrid en el navegador (D6).
Las pruebas crean sus propios datos con `backend/tests/comun/mod.rs` (`crear_usuario`,
`crear_orden`, `entrar`, `peticion`) y no usan valores demo (principio V). Las fechas de entrega de
las pruebas se fijan con un `UPDATE` relativo a `fechas::hoy()`.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: se puede hacer en paralelo (otro fichero, sin depender de tareas pendientes)
- **[Story]**: historia de usuario (US1, US2, US3, US4)

## Path Conventions

Aplicación web: `backend/src/`, `backend/tests/`, `backend/seed/`, `frontend/src/`. No hay
migraciones ni dependencias nuevas: el tablero solo lee `orden`, `vehiculo` y `usuario`.

---

## Phase 1: Setup (cambios compartidos)

**Purpose**: los datos demo y los tipos que necesitan las historias.

- [ ] T001 **Cambio compartido · revisa Persona B · commit propio**. En `backend/seed/demo.sql`, parte 3, según research D8: (a) la orden **3** (`3333DDD`) pasa de entrega `4` a `0`; (b) `fecha_entrega_prometida` pasa de `current_date + o.entrega` a `(now() AT TIME ZONE 'Europe/Madrid')::date + o.entrega`, como la parte 5; (c) el comentario de la tabla de la parte 3 añade a la orden 3 «entrega hoy: no sale retrasada (008)». **Rojo** antes del cambio, sobre una base recién migrada y cargada: `SELECT count(*) FROM orden WHERE estado <> 'facturada' AND fecha_entrega_prometida = (now() AT TIME ZONE 'Europe/Madrid')::date` da 0. **Verde** después, con la base reiniciada y el seed cargado sin errores: esa consulta da 1 (la orden 3); la orden 3 sigue `en_reparacion`, de Luis, con 40,00 € imputados; las retrasadas siguen siendo solo la 8; las entregas de las demás órdenes no cambian; el contador de órdenes sigue en 9. Comprobar además lo que research D8 da por hecho: el §6 del quickstart común (la orden 3 es la «R») no depende de la entrega. **Cubre:** US3-2 y US1-4 en el quickstart común §8 (pasos 4, 5 y 9), principio IV.
- [ ] T002 [P] **Cambio compartido · revisa Persona B · commit propio**. En `frontend/src/lib/tipos.ts`, añadir una sección «spec-008 · Tablero», sin tocar las demás: `CargaMecanico {id: number, nombre: string, activo: boolean, ordenes_abiertas: number}`, `ColumnaTablero {estado: EstadoOrden, ordenes: OrdenResumen[]}` y `Tablero {columnas: ColumnaTablero[], carga_mecanicos: CargaMecanico[]}`, como el contrato común. **Rojo**: un fichero temporal que importa `Tablero` hace fallar `npm run check`; se borra. **Verde**: `npm run check`, 0 errores. **Cubre:** FR-001, FR-003.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: la ruta registrada y la consulta compartida preparada, para que las pruebas de las
historias fallen por lo que deben y no por no compilar.

**⚠️ CRITICAL**: ninguna historia empieza hasta terminar esta fase.

- [ ] T003 Crear `backend/src/rutas/tablero.rs` con `pub fn rutas() -> Router<EstadoApp>` y `GET /tablero` que, de momento, exige sesión (`Usuario`) y responde **501**. **Cambio compartido · revisa Persona B · commit propio**: en `backend/src/rutas/mod.rs`, `pub mod tablero;` y `.merge(tablero::rutas())`. `cargo build` sin avisos. **Cubre:** FR-001 (punto de entrada).
- [ ] T004 Prueba 8 en `backend/tests/spec008_tablero.rs` (crear el fichero con `mod comun;`): con una orden facturada y dos abiertas, abiertas en momentos distintos, `GET /api/ordenes` como recepción devuelve **las tres**, de la abierta más recientemente a la más antigua. Protege la lista de spec-002 antes de tocar `resumenes()`: pasa ya, así que se anota en verde **antes** de T005 y debe seguir en verde después. **Cubre:** D3 (la lista de órdenes no cambia).
- [ ] T005 **Cambio compartido · revisa Persona B · commit propio** (fichero de la Persona A, pero lo usan las pantallas de la Persona B). En `backend/src/rutas/ordenes.rs`, según research D3: `resumenes(conexion, mecanico_id, solo_abiertas: bool)`, con `AND (NOT $3 OR o.estado <> 'facturada')` en el `WHERE` y el `bind` correspondiente; `listar_ordenes` pasa `false`. El orden de la consulta no cambia. Verde: prueba 8 y `cargo test --test spec002_ordenes`. **Cubre:** FR-001, SC-004 (solo las abiertas), D3.

**Checkpoint**: `GET /api/tablero` existe (501) y `resumenes()` puede traer solo las abiertas.

---

## Phase 3: User Story 1 - Ver las órdenes abiertas agrupadas por estado (Priority: P1) 🎯 MVP

**Goal**: el jefe ve las 6 columnas con las órdenes no facturadas, cada una en la suya y ordenadas por
fecha de entrega; los demás roles no ven el tablero.

**Independent Test**: con órdenes en varios estados y una facturada, abrir `/tablero` como `paco` y
comprobar que cada orden está en la columna de su estado y la facturada no aparece; como `ana`,
«No tienes permiso para hacer esto».

### Tests for User Story 1 ⚠️

> Escribirlas primero, en `backend/tests/spec008_tablero.rs`, y comprobar que fallan (501) antes de T010.

- [ ] T006 [US1] Prueba 1: una orden en cada uno de los 6 estados abiertos y otra `facturada` → 200 con `columnas` de **6** elementos, con `estado` en este orden: `recibida`, `en_diagnostico`, `en_reparacion`, `en_espera_piezas`, `pendiente_conformidad`, `terminada`; cada orden en la suya, con los campos de `OrdenResumen`, y la facturada en ninguna. **Cubre:** FR-001, FR-002, US1-1, US1-2, SC-004.
- [ ] T007 [US1] Prueba 2 (columnas): sin ninguna orden → 6 columnas, todas con `ordenes` vacío. **Cubre:** FR-001, caso límite «Un estado sin órdenes».
- [ ] T008 [US1] Prueba 3: como recepción, mecánico y almacén → **403** con `{"error": "No tienes permiso para hacer esto"}` (texto exacto); como jefe → 200; sin sesión → 401. Rojo: los tres primeros dan 501. **Cubre:** FR-006, US1-3 (Q2).
- [ ] T009 [US1] Prueba 4: en `en_reparacion`, cuatro órdenes con entrega pasado mañana, ayer y dos hoy (números 12 y 7) → `ordenes` en este orden de números: la de ayer, 7, 12, la de pasado mañana. **Cubre:** FR-001, US1-4 (Q4), SC-001.

### Implementation for User Story 1

- [ ] T010 [US1] En `backend/src/rutas/tablero.rs`, según research D1 y D4: `usuario.exigir_rol(&[Rol::Jefe])?`; una transacción con `estado.db.begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY")` (SQLx 0.8.6; `begin()` enviaría su propio `BEGIN`, A1 de analyze-1); `ordenes::resumenes(&mut *tx, None, true)`; repartir en las 6 columnas en el orden de `Estado`, creando las 6 aunque estén vacías; ordenar cada una por `(fecha_entrega_prometida, numero)` ascendentes; responder `{columnas, carga_mecanicos: []}` (la carga llega en T015). Verde: pruebas 1, 2, 3, 4 y 8. **Cubre:** FR-001, FR-002, FR-006, US1-1, US1-2, US1-3, US1-4, SC-004.
- [ ] T011 [P] [US1] **Cambio compartido · revisa Persona B · commit propio**. En `frontend/src/routes/+layout.svelte`, según research D7: una variable `ancho` (verdadera si `page.url.pathname` empieza por `/tablero`) quita `max-w-6xl` del `<nav>` y del `<main>`, que en el tablero ocupan todo el ancho (I3 de analyze-1). Las entradas del menú **no** cambian. `npm run check`, 0 errores; en el navegador, las demás pantallas siguen con el menú y el contenido del mismo ancho. **Cubre:** D7 (pantalla grande).
- [ ] T012 [US1] Crear `frontend/src/routes/tablero/+page.svelte` según research D7 y el [contrato de la spec](contracts/api.md): `cargar()` pide `get<Tablero>('/tablero')` al abrir; con 200, las 6 columnas con la cabecera `NOMBRE_ESTADO[estado]`, «Sin órdenes» si está vacía, y tarjetas con «Nº {numero}», la matrícula en grande, el nombre del mecánico y «Entrega {fecha(...)}»; con `ErrorApi`, solo `e.message` (el 403 da «No tienes permiso para hacer esto»), sin columnas; sin conexión, «No se pudo cargar el tablero». Diseño: con `xl` o más, rejilla de 6 columnas iguales; por debajo, una tira con `overflow-x-auto` y `scroll-snap`, con columnas de `w-72` (85 % del ancho en el móvil), dentro de su contenedor, de modo que la **página** no se desplace en horizontal a 390 px; cada columna con su propio `overflow-y-auto` y alto máximo de la pantalla menos la cabecera. Las tarjetas no son enlaces ni se arrastran. `npm run check`, 0 errores. No se marca hasta verlo en el navegador (T028). **Cubre:** FR-001, FR-002, FR-006, US1-1, US1-2, US1-3, US1-4, caso límite «Muchas órdenes en una columna».

**Checkpoint**: el tablero muestra las columnas y bloquea a los demás roles. Es el MVP: ya sustituye a la pizarra.

---

## Phase 4: User Story 2 - Carga de cada mecánico (Priority: P1)

**Goal**: el tablero muestra cuántas órdenes abiertas tiene cada mecánico, y la suma cuadra con las tarjetas.

**Independent Test**: con 3 órdenes abiertas de Luis y 1 de Marta, ver «Luis 3 · Marta 1»; un mecánico
de baja con órdenes sale con «(de baja)».

### Tests for User Story 2 ⚠️

- [ ] T013 [US2] Prueba 5 en `backend/tests/spec008_tablero.rs`: Luis con 3 abiertas y 1 facturada, Marta con 1 y un tercer mecánico activo sin órdenes → `carga_mecanicos` con 3, 1 y 0, por nombre, todos con `activo: true`; también la parte de carga de la prueba 2 (sin órdenes, los mecánicos activos con 0). Además, añadir en un auxiliar del fichero la comprobación de **SC-003** (la suma de `ordenes_abiertas` es igual al número total de tarjetas) y llamarla en las pruebas 1, 2, 4, 5 y 6. Rojo: la carga está vacía (T010). **Cubre:** FR-003, US2-1, SC-003.
- [ ] T014 [US2] Prueba 6: un mecánico **inactivo** (`crear_usuario(…, activo = false)`) con 1 orden abierta → sale con `activo: false` y 1; otro inactivo sin órdenes → no sale. Rojo: la carga está vacía. Después de T015, **mutación**: cambiar la consulta por la anterior (`WHERE u.rol = 'mecanico' AND u.activo`) → la prueba 6 y la comprobación de SC-003 fallan; restaurar y comprobar con `cmp`. **Cubre:** FR-003, US2-2 (Q1), SC-003.

### Implementation for User Story 2

- [ ] T015 [US2] En `backend/src/rutas/tablero.rs`, dentro de la misma transacción de T010, la consulta de research D2, literal: `SELECT u.id, u.nombre, u.activo, count(o.id) AS ordenes_abiertas FROM usuario u LEFT JOIN orden o ON o.mecanico_id = u.id AND o.estado <> 'facturada' GROUP BY u.id HAVING (u.rol = 'mecanico' AND u.activo) OR count(o.id) > 0 ORDER BY u.nombre, u.id`. Verde: pruebas 1–6 y 8, con SC-003 en todas. **Cubre:** FR-003, US2-1, US2-2, SC-003.
- [ ] T016 [US2] En `frontend/src/routes/tablero/+page.svelte`: encima de las columnas, «Carga por mecánico» y una fila de chips que salta de línea, «{nombre} {ordenes_abiertas}», con «(de baja)» detrás del nombre si `activo` es `false`. `npm run check`, 0 errores. No se marca hasta T028. **Cubre:** FR-003, US2-1, US2-2.

**Checkpoint**: US1 y US2 funcionan juntas; la carga cuadra con las tarjetas.

---

## Phase 5: User Story 3 - Órdenes con retraso destacadas (Priority: P1)

**Goal**: las órdenes con la entrega pasada se ven destacadas con «Retrasada».

**Independent Test**: una orden abierta con entrega ayer sale destacada con «Retrasada»; una con
entrega hoy, no.

### Tests for User Story 3 ⚠️

- [ ] T017 [US3] Prueba 7 en `backend/tests/spec008_tablero.rs`: entrega ayer → `retrasada: true`; entrega hoy → `false`; una `terminada` con entrega ayer → `true`. La regla ya existe en `OrdenResumen` (`ordenes.rs`), así que pasa al escribirla: se demuestra con **mutación**, cambiando un momento `o.fecha_entrega_prometida < $1` por `<=` en `COLUMNAS_RESUMEN` → falla el caso de hoy; restaurar y comprobar con `cmp`. **Cubre:** FR-004, US3-1, US3-2, caso límite «terminada sin facturar con la entrega pasada».

### Implementation for User Story 3

- [ ] T018 [US3] En `frontend/src/routes/tablero/+page.svelte`, según research D7: una tarjeta con `retrasada` lleva borde de 2 px `border-error-500` y la insignia «Retrasada» (`badge preset-filled-error-500`), visible también sin color por el texto. `npm run check`, 0 errores. No se marca hasta T028. **Cubre:** FR-004, US3-1, US3-2, SC-001.

**Checkpoint**: US1, US2 y US3 completas: el tablero estático cumple todas las historias P1.

---

## Phase 6: User Story 4 - Actualización automática (Priority: P2)

**Goal**: el tablero se vuelve a pedir solo al abrir una orden, con cada cambio de estado, al
reconectar y a la medianoche de Madrid.

**Independent Test**: con el tablero abierto en un navegador, terminar una orden desde otro y ver la
tarjeta cambiar de columna en menos de 5 s, sin recargar.

### Tests for User Story 4 ⚠️

- [ ] T019 [P] [US4] Crear `frontend/src/lib/medianoche.test.ts` (Vitest, `vi.useFakeTimers` y `vi.setSystemTime`, con `process.env.TZ = 'America/New_York'` al principio). **Primero**, una prueba que comprueba que la zona del proceso es de verdad otra: `new Date('2026-07-01T12:00:00Z').getTimezoneOffset()` debe ser 240 (Nueva York) y no −120 (Madrid); si no, falla con un mensaje claro (A2 de analyze-1). Después, los 4 casos de research D6, con la medianoche a las 00:01 de Madrid (un minuto de margen, I1): (1) a las 23:59:00 de Madrid, `programarMedianoche(fn)` no llama a `fn` hasta pasados 1 min + 60 s, y la llama una vez; (2) después vuelve a programarse para la medianoche siguiente; (3) empezando a las 00:01 del 28/03/2027 (23 h) y del 31/10/2027 (25 h), el siguiente salto es a las 00:01 de Madrid del día siguiente; (4) la función que devuelve cancela el temporizador. Más casos de `msHastaMedianocheMadrid` con instantes fijos. Rojo: el módulo no existe. **Cubre:** FR-005, US4-4, caso límite «Pasa la medianoche» (Q3).
- [ ] T020 [P] [US4] Crear `frontend/src/lib/recarga.test.ts` (Vitest, promesas que se resuelven a mano y reloj simulado), con los 3 casos de research D5 para `crearRecarga(fn)`: (1) tres avisos mientras una petición está en curso dan **una** sola petición más al terminar, nunca dos a la vez; (2) un aviso sin nada en curso lanza la petición enseguida; (3) si la petición falla, el siguiente aviso vuelve a pedir. Rojo: el módulo no existe. **Cubre:** FR-005, US4-1 a US4-5, SC-002.
- [ ] T021 [P] [US4] Crear `frontend/src/lib/eventos.test.ts` (Vitest con reloj simulado y un `EventSource` **simulado**: una clase con `readyState`, `onopen`, `onerror`, `addEventListener`, `close` y una lista de instancias creadas, que se pasa como `crear` a `conectarEventos`), con los 5 casos de research D5: (1) `onopen` llama a `alAbrir` la primera vez, tras una reconexión del navegador y en la conexión creada tras un cierre; (2) un evento `orden` llama a `alEvento`; (3) `onerror` con `readyState = CONNECTING` llama a `alError(false)` y **no** crea otra conexión; (4) `onerror` con `readyState = CLOSED` llama a `alError(true)`, no crea otra conexión antes de la `espera` (5 000 ms), crea **una** sola después y la anterior queda cerrada; (5) la función de cierre cierra la conexión y, con una espera pendiente, no se crea ninguna más. Rojo: el módulo no existe. **Cubre:** FR-005, US4-5, caso límite «Se pierde la conexión» (E1 de analyze-1).

### Implementation for User Story 4

- [ ] T022 [P] [US4] Crear `frontend/src/lib/medianoche.ts` según research D6: `msHastaMedianocheMadrid(ahora: Date): number`, con la fecha de Madrid sacada de `Intl.DateTimeFormat('es-ES', {timeZone: 'Europe/Madrid'})` (`formatToParts`), un día más y el instante UTC de su 00:00 con el desfase de `Intl` en dos pasadas, **sin** `getHours`, `setHours` ni otras funciones con la zona del navegador; y `programarMedianoche(fn): () => void`, con un `setTimeout` hasta la medianoche + 60 000 ms que llama a `fn`, se vuelve a programar y devuelve la cancelación. Verde: T019 con `npm test`. **Cubre:** FR-005, US4-4.
- [ ] T023 [P] [US4] Crear `frontend/src/lib/recarga.ts` según research D5: `crearRecarga(fn: () => Promise<void>): () => void`, con una petición en curso como máximo y un indicador de «pendiente» que lanza **una** más al terminar, también si la anterior falló. Verde: T020 con `npm test`. **Cubre:** FR-005, US4-1 a US4-5, SC-002.
- [ ] T024 [P] [US4] Crear `frontend/src/lib/eventos.ts` según research D5: `conectarEventos({alAbrir, alEvento, alError, crear = (url) => new EventSource(url), espera = 5000}): () => void`, sobre `/api/eventos`. `onopen` → `alAbrir()`; evento `orden` → `alEvento()`; `onerror` → `alError(cerrada)`, y si `readyState` es `CLOSED`, cerrar esa conexión y crear otra pasada la `espera`, una sola vez por cierre; la función que devuelve cierra la conexión actual y cancela la espera pendiente. Verde: T021 con `npm test`. **Cubre:** FR-005, US4-5, caso límite «Se pierde la conexión» (E1).
- [ ] T025 [US4] En `frontend/src/routes/tablero/+page.svelte`, según research D5: `const pedir = crearRecarga(cargar)`; al montar, `pedir()`. Después de esa primera carga, **en todos los casos salvo con un 403** (con un 403 se queda el mensaje y no se abre nada, E2; si falló por la red, la conexión se abre igualmente y su primer `onopen` la repite, N2 de analyze-2): `conectarEventos({alAbrir, alEvento, alError})` de `lib/eventos.ts`, con `alAbrir` → quitar el aviso y `pedir()` (el primero y cada vez que la conexión se abre de nuevo tras un error); `alEvento` → `pedir()`; `alError(cerrada)` → mostrar «Sin conexión, reintentando…» encima del tablero, que se queda como estaba, **sin** pedir el tablero, salvo si `cerrada` es verdadero: entonces `pedir()` una vez, para que un 401 de `/api/tablero` lleve a `/entrar` mediante `api.ts` (E1, N3); y `programarMedianoche(pedir)`. Al salir, la función de cierre de `conectarEventos` y la cancelación de la medianoche. `npm run check`, 0 errores. No se marca hasta T028 y T029. **Cubre:** FR-005, FR-006, US1-3, US4-1, US4-2, US4-3, US4-4, US4-5, caso límite «Se pierde la conexión», SC-002.
- [ ] T026 [US4] Comprobar en el código que los **9 movimientos** siguen publicando `CambioEstado::publicar` después del `commit`. `grep -rn '\.publicar(' backend/src` debe dar **8 llamadas**, porque `publico_presupuesto.rs` cubre dos transiciones (I2 de analyze-1): `ordenes.rs` (abrir), `presupuestos.rs`, `publico_presupuesto.rs`, `imputaciones.rs` (dos), `almacen.rs` (dos) y `facturas.rs`, como la lista del plan. Si falta alguno, se registra el hallazgo y una tarea correctora para su spec, sin corregirlo aquí. Anotar el resultado en esta tarea. **Cubre:** FR-005, US4-1, US4-2, US4-3.

**Checkpoint**: todas las historias completas.

---

## Phase 7: Polish & Cross-Cutting Concerns

- [ ] T027 **Cambio compartido · revisa Persona B**: confirmar que el §8 del [quickstart común](../000-base-comun/quickstart.md), la fila de spec-008 del [contrato común](../000-base-comun/contracts/api.md) y R13 del [research común](../000-base-comun/research.md) describen lo implementado (textos exactos, 11 pasos del §8 y los 8 del §9, `medianoche.ts`, `recarga.ts` y `eventos.ts`). Si algo cambió al implementar, corregir los documentos en un commit propio. **Cubre:** principio IV, FR-001 a FR-006.
- [ ] T028 **Verificación en el navegador** del [quickstart común §8](../000-base-comun/quickstart.md), pasos 1, 2, 3, 5, 7, 8, 9 y 10 (el 4 y el 6 se cronometran en T029; el 11 es `npm test`), con la base recién creada y cargada ese mismo día con `backend/seed/demo.sql` y después del §1–§7. En **escritorio grande** (1920 × 1080: las 6 columnas a lo ancho) y en **móvil a 390 px** (una columna entera y el borde de la siguiente, desplazamiento horizontal solo dentro del tablero, ningún desplazamiento horizontal de la página, la carga que salta de línea y el desplazamiento vertical propio de una columna). Para el paso 9, parar y volver a arrancar el servidor (la alternativa sin herramientas, C1 de analyze-1) y, además, la parte opcional con el navegador de `paco` sin conexión (herramientas de desarrollo, o `context.setOffline(true)` en un script) y `3333DDD` terminada mientras tanto. Para el paso 10, un móvil real en la misma red además de la captura a 390 px. Comprobar que ninguna tarjeta muestra datos del cliente. Guardar el informe y las capturas en `specs/008-tablero-ordenes-abiertas/evidencias/T028/`, usando la correspondencia de [quickstart.md](quickstart.md). Si un paso falla, la spec no está terminada (principio IV): se registra el hallazgo y una tarea correctora. **Cubre:** FR-001 a FR-006, US1-1, US1-2, US1-3, US1-4, US2-1, US3-1, US3-2, US4-1, US4-2, US4-3, US4-5, casos límite «Muchas órdenes en una columna» y «Se pierde la conexión», SC-003, SC-004.
- [ ] T029 **Cronometraje humano de SC-001 y SC-002** (pasos 4 y 6 del quickstart común §8), con **dos navegadores reales** y una persona con un reloj. SC-001: alguien que no haya visto antes el tablero dice qué órdenes van con retraso, desde que se ve la página (menos de 10 s; debe nombrar `8888KKK` y no `3333DDD`). SC-002: desde que `luis` pulsa «Terminar reparación» en `2222CCC` hasta que la tarjeta cambia de columna en el navegador de `paco` (menos de 5 s); repetirlo también para los pasos 7 y 8 (abrir y facturar). Se hace **en el mismo recorrido de T028**, porque esas órdenes solo se terminan y facturan una vez; si ya está hecho, se recarga la base y se repite §1–§7. Los tiempos de un navegador automático no valen. Anotar los tiempos en `specs/008-tablero-ordenes-abiertas/evidencias/T029/`. **Cubre:** SC-001, SC-002, US3-1, US4-1, US4-2, US4-3.
- [ ] T030 **Regresión del [quickstart común](../000-base-comun/quickstart.md) entero**, §0 a §9, con los datos demo nuevos de T001 y sobre una base recién cargada, con la numeración del quickstart. Partir de los scripts de la regresión T029 de spec-004 (`specs/004-aceptar-presupuesto-enlace/evidencias/T029/scripts/`), copiándolos y no modificándolos, y añadir §8 y §9. El **§9**, como está escrito tras G1 de analyze-1: el día A, al terminar el §8, se pide la cita de `3579KLM` para el **primer día que ofrezca la página**, que puede no ser el siguiente (el §1 llena alguno y no hay citas en fin de semana); el día B, el de la cita, **sin recargar los datos demo**, se recorren los pasos 2 a 8 con el tablero de `paco` abierto. Se anotan las dos fechas, la de cada parte del informe, y que las órdenes demo que salgan «Retrasada» el día B (por ejemplo `3333DDD`) son lo esperado y no un fallo. Confirmar lo que research D8 da por comprobado: la orden 3 sigue siendo la «R» del §6 sin cambios, y la tarjeta de la orden 7 en pantalla, con su entrega, sin «Retrasada» (lo que T029 de spec-004 dejó pendiente). Guardar el informe en `specs/008-tablero-ordenes-abiertas/evidencias/T030/`. **Cubre:** FR-001 a FR-006, SC-002 (día B), la coherencia de los datos demo con las demás specs, el quickstart común §9.
- [ ] T031 Ejecutar y dejar sin errores ni avisos: `cd backend && cargo test` (incluye `spec008_tablero` y `spec002_ordenes`), `cargo clippy --all-targets -- -D warnings`, `cd frontend && npm run check` y `npm test` (incluye `medianoche.test.ts`, `recarga.test.ts` y `eventos.test.ts`). Anotar los resultados en esta tarea. **Cubre:** las pruebas 1–8 y las de Vitest, es decir: FR-001 (1, 2, 4), FR-002 (1), FR-003 (5, 6), FR-004 (7), FR-005 (Vitest), FR-006 (3), SC-003 (1, 2, 4, 5, 6), SC-004 (1), US4-4 (Vitest).
- [ ] T032 **Revisión de la Persona B** de todo lo compartido antes de fusionar, con la tabla de ficheros compartidos del [plan](plan.md). **Código**: `backend/seed/demo.sql` (T001: entrega de hoy en la orden 3 y el «hoy» de Madrid en la parte 3), `frontend/src/lib/tipos.ts` (T002), `backend/src/rutas/mod.rs` (T003), `backend/src/rutas/ordenes.rs` (T005: `resumenes(…, solo_abiertas)`) y `frontend/src/routes/+layout.svelte` (T011: `<nav>` y `<main>` a todo el ancho solo en `/tablero`; las entradas del menú y el ancho de las demás pantallas no cambian). **Documentos**: `specs/000-base-comun/contracts/api.md` (fila de `GET /api/tablero`: `activo`, orden de la carga y de las tarjetas, suma igual a las tarjetas, 401 y 403, y cuándo vuelve a pedirse), `specs/000-base-comun/quickstart.md` (§8 entero, con SC-001 y SC-002 y el paso 9 sin herramientas; y el **§9 reescrito**, que es de los dos: G1 de analyze-1), `specs/000-base-comun/research.md` (R13: `medianoche.ts`, `recarga.ts` y `eventos.ts`) y `specs/002-abrir-orden-reparacion/hallazgos.md` (N1 cerrado). Confirmar también que sus pruebas de spec-003, 005, 006 y 007 siguen pasando, que la lista y la ficha de las órdenes se ven igual que antes, y que el §6 sigue funcionando con la orden 3. Marcar «Persona B (revisado)» en [decisiones-clarify.md](decisiones-clarify.md). **Cubre:** FR-001, FR-003, FR-005, FR-006 (coherencia de los documentos y ficheros compartidos).

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: T001 y T002 en paralelo, cada uno en su commit. T001 solo bloquea T028–T030;
  T002 bloquea el frontend (T012, T016, T018, T025).
- **Foundational (Phase 2)**: T003 → T004 → T005. Bloquea todas las historias.
- **US1 (Phase 3)**: depende de la Phase 2. Es el MVP.
- **US2 (Phase 4)**: depende de T010 (la transacción y la respuesta) y de T012 (la página).
- **US3 (Phase 5)**: depende de T010 y de T012. Se puede hacer en paralelo con US2.
- **US4 (Phase 6)**: T019–T024 no dependen de nada (módulos sueltos, cada prueba antes de su módulo); T025 depende de T012, T022, T023 y T024.
- **Polish (Phase 7)**: después de las historias, en el orden T027 → T028 → T029 → T030 → T031 → T032.
- **Fusión**: spec-004 no está en `main` y `spec-008` la lleva dentro, así que el orden es 004 → 008.
  `rutas/mod.rs` y `tipos.ts` los tocan también otras specs sin fusionar: quien fusione después
  resuelve ese conflicto.

### Within Each User Story

- Las pruebas (T006–T009, T013–T014, T017, T019–T021) se escriben y fallan **antes** de su
  implementación. T004 y T017 protegen reglas que ya existen: T004 se anota en verde antes y después
  de T005, y T017 se demuestra por mutación.
- La mutación de T014 se hace después de T015, con el código restaurado y comprobado con `cmp`.
- Backend antes que la pantalla que lo usa. La pantalla se va completando: T012 (columnas), T016
  (carga), T018 (retrasada) y T025 (actualización); es el mismo fichero, así que van seguidas.
- Las pruebas de backend van todas en `spec008_tablero.rs`, así que se escriben una detrás de otra.

### Parallel Opportunities

- T001 y T002 entre sí.
- T011 (`+layout.svelte`) en paralelo con T010 y T012.
- T019–T024 (módulos de Vitest) en paralelo con cualquier fase desde la 1: son ficheros
  propios sin dependencias (cada prueba antes de su módulo).
- US2 (backend, T013–T015) y US3 (T017) se pueden escribir seguidas en cualquier orden después de T010.

---

## Parallel Example: User Story 4

```bash
Task: "T019 medianoche.test.ts (rojo)"   # luego T022 medianoche.ts
Task: "T020 recarga.test.ts (rojo)"      # luego T023 recarga.ts
Task: "T021 eventos.test.ts (rojo)"      # luego T024 eventos.ts
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. T001 y T002, cada uno en su commit y con la revisión de B.
2. Phase 2.
3. US1: el jefe ve las columnas con las órdenes abiertas, ordenadas por entrega, y los demás roles no
   lo ven. Ya sustituye a la pizarra. Se valida con el quickstart común §8, pasos 1, 2 y 5.

### Incremental Delivery

1. US1 → columnas y acceso (P1).
2. US2 → carga por mecánico (P1).
3. US3 → retrasadas destacadas (P1).
4. US4 → actualización automática, reconexión y medianoche (P2).
5. Polish → documentos, navegador, cronometraje, regresión completa, comprobaciones y revisión de B.

---

## Notes

- Cada tarea se marca `[X]` al terminarla, con una nota de lo hecho y de las desviaciones.
- Las tareas de pantalla (T011, T012, T016, T018 y T025) no se marcan hasta verificarlas en el
  navegador (T028).
- Cada «cambio compartido · revisa Persona B» va en su propio commit.
- Si al implementar aparece algo que no está en la spec, se anota y se propone; no se construye
  (principio III).
