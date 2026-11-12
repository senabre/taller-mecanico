# Analyze 1 · spec-004 · Aceptar o rechazar el presupuesto desde un enlace

**Fecha**: 2026-11-04 · **Rama**: `spec-004` (commit `486bd98`) · **Artefactos**: [spec.md](spec.md),
[plan.md](plan.md), [research.md](research.md) (D1–D8), [data-model.md](data-model.md),
[contracts/api.md](contracts/api.md), [quickstart.md](quickstart.md),
[decisiones-clarify.md](decisiones-clarify.md) y [tasks.md](tasks.md) (T001–T031) · **Contexto**:
[000-base-comun](../000-base-comun/) (plan, research R9, data-model, contrato y quickstart §0 y §4) y
el código de spec-003 y spec-005 · **Constitución**: 1.2.1.

Análisis de solo lectura: no se ha corregido nada. Las decisiones se registran en
[hallazgos.md](hallazgos.md).

**Sobre «lo que está en main»**: la referencia es `origin/main` (`09c887f`), que incluye spec-003,
spec-005 y spec-006 y es antecesor de `spec-004`. La rama `main` local está 6 commits por detrás y no
tiene ese código. Entre `origin/main` y `spec-004`, `presupuestos.rs` y `Presupuesto.svelte` son
idénticos; `estados.rs` solo cambia por Q2 e `imputaciones.rs` por el refactor `51adc54` de spec-002.

**Alcance de las comprobaciones**: E1 y E2 salen de leer el código; las mutaciones no se han
ejecutado, porque `publico_presupuesto.rs` y `spec004_respuesta.rs` aún no existen.

## Specification Analysis Report

| ID | Category | Severity | Location(s) | Summary | Recommendation |
|----|----------|----------|-------------|---------|----------------|
| E1 | Coverage | HIGH | tasks.md:139 (T023) · plan.md (prueba 11) · data-model.md (regla FR-007) · `backend/src/rutas/imputaciones.rs:110`, `:116–126` | **La mutación de T023 no puede salir como está escrita.** `imputar` tiene dos barreras seguidas: el estado (`!= EnReparacion`) y «La orden no tiene un presupuesto aceptado». Si se quita solo la primera, la segunda sigue dando 409, y una prueba que compare el 409 y que no se crean imputaciones seguirá en verde. El plan y el data-model de 004 solo nombran la primera barrera. | Que la prueba 11 compare el texto exacto «La orden está en diagnóstico; solo se puede imputar en órdenes en reparación»: así, al quitar la primera barrera, falla. Nombrar las dos barreras en el plan y el data-model. |
| E2 | Coverage | MEDIUM | tasks.md:88 (T010), :117 (T020) · plan.md (prueba 4) · decisiones-clarify Q4 | **La mutación solo cubre aceptar y aceptar.** El plan pide la mutación para la prueba 4, con sus dos variantes, pero T010 la hace antes de T021, cuando `rechazar` aún da 501, y T020 no la repite. Sin `estado = 'emitido'`, aceptar y aceptar sigue dando un 200 y un 409, porque `cambiar_estado` rechaza el segundo con otro texto: solo el texto exacto de T010 lo detecta. Aceptar y rechazar es el caso grave: el rechazo sobrescribiría una aceptación y quedaría `rechazado` con la orden `en_reparacion`. | Repetir la mutación en T020, después de T021, con `cmp`. Escribir en T020 y en el plan que se compara el texto exacto del 409. |
| E3 | Coverage | MEDIUM | spec US1-5, FR-008 · quickstart común §4, pasos 6 y 7 (:130, :134) · quickstart.md (fila US1-5) · T016, T027 | **Nadie comprueba que el enlace desaparece de la orden al caducar.** El §4.3 lo mira tras responder, pero el §4.7 solo abre la página pública. No se puede añadir sin más al paso 7: el §4.6 crea otra ampliación en la orden 7, y la ficha solo enseña el presupuesto más reciente, así que la caducada ya no se vería. Tampoco hay prueba automática de la pantalla. | Comprobarlo en la orden 7 **antes** del §4.6 («Caducado el …», sin enlace ni «Copiar enlace»): adelantar el paso 7 o añadir el control al principio del 6. Actualizar la correspondencia y T027. |
| G1 | Dependency (003) | MEDIUM | quickstart común §4.5 · 002/hallazgos.md (I5, apartado b) · specs/003-diagnostico-presupuesto/ · T027 | **I5(b) no tiene quien lo resuelva.** Está asignado a spec-003, pero spec-003 ya está en `origin/main` y no hay rastro en sus documentos (no tiene `hallazgos.md`, ni nombra I5 ni `1414NNN`). `crear_presupuesto` da 409 con la orden `recibida`, así que el §4.5 falla si se sigue al pie de la letra. Ahora solo lo salva una nota de T027. No se propone tarea en 004. | Que la Persona B lo recoja en spec-003 (o se decida quién) y añada el paso de diagnóstico al §4.5. Confirmar si la nota de T027 se queda mientras tanto. |
| I1 | Inconsistency | MEDIUM | 000/plan.md:168–169 («A no modifica `presupuestos.rs`», «acordar antes de tocar») · plan.md (tabla de compartidos) · T014, T016, T031 | **El plan maestro prohíbe lo que T014 hace.** Dice que A no modifica `presupuestos.rs`, y pide acordar **antes** de tocar. spec-004 lo cambia por Q1 y `Presupuesto.svelte` por Q1 y Q5, con revisión de B solo al final (T031). Además, el plan maestro no incluye ninguno de los dos entre los compartidos, salvo `ordenes/[id]` en general. | Acordarlo con la Persona B antes de T014 y T016, y anotar la excepción en el plan maestro. |
| I2 | Inconsistency | MEDIUM | tasks.md:93 (T012) · data-model.md («el mismo criterio que `caducado()`») · `presupuestos.rs:133` · plan.md (tabla de compartidos) | `caducado()` es privada de `presupuestos.rs`. T012 pide «la misma regla», pero no dice si se hace `pub(crate)` (otro cambio en un fichero de B, que no está en la tabla ni en T014) o se copia (dos versiones de una regla que la spec quiere única). | Decidir. Propuesta: `pub(crate)` dentro del commit de T014, y anotarlo en la tabla del plan y en T012. |
| I3 | Inconsistency | MEDIUM | plan.md:170 · `origin/main` · 002/tasks.md (T031) | El plan dice que spec-002 ya está en `main`, y no es cierto: `origin/spec-002` no está fusionada (`ordenes.rs` no existe en `origin/main`) y `spec-004` la lleva dentro. Fusionar 004 arrastraría 002, con su T031 aún pendiente. | Corregir la frase y fijar el orden: revisión T031 de 002 y fusión de 002 antes que 004. |
| E4 | Coverage | LOW | T024, T025 · R9 (orden de los motivos) | Ninguna prueba cubre un presupuesto **respondido** con `caduca_el` ya pasado. Debe seguir mostrando la decisión y dar 409 «ya fue respondido», no «caducó». Si se invirtiera el orden de los motivos, nada fallaría. | Añadir ese caso a la prueba 7 (T024). |
| A1 | Ambiguity | LOW | T007 · contrato común (spec-004) · `error.rs:84–88` | T007 exige `detalle.campo = "decision"`, que no está ni en el contrato ni en el plan. Y «ausente» es ambiguo: con `{}` da 422 si el campo es `Option<String>`, pero sin cuerpo o con `{"decision": 5}` el extractor responde 400 «Los datos enviados no son válidos». | Fijar si `detalle` existe y limitar la prueba 9 a `{}`, `""` y `"otra"`, dejando el 400 como está. |
| A2 | Ambiguity | LOW | T027, T028 | T028 no dice qué presupuesto se cronometra, y cada uno solo se responde una vez: si T027 ya recorrió el paso 2, no queda ninguno. | Hacerlo en el mismo recorrido, o indicar que se recarga la base o se usa otro presupuesto. |
| A3 | Ambiguity | LOW | quickstart común §4.2 (:119) | «Con el enlace abierto antes en otra pestaña» va al final del paso, cuando ya se ha aceptado: quien siga el orden no la tendrá abierta. | Pedir la segunda pestaña al principio del paso. |
| A4 | Ambiguity | LOW | T015, T026 · contracts/api.md | Un 409 de caducidad se muestra y además vuelve a pedir el presupuesto, que enseña el mismo aviso: saldría dos veces. Y T015 no usa «Ha ocurrido un error inesperado», que el contrato de 004 sí lista. | Indicar que se ve una sola vez y cuándo se usa cada texto de error. |
| I4 | Inconsistency | LOW | 000/research.md:10 | Sigue diciendo que, con un inicial rechazado, la orden «vuelve a «en diagnóstico»». Es lo que Q2 corrigió en la spec y en el §4.5. | Cambiar por «sigue», o marcarlo como superado por Q2. Lo revisa B. |
| I5 | Inconsistency | LOW | T015 · research D1 · contracts/api.md | T015 no nombra `taller.nombre` ni `tipo`, aunque el contrato de 004 pone el taller en la página y D1 los devuelve. Un campo enviado y no usado no tendría uso descrito. | Añadir a T015 el nombre del taller y si es inicial o ampliación. |
| I6 | Inconsistency | LOW | spec.md:163 (FR-009), :180 (SC-003) | Tras Q3, siguen diciendo solo «acepte» y «aceptarse», aunque la regla vale también para rechazar. Los escenarios y FR-005 sí están al día. | Ajustar la redacción. |
| I7 | Inconsistency | LOW | contrato común (422) · research D5 | «Elige aceptar o rechazar» tutea en una ruta pública que trata de usted. No se ve en un uso normal, porque la página siempre envía una decisión válida. | Justificar o cambiar por «Elija…». |
| I8 | Inconsistency | LOW | research D5 · plan (principio II) · `Presupuesto.svelte:98`, `Imputaciones.svelte:40` | D5 descarta `confirm()` porque usa el idioma del navegador, pero spec-003 y spec-005 lo usan para el personal. No afecta a 004. | Registrar para la Persona B. |
| G2 | Shared files | LOW | 000/contracts/api.md:127–130 | El párrafo de `PresupuestoResumen` parte la tabla de spec-003: la fila de `GET /api/piezas` queda suelta y no se ve como tabla. Viene de `dd77530` (spec-003), no del clarify. | Mover la fila o el párrafo. Lo revisa B. |
| G3 | Shared files | LOW | `origin/spec-007` · T003, T002 | spec-007, sin fusionar, también añade líneas a `rutas/mod.rs` y a `tipos.ts`: conflicto probable y fácil en el segundo en fusionarse. En el quickstart solo toca el §7, sin chocar con el §0 ni el §4. | Tenerlo en cuenta al fusionar. |
| O1 | Ordering | LOW | T004, T005 · `tests/comun/mod.rs:95` | `crear_orden` deja `diagnostico` a NULL, y T004 no dice quién lo escribe, pero la prueba 1 espera verlo. | Que la utilidad de T004 lo escriba. |

### Coverage Summary Table

| Requisito | ¿Tiene tarea? | Tareas | Notas |
|-----------|---------------|--------|-------|
| FR-001 | No, a propósito | — | Lo cumple `emitir` de spec-003; se recorre en el §4.1 |
| FR-002 | Sí | T003, T005, T006, T012, T015, T027 | |
| FR-003 | Sí | T007, T009, T010, T013, T015, T017, T020–T022 | Ver E2 |
| FR-004 | Sí | T002, T008, T013, T014, T016, T017, T019, T021 | |
| FR-005 | Sí | T024–T026 | Ver E4 |
| FR-006 | Sí | T008, T013, T018, T021 | |
| FR-007 | Sí | T023 | Ver E1 |
| FR-008 | Sí | T011, T016 | Ver E3 |
| FR-009 | Sí | T018, T021, T024 | |
| SC-001 | Sí | T015, T028 | Ver A2 |
| SC-002 | Sí | T008, T014, T016, T019, T027 | |
| SC-003 | Sí | T001, T024, T025, T027 | |
| SC-004 | Sí | T023, T027 | Ver E1 |

Los 11 escenarios y los 4 casos límite tienen tarea. US1-5 queda a medias (E3). US3-2 solo se comprueba
con la prueba 7, como dice el quickstart de 004.

### Constitution Alignment Issues

Ninguno. Los cuatro SC se comprueban en la aplicación (principio IV); la consulta pública no une con
`cliente` (V) y el token demo está documentado en los sitios permitidos. T004 debe seguir sin usar
valores demo.

### Unmapped Tasks

T003 y T004 son infraestructura, y ya lo declaran en su «Cubre». T029 y T031 cubren coherencia entre
documentos, no un requisito concreto.

### Comprobado sin hallazgos

- **R9, plan, T013 y contrato**: el `UPDATE`, el orden de los motivos y los textos del 404, los 409 y
  el 422 coinciden. Ya no queda `respondido_en IS NULL` en ningún documento vigente.
- **Máquina de estados**: `TRANSICIONES` tiene 8 y coincide con el data-model común; sin
  autotransición en ningún documento vigente.
- **Quickstart §0**: cita los dos tokens demo, y no hay ningún dato real.
- **`caducado()` y `caduca_el >= $hoy`**: son exactamente opuestos, también en `crear_presupuesto`.
- **Bloqueos**: responder bloquea el presupuesto y después la orden, igual que `emitir`; ningún camino
  lo hace al revés, así que no se ve riesgo de interbloqueo.
- **Orden de 1414NNN**: es de `luis`, como dice T027. La orden 5 es de `marta` y la 7, de `luis`.
- **FR-001a de spec-003**: tras un rechazo el diagnóstico sigue fijo y se puede crear otro
  presupuesto: coherente con US2-1.
- **spec-005**: su spec y su plan dicen lo mismo sobre la ampliación rechazada.
- **Infraestructura**: `esRutaPublica`, el `fallback` a `index.html`, `table-wrap`, `nuevo_token` (32
  bytes) y `hora` y `fechaDe` ya existen. La sección Presupuesto se ve con cualquier rol.
- **`demo.sql`**: ningún fichero de `backend/tests` ni de `backend/src` lo lee; T001 es coherente con
  su estructura (partes 1 a 5 y `COMMIT` final). Ninguna prueba compara `PresupuestoResumen` entero.

Nota menor: el correo «sin importes» de spec-003 cita `spec-004 FR-001`, que no dice nada de importes.
No afecta a esta spec.

### Metrics

- Requisitos: 13 (9 FR y 4 SC) · Tareas: 31
- Cobertura: 12 de 13 con tarea en 004 (92 %); FR-001 lo cubre spec-003, así que 100 % en la práctica
- Hallazgos: 20 (0 CRITICAL, 1 HIGH, 6 MEDIUM, 13 LOW)
- Ambigüedades: 4 · Duplicaciones: 0 · Críticos: 0

## Next Actions

No hay CRITICAL, pero conviene resolver **antes de `/speckit-implement`**:

1. **E1 y E2**, que cambian cómo se escriben T010, T020 y T023.
2. **I1**, porque pide acuerdo con la Persona B antes de tocar sus ficheros, e **I2**, que depende de
   ese acuerdo.
3. **G1 y E3**, que cambian el quickstart común, e **I3**, que afecta al orden de fusión.

Los LOW se pueden corregir, justificar o asignar en la misma tanda. Después, un `analyze-2` como en
001 y 002.
