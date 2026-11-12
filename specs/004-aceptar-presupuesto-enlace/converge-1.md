# Converge 1 · spec-004 · Aceptar o rechazar el presupuesto desde un enlace

**Fecha**: 2026-11-11 · **Rama**: `spec-004` (último commit `ee9f8d6`, fusión de `origin/main`) ·
**Resultado**: `tasks_appended` (2 tareas, Phase 8: Convergence, T032–T033), las dos ya implementadas en
esta misma sesión, como pidió la Persona A · **Responsable**: Persona A · **Revisor**: Persona B

## Alcance

- **Código revisado**: `backend/src/rutas/publico_presupuesto.rs`, `backend/tests/spec004_respuesta.rs`,
  `frontend/src/routes/presupuesto/[token]/+page.svelte`, y los ficheros compartidos de la tabla del
  plan: `presupuestos.rs`, `Presupuesto.svelte`, `rutas/mod.rs`, `tipos.ts`, `demo.sql` y `estados.rs`.
  También `facturas.rs` (spec-007), tocado el 05/11 por clippy, y la ficha
  `frontend/src/routes/ordenes/[id]/+page.svelte` tras la fusión de `main`.
- **Documentos**: `spec.md` (FR-001 a FR-009, SC-001 a SC-004, US1 a US4 y casos límite), `plan.md`
  (tabla de ficheros compartidos), `research.md` (D1–D8), `tasks.md` (T001–T031) y `hallazgos.md`;
  la constitución 1.2.1.
- **Método**: cada requisito se contrastó con el código, con las pruebas y con las verificaciones del
  navegador hechas con el servidor real (T015, T016, T027 y T029). Una tarea marcada no basta para
  dar nada por cumplido. Se buscaron en `backend/` y `frontend/src` `NOT_IMPLEMENTED`, `501`,
  «provisional», `todo!`, `unimplemented!`, `TODO`, `FIXME` y `XXX`.
- **Fuera de este informe** (ya tiene tarea o responsable): T031, la revisión de la Persona B; V2, G1 e
  I8, asignados a la Persona B; §8 y §9 del quickstart, a la espera de spec-008.

## Convergence Findings

| ID | Tipo | Gravedad | Origen | Evidencia | Trabajo pendiente |
|----|------|----------|--------|-----------|-------------------|
| F1 | partial | LOW | Q5, FR-008, US1-5 · hallazgo V1 | `Presupuesto.svelte` muestra «Válido hasta» en cualquier presupuesto que no esté caducado, también en uno aceptado o rechazado ([captura 16 de T027](evidencias/T027/16-p7-orden7-dos-respuestas.png)). Q5 limita lo que ofrece la orden al presupuesto emitido y vigente, y la fecha sugiere que aún se puede responder. | T032: «Válido hasta» con la misma condición que el enlace. |
| F2 | partial | LOW | plan (tabla «Ficheros compartidos») · T031 | La columna «Estado» decía «Pendiente» en `presupuestos.rs`, `Presupuesto.svelte`, `mod.rs` y `demo.sql`, y «Parcial» en `tipos.ts`, aunque todos están hechos y con commit. Faltaba `facturas.rs` (spec-007), tocado el 05/11. La Persona B usará esa tabla en T031. | T033: poner al día la columna, con la tarea y el commit de cada fila, y añadir `facturas.rs`. |

### Resolución en esta sesión

- **T032 (F1)**: una sola condición, `vigente`, para el enlace y para «Válido hasta». Rojo en Chrome
  con 10 fallos, todos de V1, y verde con 26 correctas, a 1280 y 390 px ([informe](evidencias/T032/informe.md)).
  V1 queda cerrado en `hallazgos.md`.
- **T033 (F2)**: tabla del plan al día. Es la única modificación de `plan.md`, hecha como tarea de
  implementación y no por el converge.

### Comprobado sin hallazgos

- **Nada provisional en el código**: la búsqueda no encuentra ningún 501, ninguna respuesta provisional
  ni ningún `todo!` en `backend/` ni en `frontend/src`. El `caducado: false` provisional y el 501 del
  `GET` se quitaron en T012.
- **La ficha tras fusionar `main`**: la fusión solo añade `<Factura>` después de `<Pedidos>` en
  `ordenes/[id]/+page.svelte`, sin tocar la sección Presupuesto. En la orden demo 9, facturada, las seis
  secciones salen en orden y sin errores de JavaScript, en los dos anchos (T032).
- **FR-001**: el correo con el enlace sale al emitir (spec-003; T027 y T029, paso 4.1).
- **FR-002**: la consulta pública no une con `cliente`. Lo comprueban la prueba 1 (T005) y el navegador
  en todos los estados (T015, T027). Un token inexistente da el mismo 404 en `GET` y `POST` (T006).
- **FR-003 y US1-4**: una sola respuesta, gracias al `UPDATE` de R9 con `estado = 'emitido'`. Las
  mutaciones de T010 y T020 confirman que las pruebas la vigilan. La confirmación está dentro de la
  página, sin `confirm()`.
- **FR-004, SC-002 y US2-3**: `respondido_en` en `PresupuestoResumen` (T014), y en la ficha todas las
  respuestas, con su tipo, fecha y hora en horario de Madrid (T016, T019 y T027, paso 4.7).
- **FR-005, FR-009, SC-003 y US3**: `caduca_el >= $hoy` en el `UPDATE`, y los motivos en el orden
  correcto, confirmado por mutación (T025). Para inicial y ampliación (T024), y con el presupuesto
  caducando con la página abierta (T015).
- **FR-006**: aceptar el inicial pasa la orden de `en_diagnostico` a `en_reparacion`, y la ampliación,
  de `pendiente_conformidad` a `en_reparacion`. El evento se publica después del `commit` (T008, T021).
- **FR-007, SC-004 y US4**: la prueba 11 compara el texto exacto, confirmado por mutación (T023), y en el
  navegador no aparece «Imputar» (T027, paso 4.4).
- **FR-008, US1-5 y el caso límite «El correo no llega»**: enlace y «Copiar enlace» solo mientras el
  presupuesto está vigente (T016). La API sigue enviando `enlace` (T011).
- **SC-001**: 13 s, cronometrados por una persona (T028).
- **Constitución**:
  - **I**: una sola definición de `caducado()` y de `vigente`.
  - **II**: textos en español, importes en € y fechas en dd/mm/aaaa.
  - **III**: nada fuera de la spec. «Copiar enlace» no da ningún mensaje sin HTTPS: se propuso y no
    se construyó.
  - **IV**: cada requisito se puede comprobar en la app.
  - **V**: sin datos del cliente en la página pública, y solo tokens ficticios documentados.

### Observaciones (no generan tarea)

- **V2** (spec-003, Persona B): el cuadro del diagnóstico conserva el texto rechazado hasta recargar.
- **T031**: su enunciado no nombra `facturas.rs`. Ya está en la tabla del plan (T033), que es lo que
  revisa la Persona B.

## Comprobaciones finales (2026-11-11)

| Comprobación | Resultado |
|---|---|
| `cargo test --no-fail-fast` | 147 pasan, 0 fallan (`spec004_respuesta` 16 de 16) |
| `cargo clippy --all-targets -- -D warnings` | Sin errores ni avisos |
| `npm run check` | 0 errores, 0 avisos |
| `npm test` | 6 de 6 |
| `npm run build` | Correcto |

## Siguiente paso

Queda T031, la revisión de la Persona B, con la tabla del plan al día. Un segundo converge solo haría
falta si T031 trae cambios.
