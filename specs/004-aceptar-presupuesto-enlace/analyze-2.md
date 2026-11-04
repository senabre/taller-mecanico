# Analyze 2 · spec-004 · Aceptar o rechazar el presupuesto desde un enlace

**Fecha**: 2026-11-04 · **Rama**: `spec-004` (commit `486bd98`, con las correcciones sin confirmar) ·
**Artefactos**: los de [analyze-1.md](analyze-1.md), con las decisiones de
[hallazgos.md](hallazgos.md) ya aplicadas · **Constitución**: 1.2.1.

Análisis de solo lectura: no se ha corregido nada ni se ha tocado `hallazgos.md`. Los cambios afectan a
10 documentos y a ningún fichero de `backend/` ni de `frontend/`; `tasks.md` y los analyze de la base
común siguen igual.

**Alcance**: se ha comprobado que los documentos dicen lo decidido. Las mutaciones de T010, T020 y T023
siguen sin ejecutarse, porque el código aún no existe: se verán al implementar.

## Verificación de analyze-1

| ID | Resultado | Comprobación |
|----|-----------|--------------|
| E1 | Verificado | T023 y la prueba 11 del plan comparan el texto exacto del estado; la mutación nombra la condición de `imputar` y avisa de que `terminar` tiene otra igual. La fase 5, el plan y el data-model nombran las dos barreras |
| E2 | Verificado | T020 lleva su mutación después de T021, con `cmp`. T010, T020, la prueba 4 y D2 piden el texto exacto o las dos variantes |
| E3 | Verificado | Nuevo paso 6 del §4, antes de la ampliación rechazada; captura en T027. Correspondencia de 004, tareas y D7 renumeradas: quedan 9 pasos y ninguna referencia antigua en los documentos vigentes |
| G1 | Asignado | A la Persona B (spec-003). T027 sigue escribiendo el diagnóstico y anotándolo; no hay tarea en 004 |
| I1 | **Sigue abierto** | El plan y T014 ya dicen que se avisa a B y que hay commit propio. Falta la conversación, y el plan maestro sigue diciendo «A no modifica `presupuestos.rs`» (decidido no cambiarlo). Ver N1 y N2 |
| I2 | Verificado | `pub(crate)` en T012, T014, tabla del plan, D1 y data-model, sin copia de la regla |
| I3 | Verificado | El plan ya no dice que spec-002 esté en `main`, y fija 001 → 002 → 004. Contrastado: ni `origin/spec-001` ni `origin/spec-002` están en `origin/main`, y 001 va dentro de 002 |
| E4 | Verificado | Caso añadido en T024 y en la prueba 7 |
| A1 | Verificado | Sin `detalle`; ninguna referencia a `detalle.campo` en los documentos vigentes |
| A2 | Verificado | T028 se hace en el recorrido de T027, o recargando la base |
| A3 | Verificado | El §4.2 abre la segunda pestaña al principio. Ver A5 |
| A4 | Verificado | T015, T026 y el contrato de 004 |
| I4 | Verificado | Sin «vuelve a» en ningún documento vigente; en T031 y en la tabla del plan |
| I5 | Verificado | T015 |
| I6 | Verificado | FR-009, SC-003 y la fila de la correspondencia |
| I7 | Verificado | «Elige aceptar» solo queda en analyze-1 y en `hallazgos.md`, que son registro |
| I8 | Asignado | A la Persona B |
| G2 | Verificado | `GET /api/piezas` vuelve a la tabla de spec-003, y `PresupuestoResumen` va después |
| G3 | Registrado | Anotado en «Dependencies» |
| O1 | Verificado | T004 |

Resumen: 16 verificados, 2 asignados, 1 registrado y 1 abierto.

## Specification Analysis Report

Hallazgos nuevos. Los tres vienen de las correcciones y son LOW.

| ID | Category | Severity | Location(s) | Summary | Recommendation |
|----|----------|----------|-------------|---------|----------------|
| N1 | Shared files | LOW | tasks.md:99 (T016) · plan.md:110–112 · 000/plan.md («acordar antes de tocar») | El aviso previo a B solo está escrito para `presupuestos.rs` (T014), pero T016 toca `Presupuesto.svelte`, que también es suyo y se revisa al final. El plan maestro reparte esa sección entre 003 y 004, así que no es una contradicción, pero sí un hueco. | Incluirlo en la conversación de I1 y, si B lo pide, repetir en T016 lo del aviso. |
| N2 | Ordering | LOW | tasks.md:97 (T014), :200 | Para ver T008 en rojo, la parte (b) de T014 va después de T013. Así, el commit que B debe revisar «pronto» no existe hasta tener el `POST` hecho, aunque va delante del de T012 y T013. Sigue siendo mucho antes de T031. | Justificar, o enseñar a B solo la parte (a) antes si quiere verlo cuanto antes. |
| A5 | Ambiguity | LOW | quickstart común §4.2 · T028 | Con la segunda pestaña al principio del paso, no queda claro si abrirla cuenta en el tiempo de SC-001. Afecta a unos segundos. | Decir que el cronómetro empieza en la primera pestaña, después de abrir la segunda. |

### Coverage Summary Table

Sin cambios respecto a [analyze-1](analyze-1.md), salvo estos cuatro:

| Requisito | ¿Tiene tarea? | Tareas | Notas |
|-----------|---------------|--------|-------|
| FR-003 | Sí | T007, T009, T010, T013, T015, T017, T020–T022 | Con mutación en las dos variantes |
| FR-005 | Sí | T014, T024–T026 | Incluye un respondido con la caducidad pasada |
| FR-007, SC-004 | Sí | T023, T027 | La mutación ya puede fallar, por el texto |
| FR-008 | Sí | T011, T016, T027 | El §4.6 cubre el caso caducado |

Los 11 escenarios y los 4 casos límite están cubiertos, también US1-5 entera.

### Constitution Alignment Issues

Ninguno.

### Unmapped Tasks

Las mismas de analyze-1: T003 y T004 son infraestructura; T029 y T031, coherencia entre documentos.

### Comprobado sin hallazgos

- **`tasks.md`**: siguen 31 tareas, todas con «Cubre», con los mismos ID. T014 ya no lleva `[P]`, y
  «Dependencies» explica sus dos pasos.
- **T031**: incluye todos los hallazgos marcados «revisión de B en T031» (E3, I2, A3, I4, I7 y G2).
- **Paso 6 del §4**: la ficha ya enseña «Caducado el …», y T016 solo pone el enlace con un presupuesto
  `emitido` y sin `caducado`.
- **Prueba 7**: el caso nuevo pasa ya con T013, así que no aporta rojo; protege el orden de T025.

### Metrics

- Requisitos: 13 (9 FR y 4 SC) · Tareas: 31
- Cobertura: 12 de 13 con tarea en 004 (92 %); FR-001 lo cubre spec-003
- Hallazgos nuevos: 3 (0 CRITICAL, 0 HIGH, 0 MEDIUM, 3 LOW) · Abiertos de analyze-1: 1 (I1)
- Ambigüedades: 1 · Duplicaciones: 0 · Críticos: 0

## Next Actions

Se puede pasar a `/speckit-implement`. Antes de empezar:

1. Hablar con la Persona B (I1), incluyendo N1 y N2.
2. Marcar en `hallazgos.md` lo verificado aquí y decidir N1, N2 y A5.
