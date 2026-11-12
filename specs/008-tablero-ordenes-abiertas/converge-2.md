# Converge 2 · spec-008 · Tablero de órdenes abiertas

**Fecha**: 2026-11-12 · **Rama**: `spec-008` (último commit `02e9868`, más la marca de T030 sin
commit) · **Resultado**: `tasks_appended` (2 tareas, Phase 9: Convergence, T039–T040), sin
implementar · **Responsable**: Persona A · **Revisor**: Persona B

## Alcance

- **Código revisado**: `backend/src/rutas/tablero.rs` (sin transacción, tras V3),
  `backend/tests/spec008_tablero.rs` (pruebas 1–9, la 9 con `#[ignore]`),
  `frontend/src/routes/tablero/+page.svelte`, `frontend/src/lib/{medianoche,recarga,eventos}.ts` y
  los ficheros compartidos de la tabla del plan.
- **Documentos**: `spec.md` (FR-001 a FR-006, SC-001 a SC-004, US1 a US4 y los 7 casos límite,
  incluida la limitación aceptada de V1(b)), `plan.md`, `research.md` (D1–D9), `tasks.md`
  (T001–T038), `hallazgos.md`, [converge-1.md](converge-1.md) y las evidencias de T028, T029 y
  T030; la constitución 1.2.1.
- **Método**: cada requisito se contrastó con el código, las pruebas y las verificaciones del
  navegador (T028 y su complemento), el cronometraje (T029) y la regresión (T030, §0–§8 sin fallos
  de la aplicación). En el código de 008 no queda ningún `NOT_IMPLEMENTED`, `todo!`,
  `unimplemented!`, `TODO` ni `FIXME`.
- **Fuera de este informe** (ya tiene tarea o responsable): T032, la revisión de la Persona B; V2,
  asignado a la Persona B.

## Convergence Findings

| ID | Tipo | Gravedad | Origen | Evidencia | Trabajo pendiente |
|----|------|----------|--------|-----------|-------------------|
| F1 | partial | MEDIUM | Quickstart común §9 · T030 · US4-1 a US4-3, SC-002 con una orden nueva | T030 se marcó con el §9 pendiente: la regresión cubre §0–§8, pero el recorrido completo con un vehículo nuevo, desde la cita hasta la factura y con el tablero abierto, necesita dos días reales y a las dos personas ([verificación de T030](evidencias/T030/regresion/verificacion.md)). Sin él no queda constancia de una orden real que pase por todas las columnas. | Recorrer el §9 en sus dos días y guardar el informe. |
| F2 | partial | LOW | Plan, tabla «Ficheros compartidos» · T032 | La columna «Estado» sigue en «Pendiente» en `ordenes.rs`, `mod.rs`, `tipos.ts`, `demo.sql` y `+layout.svelte`, aunque están hechos y con commit. Faltan los cambios comunes de converge-1 (§8.9 del quickstart sin la parte opcional y el aviso de R14 en el research), que la Persona B tiene que revisar en T032, que trabaja con esa tabla. | Poner al día la tabla con lo hecho y el commit de cada fila. |

**Resumen**

- Requisitos y escenarios comprobados: FR-001 a FR-006, SC-001 a SC-004, los escenarios de US1 a US4
  y los 7 casos límite.
- Decisiones del plan comprobadas: D1 a D9 (D1 y D2, en su versión revisada por V3).
- Constitución: los cinco principios. Ningún incumplimiento.
- Hallazgos por tipo: 0 missing, 2 partial, 0 contradicts, 0 unrequested.
- Hallazgos por gravedad: 0 CRITICAL, 0 HIGH, 1 MEDIUM, 1 LOW.

El código cumple la spec; lo que falta es una verificación manual (F1) y la documentación para la
revisión (F2).

## Tareas añadidas

T039–T040, en la «Phase 9: Convergence» de [tasks.md](tasks.md). No se han implementado.
