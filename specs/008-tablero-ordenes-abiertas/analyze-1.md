# Analyze 1 · spec-008 · Tablero de órdenes abiertas

**Fecha**: 2026-11-12 · **Rama**: `spec-008` (commit `fbd54a8`) · **Artefactos**: [spec.md](spec.md),
[plan.md](plan.md), [research.md](research.md) (D1–D9), [data-model.md](data-model.md),
[contracts/api.md](contracts/api.md), [quickstart.md](quickstart.md),
[decisiones-clarify.md](decisiones-clarify.md) y [tasks.md](tasks.md) (T001–T030) · **Contexto**:
[000-base-comun](../000-base-comun/) (contrato, quickstart §8 y §9, research R11–R14) y el código de
la rama · **Constitución**: 1.2.1.

Análisis de solo lectura: no se ha corregido nada. Las decisiones se registran en
[hallazgos.md](hallazgos.md).

**Sobre el código de referencia**: `spec-008` contiene `origin/main` (001, 002, 003, 005, 006 y 007)
y `origin/spec-004`, que aún no está en `main`. `origin/spec-008` está en el mismo commit que la rama
local.

**Alcance de las comprobaciones**: todo sale de leer el código y los documentos. No se ha ejecutado
nada: `tablero.rs`, `spec008_tablero.rs`, `medianoche.ts` y `recarga.ts` aún no existen.

## Specification Analysis Report

| ID | Category | Severity | Location(s) | Summary | Recommendation |
|----|----------|----------|-------------|---------|----------------|
| E1 | Coverage | MEDIUM | research.md:95 (D5) · tasks.md:159 (T023) · contracts/api.md (fila `onerror`) · `backend/src/eventos.rs:48–49` | **`EventSource` no se reconecta siempre.** Solo lo hace tras un error de red. Si la respuesta de `/api/eventos` no es un 200 con `text/event-stream` (por ejemplo, un 401 porque la sesión caducó mientras se cortaba la red), el navegador cierra la conexión para siempre (`readyState === CLOSED`). El tablero se queda con «Sin conexión, reintentando…», sin eventos y sin ir a `/entrar`, hasta la petición de medianoche. D5 dice «la reconexión la hace `EventSource` solo», y eso no es cierto en este caso. | En `onerror`, si `readyState` es `CLOSED`, llamar a `pedir()` (que con un 401 lleva a `/entrar`) y volver a crear el `EventSource` pasado un rato. Añadirlo a D5, a T023 y al contrato de la spec. |
| G1 | Quickstart §9 | MEDIUM | quickstart común:219–223 (§9) · tasks.md:171 (T028) · data-model común (cita «de lunes a viernes, desde mañana») · quickstart común §1.3–§1.6 | **El §9 se puede recorrer con lo que habrá al terminar 008, pero no dice cómo.** Las 8 llamadas que publican eventos, la ficha, Mailpit y el tablero estarán todos. Pero: (a) «recorrer §1 → §8 seguidos con un vehículo nuevo» no dice qué pasos se repiten, porque los de §1–§8 nombran vehículos demo; (b) «ocupa dos días» no tiene por qué ser dos días seguidos: el §1 llena el primer día libre (cupo de 2), y los fines de semana no se dan citas; (c) el día de la cita los datos demo ya no son de «hoy», y no se pueden recargar sin borrar la cita del primer día: `3333DDD` sale «Retrasada», y el §8 ya no da lo que dice; (d) T028 no cuenta con nada de esto. | Escribir el §9 como una lista de pasos con el vehículo nuevo (pedir cita; el día de la cita, abrir la orden, diagnóstico, presupuesto, aceptar desde Mailpit, imputar, terminar y facturar), con lo que debe verse en el tablero tras cada paso. Avisar de que el día de la cita los datos demo están desfasados, y ajustar T028. |
| I1 | Inconsistency | MEDIUM | spec.md:30, :122 (US4-4), :158 (FR-005) · plan.md:139 · research.md D6 · contracts/api.md:49 · tasks.md:152, :157 | **La spec dice «a las 00:00» y el plan, «a las 00:01».** D6 añade 60 s de margen por la diferencia de reloj con el servidor, y la prueba y la tarea están escritas para las 00:01. La spec (FR-005, US4-4 y la aclaración de Q3) sigue diciendo 00:00, así que la prueba y el requisito no dicen lo mismo. | Cambiar la spec a «al empezar el día (00:01 de Madrid, research D6)», o quitar el margen. |
| E2 | Coverage | LOW | tasks.md:159 (T023) · research.md D4 | Con un 403, T023 abre igualmente el `EventSource` y programa la medianoche: cada evento de cualquier orden repite una petición que vuelve a dar 403. | Conectar el `EventSource` y la medianoche solo después de una primera carga con 200. |
| A1 | Ambiguity | LOW | tasks.md:94 (T010) · plan.md (Reglas clave) | T010 pide `BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY` y el plan, `resumenes(&mut tx, …)`. Con SQLx 0.8.6 (`Cargo.lock`), `pool.begin()` ya envía su propio `BEGIN`, y una transacción se pasa como `&mut *tx`. | Nombrar `estado.db.begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY")`, que existe en 0.8.6, y `&mut *tx`. |
| A2 | Ambiguity | LOW | tasks.md:152 (T019) · research.md D6 | T019 cambia `process.env.TZ` dentro de la prueba. Funciona si Vitest ejecuta las pruebas en procesos aparte (`forks`), pero si algún día cambia la configuración, la prueba podría dar verde con la zona del sistema sin que nadie lo note. | Poner `TZ` en `test.env` de `vite.config.ts`, o añadir al principio de la prueba una comprobación de que la zona de verdad no es la de Madrid (`getTimezoneOffset()`). |
| I2 | Inconsistency | LOW | tasks.md:160 (T024) · plan.md (Reglas clave) | T024 dice «los 9 movimientos» y espera, con `grep`, los ficheros del plan. Pero `grep` da **8** llamadas, porque `publico_presupuesto.rs:204` cubre dos transiciones. Quien la haga puede pensar que falta una. | Decir «8 llamadas, que cubren 9 movimientos». |
| C1 | Constitution (IV) | LOW | quickstart común:207 (§8.9), :211 (§8.10) | Los pasos 9 y 10 usan las herramientas de desarrollo. No es un criterio de éxito (SC-001–SC-004 se comprueban en la app), así que no incumple el principio IV, pero una persona no técnica no sabría hacerlos. | Ofrecer la alternativa sin herramientas: desactivar el wifi o el cable en el paso 9 y abrirlo en un móvil en el paso 10. |
| I3 | Inconsistency | LOW | `frontend/src/routes/+layout.svelte:44` (`<nav>`), :65 (`<main>`) · research.md D7 · tasks.md (T011) | T011 solo ensancha el `<main>`: en una pantalla grande, el menú queda centrado en 1152 px y el tablero ocupa todo el ancho. Es solo estético. | Decidir si el `<nav>` también se ensancha en `/tablero`, o justificar que se deja así. |
| I4 | Inconsistency | LOW | decisiones-clarify.md:69 («Cambios en ficheros compartidos») · plan.md (tabla de compartidos) | La tabla del clarify solo lista el contrato, `tipos.ts` y N1. La del plan (la que usa T030) tiene además `demo.sql`, `ordenes.rs`, `mod.rs`, `+layout.svelte`, el quickstart §8 y R13. Las dos dicen ser la lista para la Persona B. | Dejar en el clarify una nota que remita a la tabla del plan como lista completa. |

### Comprobaciones pedidas sin hallazgo

| Comprobación | Resultado |
|---|---|
| **D3 no rompe a quien ya usa `resumenes()`** | En la rama solo la llama `listar_ordenes` (`ordenes.rs:424`), que pasará `false`. La ficha (`ordenes.rs:178`) usa `COLUMNAS_RESUMEN` y `DESDE` con su propia consulta, y no cambia. Ninguna prueba la llama directamente. Ninguna rama remota (`origin/spec-001`…`spec-008`, todas ya dentro de `spec-008`) tiene otra llamada. T004 protege la lista antes del cambio. |
| **Los 9 puntos que publican eventos** | 8 llamadas a `.publicar(` en `ordenes.rs:399`, `presupuestos.rs:310`, `publico_presupuesto.rs:204` (dos transiciones), `imputaciones.rs:193` y `:227`, `almacen.rs:430` y `:501` y `facturas.rs:291`. Todas van después del `commit`. Fuera de `estados.rs:73` no hay ningún `UPDATE … SET estado` de `orden`, así que ningún cambio de estado se salta el evento. |
| **`eventos.rs`** | Pide sesión de cualquier rol (`_usuario: Usuario`), envía `{id, estado}` y se mantiene viva con `KeepAlive`. Si un tablero va lento, los eventos que pierde (`Lagged`) se ignoran, y el siguiente aviso pide todo el tablero (D5). No hay ninguna capa de compresión (`lib.rs:47`, solo `TraceLayer`) que pudiera retener los eventos. La reconexión tiene un límite: E1. |
| **`+layout.svelte`** | El menú ya enseña «Tablero» solo al jefe, e `INICIO_ROL` lo lleva a `/tablero`: no cambian, como dice el plan. `<main class="mx-auto max-w-6xl">` (línea 65) es lo que cambia T011. Ver I3. |
| **Contrato común** | La fila de `GET /api/tablero` coincide con D1, D2 y D4: `activo`, orden de la carga y de las tarjetas, la suma igual a las tarjetas, 401 y 403 con el texto de `ErrorApi::Prohibido` («No tienes permiso para hacer esto», `error.rs:48–50`). |
| **Quickstart común §8** | Los 11 pasos coinciden con la spec corregida (403 dentro de la página), con D8 (`3333DDD` con la entrega hoy), D9 y la correspondencia de [quickstart.md](quickstart.md). La carga «Luis 6 · Marta 3» tras el §1–§7 sale de los datos demo (Luis 5 + `1414NNN`; `1313MMM` ya facturada). `2222CCC` sigue «En reparación» y con imputaciones al llegar al paso 6. |
| **Research R13** | Dice «Vitest para `formato.ts`, `medianoche.ts` y `recarga.ts`», igual que el plan, D5, D6 y T019–T022. |
| **`OrdenResumen` y «retrasada»** | `ordenes.rs:98`: `o.estado <> 'facturada' AND o.fecha_entrega_prometida < $1`, con `$1 = fechas::hoy()`. Coincide con el data-model común, la spec y la prueba 7 (T017, con mutación). |
| **D2 en PostgreSQL** | `GROUP BY u.id` con `u.nombre`, `u.activo` y `u.rol` en el `SELECT` y el `HAVING` es válido, porque `id` es la clave primaria de `usuario`. |

**Coverage Summary Table:**

| Requirement Key | Has Task? | Task IDs | Notes |
|-----------------|-----------|----------|-------|
| FR-001 (6 columnas, solo abiertas, orden) | Sí | T005–T007, T009, T010, T012, T026, T028 | |
| FR-002 (contenido de la tarjeta) | Sí | T006, T012, T026 | |
| FR-003 (carga, activos con 0, «(de baja)») | Sí | T013–T016, T026 | El «(de baja)» solo se comprueba en pruebas automáticas (D8) |
| FR-004 (retrasada) | Sí | T017, T018, T026, T027 | |
| FR-005 (actualización) | Sí | T019–T024, T026, T027 | Ver E1 e I1 |
| FR-006 (solo el jefe) | Sí | T008, T010, T012, T026 | Ver E2 |
| SC-001 (retrasadas en < 10 s) | Sí | T027 (cronometrado), T009, T018 | |
| SC-002 (< 5 s) | Sí | T027 (cronometrado), T020, T022, T023 | |
| SC-003 (carga = tarjetas) | Sí | T013–T015, T026 | Comprobado en cinco pruebas |
| SC-004 (todas las abiertas, ninguna facturada) | Sí | T006, T010, T026 | |
| Casos límite (vacía, terminada retrasada, muchas, conexión, medianoche) | Sí | T007, T017, T012/T026, T023/T026, T019/T021 | |

**Constitution Alignment Issues:** ninguno grave. C1 (LOW) es una mejora de cara al principio IV en
pasos que no son criterios de éxito.

**Unmapped Tasks:** ninguna. Todas terminan con «**Cubre:**». T003 y T025 cubren la infraestructura
y los documentos.

**Metrics:**

- Total Requirements: 10 (FR-001–FR-006, SC-001–SC-004)
- Total Tasks: 30
- Coverage %: 100 % (10 de 10 con al menos una tarea)
- Ambiguity Count: 2 (A1, A2)
- Duplication Count: 0
- Critical Issues Count: 0
- Por gravedad: 0 CRITICAL, 0 HIGH, 3 MEDIUM (E1, G1, I1) y 7 LOW (E2, A1, A2, I2, C1, I3, I4):
  10 hallazgos en total.

## Next Actions

- No hay CRITICAL ni HIGH: se puede pasar a `/speckit-implement` después de decidir los tres MEDIUM.
  E1 e I1 cambian una tarea (T023) y la redacción de la spec. G1 cambia el §9 del quickstart común y
  T028, y conviene acordarlo con la Persona B, porque el §9 es de los dos.
- Los LOW se pueden corregir en el mismo paso: son cambios de una línea en las tareas o en el
  quickstart.
- Después, un `analyze-2.md` para comprobar las correcciones, como en 001, 002 y 004.
