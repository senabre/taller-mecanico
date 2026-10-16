# Análisis de coherencia 1 · spec-000 Base común

**Fecha**: 2026-10-15 · **Alcance**: `spec.md`, `plan.md`, `research.md`, `data-model.md`,
`contracts/api.md`, `quickstart.md` y `tasks.md` de `specs/000-base-comun/`, contrastados con la
constitución (v1.1.0) y con lo que los `spec.md`/`plan.md` de 001–008 (y `research.md`/`tasks.md`
de 006) esperan de la base común.
**Modo**: solo lectura. Este informe no modifica ningún otro fichero.

## Hallazgos

| ID | Categoría | Gravedad | Ubicación | Resumen | Recomendación |
|----|-----------|----------|-----------|---------|---------------|
| K1 | Constitución | **CRITICAL** | quickstart «Arrancar» (`demo1234`); tasks T030 (hash argon2 de `demo1234` en `demo.sql`), T036 (token conocido `demo-presupuesto-emitido`), T004 (`POSTGRES_PASSWORD`/`DATABASE_URL` con contraseña en `.env.example`) | El principio V dice «PROHIBIDO escribir claves, contraseñas, tokens u otros secretos en el código o en el repositorio» y no hace excepciones para datos demo. La contraseña demo ya está en el quickstart, y las tareas añaden su hash y un token en `demo.sql`. | Hay dos opciones. **(a)** Enmendar la constitución (PATCH/MINOR) con una excepción explícita: «credenciales ficticias de demostración, solo para desarrollo, documentadas como tales». **(b)** Quitar las credenciales del repositorio: `demo.sql` lee la contraseña con `\set` desde una variable `DEMO_CONTRASENA` del `.env`, o los usuarios se crean con `crear-usuario` en un script; el token demo se genera al cargar. La (a) es la más simple, pero la tiene que decidir el equipo; analyze no puede rebajar el principio por su cuenta. |
| I1 | Inconsistencia | **HIGH** | data-model `presupuesto` («como mucho un presupuesto `borrador` o `emitido`»); tasks T007 (`presupuesto_abierto_unico … WHERE estado IN ('borrador','emitido')`); spec-003 FR-008 («como máximo un presupuesto emitido **pendiente de respuesta**»); spec-004 US3 | Un presupuesto caducado sigue siendo `emitido` (lo de caducado se calcula). Con el índice único, la orden nunca puede tener otro presupuesto: queda bloqueada en `en_diagnostico` (o en `pendiente_conformidad` si era una ampliación) y nadie puede sacarla. spec-003 FR-008 solo limita los «pendientes de respuesta», y un caducado ya no lo está. | Decidirlo antes de la migración. Opción simple: quitar `emitido` del índice (`WHERE estado = 'borrador'`) y que spec-003 compruebe en la transacción que no hay otro `emitido` **vigente** (`caduca_el >= hoy`). Alternativa: añadir el valor `caducado` al enum y marcarlo al crear el siguiente. Actualizar data-model, T007 y spec-003. |
| I2 | Inconsistencia | MEDIUM | data-model «Máquina de estados» («Todas las transiciones pasan por `estados::cambiar_estado`»); 008/plan Summary («al que publica `estados::cambiar_estado`»); 006/research.md:75 («`cambiar_estado` publica el evento») | R14 ya dice que `cambiar_estado` devuelve `CambioEstado` y que quien la llama publica después del `commit`, pero estos tres textos siguen con la versión anterior. | Actualizar las tres frases para que digan «`cambiar_estado` + `CambioEstado::publicar` tras el commit». Añadir en los planes de 003, 004, 005, 006 y 007 un recordatorio de una línea. |
| I3 | Inconsistencia | MEDIUM | research R11 (`{"tipo":"orden","id":…}`) frente a contract «Base común» (eventos `orden` con `{id, estado}`) y tasks T014/T029 | El formato del evento SSE no coincide. | Dejar en R11 el formato del contrato: evento SSE `orden` con datos `{id, estado}`. |
| I4 | Inconsistencia | MEDIUM | data-model, fila «— → recibida»; R14; T015 («es un `INSERT` de spec-002 y no pasa por aquí») | Abrir una orden no pasa por `cambiar_estado` y ninguna tarea ni plan publica su evento. El tablero (spec-008 FR-005, SC-004) no vería las órdenes nuevas hasta que se recargue. | Añadir en `estados.rs` (T015) `CambioEstado::nueva(id)` o `eventos::publicar_orden(id, Estado::Recibida)`, y decir en data-model y en 002/plan que 002 lo publica tras el commit. |
| I5 | Inconsistencia | LOW | data-model (transición `en_diagnostico → en_diagnostico`); T015 (incluida en `TRANSICIONES`); 004/plan («Rechazar … se queda; sin transición») | Rechazar el presupuesto inicial aparece como transición en un sitio y como «sin transición» en otro. | Quitar la autotransición de data-model y de T015, porque el estado no cambia y no hay nada que publicar. Mantener la redacción de R1/Q1. |
| I6 | Inconsistencia | LOW | 002/plan «Endpoints» (`GET /api/mecanicos`); contract «Base común»; T028 | El mismo endpoint tiene dos dueños. | Quitarlo de la lista de 002/plan, o indicarlo como «(base común)». |
| I7 | Inconsistencia | LOW | 008/spec US2 («Luis 3 · Ana 1», con Ana como mecánica); quickstart (Ana es de recepción; los mecánicos son Luis y Marta) | El ejemplo de la spec no cuadra con los usuarios demo. | Cambiar «Ana» por «Marta» en spec-008. |
| I8 | Inconsistencia | LOW | plan «Scale/Scope» («unas 12 tablas»); plan «Project Structure»; plan «ficheros compartidos» | Hay 14 tablas. La estructura no recoge `backend/src/lib.rs`, `rutas/mod.rs` ni `backend/tests/comun/mod.rs`, que crean T017 y T021 y ya usa 006/tasks T005. La lista de ficheros compartidos no incluye `correo.rs`, `rutas/mod.rs`, `lib.rs`, `seed/demo.sql` ni `+layout.svelte`, que sí cita 006/tasks. | Actualizar plan.md. |
| I9 | Inconsistencia | LOW | contract «Rutas de pantalla» frente a 007/plan (`/facturas/[id]`) y T032 (`/`) | Faltan dos rutas de pantalla en el contrato. | Añadir `/` (todos, 000) y `/facturas/[id]` (recepcion, 007). |
| I10 | Inconsistencia | LOW | research R3 frente a tasks T001 | T001 añade crates que R3 no recoge (`sha2`, `rand`, `base64`, `dotenvy`, `tokio-stream`, `futures-util`, `rust_decimal_macros`, `axum` `macros`). R3 y T001 incluyen la feature `uuid` de SQLx, pero ningún id es UUID. | Actualizar R3 con los crates que hacen falta y quitar `uuid` de R3 y de T001 (principio I). |
| D1 | Duplicación | LOW | research R9 (`WHERE id = $1 AND respondido_en IS NULL`) frente a 004/plan (`WHERE token = $1 AND estado = 'emitido' AND caduca_el >= hoy`) | La regla de «solo cuenta la primera respuesta» tiene dos versiones. | Dejar una sola en R9 (la de 004/plan, que también cubre la caducidad) y que 004 remita a R9. |
| A1 | Ambigüedad | MEDIUM | spec US1 escenario 1 y quickstart §0.1 («no ve Facturación ni Almacén») | No hay ninguna entrada de menú «Facturación»: facturar es un botón dentro de la orden (spec-007), así que esa comprobación no comprueba nada. Además, «ve sus órdenes» depende de la página de spec-002. | Reescribir así: «`luis` ve «Mis órdenes» e «Historial» y no ve Citas, Almacén, Pedidos ni Tablero». |
| A2 | Ambigüedad | MEDIUM | quickstart §0 frente a spec SC-002 («entrando con un usuario de cada rol») y plan, fase 0 («entrar con cada rol») | El quickstart solo comprueba `luis` y `paco`, y es el criterio de terminado. T039 sí añade `ana` y `pepe`, pero no figuran en el quickstart. | Añadir a §0 un paso con `ana` y `pepe` y el menú que debe ver cada uno (principio IV). |
| G1 | Hueco para 001–008 | MEDIUM | 001/plan (`normalizar_matricula` como `pub fn` en `citas.rs`); 007/plan («usa `citas::normalizar_matricula`»); contract («el servidor la normaliza siempre»); T006 (CHECK `^[A-Z0-9]+$`) | Una regla que comparte todo el sistema vive en un fichero de la Persona A, y la Persona B (007) depende de él antes de que exista. | Sacar `normalizar_matricula` a la base común (por ejemplo, un `backend/src/matricula.rs` o dentro de `fechas.rs`/`lib.rs`) con su prueba, y añadirla a tasks. |
| G2 | Hueco para 001–008 | MEDIUM | contract (403 «la orden no es del mecánico»); 003/spec («una orden no asignada… no puede»); 005/plan («mecánico asignado (o jefe)»); T026 (solo `exigir_rol`) | La comprobación de «mecánico asignado» la necesitan 003 y 005, y ninguna tarea la crea. Tampoco está escrito en data-model ni en spec-000 FR-002 si el jefe pasa esta comprobación. | Añadir a T026 `Usuario::exigir_mecanico_de(&orden)`: pasa si es el `mecanico_id` o si el rol es `jefe` (FR-002), y si no, 403. Añadir el caso a T025. |
| G3 | Hueco para 001–008 | MEDIUM | 002/plan («Km con separador de miles (85.000)»; mensaje 422 del backend «(85.000 km)»); 005/plan («horas con coma (1,25 h)»); T019 | `formato.ts` no tiene formato de números ni de cantidades, y el backend no tiene ningún helper para escribir números en español en los mensajes. | Añadir a T019 `numero(n)` (es-ES, `useGrouping: 'always'`) y `cantidad(c: string, tipo)` («1,5 h» / «2 uds»), con su prueba en T024. Añadir a `backend` un `fn miles(i64) -> String` (por ejemplo en `error.rs`) con su prueba. |
| G4 | Hueco para 001–008 | MEDIUM | T035/T036 frente a 007/plan («una orden `terminada` con imputaciones en los datos demo»), data-model («una orden `en_espera_piezas` solo sale de ahí al recibir su último pedido pendiente») y 005 (umbral del 110 %) | En los datos demo, la orden `en_espera_piezas` no tiene ningún pedido pendiente y se queda atascada para siempre. La orden `terminada` no tiene imputaciones (007 no se puede probar). La orden `pendiente_conformidad` no tiene imputaciones por encima del 110 %, así que su estado no es coherente. | Precisar T035/T036: un pedido `pendiente` (con su proveedor y su pieza) para la orden `en_espera_piezas`, imputaciones para las órdenes `en_reparacion` (por debajo del 110 %), `pendiente_conformidad` (por encima del 110 %) y `terminada`. |
| G5 | Hueco | MEDIUM | T032/T033 (`INICIO_ROL` → `/citas`, `/ordenes`, `/almacen`, `/tablero`) frente al criterio de terminado (quickstart §0) | En la fase 0 esas páginas no existen, así que tras entrar cualquier rol cae en la página de error de SvelteKit. El quickstart §0 «funciona», pero se ve como un fallo. | En la fase 0, que `/` sea una portada mínima con el layout («Hola, Luis») y que tras entrar se vaya a `/`. `INICIO_ROL` se activa cuando existan las pantallas, o cada spec la cambia. |
| G6 | Hueco | LOW | plan Project Structure (`componentes/EstadoOrden.svelte`) | Ningún plan de 001–008 ni ninguna tarea de 000 crea `EstadoOrden.svelte` (003 crea `TablaLineas` y `Totales`). | Asignarlo a 002 (A), o quitarlo del plan si basta con `NOMBRE_ESTADO`. |
| G7 | Hueco | LOW | T016 (`correo::enviar` genérico) frente a 003/plan (`correo::enviar_enlace_presupuesto`) y 007/plan (`correo::aviso_recogida`) | Las dos personas editarán `correo.rs`, y no está en la lista de ficheros compartidos. | Añadir `correo.rs` a la lista de ficheros compartidos (ver I8), o que cada spec redacte su correo en su propio módulo de rutas y llame a `enviar`. |
| G8 | Hueco | LOW | T017 (`ServeDir::new("../frontend/build")`) frente a plan Target Platform («servidor Linux (o Docker)») | La ruta del frontend depende del directorio desde el que se arranca. En producción o en Docker no servirá sin cambiar el código. | Sin acción en la fase 0. Anotarlo para cuando se planifique el despliegue (por ejemplo, una variable `DIR_FRONTEND` con ese valor por defecto, que habría que añadir a R7). |
| T1 | Orden de tareas | MEDIUM | T014 [P] (`EventoOrden { estado: Estado }`) y T015 (define `Estado`, depende de T014) | Dependencia circular: T014 necesita el `enum Estado` de T015, y T015 depende de T014. | Mover `enum Estado` al principio de T015 y hacer T015 → T014, o que T014 defina `EventoOrden` después de T015. Quitar `[P]` de T014. |
| T2 | Orden de tareas | MEDIUM | T021 (fase 2: `crear_usuario`, `entrar`) frente a T026 (`hash_contrasena`) y T028 (`POST /api/sesion`), ambas de la fase 3 | Las utilidades de prueba de la fase 2 usan código de US1, así que T021 no compila hasta la fase 3. | Dividir T021: `estado_prueba`, `peticion` y `crear_orden` en la fase 2; `crear_usuario` y `entrar` como tarea nueva de US1 después de T028. |
| T3 | Orden de tareas | LOW | T030 [P] | Necesita el hash que genera `crear-usuario` (T027). | Quitar `[P]` y añadir la dependencia de T027. |

## Cobertura

| Requisito | ¿Tiene tareas? | Tareas | Notas |
|-----------|----------------|--------|-------|
| FR-001 usuario + contraseña, un rol | Sí | T005, T025, T026, T027, T028, T030, T033 | |
| FR-002 el jefe puede todo | Parcial | T026 (`exigir_rol`), T031 (menú) | Falta el caso «mecánico asignado» (G2) |
| FR-003 páginas públicas | Sí | T031, T020, T039 | Solo se puede comprobar del todo cuando existan 001 y 004 |
| FR-004 configuración fuera del código | Sí | T004, T009 | |
| FR-005 estados únicos | Sí | T005, T015, T019, T023 | Ver I4 e I5 |
| FR-006 español, € y dd/mm/aaaa | Sí | T010, T019, T024 | Falta el formato de números (G3) |
| SC-001 entrar en < 15 s | Sí | T033 | |
| SC-002 ningún rol ve opciones ajenas | Sí | T031, T039 | El quickstart no lo recoge (A2) |
| Caso límite: sesión de 12 h | Sí | T026, T025 | |
| Caso límite: usuario desactivado | Sí | T025, T026, T027, T028 | |
| Supuesto: alta y restablecimiento por orden de administración | Sí | T027, T037 | |
| Plan fase 0: esqueleto, compose, `.env.example`, migración, módulos, frontend, datos demo | Sí | T001–T024, T030, T034–T036 | |

**Tareas sin requisito de spec-000**: T034–T036 (datos demo de 003–008) y T037–T038. Todas se
justifican en el plan maestro («Reparto y orden de trabajo») y en el principio V. No hay alcance
fantasma.

**Problemas con la constitución**: K1 (principio V). El resto de principios se cumplen: no hay
pantallas de configuración ni de usuarios (III), el quickstart es la aceptación (IV) y la
complejidad de la autenticación está justificada en Complexity Tracking (I).

## Métricas

- Requisitos analizados: 8 (6 FR + 2 SC), más 2 casos límite y 1 supuesto
- Tareas: 39
- Cobertura: 100 % de los requisitos tiene al menos una tarea (FR-002, parcial)
- Ambigüedades: 2 · Duplicaciones: 1 · Inconsistencias: 10 · Huecos: 8 · Orden de tareas: 3
- Hallazgos: 25 · **CRITICAL: 1** · **HIGH: 1** · MEDIUM: 12 · LOW: 11

## Siguientes pasos

1. **Antes de `/speckit-implement`**:
   - Resolver **K1**: decidir entre enmendar la constitución (`/speckit-constitution`) y sacar las credenciales demo del repositorio.
   - Resolver **I1**: cambiar data-model, T007 y spec-003 FR-008.
   - Ambos afectan a la migración 0001, que es difícil de cambiar después.
2. **En la misma pasada (cambios a mano en `tasks.md`)**:
   - T1, T2 y T3 (orden y dependencias).
   - G2, G3 y G5 (añadir a T026, T019 y T032).
   - G4 (concretar T035 y T036).
   - G1 (nueva tarea `normalizar_matricula`).
3. **Documentación (sin bloquear)**:
   - I2–I10, D1, A1 y A2 en research, data-model, contract, quickstart, plan y en las specs/plans 002, 004, 006 y 008.
4. Volver a ejecutar `/speckit-analyze` y guardar el resultado como `analyze-2.md`.
