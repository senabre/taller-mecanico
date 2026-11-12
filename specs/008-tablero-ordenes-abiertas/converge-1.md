# Converge 1 · spec-008 · Tablero de órdenes abiertas

**Fecha**: 2026-11-12 · **Rama**: `spec-008` (último commit `c01fb8e`, más las marcas de T011, T012,
T016, T018, T025, T028 y T029 sin commit) · **Resultado**: `tasks_appended` (6 tareas, Phase 8:
Convergence, T033–T038), implementadas en esta misma sesión, como pidió la Persona A ·
**Responsable**: Persona A · **Revisor**: Persona B

## Alcance

- **Código revisado**: `backend/src/rutas/tablero.rs`, `backend/src/rutas/ordenes.rs` (`resumenes`),
  `backend/tests/spec008_tablero.rs`, `frontend/src/routes/tablero/+page.svelte`,
  `frontend/src/lib/{medianoche,recarga,eventos}.ts` y los ficheros compartidos de la tabla del plan.
- **Documentos**: `spec.md` (FR-001 a FR-006, SC-001 a SC-004, US1 a US4 y casos límite),
  `plan.md`, `research.md` (D1–D9), `data-model.md`, `tasks.md` (T001–T032), `hallazgos.md` y
  las evidencias de T028, T029 y T030; la constitución 1.2.1.
- **Método**: cada requisito se contrastó con el código, las pruebas y las verificaciones en el
  navegador (T028 y su complemento) y en la regresión (T030). Se buscaron en el código de 008
  `NOT_IMPLEMENTED`, `501`, «provisional», `todo!`, `unimplemented!`, `TODO`, `FIXME` y `XXX`: no
  queda ninguno.
- **Fuera de este informe** (ya tiene tarea o responsable): T030, la regresión, que se repite entera
  después de estas tareas; T032, la revisión de la Persona B; V2, asignado a la Persona B.

## Convergence Findings

| ID | Tipo | Gravedad | Origen | Evidencia | Trabajo pendiente |
|----|------|----------|--------|-----------|-------------------|
| F1 | contradicts | HIGH | SC-003, FR-001 · research D1 · hallazgo V3 | `tablero.rs` abre `begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY")`. Si el navegador corta la petición a mitad, la conexión puede volver al *pool* dentro de la transacción, y las peticiones siguientes de **toda la aplicación** fallan («cannot execute UPDATE in a read-only transaction»). En la regresión T030, el §1 se quedó en «§1.9 paco»; `estres.sh` da 18 errores del servidor con el código actual ([hallazgo](evidencias/T030/hallazgo-conexiones.md)). | Quitar la transacción: una sola consulta de órdenes; la carga, contada en Rust a partir de esa misma lista; a la base, solo los mecánicos. Prueba de cancelación en rojo primero. Documentos y `estres.sh` adaptado. |
| F2 | partial | MEDIUM | US4-5, caso límite «Se pierde la conexión» · quickstart común §8.9 · hallazgo V1 | La parte opcional del §8.9 (red cortada con las herramientas de desarrollo) no corta un `EventSource` ya abierto, así que no prueba lo que dice (T028, fila 12). | Quitarla del §8.9 y de la guía de la spec; dejar «parar y arrancar el servidor» y terminar `3333DDD` con el tablero conectado. |
| F3 | partial | LOW | Research común (transacciones) · hallazgo V3, «Para la Persona B» | El research común no advierte de que una transacción cancelada a mitad puede dejar la conexión dentro de la transacción. Las demás transacciones (`POST` de citas, órdenes, presupuestos, imputaciones, facturas, sesión) podrían sufrirlo, aunque es mucho menos probable. | Anotarlo en el research común, para la Persona B, sin cambiar sus ficheros. |

**Resumen**

- Requisitos y escenarios comprobados: FR-001 a FR-006, SC-001 a SC-004, los escenarios de US1 a US4
  y los 6 casos límite.
- Decisiones del plan comprobadas: D1 a D9; la de D1 es la que falla (F1).
- Constitución: los cinco principios. Ningún incumplimiento.
- Hallazgos por tipo: 0 missing, 2 partial, 1 contradicts, 0 unrequested.
- Hallazgos por gravedad: 0 CRITICAL, 1 HIGH, 1 MEDIUM, 1 LOW.

**No cubierto por estas tareas**: la segunda parte de V1, una caída «silenciosa» de la red en la
que la conexión de eventos no se cierra y el tablero no lo nota hasta que el sistema operativo la da
por perdida. Queda abierta en [hallazgos.md](hallazgos.md) para que la autora decida si se acepta o
se cubre.

## Tareas añadidas

T033–T038, en la «Phase 8: Convergence» de [tasks.md](tasks.md). El resultado de cada una está en
su nota.

## Comprobaciones finales (2026-11-12)

`cargo test --no-fail-fast`: **156 pasan y 0 fallan** (las 155 anteriores más la prueba 9);
`cargo clippy --all-targets -- -D warnings`: sin avisos; `npm run check`: 0 errores y 0 avisos;
`npm test`: 21 de 21; `npm run build`: correcto. Salida en
[evidencias/T033-T038-comprobaciones.txt](evidencias/T033-T038-comprobaciones.txt).

Queda: repetir la regresión T030 entera y la revisión de la Persona B (T032). V1(b), la caída
silenciosa de la red, se aceptó después como limitación documentada (casos límite de la spec). La
prueba 9 lleva `#[ignore]` desde entonces (tarda unos 100 s) y se lanza aparte con
`cargo test --test spec008_tablero -- --ignored`.
