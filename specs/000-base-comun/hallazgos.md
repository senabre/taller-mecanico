# Seguimiento de hallazgos · spec-000 Base común

**Origen**: [analyze-1.md](analyze-1.md) (2026-10-15) · **Próxima revisión**: `analyze-2.md`

Los IDs de tarea de analyze-1 son los de la numeración anterior (39 tareas). La columna «Tarea»
muestra `antes → ahora` según el [tasks.md](tasks.md) actual (43 tareas); «nueva» indica una tarea
que no existía.

**Decisión**: *corregido* = se ha cambiado el documento o la tarea; *justificado* = se mantiene
el comportamiento con un motivo explícito; *pendiente* = no se ha tratado y se revisa en
analyze-2.

**Responsable**: sigue el reparto del plan maestro. La Persona A lleva la base de datos, la
sesión, los datos demo y sus specs (001, 002, 004, 008). La Persona B lleva los módulos comunes,
el frontend común y sus specs (003, 005, 006, 007).

## Tabla de hallazgos

| ID | Documento o criterio afectado | Gravedad | Evidencia | Decisión | Responsable | Estado | Tarea |
|----|-------------------------------|----------|-----------|----------|-------------|--------|-------|
| K1 | Constitución, principio V; quickstart; `seed/demo.sql`; `.env.example` | CRITICAL | `demo1234` en el quickstart, hash argon2 y token `demo-presupuesto-emitido` en el seed, contraseña de ejemplo en `.env.example`; el principio V no tenía excepciones | Corregido: constitución v1.2.0, que permite credenciales y tokens ficticios de demostración en `seed/demo.sql`, `quickstart.md` y los valores de ejemplo de `.env.example`, nunca en producción | A | Cerrado | T030 → T033, T036 → T040, T038 → T042 |
| I1 | data-model `presupuesto`; migración (índice único); spec-003 FR-008; contrato | HIGH | El índice `WHERE estado IN ('borrador','emitido')` bloqueaba para siempre una orden con un presupuesto caducado | Corregido: índice solo para `borrador`; el `emitido` sin caducar se comprueba en el código de spec-003 (con `FOR UPDATE`); FR-008, plan de 003, data-model y contrato actualizados | B | Cerrado | T007 (sin cambio) |
| I2 | data-model (máquina de estados); plan de 008; research de 006 (D8) | MEDIUM | Seguían diciendo que `cambiar_estado` publica el evento, en contra de R14 | Corregido: los tres dicen «`CambioEstado::publicar` después del `commit`» | B | Cerrado | T015 → T016, T014 → T017 |
| I3 | research R11 frente al contrato | MEDIUM | R11 dice `{"tipo":"orden","id":…}`; el contrato y las tareas usan el evento `orden` con `{id, estado}` | Pendiente | B | Pendiente (analyze-2) | T014 → T017, T029 → T030 |
| I4 | data-model, fila «— → recibida»; plan de 002 | MEDIUM | Abrir una orden es un `INSERT` que no publicaba ningún evento; el tablero no veía las órdenes nuevas | Corregido: el plan de 002 publica `CambioEstado { id, Recibida }` después del `commit`; data-model y la tarea de `eventos.rs` lo recogen | A | Cerrado | T014 → T017 |
| I5 | data-model y tarea de `estados.rs` frente al plan de 004 | LOW | La autotransición `en_diagnostico → en_diagnostico` está en `TRANSICIONES`, pero el plan de 004 dice «sin transición» | Pendiente | A | Pendiente (analyze-2) | T015 → T016 |
| I6 | Plan de 002 frente al contrato y la tarea de `rutas/sesion.rs` | LOW | `GET /api/mecanicos` figura en los endpoints de 002 y también en la base común | Pendiente | A | Pendiente (analyze-2) | T028 → T029 |
| I7 | spec-008 US2 frente a los usuarios demo | LOW | El ejemplo «Luis 3 · Ana 1» trata a Ana como mecánica; en los datos demo es de recepción | Pendiente | A | Pendiente (analyze-2) | — |
| I8 | plan.md: alcance, estructura y ficheros compartidos | LOW | Dice «unas 12 tablas» (hay 14); faltan `lib.rs`, `rutas/mod.rs`, `tests/comun/mod.rs` y varios ficheros compartidos | Pendiente (en parte resuelto: la estructura ya incluye `matricula.rs` y `formato.rs`) | A | Pendiente (analyze-2) | T017 → T019, T021 → T023 |
| I9 | Contrato, «Rutas de pantalla» | LOW | Faltan `/` (000) y `/facturas/[id]` (plan de 007) | Pendiente | B | Pendiente (analyze-2) | T032 → T036 |
| I10 | research R3 frente a la tarea de `Cargo.toml` | LOW | La tarea añade crates que R3 no recoge; la feature `uuid` de SQLx no se usa | Pendiente | A | Pendiente (analyze-2) | T001 (sin cambio) |
| D1 | research R9 frente al plan de 004 | LOW | Dos versiones del `UPDATE` de «solo cuenta la primera respuesta» | Pendiente | A | Pendiente (analyze-2) | — |
| A1 | spec US1, escenario 1; quickstart §0.1 | MEDIUM | «No ve Facturación»: no existe ninguna entrada de menú «Facturación», así que la comprobación no comprueba nada | Pendiente | A | Pendiente (analyze-2) | — |
| A2 | quickstart §0 frente a SC-002 y al plan (fase 0) | MEDIUM | El quickstart solo comprueba `luis` y `paco`; faltan `ana` y `pepe` (la verificación final sí los incluye) | Pendiente | A | Pendiente (analyze-2) | T039 → T043 |
| G1 | Planes de 001 y 007; contrato (matrícula normalizada) | MEDIUM | `normalizar_matricula` vivía en `citas.rs` (A) y la necesita 007 (B) | Corregido: pasa a `backend/src/matricula.rs` en la base común; planes de 001 y 007 actualizados | A | Cerrado | nueva T014 |
| G2 | Contrato (403 «la orden no es del mecánico»); planes de 003 y 005 | MEDIUM | Ninguna tarea creaba la comprobación de «mecánico asignado» | Corregido: `exigir_mecanico_de` en `auth.rs` (el jefe siempre pasa), con sus pruebas; planes de 003 y 005 la citan | A | Cerrado | T026 → T027, T025 → T032 |
| G3 | Plan de 002 (85.000 km); plan de 005 (1,25 h); `formato.ts` | MEDIUM | No había formato de números en español ni en el frontend ni en el backend | Corregido: `numero()` en `formato.ts` con sus pruebas y `backend/src/formato.rs`; el plan de 002 lo usa | B | Cerrado | T019 → T021, T024 → T026, nueva T015 |
| G4 | Datos demo frente a data-model y a los planes de 005 y 007 | MEDIUM | Orden en espera de piezas sin pedido pendiente (atascada); orden terminada sin imputaciones; pendiente de conformidad sin acumulado > 110 % | Corregido: pedido pendiente e imputaciones coherentes con cada estado; el stock de PF-123 se queda en 4 | A | Cerrado | T035 → T039, T036 → T040 |
| G5 | Página de inicio por rol frente a quickstart §0 | MEDIUM | En la fase 0, tras entrar, cada rol va a una página que todavía no existe | Justificado: sin páginas de relleno; `+error.svelte` en español dentro del layout, con el menú visible | B | Cerrado | nueva T035 |
| G6 | plan.md, `componentes/EstadoOrden.svelte` | LOW | Ninguna spec ni tarea crea `EstadoOrden.svelte` | Pendiente | A | Pendiente (analyze-2) | — |
| G7 | `correo.rs` frente a los planes de 003 y 007 | LOW | Las dos personas añadirán funciones a `correo.rs`, que no está en la lista de ficheros compartidos | Pendiente | B | Pendiente (analyze-2) | T016 → T018 |
| G8 | Arranque (`ServeDir("../frontend/build")`) frente al plan (Linux o Docker) | LOW | La ruta del frontend depende del directorio desde el que se arranca | Pendiente | B | Pendiente (analyze-2) | T017 → T019 |
| T1 | Tareas de `eventos.rs` y `estados.rs` | MEDIUM | Dependencia circular: `eventos.rs` usaba `Estado` y `estados.rs` dependía de `eventos.rs` | Corregido: `estados.rs` va primero y `CambioEstado::publicar` está en `eventos.rs` | B | Cerrado | T015 → T016, T014 → T017 |
| T2 | Utilidades de prueba `tests/comun/mod.rs` | MEDIUM | La parte de la fase 2 usaba `auth.rs` y `POST /api/sesion`, que son de la fase 3 | Corregido: parte 1 en la fase 2 (`insertar_usuario`, `crear_orden`…); `crear_usuario` y `entrar` en US1 | A | Cerrado | T021 → T023, nueva T031 |
| T3 | Tarea de los usuarios demo | LOW | Marcada `[P]` aunque necesita el hash que genera `crear-usuario` | Corregido: quitado el `[P]` | A | Cerrado | T030 → T033 |

## Resumen

- **Corregidos**: 11 (K1, I1, I2, I4, G1, G2, G3, G4, T1, T2, T3)
- **Justificados**: 1 (G5)
- **Pendientes de analyze-2**: 13 (I3, I5, I6, I7, I8, I9, I10, D1, A1, A2, G6, G7, G8)
- No queda ningún hallazgo CRITICAL ni HIGH sin cerrar.

## Correspondencia de numeración (tasks.md)

| Antes | Ahora | | Antes | Ahora | | Antes | Ahora |
|---|---|---|---|---|---|---|---|
| T001–T013 | T001–T013 | | T021 | T023 | | T030 | T033 |
| — | T014 (nueva, `matricula.rs`) | | T022 | T024 | | T031 | T034 |
| — | T015 (nueva, `formato.rs`) | | T023 | T025 | | — | T035 (nueva, `+error.svelte`) |
| T015 | T016 | | T024 | T026 | | T032 | T036 |
| T014 | T017 | | T026 | T027 | | T033 | T037 |
| T016 | T018 | | T027 | T028 | | T034 | T038 |
| T017 | T019 | | T028 | T029 | | T035 | T039 |
| T018 | T020 | | T029 | T030 | | T036 | T040 |
| T019 | T021 | | — | T031 (nueva, `tests/comun` parte 2) | | T037 | T041 |
| T020 | T022 | | T025 | T032 | | T038 | T042 |
| | | | | | | T039 | T043 |
