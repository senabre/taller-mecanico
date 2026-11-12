# Implementation Plan: spec-008 · Tablero de órdenes abiertas

**Branch**: `008-tablero-ordenes-abiertas` (rama de trabajo `spec-008`) | **Date**: 2026-10-15, completado el 2026-11-12 tras el clarify Q1–Q4 | **Spec**: [spec.md](spec.md) | **Responsable**: Persona A

**Input**: Feature specification from `/specs/008-tablero-ordenes-abiertas/spec.md`

**Note**: Lo común (pila, arquitectura, fechas, errores, roles, bus de eventos, transiciones,
estructura) está en el **plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md). Este plan
solo recoge lo propio de la spec. Decisiones: [research.md](research.md) (D1–D9). El modelo de datos,
el contrato y la guía de validación son los comunes; [data-model.md](data-model.md),
[contracts/api.md](contracts/api.md) y [quickstart.md](quickstart.md) solo explican cómo los usa esta
historia.

## Summary

`/tablero`, solo para el jefe, muestra 6 columnas con las órdenes no facturadas, ordenadas por fecha
de entrega, la carga de cada mecánico y las retrasadas destacadas. `GET /api/tablero` lee una
instantánea con dos consultas: las tarjetas reutilizan `OrdenResumen` de spec-002 y la carga cuadra
siempre con ellas. La pantalla vuelve a pedir el tablero entero con cada evento de
`GET /api/eventos` (SSE, research R11), al reconectar y a la medianoche de Madrid. Cada spec que
cambia un estado publica el evento con `CambioEstado::publicar` después del `commit` (research R14).

## Technical Context

**Language/Version · Dependencies · Storage · Testing · Platform**: igual que el plan maestro
(Rust + Axum + SQLx, SvelteKit, PostgreSQL 16, `#[sqlx::test]`, Vitest). **No se añade ninguna
dependencia ni migración**: la medianoche se calcula con `Intl` y se prueba con el reloj simulado que
ya trae Vitest (research D6).

**Project Type**: aplicación web; módulo `tablero.rs` dentro del backend común.

**Performance Goals**: un cambio aparece en el tablero en menos de 5 s (SC-002); las retrasadas se
identifican en menos de 10 s (SC-001); `GET /api/tablero` en menos de 1 s con unas decenas de
órdenes abiertas.

**Constraints**: todo en español; fechas dd/mm/aaaa y «hoy» de Madrid (R12), también en el
navegador (D6); solo lectura; a 390 px sin desplazamiento horizontal de la página, y en una pantalla
grande las 6 columnas a lo ancho (D7).

**Scale/Scope**: solo lectura de `orden`, `vehiculo` y `usuario`; 1 endpoint nuevo
(`GET /api/tablero`) y el común `GET /api/eventos`; 1 pantalla nueva.

- **Endpoints** ([contrato común](../000-base-comun/contracts/api.md#spec-008--tablero)):
  `GET /api/tablero` y `GET /api/eventos` (base común).
- **Pantalla**: `/tablero`.

No queda ningún NEEDS CLARIFICATION: el clarify del 2026-11-12 cerró Q1–Q4. Lo que dejó para el
plan está en [research.md](research.md): el cambio en `resumenes()` (D3), cómo se vuelve a pedir el
tablero (D5), la medianoche de Madrid (D6), el diseño en pantalla grande y en el móvil (D7) y los
datos demo (D8). Q2 se ha corregido aquí: el 403 se muestra como en las demás pantallas protegidas,
no con `+error.svelte`, que ninguna usa para eso (D4).

## Constitution Check

*GATE: antes de Phase 0 y de nuevo después de Phase 1.* Constitución 1.2.1.

| Principio | Comprobación | Estado |
|---|---|---|
| I. Simplicidad | SSE en vez de WebSockets (R11). Una consulta para las tarjetas, que ya existía (`OrdenResumen`), con un parámetro más (D3), y otra para la carga (D2). La pantalla siempre vuelve a pedir el tablero entero, por un solo camino (D5). Sin dependencias: `Intl` y el reloj simulado de Vitest (D6). Sin arrastre de tarjetas. | ✅ |
| II. Español y euro | Textos fijados en español (D7). Fechas dd/mm/aaaa con `fecha()` de `formato.ts`. «Hoy» de Madrid en el servidor (R12) y la medianoche de Madrid en el navegador, aunque esté en otra zona (D6). | ✅ |
| III. Cero alcance fantasma | Solo consulta: sin botones, sin arrastre y sin enlaces en las tarjetas. «Sin órdenes» y «(de baja)» solo explican lo que pide la spec. No se añaden filtros, totales por columna ni sonidos. | ✅ |
| IV. Verificable sin código | Cada escenario tiene su paso en el [quickstart común §8](../000-base-comun/quickstart.md) ([correspondencia](quickstart.md)), con dos navegadores y SC-001 y SC-002 cronometrados (D9). Para ver una orden con entrega hoy se cambia la entrega de la orden 3 en los datos demo (D8). La medianoche, un mecánico de baja con órdenes y una columna vacía no se pueden provocar a mano con los datos demo: quedan en pruebas automáticas, y el quickstart lo dice. | ✅ |
| V. Datos con respeto | Las tarjetas solo muestran número, matrícula, mecánico y entrega: la consulta no une con `cliente`. Los eventos solo llevan `{id, estado}`. Las pruebas no usan valores demo. | ✅ |

**Re-check post-diseño (Phase 1)**: los cambios en el contrato, el research, el quickstart y los
datos demo comunes solo concretan reglas de la spec (Q1–Q4) y no añaden tablas. El cambio de ancho del
`<nav>` y el `<main>` de `+layout.svelte` afecta solo a `/tablero`. Los ficheros compartidos están en la tabla de
abajo, para la revisión de la Persona B. ✅

## Project Structure

### Documentation (this feature)

```text
specs/008-tablero-ordenes-abiertas/
├── spec.md
├── plan.md                 # este fichero
├── research.md             # decisiones D1–D9
├── data-model.md           # qué lee esta historia del modelo común
├── contracts/api.md        # qué hace la pantalla con cada llamada del contrato común
├── quickstart.md           # escenarios de la spec ↔ pasos del quickstart común §8
├── decisiones-clarify.md   # registro del clarify Q1–Q4
└── checklists/requirements.md
```

### Source Code (repository root)

```text
backend/src/rutas/tablero.rs                 # GET /api/tablero (D1, D2, D4)
backend/src/rutas/mod.rs                     # COMPARTIDO: pub mod tablero y .merge(tablero::rutas())
backend/src/rutas/ordenes.rs                 # DE A (spec-002): resumenes(…, solo_abiertas) (D3)
backend/tests/spec008_tablero.rs             # pruebas de reglas (ver abajo)
backend/seed/demo.sql                        # COMPARTIDO: entrega de la orden 3 hoy, en Madrid (D8)
frontend/src/lib/tipos.ts                    # COMPARTIDO: Tablero, ColumnaTablero, CargaMecanico
frontend/src/lib/medianoche.ts               # msHastaMedianocheMadrid y programarMedianoche (D6)
frontend/src/lib/medianoche.test.ts          # Vitest con reloj simulado y TZ de otra zona (D6)
frontend/src/lib/recarga.ts                  # crearRecarga: una sola petición a la vez, más una (D5)
frontend/src/lib/recarga.test.ts             # Vitest con promesas manuales y reloj simulado (D5)
frontend/src/lib/eventos.ts                  # conectarEventos: EventSource y reconexión tras CLOSED (D5, E1)
frontend/src/lib/eventos.test.ts             # Vitest con EventSource simulado y reloj simulado (D5, E1)
frontend/src/routes/tablero/+page.svelte     # la pantalla (D5, D7)
frontend/src/routes/+layout.svelte           # COMPARTIDO: <nav> y <main> a todo el ancho en /tablero (D7)
```

**Structure Decision**: la del plan maestro. Esta spec añade su módulo de rutas, su fichero de
pruebas, una pantalla y un módulo de frontend, y toca los ficheros compartidos de la tabla.

### Ficheros compartidos (los revisa la Persona B en una tarea final, como T031 en spec-002 y spec-004)

| Fichero | Cambio | Por qué | Estado |
|---|---|---|---|
| `specs/000-base-comun/contracts/api.md` | `GET /api/tablero`: `activo` en `carga_mecanicos`, orden de la carga y de las tarjetas, la suma igual a las tarjetas, 401 y 403 con su texto; cuándo vuelve a pedirse el tablero | Q1, Q2, Q3, Q4 | Hecho (clarify y plan) |
| `specs/000-base-comun/quickstart.md` | §8 reescrito: 11 pasos, con SC-001 y SC-002 cronometrados, dos navegadores, desconexión, 390 px y la medianoche en `npm test` | Principio IV | Hecho |
| `specs/000-base-comun/research.md` | R13: Vitest también para `medianoche.ts`, `recarga.ts` y `eventos.ts` | Q3, D5, D6, E1 | Hecho |
| `specs/002-abrir-orden-reparacion/hallazgos.md` | N1 cerrado | N1 | Hecho en el clarify |
| `backend/seed/demo.sql` | Parte 3: orden 3 con la entrega hoy; `fecha_entrega_prometida` con el «hoy» de Madrid; comentario de la orden 3 | D8. La orden 3 es la «R» del §6 y de la regresión T029 de spec-004: comprobado que la entrega no interviene en nada de lo que esperan (tabla en D8) | **Pendiente**: primera tarea, antes de recorrer el §8 |
| `backend/src/rutas/ordenes.rs` (spec-002, Persona A) | `resumenes(conexion, mecanico_id, solo_abiertas)`; `listar_ordenes` pasa `false` | D3 | **Pendiente**: al implementar, con su prueba antes. Lo usan las pantallas de B (lista y ficha), así que B lo revisa |
| `backend/src/rutas/mod.rs` | `pub mod tablero;` y `.merge(tablero::rutas())` | Registrar el módulo | **Pendiente** |
| `frontend/src/lib/tipos.ts` | `CargaMecanico {id, nombre, activo, ordenes_abiertas}`, `ColumnaTablero {estado, ordenes: OrdenResumen[]}`, `Tablero {columnas, carga_mecanicos}` | Tipos del contrato | **Pendiente** |
| `frontend/src/routes/+layout.svelte` | `<nav>` y `<main>` sin `max-w-6xl` cuando la ruta empieza por `/tablero`. **Las entradas del menú no cambian** (ya enseña «Tablero» solo al jefe), y en las demás pantallas el menú y el contenido siguen con el mismo ancho | D7, I3 | **Pendiente** |

`formato.ts` no cambia: ya tiene `fecha()`, `NOMBRE_ESTADO` y `ESTADOS_ABIERTOS`. `eventos.rs` y
`estados.rs` tampoco: los 9 movimientos ya publican su evento, en 8 llamadas.

### Reglas clave

- **`GET /api/tablero`** (jefe): `exigir_rol(&[Rol::Jefe])` → 403 «No tienes permiso para hacer
  esto» a los demás (D4). Con una conexión normal y **sin transacción** (D1, revisado por V3):
  1. `resumenes(&mut conexion, None, true)`: las no facturadas, con `retrasada` calculada como dice el
     [data-model común](../000-base-comun/data-model.md) (no facturada y entrega < `fechas::hoy()`,
     Europe/Madrid; hallazgos I1 e I2 de spec-002). No hay una segunda definición.
  2. Se reparten en las 6 columnas, en el orden de `Estado`, aunque estén vacías, y cada una se
     ordena por `(fecha_entrega_prometida, numero)` ascendentes (Q4).
  3. La carga, contada en Rust sobre esa misma lista (D2): activos aunque tengan 0; cualquier
     usuario con órdenes abiertas, aunque esté inactivo; por nombre (Q1). A la base solo se le
     piden esos usuarios. La suma es el número de tarjetas por construcción (SC-003).
- **Frontend `/tablero`** (D5, D7): `cargar()` pide el tablero entero al abrir. Después de esa
  primera carga, **en todos los casos salvo con un 403** (con un 403, solo el mensaje y ninguna
  conexión, E2), la página abre `conectarEventos` (`eventos.ts`) y `programarMedianoche`. Si la
  primera carga falló por otra causa, el primer `onopen` la repite y el tablero se recupera solo
  (N2). El tablero se vuelve a pedir: en cada `onopen` (el primero y cada vez que la conexión se
  abre de nuevo tras un error, reconecte el navegador solo o la cree la página tras un cierre), con
  cada evento `orden` y a medianoche. Un error **no** lo pide, salvo una excepción: con la conexión
  cerrada (`CLOSED`, por ejemplo un 401), se pide **una vez** antes de crear otra conexión pasados
  5 s, y si `/api/tablero` da 401, `api.ts` lleva a `/entrar` (E1, N3). `onerror` muestra «Sin
  conexión, reintentando…». Solo una petición a la vez, más una si llega otro aviso mientras tanto
  (`crearRecarga`, en `recarga.ts`). Al salir, se cierran la conexión, su espera y el temporizador.
- **Medianoche** (D6): `programarMedianoche` salta a medianoche, a las 00:01 de Madrid (00:00 + 60 s de margen
  para la diferencia de reloj con el servidor), calculadas con `Intl`, nunca con la zona del
  navegador, y se vuelve a programar en cada salto.
- **Eventos que mueven el tablero** (comprobado en la rama spec-008): 9 movimientos en 8 llamadas. `ordenes.rs:403` (abrir →
  recibida, spec-002); `presupuestos.rs:310` (recibida → en diagnóstico, 003);
  `publico_presupuesto.rs:204` (aceptar el inicial: en diagnóstico → en reparación; aceptar la
  ampliación: pendiente de conformidad → en reparación, 004); `imputaciones.rs:193` (→ pendiente de
  conformidad al pasar del 110 %) e `imputaciones.rs:227` (→ terminada, 005); `almacen.rs:430` (→ en
  espera de piezas) y `almacen.rs:501` (→ en reparación al recibir la última pieza, 006);
  `facturas.rs:291` (→ facturada, 007). Todos después del `commit`.

### Pruebas

`backend/tests/spec008_tablero.rs` (`#[sqlx::test]`, con datos creados en cada prueba y sin valores
demo, principio V). En todas las que devuelven el tablero se comprueba además que la suma de
`ordenes_abiertas` es igual al número de tarjetas (SC-003):

1. Una orden en cada estado abierto y una facturada → 6 columnas en el orden de `Estado`, cada orden
   en la suya y la facturada en ninguna, ni en las tarjetas ni en la carga (US1-1, US1-2, SC-004).
2. Sin ninguna orden → 6 columnas vacías, y los mecánicos activos con 0 (caso límite, US2-1).
3. Acceso: recepción, mecánico y almacén → 403 «No tienes permiso para hacer esto» (texto exacto);
   jefe → 200; sin sesión → 401 (US1-3, FR-006, Q2).
4. Orden de las tarjetas: en una columna, entregas pasado mañana, ayer y dos de hoy (números 12 y 7)
   → ayer, 7, 12, pasado mañana (US1-4, Q4).
5. Carga: Luis con 3 abiertas y 1 facturada, Marta con 1 y un tercer mecánico activo sin órdenes →
   3, 1 y 0, por nombre (US2-1).
6. Inactivos: uno con 1 orden abierta → sale con `activo: false` y 1; otro sin órdenes → no sale
   (US2-2, Q1). **Se comprueba que falla con la condición anterior**
   (`WHERE u.rol = 'mecanico' AND u.activo`).
7. Retrasada: entrega ayer → `retrasada: true`; entrega hoy → `false`; terminada sin facturar con
   entrega ayer → `true` (US3-1, US3-2, caso límite).
8. `resumenes(…, false)` sigue devolviendo también las facturadas, en el mismo orden que antes: la
   lista de órdenes de spec-002 no cambia (D3).
9. Peticiones cortadas a mitad (V3): unas 4600 peticiones al tablero, cortadas en todos sus puntos
   de espera (tras 1–200 vueltas del planificador, 25 pasadas); tras **cada** corte, con una conexión
   aparte, ninguna conexión queda «idle in transaction»; y al final, entrar sigue funcionando. Falla
   con la transacción que tenía D1. Como tarda unos 100 s, lleva `#[ignore]` y la suite normal la
   salta: se lanza aparte con `cargo test --test spec008_tablero -- --ignored` (en T031 y en la
   revisión de la Persona B, T032).

`frontend/src/lib/medianoche.test.ts` (Vitest, `vi.useFakeTimers`, `vi.stubEnv('TZ',
'America/New_York')`, sin `@types/node`): primero comprueba que la zona del proceso es de verdad otra y, si no, falla
(A2); después, los 4 casos de [research](research.md) D6 (US4-4).
`frontend/src/lib/recarga.test.ts` (Vitest, promesas que se resuelven a mano y reloj simulado): los
3 casos de [research](research.md) D5 (US4-1 a US4-5, SC-002).
`frontend/src/lib/eventos.test.ts` (Vitest, `EventSource` simulado y reloj simulado): los 5 casos
de [research](research.md) D5, incluida la reconexión tras `CLOSED` (US4-5, E1).

Lo que depende del navegador real y del tiempo real (el `EventSource` de verdad y los 5 s) se
comprueba en el quickstart §8 con dos navegadores (D9), no con pruebas E2E (R13).

**Dependencias**: base común (`eventos`, `cambiar_estado`); spec-002 (`OrdenResumen`,
`resumenes()` y el evento de abrir), ya en `main`; los eventos de las specs 003, 005, 006 y 007, ya en
`main`; y los de spec-004, que **no** está en `main` (PR pendiente): `spec-008` la lleva dentro, así
que 008 se fusiona después de 004 (orden 001 → 002 → 004 → 008).

**Cambios en documentos compartidos** (los revisa la Persona B antes de fusionar, en la tarea final):
los de la tabla de arriba. Los cambios en código compartido se hacen al implementar, con la misma
revisión.

## Complexity Tracking

Sin violaciones de la constitución que justificar.
