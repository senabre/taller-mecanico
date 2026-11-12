# Seguimiento de hallazgos · spec-008 Tablero de órdenes abiertas (HU-08)

**Origen**: [analyze-1.md](analyze-1.md) (2026-11-12) · **Revisiones**: [analyze-2.md](analyze-2.md) y [analyze-3.md](analyze-3.md)
(2026-11-12). Decisiones del 2026-11-12.

**Decisión**: *pendiente* = sin decidir; *corregido* = se ha cambiado el documento o la tarea;
*justificado* = no se cambia, con un motivo explícito; *falso positivo* = el hallazgo no se
sostiene; *registrado* = se resuelve fuera de esta spec.

**Estado**: *Abierto* = sin decisión; *Cerrado (corregido; pendiente de verificar en analyze-2)* =
cambio aplicado y todavía sin comprobar en un nuevo análisis; *Cerrado (corregido; revisión de B en
T032)* = además toca documentos comunes y la Persona B lo revisa en T032; *Cerrado (verificado en
analyze-2)* o *(verificado en analyze-3)* = ese análisis ha comprobado el cambio; *Cerrado (verificado en analyze-2; revisión de B en
T032)* = comprobado, pero la revisión de la Persona B se da por hecha solo cuando se marque T032; *Cerrado (justificado)* = no se cambia, con su
motivo; *Asignado a …* = lo resuelve esa spec o persona.

## Hallazgos de analyze-1

| ID | Historia o criterio | Gravedad | Evidencia | Decisión | Responsable | Estado |
|----|---------------------|----------|-----------|----------|-------------|--------|
| E1 | FR-005, US4-5, caso límite «Se pierde la conexión» · research D5 · T025 · `eventos.rs` | MEDIUM | `EventSource` solo se reconecta tras un error de red. Si `/api/eventos` responde algo que no es un 200 con `text/event-stream` (un 401 con la sesión caducada), la conexión se cierra para siempre: «Sin conexión, reintentando…» se queda fijo, sin eventos y sin ir a `/entrar`, hasta la petición de medianoche. D5 decía que la reconexión es siempre automática. | Corregido (2026-11-12): con la conexión cerrada (`CLOSED`) se crea otra pasados 5 s; `alError` pide el tablero, y si `/api/tablero` da 401, `api.ts` lleva a `/entrar`; al reconectar se pide el tablero entero (Q3). La conexión va en `lib/eventos.ts` (`conectarEventos`), con su prueba de Vitest y un `EventSource` simulado: T021 (rojo) y T024 (verde); T025 la usa. Actualizados research D5, el plan, el contrato de la spec y R13 común (`eventos.ts`). Las tareas desde la antigua T023 se renumeran (+2), y las de US4 quedan T019–T026. | Persona A | Cerrado (verificado en analyze-2; revisión de B en T032, por R13). La redacción de `alError` en D5, en N3 |
| G1 | Quickstart común §9 · T030 (antes T028) | MEDIUM | El §9 se puede recorrer con lo que habrá al terminar 008, pero no decía qué pasos se repiten con el vehículo nuevo, que los dos días pueden no ser seguidos, ni que el día de la cita los datos demo están desfasados y no se pueden recargar. | Corregido (2026-11-12), redactado por la Persona A: el §9 son 8 pasos con `3579KLM`. El día A, después del §8, se pide la cita para el primer día que ofrezca la página, que puede no ser el siguiente. El día B no se recargan los datos demo, y el paso lo dice: es normal que alguna orden demo salga «Retrasada» (por ejemplo, `3333DDD`). Después, orden Nº 13, diagnóstico, presupuesto, aceptación desde Mailpit, imputación, terminar y facturar, con lo que se ve en el tablero en cada paso. T030 sigue esos pasos. | Persona A (redacción) y Persona B (revisión) | Cerrado (verificado en analyze-2; revisión de B en T032) |
| I1 | FR-005, US4-4, aclaración de Q3 · research D6 · contracts/api.md · T019, T022 | MEDIUM | La spec decía que el tablero se vuelve a pedir «a las 00:00»; D6, el contrato de la spec y las tareas lo hacen a las 00:01, con 60 s de margen. | Corregido (2026-11-12): la spec se alinea con el plan, «a medianoche (00:01 de Madrid, con un minuto de margen)», en la aclaración de Q3, US4-4 y FR-005. También en el plan y en T019. | Persona A | Cerrado (verificado en analyze-2) |
| E2 | FR-006, US1-3 · research D4 y D5 · T025 | LOW | Con un 403, la página abría igualmente la conexión de eventos y programaba la medianoche. | Corregido (2026-11-12): la conexión y la medianoche solo se abren después de una primera carga con 200; con un 403 queda solo el mensaje. En D5, el plan, el contrato de la spec y T025. | Persona A | Cerrado (verificado en analyze-2). Escrito más estricto que lo decidido: ver N2 |
| A1 | T010 · plan (Reglas clave) | LOW | `BEGIN ISOLATION LEVEL …` y `resumenes(&mut tx, …)` no son la forma de hacerlo con SQLx 0.8.6. | Corregido (2026-11-12): T010 nombra `estado.db.begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY")` y `&mut *tx`. | Persona A | Cerrado (verificado en analyze-2) |
| A2 | US4-4 · research D6 · T019 | LOW | Cambiar `process.env.TZ` dentro de la prueba depende de cómo ejecute Vitest las pruebas; si no surte efecto, la prueba daría verde sin probar nada. | Corregido (2026-11-12): la prueba comprueba primero que la zona del proceso es de verdad otra (desfase de 240 min, no −120) y, si no, falla con un mensaje claro. En D6, el plan y T019. | Persona A | Cerrado (verificado en analyze-2) |
| I2 | FR-005 · T026 (antes T024) · plan | LOW | La tarea hablaba de «los 9 movimientos», pero el `grep` da 8 llamadas. | Corregido (2026-11-12): «9 movimientos en 8 llamadas», en T026 y en el plan. | Persona A | Cerrado (verificado en analyze-2) |
| C1 | Principio IV · quickstart común §8.9 y §8.10 | LOW | Los pasos 9 y 10 usaban las herramientas de desarrollo. | Corregido (2026-11-12): el paso 9 se hace parando el servidor (Ctrl+C en `cargo run`) y volviendo a arrancarlo; la parte con la red cortada en las herramientas de desarrollo queda opcional. El paso 10 se hace en un móvil en la misma red (el servidor escucha en `0.0.0.0:3000`), con las herramientas de desarrollo como alternativa. T028 hace las dos cosas. | Persona A | Cerrado (verificado en analyze-2; revisión de B en T032) |
| I3 | Research D7 · T011 · `+layout.svelte` | LOW | T011 solo ensanchaba el `<main>`, y en una pantalla grande el menú quedaba estrecho. | Corregido (2026-11-12): en `/tablero`, el `<nav>` y el `<main>` pierden `max-w-6xl` con una sola variable; en las demás pantallas el menú y el contenido siguen exactamente igual. En D7, el plan y T011; T032 lo revisa. | Persona A | Cerrado (verificado en analyze-2; revisión de B en T032) |
| I4 | decisiones-clarify · plan (tabla de compartidos) · T032 | LOW | Había dos tablas de ficheros compartidos que decían ser la lista para la Persona B. | Corregido (2026-11-12): queda solo la del plan; el clarify remite a ella. | Persona A | Cerrado (verificado en analyze-2) |

## Hallazgos de analyze-2

| ID | Historia o criterio | Gravedad | Evidencia | Decisión | Responsable | Estado |
|----|---------------------|----------|-----------|----------|-------------|--------|
| N1 | T030 | MEDIUM | Al renumerar las tareas por E1, la ruta de los scripts de spec-004 en T030 pasó a `specs/004-aceptar-presupuesto-enlace/evidencias/T031/scripts/`, que no existe: son los de `evidencias/T029/scripts/`. El texto «T029 de spec-004» sí se corrigió. Error introducido al aplicar las decisiones de analyze-1. | Corregido (2026-11-12): T030 vuelve a `specs/004-aceptar-presupuesto-enlace/evidencias/T029/scripts/`. | Persona A | Cerrado (verificado en analyze-3) |
| N2 | FR-005, caso límite «Se pierde la conexión» · research D5 · plan · contracts/api.md · T025 | MEDIUM | Lo decidido en E2 fue «con un 403 no se abre la conexión», pero los documentos dicen «solo tras una primera carga con 200». Si la primera carga falla por la red (el servidor se está reiniciando), no se abre la conexión y el tablero no se recupera solo. | Corregido (2026-11-12): la conexión de eventos y la medianoche se abren después de la primera carga en todos los casos salvo con un 403. Si la primera carga falló porque el servidor se estaba reiniciando, el primer `onopen` la repite y el tablero se recupera solo. En D5, el plan, el contrato de la spec y T025. | Persona A | Cerrado (verificado en analyze-3) |
| N3 | Research D5 · T024, T025 · contracts/api.md | LOW | D5 escribe `alError()` y dice que pide el tablero en cualquier error; T024 lo define como `alError(cerrada)`, y T025 y el contrato solo lo piden con la conexión cerrada. | Corregido (2026-11-12). Regla general, decidida por la autora: el tablero se pide entero cada vez que la conexión se abre (`onopen`): la primera vez y cada vez que se vuelve a abrir tras un error, tanto si el navegador reconecta solo como si la página la crea tras quedar cerrada; un error no lo pide. Única excepción, elegida por la autora al ver que con un 401 la conexión no vuelve a abrirse nunca: con la conexión cerrada (`CLOSED`), se pide el tablero **una vez** antes de crear la nueva, para que un 401 lleve a `/entrar` (E1). `alError(cerrada: boolean)` en D5, T021, T024, T025, el plan y el contrato de la spec. | Persona A | Cerrado (verificado en analyze-3) |
| N4 | quickstart.md de la spec | LOW | Solo nombra `medianoche.test.ts`, no tiene fila para el §9 reescrito ni dice que el paso 9 ya no necesita herramientas. | Corregido (2026-11-12): la guía de la spec remite también al §9, explica sus dos sesiones, añade tres filas del §9 (US4-1 a US4-3, SC-002 y SC-003), la nota del paso 9 sin herramientas y las tres pruebas de Vitest. | Persona A | Cerrado (verificado en analyze-3) |

## Resumen

- **Hallazgos de analyze-1**: 10 (0 CRITICAL, 0 HIGH, 3 MEDIUM, 7 LOW), todos corregidos el
  2026-11-12 y verificados en [analyze-2.md](analyze-2.md). G1, C1, I3 y la parte de R13 de E1
  quedan pendientes de la revisión de B en T032. No se ha tocado código.
- **Hallazgos de analyze-2**: 4 nuevos (2 MEDIUM: N1 y N2; 2 LOW: N3 y N4), corregidos el
  2026-11-12 y verificados en [analyze-3.md](analyze-3.md), que no encontró hallazgos nuevos.
- **Abiertos**: ninguno. Nada bloquea `/speckit-implement`; los cambios en documentos comunes (G1,
  C1, I3 y R13 de E1) esperan la revisión de la Persona B en T032.
- **Tocan documentos comunes** (los revisa la Persona B en T032): E1 (R13), G1 (§9), C1 (§8.9 y
  §8.10) e I3 (`+layout.svelte`, al implementar).
- **Renumeración**: las dos tareas nuevas de E1 (T021 y T024) desplazan las siguientes. La
  correspondencia es antigua → nueva: T021 → T022, T022 → T023, T023 → T025, T024 → T026,
  T025 → T027, T026 → T028, T027 → T029, T028 → T030, T029 → T031, T030 → T032.
  [analyze-1.md](analyze-1.md) conserva la numeración antigua.
- **Comprobaciones sin hallazgo** (detalle en [analyze-1.md](analyze-1.md)): el cambio de
  `resumenes()` (D3) no afecta a nadie más; las 8 llamadas que publican los 9 movimientos van después
  del `commit`, y ningún cambio de estado se salta el evento; el contrato y R13 comunes coinciden con
  la spec, el plan y las tareas.
