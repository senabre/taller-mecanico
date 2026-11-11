# Converge 2 · spec-004 · Aceptar o rechazar el presupuesto desde un enlace

**Fecha**: 2026-11-11 · **Rama**: `spec-004` (último commit `6518624`) · **Resultado**: `tasks_appended`
(1 tarea de documentación, Phase 9: Convergence, T034). En el código la spec ha convergido. ·
**Responsable**: Persona A · **Revisor**: Persona B

## Alcance

- **Segunda pasada**, tras T032 y T033 (commits `c839ef0` y `6518624`). Se repitió el contraste de
  converge-1 con los mismos criterios: spec (FR-001 a FR-009, SC-001 a SC-004, US1 a US4 y casos
  límite), plan (tabla de ficheros compartidos), `tasks.md`, `hallazgos.md` y la constitución 1.2.1.
- **Código**: el árbol de trabajo coincide con `HEAD` y no hay ningún cambio sin commit. El código de
  la aplicación es el mismo que pasó las comprobaciones finales de converge-1: `cargo test` con 147 de
  147, clippy, `npm run check`, `npm test` y `npm run build`, todo en verde. Como no ha cambiado, no
  se repiten.
- **No se ha tocado** código, `spec.md` ni `plan.md`. La única escritura es la Phase 9 al final de
  `tasks.md`.

## Convergence Findings

| ID | Tipo | Gravedad | Origen | Evidencia | Trabajo pendiente |
|----|------|----------|--------|-----------|-------------------|
| F1 | partial | LOW | plan (tabla «Ficheros compartidos») · T033 · T031 | La fila de `Presupuesto.svelte` en `plan.md` aún dice ««Válido hasta» solo con el presupuesto vigente (T032, V1): pendiente de commit». Era cierto al escribirlo en T033, pero T032 ya está en `c839ef0`. Es el único dato desfasado que se encontró en la tabla que la Persona B usará en T031. | T034: poner el commit `c839ef0` en esa fila. |

## Comprobado sin hallazgos

- **T032 en el código**: `Presupuesto.svelte` tiene una sola condición, `vigente`, que usan el enlace
  (línea 68) y «Válido hasta» (línea 164). Coincide con la regla de Q5 y con lo comprobado en el
  navegador ([evidencias/T032/](evidencias/T032/informe.md)).
- **Nada provisional**: la búsqueda de `NOT_IMPLEMENTED`, `501`, «provisional», `todo!`,
  `unimplemented!`, `TODO` y `FIXME` en `backend/` y `frontend/src` no encuentra nada.
- **Tareas**: 32 de 34 hechas. Quedan T031, la revisión de la Persona B, que no le toca a la Persona A,
  y T034, la de este informe.
- **Hallazgos de la Persona A**: ninguno abierto; V1 está cerrado en T032. Siguen abiertos solo los de
  la Persona B: G1 (spec-003), I8 y V2 (spec-003).
- **Requisitos y constitución**: sin cambios respecto a converge-1, porque el código no ha cambiado
  desde entonces.

## Siguiente paso

Hacer T034, una línea en `plan.md`. Con eso, la spec queda convergida: todas las tareas hechas salvo
T031, y sin hallazgos abiertos de la Persona A. No hace falta un tercer converge, salvo que T031 traiga
cambios.
