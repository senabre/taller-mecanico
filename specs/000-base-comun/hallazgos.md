# Seguimiento de hallazgos · spec-000 Base común

**Origen**: [analyze-1.md](analyze-1.md) (2026-10-15) · **Revisiones**: [analyze-2.md](analyze-2.md), [analyze-3.md](analyze-3.md) (2026-10-15)

Los IDs de tarea de analyze-1 son los de la numeración anterior (39 tareas). La columna «Tarea»
muestra `antes → ahora` según el [tasks.md](tasks.md) actual (43 tareas); «nueva» indica una tarea
que no existía.

**Decisión**: *corregido* = se ha cambiado el documento o la tarea; *justificado* = se mantiene
el comportamiento con un motivo explícito; *pendiente* = no se ha tratado y se revisa en
analyze-3 (o se ha mantenido abierto en analyze-2).

**Estado**: *Cerrado (verificado en analyze-3)* = cambio aplicado después de analyze-2 y
comprobado en analyze-3; *Cerrado (verificado de forma puntual tras analyze-3)* = solo se
ha comprobado ese hallazgo, sin un análisis completo; *Abierto (nuevo en analyze-3)* = lo ha encontrado analyze-3;
*Cerrado (justificado)* = no se cambia, con el motivo en «Decisión»; *Asignado a spec-00X* = lo
resuelve esa spec en su propio trabajo; *Cerrado (verificado en analyze-2)* = analyze-2 ha
comprobado que el cambio está
aplicado en todos los documentos; *Abierto (sigue en analyze-2)* = el hallazgo de analyze-1 sigue
igual; *Abierto (nuevo en analyze-2)* = lo ha encontrado analyze-2.

**Responsable**: sigue el reparto del plan maestro. La Persona A lleva la base de datos, la
sesión, los datos demo y sus specs (001, 002, 004, 008). La Persona B lleva los módulos comunes,
el frontend común y sus specs (003, 005, 006, 007).

## Tabla de hallazgos

| ID | Documento o criterio afectado | Gravedad | Evidencia | Decisión | Responsable | Estado | Tarea |
|----|-------------------------------|----------|-----------|----------|-------------|--------|-------|
| K1 | Constitución, principio V; quickstart; `seed/demo.sql`; `.env.example` | CRITICAL | `demo1234` en el quickstart, hash argon2 y token `demo-presupuesto-emitido` en el seed, contraseña de ejemplo en `.env.example`; el principio V no tenía excepciones | Corregido: constitución v1.2.0, que permite credenciales y tokens ficticios de demostración en `seed/demo.sql`, `quickstart.md` y los valores de ejemplo de `.env.example`, nunca en producción | A | Cerrado (verificado en analyze-2) | T030 → T033, T036 → T040, T038 → T042 |
| I1 | data-model `presupuesto`; migración (índice único); spec-003 FR-008; contrato | HIGH | El índice `WHERE estado IN ('borrador','emitido')` bloqueaba para siempre una orden con un presupuesto caducado | Corregido: índice solo para `borrador`; el `emitido` sin caducar se comprueba en el código de spec-003 (con `FOR UPDATE`); FR-008, plan de 003, data-model y contrato actualizados | B | Cerrado (verificado en analyze-2) | T007 (sin cambio) |
| I2 | data-model (máquina de estados); plan de 008; research de 006 (D8) | MEDIUM | Seguían diciendo que `cambiar_estado` publica el evento, en contra de R14 | Corregido: los tres dicen «`CambioEstado::publicar` después del `commit`» | B | Cerrado (verificado en analyze-2) | T015 → T016, T014 → T017 |
| I3 | research R11 frente al contrato | MEDIUM | R11 dice `{"tipo":"orden","id":…}`; el contrato y las tareas usan el evento `orden` con `{id, estado}` | Corregido: R11 describe un evento SSE `orden` con `{id, estado}`, igual que el contrato y las tareas | B | Cerrado (verificado en analyze-3) | T014 → T017, T029 → T030 |
| I4 | data-model, fila «— → recibida»; plan de 002 | MEDIUM | Abrir una orden es un `INSERT` que no publicaba ningún evento; el tablero no veía las órdenes nuevas | Corregido: el plan de 002 publica `CambioEstado { id, Recibida }` después del `commit`; data-model y la tarea de `eventos.rs` lo recogen | A | Cerrado (verificado en analyze-2) | T014 → T017 |
| I5 | data-model y tarea de `estados.rs` frente al plan de 004 | LOW | La autotransición `en_diagnostico → en_diagnostico` está en `TRANSICIONES`, pero el plan de 004 dice «sin transición» | Asignado: al implementar la respuesta del cliente, spec-004 decide si la autotransición sobra y, si es así, la quita de data-model y de `estados.rs` (revisan A y B, porque es compartido) | A | Asignado a spec-004 | T015 → T016 |
| I6 | Plan de 002 frente al contrato y la tarea de `rutas/sesion.rs` | LOW | `GET /api/mecanicos` figura en los endpoints de 002 y también en la base común | Justificado: el plan de 002 enumera los endpoints que usa; el contrato deja `GET /api/mecanicos` en la base común (T029) y 002 no lo implementa | A | Cerrado (justificado) | T028 → T029 |
| I7 | spec-008 US2 frente a los usuarios demo | LOW | El ejemplo «Luis 3 · Ana 1» trata a Ana como mecánica; en los datos demo es de recepción | Asignado: spec-008 cambia «Ana» por «Marta» en su US2 cuando revise su spec | A | Asignado a spec-008 | — |
| I8 | plan.md: alcance, estructura y ficheros compartidos | LOW | Dice «unas 12 tablas» (hay 14); faltan `lib.rs`, `rutas/mod.rs`, `tests/comun/mod.rs` y varios ficheros compartidos | Justificado: la lista vigente de ficheros compartidos es la de tasks.md («Después de la fase 0»), y el plan da cifras orientativas («unas 12 tablas»). Para `correo.rs`, ver G7 | A | Cerrado (justificado) | T017 → T019, T021 → T023 |
| I9 | Contrato, «Rutas de pantalla» | LOW | Faltan `/` (000) y `/facturas/[id]` (plan de 007) | Asignado: spec-007, al crear `/facturas/[id]`, la añade a las rutas de pantalla del contrato junto con `/` (base común) | B | Asignado a spec-007 | T032 → T036 |
| I10 | research R3 frente a la tarea de `Cargo.toml` | LOW | La tarea añade crates que R3 no recoge; la feature `uuid` de SQLx no se usa | Asignado: al hacer T001 se quita la feature `uuid` (no se usa) y R3 se completa con los crates que queden en `Cargo.toml` | A | Asignado a spec-000 (T001) | T001 (sin cambio) |
| D1 | research R9 frente al plan de 004 | LOW | Dos versiones del `UPDATE` de «solo cuenta la primera respuesta» | Asignado: al implementar la respuesta, spec-004 deja una sola versión del `UPDATE` en R9 y su plan remite a ella | A | Asignado a spec-004 | — |
| A1 | spec US1, escenario 1; quickstart §0.1 | MEDIUM | «No ve Facturación»: no existe ninguna entrada de menú «Facturación», así que la comprobación no comprueba nada | Corregido: la prueba independiente y el escenario 1 de la spec, el quickstart §0.1 y la prueba de la fase 3 de tasks comprueban el menú real («Mis órdenes» e «Historial»; sin Citas, Almacén, Pedidos ni Tablero) | A | Cerrado (verificado en analyze-3) | — |
| A2 | quickstart §0 frente a SC-002 y al plan (fase 0) | MEDIUM | El quickstart solo comprueba `luis` y `paco`; faltan `ana` y `pepe` (la verificación final sí los incluye) | Corregido: quickstart §0 paso 4 con el menú esperado de `ana` y de `pepe`; la prueba de la fase 3 de tasks también lo menciona | A | Cerrado (verificado en analyze-3) | T039 → T043 |
| G1 | Planes de 001 y 007; contrato (matrícula normalizada) | MEDIUM | `normalizar_matricula` vivía en `citas.rs` (A) y la necesita 007 (B) | Corregido: pasa a `backend/src/matricula.rs` en la base común; planes de 001 y 007 actualizados | A | Cerrado (verificado en analyze-2) | nueva T014 |
| G2 | Contrato (403 «la orden no es del mecánico»); planes de 003 y 005 | MEDIUM | Ninguna tarea creaba la comprobación de «mecánico asignado» | Corregido: `exigir_mecanico_de` en `auth.rs` (el jefe siempre pasa), con sus pruebas; planes de 003 y 005 la citan | A | Cerrado (verificado en analyze-2) | T026 → T027, T025 → T032 |
| G3 | Plan de 002 (85.000 km); plan de 005 (1,25 h); `formato.ts` | MEDIUM | No había formato de números en español ni en el frontend ni en el backend | Corregido: `numero()` en `formato.ts` con sus pruebas y `backend/src/formato.rs`; el plan de 002 lo usa | B | Cerrado (verificado en analyze-2) | T019 → T021, T024 → T026, nueva T015 |
| G4 | Datos demo frente a data-model y a los planes de 005 y 007 | MEDIUM | Orden en espera de piezas sin pedido pendiente (atascada); orden terminada sin imputaciones; pendiente de conformidad sin acumulado > 110 % | Corregido: pedido pendiente e imputaciones coherentes con cada estado; el stock de PF-123 se queda en 4 | A | Cerrado (verificado en analyze-2) | T035 → T039, T036 → T040 |
| G5 | Página de inicio por rol frente a quickstart §0 | MEDIUM | En la fase 0, tras entrar, cada rol va a una página que todavía no existe | Justificado: sin páginas de relleno; `+error.svelte` en español dentro del layout, con el menú visible | B | Cerrado (verificado en analyze-2) | nueva T035 |
| G6 | plan.md, `componentes/EstadoOrden.svelte` | LOW | Ninguna spec ni tarea crea `EstadoOrden.svelte` | Asignado: spec-002 crea `EstadoOrden.svelte`, o lo quita del plan si basta con `NOMBRE_ESTADO` | A | Asignado a spec-002 | — |
| G7 | `correo.rs` frente a los planes de 003 y 007 | LOW | Las dos personas añadirán funciones a `correo.rs`, que no está en la lista de ficheros compartidos | Justificado: solo amplían `correo.rs` spec-003 y spec-007, las dos de la Persona B, así que no lo editan las dos personas. (analyze-2 decía, por error, que tasks.md ya lo recogía como compartido; no es así) | B | Cerrado (justificado) | T016 → T018 |
| G8 | Arranque (`ServeDir("../frontend/build")`) frente al plan (Linux o Docker) | LOW | La ruta del frontend depende del directorio desde el que se arranca | Justificado: el despliegue no está en ninguna spec de la v1 (principio III); en desarrollo se arranca desde `backend/`, como indica el quickstart. Se revisará cuando se planifique el despliegue | B | Cerrado (justificado) | T017 → T019 |
| T1 | Tareas de `eventos.rs` y `estados.rs` | MEDIUM | Dependencia circular: `eventos.rs` usaba `Estado` y `estados.rs` dependía de `eventos.rs` | Corregido: `estados.rs` va primero y `CambioEstado::publicar` está en `eventos.rs` | B | Cerrado (verificado en analyze-2) | T015 → T016, T014 → T017 |
| T2 | Utilidades de prueba `tests/comun/mod.rs` | MEDIUM | La parte de la fase 2 usaba `auth.rs` y `POST /api/sesion`, que son de la fase 3 | Corregido: parte 1 en la fase 2 (`insertar_usuario`, `crear_orden`…); `crear_usuario` y `entrar` en US1 | A | Cerrado (verificado en analyze-2) | T021 → T023, nueva T031 |
| T3 | Tarea de los usuarios demo | LOW | Marcada `[P]` aunque necesita el hash que genera `crear-usuario` | Corregido: quitado el `[P]` | A | Cerrado (verificado en analyze-2) | T030 → T033 |

## Hallazgos nuevos de analyze-2

| ID | Documento o criterio afectado | Gravedad | Evidencia | Decisión | Responsable | Estado | Tarea |
|----|-------------------------------|----------|-----------|----------|-------------|--------|-------|
| K2 | Constitución, principio V (excepción v1.2.0); tasks T031 y T042; `tasks.md`, `analyze-1.md` y `hallazgos.md` | CRITICAL | T031 escribiría `demo1234` en `backend/tests/comun/mod.rs`, fuera de los tres ficheros que permite la excepción; la revisión de secretos (T042) fallaría porque esos valores también aparecen en documentos de `specs/` | Corregido: T031 genera una contraseña aleatoria con `auth::nuevo_token()` (las pruebas no contienen ninguna escrita a mano); constitución 1.2.1 permite citar en `specs/` los valores demo del quickstart; T042 comprueba que nunca están en `backend/src/`, `backend/tests/` ni `frontend/src/` | A | Cerrado (verificado en analyze-3) | T031, T042 |
| I11 | T014 (`matricula.rs`, `None` → 422) frente al plan de 001 y el contrato | LOW | El contrato solo prevé 404 en `GET /api/publico/vehiculos/{matricula}` y no dice qué pasa con una matrícula inválida | Pendiente: se decide en el clarify de spec-001 (qué responde una matrícula inválida y su reflejo en el contrato) | A | Asignado a spec-001 (clarify) | T014 |
| D2 | tasks de 006 (T007) frente a `tests/comun/mod.rs` (T023, T031) | LOW | 006 vuelve a escribir «crear usuario con rol, iniciar sesión y devolver la cookie», que ya da la base común | Pendiente: lo corrige spec-006 en su propia tarea T007 para usar `tests/comun`; no se tocan sus ficheros desde la base común. Nota de analyze-3: `crear_usuario` de `tests/comun` devuelve ahora `(id, contraseña)` y `entrar` recibe la contraseña (K2) | B | Asignado a spec-006 | 006/T007 |
| A3 | T035 (`+error.svelte`) | LOW | «Si el mensaje viene del servidor en español, ese mensaje» no se puede comprobar | Corregido: T035 muestra siempre texto fijo («Esta página no existe» / «Ha ocurrido un error») | B | Cerrado (verificado en analyze-3) | T035 |
| G9 | T039 (pedido demo de `PF-123`) frente a quickstart §5.1 | LOW | Si se recibe el pedido demo antes del §5, el stock de PF-123 pasa a 5 y «de 4 a 2» falla | Corregido: nueva pieza demo `BJ-7 Bujía` (stock 0) en T038, usada solo por el pedido pendiente de T039; el quickstart la menciona | A | Cerrado (verificado en analyze-3) | T038, T039 |

## Hallazgos nuevos de analyze-3

| ID | Documento o criterio afectado | Gravedad | Evidencia | Decisión | Responsable | Estado | Tarea |
|----|-------------------------------|----------|-----------|----------|-------------|--------|-------|
| K3 | Constitución 1.2.1 (principio V); tasks T040 y T042; `analyze-1.md`, `analyze-2.md`, `hallazgos.md`; quickstart | CRITICAL | El token demo `demo-presupuesto-emitido` se cita en documentos de `specs/`, pero el quickstart no lo publica, y 1.2.1 solo permite citar lo que publica el quickstart | Corregido: el quickstart («Arrancar») publica `/presupuesto/demo-presupuesto-emitido` como token ficticio de demostración. Verificación puntual: las 9 apariciones del token están en documentos de `specs/000-base-comun/` (quickstart, tasks T040/T042, analyze-1/2/3, hallazgos) y ninguna fuera de `specs/`, así que todas quedan cubiertas por la constitución 1.2.1 | A | Cerrado (verificado de forma puntual tras analyze-3) | T040, T042 |
| I12 | Constitución, principio V | LOW | «Única excepción» seguida de una viñeta que permite citar en `specs/` | Justificado: la segunda viñeta no permite ningún valor nuevo; solo deja nombrar en documentos los que ya cubre la excepción. Se puede pulir en una próxima enmienda PATCH | A | Cerrado (justificado) | — |
| D3 | T043 frente al quickstart §0.4 | LOW | La verificación final repite «entrar como `ana` y `pepe`», que ya es el paso 4 del quickstart | Justificado: T043 consiste en recorrer el quickstart §0; la mención no cambia el resultado | A | Cerrado (justificado) | T043 |

## Resumen

- **Cerrados y verificados**: 19
  - analyze-2 (12): K1, I1, I2, I4, G1, G2, G3, G4, G5, T1, T2, T3
  - analyze-3 (6): K2, I3, A1, A2, A3, G9
  - verificación puntual tras analyze-3, sin análisis completo (1): K3
- **Cerrados como justificados**: 6 (I6, I8, G7, G8, I12, D3)
- **Asignados a una spec**: 8
  - spec-000 (T001): I10
  - spec-001 (clarify): I11
  - spec-002: G6
  - spec-004: I5, D1
  - spec-006: D2
  - spec-007: I9
  - spec-008: I7
- **Abiertos**: 0. No queda nada que bloquee `/speckit-implement` en la base común.

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
