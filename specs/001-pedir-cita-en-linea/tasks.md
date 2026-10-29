---

description: "Tareas de spec-001 · Pedir cita en línea (HU-01)"
---

# Tasks: spec-001 · Pedir cita en línea (HU-01)

**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md) (D1–D7),
[data-model.md](data-model.md), [contracts/api.md](contracts/api.md) y
[quickstart.md](quickstart.md), que remiten a los documentos comunes de
[000-base-comun](../000-base-comun/).

**Responsable**: Persona A · **Revisor**: Persona B

**Tests**: se incluyen. El plan maestro (research R13) exige pruebas `#[sqlx::test]` para cada regla
de negocio, y el plan de esta spec enumera 12 en `backend/tests/spec001_citas.rs` (aquí «prueba
1»…«prueba 12»). Cada prueba se escribe **antes** de la tarea que la hace pasar y debe fallar
hasta entonces.

**Trazabilidad**: cada tarea termina con «**Cubre:**», que indica los requisitos (FR-xxx) y los
escenarios (US1-n, US2-n, US3-n) de la spec que atiende. Sirve para construir la matriz de
trazabilidad.

**Organización**: una fase por historia de usuario. Todo texto visible y todo mensaje de error,
en español de España; fechas dd/mm/aaaa con `formato.ts`; «hoy» siempre con `fechas::hoy()`
(Europe/Madrid, R12). Las pruebas crean sus propios datos y no usan valores demo (principio V).

## Format: `[ID] [P?] [Story] Description`

- **[P]**: se puede hacer en paralelo (otro fichero, sin depender de tareas pendientes)
- **[Story]**: historia de usuario (US1, US2, US3)

## Path Conventions

Aplicación web: `backend/src/`, `backend/tests/`, `frontend/src/`. No hay migraciones nuevas.

---

## Phase 1: Setup (cambio compartido y base común)

**Purpose**: aplicar el único cambio en código compartido y comprobar que la base común da lo que
necesita esta spec.

- [X] T001 **Cambio compartido · revisa Persona B · commit propio**. En `backend/src/matricula.rs`, hacer que `normalizar_matricula` devuelva `None` también cuando la matrícula normalizada tenga menos de 2 o más de 10 caracteres (regla del data-model común: «de 2 a 10 letras o cifras»), además de los casos actuales (vacía o con caracteres fuera de `[A-Z0-9]`). Actualizar el comentario de la función, que ahora dice «Devuelve `None` si queda vacía o con caracteres fuera de `[A-Z0-9]`». Primero, ampliar las pruebas unitarias del mismo fichero y comprobar que fallan: `matriculas_invalidas` con `"A"` y `"ABCDE123456"` (11) → `None`, y una prueba nueva `limites_de_longitud` con `"AB"` → `Some("AB")` y `"ABCDE12345"` (10) → `Some("ABCDE12345")`. Ejecutar `cargo test matricula`, hacer un commit solo con este fichero y pedir la revisión de la Persona B antes de fusionar (lo usa spec-007). No tocar la migración: el `CHECK (matricula ~ '^[A-Z0-9]+$')` se queda igual (research D1). **Cubre:** FR-002, US1-5 (cierra I11).
  - *Hecho el 2026-10-22. Desviaciones*: (1) `limites_de_longitud` **no falló** antes del cambio, y no podía fallar: comprueba que `"AB"` y `"ABCDE12345"` se aceptan, y el código anterior aceptaba cualquier longitud. Sirve para que el límite no quede demasiado estricto. La que falló en rojo fue `matriculas_invalidas` (`"A"` → `Some("A")`). (2) **Sin commit todavía**, por indicación de la responsable: el commit propio y la petición de revisión a la Persona B quedan pendientes y se revisan en T031. Resultado: `cargo test` correcto (53 pruebas pasan, 2 ignoradas de spec-006 que esperan a spec-002 y spec-005) y `cargo clippy --all-targets -- -D warnings` sin avisos.
- [X] T002 [P] Comprobar en la base común, sin cambiarla salvo que falte algo: `Usuario::exigir_rol` en `backend/src/auth.rs` (deja pasar siempre al jefe); `fechas::hoy()` en `backend/src/fechas.rs`; `ErrorApi::Validacion { mensaje, detalle }` en `backend/src/error.rs`; `config.cupo_diario` en `backend/src/config.rs` (vale 2 en `estado_prueba` de `backend/tests/comun/mod.rs`); `esRutaPublica('/cita')` y `ErrorApi.detalle` en `frontend/src/lib/api.ts`; y la entrada «Citas» (`/citas`) con `roles: ['recepcion', 'jefe']` en `frontend/src/routes/+layout.svelte`. **Cubre:** FR-001, FR-009, US1-10.
  - *Hecho el 2026-10-22*: los seis elementos existen tal como se describen y no ha hecho falta cambiar la base común. Además, el extractor `Usuario` (`backend/src/auth.rs`) devuelve `ErrorApi::SinSesion` (401) sin cookie válida, como necesita T017.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: esqueleto propio de spec-001: módulo, tipos, utilidades de prueba, rango de días y la
transacción del `POST` con los dos bloqueos. Lo usan las tres historias.

**⚠️ CRITICAL**: ninguna historia puede empezar sin esta fase.

- [X] T003 Crear `backend/src/rutas/citas.rs` con una función `rutas()` que devuelva un `Router<EstadoApp>` con las cuatro rutas del contrato (`GET /api/publico/vehiculos/{matricula}`, `GET /api/publico/dias-disponibles`, `POST /api/publico/citas`, `GET /api/citas`), que de momento responden 501, y registrarla en `backend/src/rutas/mod.rs` (`pub mod citas;` y `.merge(citas::rutas())` en `api()`). Añadir una función `error_campo(campo: &str, mensaje: &str) -> ErrorApi` que construya `ErrorApi::Validacion` con `detalle = {"campo": campo}` (contrato común, tabla de validación). **Cubre:** FR-001.
  - *Hecho el 2026-10-24. Desviación*: los 501 provisionales se devuelven con `StatusCode::NOT_IMPLEMENTED`, porque `ErrorApi` no tiene variante 501 y no merece la pena añadirla para algo temporal. En `rutas/mod.rs` el módulo se registra después de `presupuestos` (spec-003, ya en `main`), sin tocar sus líneas.
- [X] T004 [P] Añadir a `frontend/src/lib/tipos.ts` (compartido) los tipos del contrato: `VehiculoPublico {matricula: string, marca: string, modelo: string, cita_pendiente: boolean}`, `PeticionCita {matricula, fecha: Fecha, motivo, vehiculo?: {marca, modelo}, cliente?: {nombre, telefono, email}}`, `CitaCreada {fecha: Fecha, motivo, matricula}` y `CitaDelDia {id: number, matricula, marca, modelo, motivo, cliente: {nombre: string, telefono: string}, orden_id: number | null}`, **sin ningún campo de correo** en `CitaDelDia`. **Cubre:** FR-003, FR-009.
  - *Hecho el 2026-10-24*: sección «spec-001 · Citas» delante de la de spec-003, sin tocar las demás. `npm run check`: 0 errores.
- [X] T005 Crear `backend/tests/spec001_citas.rs` con `mod comun;`, reutilizando `app_prueba`, `estado_prueba`, `peticion`, `crear_usuario` y `entrar`, y añadir solo las utilidades que faltan, por SQL directo: `insertar_vehiculo(pool, matricula, marca, modelo, nombre, telefono, email) -> i64`, `insertar_cita(pool, vehiculo_id, fecha, motivo)`, `laborable(n) -> NaiveDate` (el n-ésimo día de lunes a viernes a partir de mañana según `fechas::hoy()`), `contar(pool, tabla) -> i64` y `cliente_de(pool, matricula) -> (nombre, telefono, email)`. **Cubre:** infraestructura de pruebas, sin requisito propio (la usan las pruebas 1–12).
  - *Hecho el 2026-10-24. Desviaciones*: `insertar_vehiculo`, `insertar_cita`, `laborable` y `cliente_de` llevan `#[allow(dead_code)]` porque no se usan hasta las fases 3–5 y `clippy -D warnings` lo trataría como error; hay que quitarlo cuando se usen. Se añaden dos utilidades que la tarea no nombra: `es_laborable` y `cita_nueva` (una petición válida con matrícula nueva), que usa la prueba 11.
- [X] T006 Escribir la **prueba 11** en `backend/tests/spec001_citas.rs`: un `POST /api/publico/citas` con una matrícula nueva y datos válidos para un sábado, para hoy y para hoy + 31 días → 422 «Elige uno de los días disponibles» con `detalle.campo = "fecha"`, y `cita` sigue vacía. Caso positivo en el límite: el último día de lunes a viernes que sea ≤ `hoy + 30` aparece en `GET /api/publico/dias-disponibles` y un `POST` con esa fecha **no** devuelve el 422 de `fecha`. Debe fallar. **Cubre:** FR-006.
  - *Hecho el 2026-10-24*: falló en rojo antes de T007 y T008 (`left: 501, right: 422`).
- [X] T007 En `backend/src/rutas/citas.rs`, escribir `dias_ofrecidos(hoy: NaiveDate) -> Vec<NaiveDate>`: de lunes a viernes, desde `hoy + 1` hasta `hoy + 30` incluidos (FR-006, «durante los próximos 30 días naturales»). Implementar `GET /api/publico/dias-disponibles` 🔓 para que devuelva de momento todos los días de `dias_ofrecidos(fechas::hoy())` como `["AAAA-MM-DD", …]`; el filtro por cupo lo añade T029. **Cubre:** FR-006.
  - *Hecho el 2026-10-24*: la prueba 11 pasa, incluido el límite positivo.
- [X] T008 En `backend/src/rutas/citas.rs`, implementar el esqueleto de `POST /api/publico/citas` 🔓 con el orden de la tabla de validación del contrato común (plan, reglas clave; research D2 y D3):
  1. Validar en este orden y devolver el primer error con `error_campo`: `matricula` con `normalizar_matricula` (`None` → «La matrícula no es válida», campo `matricula`); `fecha` incluida en `dias_ofrecidos(fechas::hoy())` («Elige uno de los días disponibles», campo `fecha`); `motivo` no vacío tras `trim` («El motivo es obligatorio», campo `motivo`).
  2. **Antes de los bloqueos**, buscar el vehículo por matrícula normalizada (sin transacción). Si no existe, dejar marcado el hueco donde T024 valida los datos del vehículo nuevo (422). Hasta entonces, con una matrícula nueva se responde 422.
  3. Abrir una transacción y tomar **los dos bloqueos en este orden (research D3)**. Primero la fecha, con la forma de **una clave de 64 bits**: `SELECT pg_advisory_xact_lock(($1::date - DATE '2000-01-01')::bigint)`. Después la matrícula normalizada, con la forma de **dos claves de 32 bits**: `SELECT pg_advisory_xact_lock(1, hashtext($1))`. PostgreSQL no solapa los espacios de claves de una y de dos claves, así que los dos bloqueos nunca coinciden (convención registrada en R6 del plan maestro). Respetar siempre el orden fecha → matrícula para que no haya interbloqueos. Documentarlo con un comentario que remita a R6 y D3.
  4. **Con los bloqueos tomados, volver a buscar el vehículo.** Si ahora existe (otra persona lo dio de alta mientras tanto), tratarlo como vehículo conocido: aplicar D4 e ignorar `vehiculo` y `cliente`. Dejar marcados los huecos de los pasos que añaden T016 (cita pendiente), T029 (cupo) y T024 (alta del vehículo nuevo).
  
  La prueba 11 (T006) debe pasar. **Cubre:** FR-002, FR-005, FR-006, FR-007, FR-010.
  - *Hecho el 2026-10-24. Desviaciones*: (1) **Provisional hasta T024**: con una matrícula nueva se responde 422 «Todavía no se pueden dar de alta vehículos nuevos», sin `detalle.campo`. Ese texto no está en el contrato y T024 tiene que quitarlo. (2) **Provisional hasta T016**: con un vehículo conocido, después de los bloqueos se responde 501 sin `commit`. (3) `PeticionCita` solo tiene `matricula`, `fecha` y `motivo`; `vehiculo` y `cliente` los añade T024 (serde ignora los campos que sobran) para no dejar campos sin leer. (4) Ninguna prueba de esta fase llega a los bloqueos, porque una matrícula nueva se detiene en el paso 2. Las dos consultas se comprobaron a mano en PostgreSQL: `pg_locks` muestra la fecha con `objsubid = 1` (una clave) y la matrícula con `objsubid = 2` (dos claves), en espacios separados. Las pruebas 7 y 10 (T012, T013) las ejercitarán en la Phase 3.

**Checkpoint**: módulo registrado, tipos listos, la prueba 11 pasa y el `POST` ya toma los dos
bloqueos.

---

## Phase 3: User Story 1 - Cliente con vehículo ya conocido pide cita (Priority: P1) 🎯 MVP

**Goal**: con un vehículo registrado, el cliente escribe la matrícula, ve marca y modelo, elige
día y motivo y confirma; si ya tiene una cita pendiente, no puede pedir otra. Recepción y el jefe
ven la cita en la lista del día.

**Independent Test**: con un vehículo registrado (p. ej., demo `1111BBB`), pedir una cita desde
`/cita` y comprobar la confirmación; volver a escribir la matrícula y ver el aviso; entrar como
`ana` y verla en `/citas` con nombre y teléfono (quickstart §1, pasos 1, 4, 6–9).

### Tests for User Story 1 ⚠️

> Se escriben primero y deben fallar. Todas van en `backend/tests/spec001_citas.rs`, así que se
> hacen una detrás de otra, en cualquier orden entre ellas.

- [X] T009 [US1] **Prueba 2** (vehículo reconocido): con `1234BCD` insertado, `GET /api/publico/vehiculos/1234-bcd` → 200 `{matricula: "1234BCD", marca, modelo, cita_pendiente: false}` y la respuesta **no** tiene `nombre`, `telefono`, `email` ni `cliente`; `POST` con `matricula: "1234 bcd"` → 201 `{fecha, motivo, matricula: "1234BCD"}` y una sola fila en `vehiculo`. **Cubre:** FR-002, FR-003, FR-008, US1-1, US1-2.
  - *Hecho el 2026-10-24*: falló en rojo (501) antes de T015 y T016. Comprueba además que el motivo se guarda sin espacios al principio ni al final. Con estas pruebas se quitan los `#[allow(dead_code)]` de `insertar_vehiculo`, `insertar_cita`, `laborable` y `cliente_de` (T005).
- [X] T010 [US1] **Prueba 3** (matrícula inválida): `GET` y `POST` con `12#4`, `A` y `ABCDE123456` → 422 «La matrícula no es válida» (`detalle.campo = "matricula"` en el `POST`) y ninguna fila nueva en `cliente`, `vehiculo` ni `cita`; `GET /api/publico/vehiculos/AB` → 404 (es válida, pero no existe). **Cubre:** FR-002, US1-5.
  - *Hecho el 2026-10-24*: falló en rojo (501). En la URL del `GET`, «12#4» va codificada como `12%234`, porque «#» empezaría el fragmento; el formulario usa `encodeURIComponent` (T018).
- [X] T011 [US1] **Prueba 6** (vehículo conocido): con `1234BCD` de «Ana Pérez», 600000001 y ana@ejemplo.test, un `POST` que trae `vehiculo: {marca: "Otra", modelo: "Otro"}` y `cliente: {nombre: "Intruso", telefono: "699999999", email: "otro@ejemplo.test"}` → 201; `cliente_de("1234BCD")` sigue igual, la marca y el modelo no cambian y `contar("cliente")` no aumenta. **Cubre:** FR-003, US1-7.
  - *Hecho el 2026-10-24*: falló en rojo (501).
- [X] T012 [US1] **Prueba 7** (cita pendiente): con una cita del vehículo para hoy (por SQL) → `GET` con `cita_pendiente: true` y sin ninguna fecha en la respuesta, y `POST` → 409 «Este vehículo ya tiene una cita pendiente.» (texto exacto); lo mismo con una cita para `laborable(3)`. Con la única cita en `hoy - 1` → `cita_pendiente: false` y `POST` → 201. **Cubre:** FR-010, US1-4, US1-6.
  - *Hecho el 2026-10-24*: falló en rojo (501). Cada caso usa su propio vehículo (`1111AAA` con una cita para hoy, `2222AAA` para `laborable(3)` y `3333AAA` con una cita de ayer).
- [X] T013 [US1] **Prueba 10** (concurrencia de la cita pendiente): con `1234BCD` sin citas, lanzar a la vez con `tokio::join!` dos `POST` de la misma matrícula para `laborable(1)` y `laborable(2)` (días **distintos**, así que el bloqueo por fecha no los serializa) → uno 201 y otro 409 «Este vehículo ya tiene una cita pendiente.», y una sola cita en la base de datos. Repetir 20 veces dentro de la prueba para no depender de la suerte. **Cubre:** FR-010 (research D3).
  - *Hecho el 2026-10-24*: falló en rojo (501). **Desviación**: entre vuelta y vuelta se borran las citas, para repetir 20 veces sin llenar el cupo de esos días cuando exista T029. Se comprobó que la prueba detecta el fallo: quitando temporalmente el bloqueo por matrícula, las dos peticiones crean su cita (`[201, 201]`) y la prueba falla; después se restauró el bloqueo.
- [X] T014 [US1] **Prueba 12** (lista de recepción): `GET /api/citas` sin sesión → 401; con `mecanico` y con `almacen` → 403; con `recepcion` y con `jefe` → 200. Crear antes por SQL, con `insertar_vehiculo`, los vehículos `1234BCD` y `5678CDF` (así la prueba no depende del alta de US2) y pedir por `POST`, primero para `1234BCD` y después para `5678CDF`, dos citas del mismo día; `GET /api/citas?fecha=…` las devuelve en ese orden con `matricula`, `marca`, `modelo`, `motivo`, `cliente.nombre`, `cliente.telefono` y `orden_id: null`, y **ningún objeto contiene la clave `email`**. Sin `fecha` → las de `fechas::hoy()`. Un día sin citas → `[]`. **Cubre:** FR-009, US1-3, US1-8, US1-9, US1-10.

### Implementation for User Story 1

  - *Hecho el 2026-10-24*: falló en rojo (501). Usa sus propios usuarios (no los demo). La cita de hoy para la lista sin `fecha` se inserta por SQL, porque el `POST` no admite hoy.
- [X] T015 [US1] En `backend/src/rutas/citas.rs`, implementar `GET /api/publico/vehiculos/{matricula}` 🔓: `normalizar_matricula` → `None` = 422 «La matrícula no es válida»; buscar el vehículo → 404 «No hay ningún vehículo con esa matrícula» (texto del contrato común); responder `{matricula, marca, modelo, cita_pendiente}`, con `cita_pendiente = EXISTS (SELECT 1 FROM cita WHERE vehiculo_id = $1 AND fecha >= $hoy)` y `$hoy = fechas::hoy()`. No seleccionar ningún campo de `cliente` ni la fecha de la cita. Hace pasar T009 (GET) y T010 (GET). **Cubre:** FR-002, FR-003, FR-010, US1-1, US1-4, US1-5.
  - *Hecho el 2026-10-24. Desviación*: el 422 del `GET` también lleva `detalle.campo = "matricula"` (igual que el `POST`), aunque el contrato solo exige el mensaje; así el formulario usa el mismo código para los dos.
- [X] T016 [US1] En el `POST` de `backend/src/rutas/citas.rs`, después de los dos bloqueos de T008: si el vehículo existe y tiene una cita con `fecha >= fechas::hoy()` → 409 `ErrorApi::Conflicto("Este vehículo ya tiene una cita pendiente.")`. Si existe y no la tiene → **ignorar `vehiculo` y `cliente` de la petición, sin ningún `UPDATE` de `cliente` ni de `vehiculo`** (research D4), insertar la cita (`motivo` con `trim`) y responder 201 `{fecha, motivo, matricula}`. Usa el resultado de la **segunda** búsqueda del vehículo (paso 4 de T008). Esta comprobación va **antes** de la del cupo (research D3). Hace pasar T009, T010, T011, T012 y T013. **Cubre:** FR-003, FR-008, FR-010, US1-2, US1-4, US1-6, US1-7.
  - *Hecho el 2026-10-24*: se quita el 501 provisional de la Phase 2. **Sigue provisional hasta T024**: con una matrícula nueva se responde 422 «Todavía no se pueden dar de alta vehículos nuevos» (paso 2), y el mismo error queda en el paso 7, que es inalcanzable hasta T024. El hueco del cupo (T029) está marcado entre la cita pendiente y el alta.
- [X] T017 [US1] En `backend/src/rutas/citas.rs`, implementar `GET /api/citas?fecha=` con el extractor `Usuario` (401 sin sesión) y `usuario.exigir_rol(&[Rol::Recepcion])?` (el jefe pasa y el resto recibe 403). `fecha` es opcional (`Option<NaiveDate>`, por defecto `fechas::hoy()`). Consulta: `cita JOIN vehiculo JOIN cliente LEFT JOIN orden`, seleccionando solo `cita.id, matricula, marca, modelo, motivo, cliente.nombre, cliente.telefono, orden.id` (**nunca `cliente.email`**), con `ORDER BY cita.creada_en, cita.id`. Respuesta: `[{id, matricula, marca, modelo, motivo, cliente: {nombre, telefono}, orden_id}]`. Hace pasar T014. **Cubre:** FR-009, US1-3, US1-8, US1-9, US1-10.
  - *Hecho el 2026-10-24. Desviación*: una `fecha` mal escrita (`?fecha=ayer`) responde 400 «La fecha no es válida», un caso que el contrato no recoge; el formulario de `/citas` siempre envía una fecha válida.
- [X] T018 [P] [US1] Crear `frontend/src/routes/cita/+page.svelte` (pública, usable desde el móvil). Es un formulario con `novalidate` (research D2) y campos de matrícula, día (selector con `GET /api/publico/dias-disponibles`, mostrado como dd/mm/aaaa con `formato.fecha`) y motivo. Comportamiento:
  - Al salir del campo matrícula, llamar a `GET /api/publico/vehiculos/{matricula}`. Con un 200 se muestran marca y modelo y no se piden datos de contacto ni se muestra ningún aviso para cambiarlos. Con `cita_pendiente: true` se muestra exactamente «Este vehículo ya tiene una cita pendiente.» y Confirmar queda desactivado. Con un 422 se muestra el mensaje bajo la matrícula y Confirmar queda desactivado.
  - Al confirmar, `POST /api/publico/citas`. Con un 201 se muestra la fecha (dd/mm/aaaa) y el motivo registrado. Con un 422, el mensaje se muestra bajo el campo de `error.detalle.campo`. Con un 409 de cita pendiente, se muestra el mismo aviso.
  - La página no pide sesión (ya está en `esRutaPublica`).
  
  El 404 (vehículo nuevo) lo completa T025 y el 409 de cupo, T030. **Cubre:** FR-001, FR-002, FR-003, FR-008, FR-010, US1-1, US1-2, US1-4, US1-5, US1-7.
  - *Hecha el 2026-10-24; prueba manual de US1 (11 pasos) correcta, sin ajustes en esta pantalla*. Notas: (1) con un 404 (matrícula nueva) no se muestra nada y no se puede confirmar, hasta T025; (2) si no hay días, el selector queda vacío, hasta T030; (3) un 409 que no sea el de cita pendiente (el del cupo, cuando exista T029) se muestra como error general hasta T030; (4) tras confirmar, un botón «Pedir otra cita» vuelve a mostrar el formulario vacío; (5) el estado se declara con `$state<T>()` para que TypeScript no estreche el tipo a `null`.
- [X] T019 [P] [US1] Crear `frontend/src/routes/citas/+page.svelte`:
  - Al abrir llama a `GET /api/citas` sin `fecha` y muestra las citas de hoy; tiene un selector de fecha para pedir otro día con `?fecha=`.
  - La tabla tiene las columnas Matrícula, Marca, Modelo, Motivo, Nombre y Teléfono, en el orden que devuelve la API, y **sin columna de correo**.
  - Con una lista vacía muestra «No hay citas para este día». Con un error (p. ej., 403) muestra `error.message` y ninguna tabla.
  - Dejar un hueco por fila para el botón «Abrir orden» de spec-002, sin implementarlo.
  
  **Cubre:** FR-009, US1-3, US1-8, US1-9, US1-10.
  - *Hecha el 2026-10-24; prueba manual de US1 (11 pasos) correcta, con dos ajustes que salieron de la prueba*:
    1. **El selector abre con la fecha de hoy** (FR-009). Antes salía vacío (dd/mm/aaaa) aunque el título dijera «Citas de hoy». Ahora la página calcula hoy en hora de Madrid (`Intl.DateTimeFormat('en-CA', { timeZone: 'Europe/Madrid' })`, R12) y pide siempre `GET /api/citas?fecha=…`. **Desviación de research D5**, que decía que el frontend no calcula «hoy», y del contrato de 001, que dice que al abrir se llama sin `fecha`. El servidor sigue aceptando la llamada sin `fecha`. Si se borra la fecha del selector, vuelve a hoy. El título es «Citas de hoy» o «Citas del dd/mm/aaaa», y el botón «Hoy» solo aparece con otro día.
    2. **Con un 403** (mecánico o almacén) se muestra solo «No tienes permiso para hacer esto», sin el título ni el selector.

    `npm run check` (0 errores) y `npm test` (6 pasan) después de los ajustes. El hueco de spec-002 es un comentario dentro de cada fila.

**Checkpoint**: US1 funciona sola con vehículos demo; las pruebas 2, 3, 6, 7, 10 y 12 pasan.

---

## Phase 4: User Story 2 - Cliente con vehículo nuevo pide cita (Priority: P1)

**Goal**: con una matrícula desconocida, el formulario pide marca, modelo, nombre, teléfono y
correo, los valida en el servidor y da de alta el vehículo, el cliente y la cita.

**Independent Test**: pedir cita con una matrícula inexistente, probar un teléfono y un correo
incorrectos, corregirlos y confirmar; al volver a escribir la matrícula, el vehículo se reconoce
(quickstart §1, pasos 2, 3 y 5).

### Tests for User Story 2 ⚠️

- [X] T020 [US2] **Prueba 1** (alta completa): `POST` con `matricula: "5678CDF"`, `vehiculo` y `cliente` válidos → 201; existen un cliente, un vehículo con `matricula = "5678CDF"` y una cita; nombre, teléfono y correo están guardados tal como se escribieron, sin espacios al principio ni al final; después, `GET /api/publico/vehiculos/5678CDF` → 200. **Cubre:** FR-004, US2-1, US2-2.
  - *Hecho el 2026-10-29*: falló en rojo (422 provisional «Todavía no se pueden dar de alta vehículos nuevos»). **Desviación**: la prueba envía la matrícula como `"5678 cdf"` y marca, modelo, nombre, teléfono y correo con espacios al principio y al final, para comprobar a la vez la normalización y el `trim`; el `GET` final comprueba además `cita_pendiente: true`.
- [X] T021 [US2] **Prueba 4** (teléfono y correo): con una matrícula nueva y el resto válido, `telefono: "12345"` → 422 «El teléfono no es válido» (`detalle.campo = "cliente.telefono"`); `email: "ana@correo"` y `email: ""` → 422 «El correo no es válido» (`cliente.email`); `telefono: "+34 612 345 678"` y `telefono: "612-345-678"` con `email: "ana@correo.es"` → 201. Añadir casos límite de la regla («tras quitar espacios y guiones, «+» inicial opcional y de 9 a 15 cifras»): 8 cifras → 422, 15 cifras → 201, 16 → 422, `"6123+45678"` → 422; y del correo («una sola «@», texto delante y un dominio con un punto que tiene texto a ambos lados»): `"a@@b.es"`, `"@b.es"`, `"a@.es"` y `"a@b."` → 422. Orden de las comprobaciones: con el día ya lleno (dos citas insertadas por SQL) y una matrícula nueva con `telefono: "12345"` → 422 «El teléfono no es válido», **no** 409 de cupo, porque los datos del vehículo nuevo se validan antes de los bloqueos. **Cubre:** FR-004a, US2-3, US2-4.
  - *Hecho el 2026-10-29*: falló en rojo (mismo 422 provisional). **Desviaciones**: (1) se añade `telefono: ""` → «El teléfono no es válido» (US2-3, «deja vacío alguno de los dos»). (2) Los tres casos 201 usan matrículas y días distintos (`laborable(1..3)`) para no llenar el cupo cuando exista T029. (3) El caso del día lleno solo puede distinguir el orden de las comprobaciones cuando exista el 409 de cupo (T029): hoy pasa en verde aunque la validación se hiciera después de los bloqueos. Hay que volver a ejecutarlo tras T029. *Hecho el 2026-10-29, con T029*: pasa; con la mutación de validar el vehículo nuevo después de los bloqueos, falla con 409 en lugar de 422, así que ya comprueba el orden de I3.
- [X] T022 [US2] **Prueba 5** (obligatorios): con una matrícula nueva, `motivo: "   "` → 422 «El motivo es obligatorio»; `vehiculo.marca` vacía → 422 «La marca es obligatoria» (`vehiculo.marca`); sin el objeto `cliente` → 422 «El nombre es obligatorio» (`cliente.nombre`). En ningún caso se crea nada. **Cubre:** FR-004, FR-005.
  - *Hecho el 2026-10-29*: falló en rojo (mismo 422 provisional). **Desviación**: se añaden tres casos que la tarea no nombra: `vehiculo.modelo` en blanco → «El modelo es obligatorio», sin el objeto `vehiculo` → «La marca es obligatoria» (`vehiculo.marca`) y `cliente.nombre` en blanco → «El nombre es obligatorio».

### Implementation for User Story 2

- [X] T023 [P] [US2] En `backend/src/rutas/citas.rs`, escribir `telefono_valido(&str) -> bool`: quitar espacios y guiones; un «+» opcional **solo al principio**; el resto, de 9 a 15 dígitos ASCII. Escribir también `correo_valido(&str) -> bool`: exactamente una «@», al menos un carácter delante, y detrás un dominio que contiene un «.» con texto a ambos lados (no empieza ni termina por «.»). Solo se comprueban caracteres, sin el crate `regex` ni ninguna dependencia nueva (research D2). Incluir pruebas unitarias `#[cfg(test)]` con los mismos casos que T021. **Cubre:** FR-004a.
  - *Hecho el 2026-10-29*: las cuatro pruebas unitarias se escribieron con las funciones en `todo!()` y fallaron en rojo antes de implementarlas. **Desviación**: casos de más en las unitarias: `"+"`, `"61234567a"`, `"a@b@c.es"` y `"ana.es"` (no válidos); `"a@b.c"` y `"ana.perez@mi.correo.es"` (válidos).
- [X] T024 [US2] En el `POST` de `backend/src/rutas/citas.rs`, rellenar los dos huecos del vehículo nuevo que deja T008. **Primer hueco (paso 2, antes de los bloqueos)**: si la primera búsqueda no encuentra el vehículo, validar sus datos. Si falta un objeto, se trata como campos vacíos. Validar en este orden, con `trim` y `error_campo`:
  1. `vehiculo.marca` («La marca es obligatoria»)
  2. `vehiculo.modelo` («El modelo es obligatorio»)
  3. `cliente.nombre` («El nombre es obligatorio»)
  4. `cliente.telefono` con `telefono_valido` («El teléfono no es válido»)
  5. `cliente.email` con `correo_valido` («El correo no es válido»)
  
  **Segundo hueco (dentro de la transacción, después de la cita pendiente y del cupo)**: si la segunda búsqueda tampoco encuentra el vehículo, `INSERT` en `cliente` (valores con `trim`, tal como se escribieron), en `vehiculo` (matrícula normalizada) y en `cita`, y responder 201. Si la segunda búsqueda lo encuentra, no se inserta nada en `cliente` ni en `vehiculo` (D4, paso 4 de T008). Hace pasar T020, T021 y T022. **Cubre:** FR-004, FR-004a, FR-005, US2-1, US2-2, US2-3, US2-4.
  - *Hecho el 2026-10-29*: quitado el 422 provisional «Todavía no se pueden dar de alta vehículos nuevos» de los dos sitios del `POST` (paso 2 y paso 7). Pasan las pruebas 1, 4 y 5, y las 10 de `spec001_citas`. **Desviaciones**: (1) la validación está en `validar_alta`, que devuelve los cinco valores ya recortados; el paso 2 la llama solo si la primera búsqueda no encuentra el vehículo, antes de abrir la transacción (I3). (2) El alta de cliente y vehículo es un solo `INSERT … WITH … RETURNING`, igual que `insertar_vehiculo` de las pruebas. (3) Si el vehículo existía en la primera búsqueda y ya no en la segunda, se responde 500 (`ErrorApi::Interno`): ninguna operación borra vehículos, así que no puede pasar. (4) `vehiculo` y `cliente` se añaden a `PeticionCita` como `DatosVehiculo` y `DatosCliente`, con todos los campos opcionales (un campo que falta se trata como vacío).
- [X] T025 [US2] En `frontend/src/routes/cita/+page.svelte`, con un 404 de la matrícula, mostrar los campos Marca, Modelo, Nombre, Teléfono y Correo electrónico. Enviarlos en `vehiculo` y `cliente` solo en ese caso, y ocultarlos y vaciarlos si después se escribe una matrícula conocida. Cada campo muestra bajo él el mensaje del 422 cuando `detalle.campo` coincide (`vehiculo.marca`, `cliente.telefono`, …). Usar `type="tel"` y `type="email"` solo por el teclado del móvil; no hay validación del navegador, porque el formulario tiene `novalidate`. **Cubre:** FR-004, FR-004a, US2-1, US2-3.
  - *Hecho el 2026-10-29*. **Prueba manual en el navegador (responsable): US2-1, US2-2, US2-3 y US2-4 se cumplen** (quickstart común §1, pasos 2, 3 y 5, salvo la desaparición del día lleno del paso 5, que depende de T029). `npm run check` y `npm test` correctos. **Desviaciones**: (1) mientras se escribe la matrícula, los cinco campos se ocultan (igual que la marca y el modelo de un vehículo conocido) pero conservan lo escrito; se vacían solo cuando la búsqueda encuentra un vehículo conocido, y al pulsar «Pedir otra cita». (2) Encima de los campos hay un texto que no está en la spec ni en el contrato: «No conocemos este vehículo. Cuéntanos qué coche es y cómo contactarte.» La responsable lo confirma con estilo de texto normal (no de título) y queda anotado en el plan (Frontend `/cita`) como texto de pantalla. (3) `autocomplete` `name`, `tel` y `email` en nombre, teléfono y correo.

**Checkpoint**: US1 y US2 funcionan; las pruebas 1, 4 y 5 pasan.

---

## Phase 5: User Story 3 - Los días completos no se ofrecen (Priority: P1)

**Goal**: un día con tantas citas como el cupo diario deja de ofrecerse, y nunca se supera el
cupo, aunque dos clientes confirmen a la vez.

**Independent Test**: con cupo 2, pedir dos citas para el mismo día y comprobar que el día
desaparece del selector (quickstart §1, paso 5).

### Tests for User Story 3 ⚠️

- [X] T026 [US3] **Prueba 8** (cupo): todos los vehículos se crean por SQL con `insertar_vehiculo` (la prueba no depende del alta de US2). Con `cupo_diario = 2` (de `estado_prueba`) y dos citas en `laborable(1)` insertadas con `insertar_cita` → `GET /api/publico/dias-disponibles` no incluye esa fecha, pero sí `laborable(2)`; un tercer `POST` para `laborable(1)` con otro vehículo ya insertado → 409 «Ese día ya está completo». Si el vehículo tiene además una cita pendiente → 409 de cita pendiente, no de cupo (orden de D3). **Cubre:** FR-007, US3-1.
  - *Hecho el 2026-10-29*: falló en rojo (el día lleno seguía en `dias-disponibles`). **Desviación**: el vehículo con cita pendiente tiene esa cita en `laborable(3)`, otro día, para que el día lleno siga con 2 citas; al final se comprueba que siguen siendo 2.
- [X] T027 [US3] **Prueba 9** (concurrencia del cupo): con tres vehículos creados por SQL con `insertar_vehiculo` (la prueba no depende del alta de US2) y una cita del primero en `laborable(1)`, lanzar a la vez con `tokio::join!` dos `POST` de los otros dos para ese día → uno 201 y otro 409 «Ese día ya está completo», y exactamente 2 citas ese día. Repetir 20 veces. **Cubre:** FR-007, US3-2, SC-003.
  - *Hecho el 2026-10-29*: falló en rojo (`[201, 201]`). **Desviaciones**: (1) entre vuelta y vuelta se borran las citas de los vehículos 2 y 3 (no la del primero), para que en cada vuelta se vuelva a competir por la última plaza y ninguno tenga cita pendiente. (2) También se comprueba que el 409 es «Ese día ya está completo». Se comprobó que la prueba detecta el fallo: quitando temporalmente el bloqueo por fecha, las dos peticiones crean su cita (`[201, 201]`); después se restauró.

### Implementation for User Story 3

- [X] T028 [US3] En `backend/src/rutas/citas.rs`, completar `GET /api/publico/dias-disponibles`: quitar de `dias_ofrecidos(fechas::hoy())` las fechas con `count(*) >= config.cupo_diario`, con una sola consulta `SELECT fecha, count(*) FROM cita WHERE fecha BETWEEN $1 AND $2 GROUP BY fecha`. **Cubre:** FR-007, US3-1.
  - *Hecho el 2026-10-29*. **Desviación**: el cupo se compara como `i64` (`i64::from(config.cupo_diario)`, que es `u32`) con el `count(*)` de PostgreSQL.
- [X] T029 [US3] En el `POST` de `backend/src/rutas/citas.rs`, entre la comprobación de cita pendiente (T016) y el alta del vehículo nuevo (T024), y siempre con los dos bloqueos ya tomados: contar las citas de la fecha y, si `>= config.cupo_diario`, responder 409 `ErrorApi::Conflicto("Ese día ya está completo")`. Hace pasar T026 y T027. **Recordatorio (T021)**: después, volver a ejecutar la prueba 4 y comprobar que el caso «día lleno → 422» sigue dando 422 «El teléfono no es válido» y no 409 «Ese día ya está completo»; hasta que existe el 409 de cupo, ese caso no podía fallar. **Cubre:** FR-007, US3-1, US3-2.
  - *Hecho el 2026-10-29*: pasan las pruebas 8 y 9 y las 12 de `spec001_citas`. Recordatorio de T021 cumplido: el caso «día lleno → 422» de la prueba 4 pasa y, con la mutación de validar el vehículo nuevo después de los bloqueos, falla con 409 en lugar de 422 (código restaurado después). Sin desviaciones: el texto «Ese día ya está completo» está en la constante `DIA_COMPLETO`, junto a `CITA_PENDIENTE`.
- [ ] T030 [US3] En `frontend/src/routes/cita/+page.svelte`: si `dias-disponibles` devuelve `[]`, mostrar en lugar del selector exactamente «No quedan fechas libres en los próximos 30 días. Llama al taller para pedir cita.», sin permitir confirmar. Con un 409 «Ese día ya está completo», volver a pedir los días, quitar la selección y mostrar «Ese día se acaba de completar; elige otro». **Cubre:** FR-006, FR-007, US3-1, US3-2.
  - *Implementado el 2026-10-29; sin marcar porque **falta la verificación en el navegador***. El comportamiento del servidor en el que se apoya la pantalla (días completos fuera de `dias-disponibles` y 409 «Ese día ya está completo», también con dos peticiones a la vez) está cubierto por las pruebas 8 y 9. Se comprobó que la prueba 9 falla (`[201, 201]`) sin el bloqueo por fecha. La pantalla en sí no tiene pruebas automáticas. `npm run check` y `npm test` correctos. **La prueba manual de US3-1, US3-2 y «No quedan fechas libres» queda pendiente y se hará dentro de T032** (pasos añadidos a su lista). **Desviaciones**: (1) `diasCargados` distingue «todavía no se han cargado» de «no queda ningún día», para que el mensaje no aparezca un instante al abrir la página. (2) «Ese día se acaba de completar; elige otro» se muestra bajo el selector de día (como error del campo `fecha`). (3) Si al volver a pedir los días ya no queda ninguno, se ve el mensaje de «No quedan fechas libres» en lugar del selector, y ese error desaparece con él. (4) Para la prueba manual del mensaje, se arranca el backend con `TALLER_CUPO_DIARIO=0` (todos los días cuentan como llenos; `dotenvy` no sustituye una variable ya definida).

**Checkpoint**: las 12 pruebas del plan pasan; las tres historias funcionan juntas.

---

## Phase 6: Polish & Cross-Cutting Concerns

- [ ] T031 Revisión de la **Persona B** de todo lo compartido antes de fusionar: el commit de T001 (`backend/src/matricula.rs`), `frontend/src/lib/tipos.ts` (T004) y los documentos comunes que cambió el plan: `specs/000-base-comun/contracts/api.md` (sección spec-001 y tabla de validación), `specs/000-base-comun/data-model.md` (`cliente`, `vehiculo`, `cita`) `specs/000-base-comun/quickstart.md` (§1, con el paso 10, y §2.3), `specs/000-base-comun/plan.md` (lista de ficheros compartidos, [hallazgos](hallazgos.md) I5) y `specs/000-base-comun/research.md` (R6, convención de claves, G1). Confirmar también que el historial de spec-007 sigue funcionando con matrículas válidas. **Añadido por T038 (converge-1 F4)**: revisar en `specs/000-base-comun/contracts/api.md` el `detalle.campo` del 422 de `GET /api/publico/vehiculos/{matricula}` y el 400 «La fecha no es válida» de `GET /api/citas`. **Cubre:** FR-002, FR-004a, FR-009, FR-010 (coherencia de los documentos compartidos).
- [ ] T032 **Verificación manual en el navegador** del [quickstart común §1](../000-base-comun/quickstart.md), pasos 1 a 10, con la base de datos recién creada con `backend/seed/demo.sql`. En el paso 10, cronometrar la cita de un vehículo conocido (< 1 min, SC-001) y la de uno nuevo (< 3 min, SC-002) y anotar los tiempos. Para cada paso, anotar si se cumple usando la correspondencia de [quickstart.md](quickstart.md). Probar `/cita` también con el ancho de un móvil (herramientas del navegador). **Pasos añadidos por T030** (pendientes de su prueba manual): (a) **US3-1**: con cupo 2, tras dos citas el mismo día, al volver a `/cita` ese día ya no aparece en el selector y el siguiente día laborable sí; (b) **US3-2**: en dos pestañas de `/cita`, el mismo día con una sola cita y dos matrículas sin cita pendiente; se confirma en una y, al confirmar en la otra, sale «Ese día se acaba de completar; elige otro» bajo el selector, el selector vuelve a «Elige un día», ese día ya no está en la lista y Confirmar queda desactivado hasta elegir otro, con el que la cita se confirma; (c) **«No quedan fechas libres»**: con el backend arrancado con `TALLER_CUPO_DIARIO=0 cargo run`, `/cita` muestra exactamente «No quedan fechas libres en los próximos 30 días. Llama al taller para pedir cita.» en lugar del selector, sin dejar confirmar y sin que el mensaje aparezca un instante al recargar; después se vuelve a arrancar el backend normal. Cuando estos tres pasos se cumplan, marcar también T030. **Pasos añadidos por la Phase 7** (pendientes): (d) **T035, FR-010**: dos pestañas de `/cita` con la misma matrícula nueva (p. ej. `7777XYZ`) y datos distintos; se confirma en la pestaña B y después en la A. En A deben aparecer la marca y el modelo de B y el aviso «Este vehículo ya tiene una cita pendiente.», Confirmar desactivado y los campos del vehículo nuevo ocultos. En `/citas`, ese vehículo tiene una sola cita, con el nombre de B. Si se cumple, marcar T035. (e) **T037, FR-009**: en `/citas`, cambiar de día varias veces seguidas (y pulsar «Hoy»); la tabla corresponde siempre al día del título. Si se cumple, marcar T037. Si un paso falla, la spec no está terminada (principio IV). **Cubre:** FR-001, FR-002, FR-003, FR-004, FR-004a, FR-005, FR-006, FR-007, FR-008, FR-009, FR-010, US1-1, US1-2, US1-3, US1-4, US1-5, US1-6, US1-7, US1-8, US1-9, US1-10, US2-1, US2-2, US2-3, US2-4, US3-1, SC-001, SC-002, SC-003, SC-004.
- [X] T033 Ejecutar y dejar sin errores ni avisos: `cd backend && cargo test` (incluye `spec001_citas` y `matricula`), `cargo clippy --all-targets -- -D warnings`, `cd frontend && npm run check` y `npm test`. **Cubre:** las pruebas 1–12 y las unitarias de T001 y T023, es decir: FR-002 (pruebas 2, 3 y T001), FR-003 (pruebas 2 y 6), FR-004 (pruebas 1 y 5), FR-004a (prueba 4 y T023), FR-005 (prueba 5), FR-006 (prueba 11), FR-007 (pruebas 8 y 9), FR-008 (prueba 2), FR-009 (prueba 12), FR-010 (pruebas 7 y 10), SC-003 (prueba 9).
  - *Hecho el 2026-10-29, sobre el commit `463fa90`*. Las cuatro comprobaciones terminan con código 0, **sin errores ni avisos**. (1) `cargo test`: 0 avisos de compilación. Unitarias: 15 (incluidas las de `matricula` y las 4 de `telefono_valido` y `correo_valido` de T023). `spec001_citas`: **14/14**, es decir, las pruebas 1–12 más `alta_simultanea_de_la_misma_matricula_nueva` (T036) y `errores_documentados_en_el_contrato` (T038). Resto del backend: `spec000_contador` 3, `spec000_estados` 6, `spec000_sesion` 12, `spec003_presupuestos` 20, `spec005_imputaciones` 12 y `spec006_pedidos` 22 con 1 ignorada (`la_ficha_de_la_orden_trae_sus_pedidos`, que espera `GET /api/ordenes/{id}` de spec-002; ajena a spec-001 y ya ignorada antes de empezar). (2) `cargo clippy --all-targets -- -D warnings`: sin avisos; se forzó la revisión tocando `src/lib.rs` para que no se usara un resultado guardado. (3) `npm run check`: 222 ficheros, 0 errores y 0 avisos. (4) `npm test`: 6/6. **Desviación**: T033 cita «las pruebas 1–12»; ahora son 14, por las dos que añadió la Phase 7.
- [ ] T034 Comprobar que el [quickstart común §2.3](../000-base-comun/quickstart.md) sigue funcionando con `9999LLL`. **Ya con spec-001**: en `/cita`, `9999LLL` muestra «Opel Corsa», no muestra el aviso de cita pendiente y deja pedir cita; después aparece en `/citas`, en su día. **Cuando spec-002 esté implementada**: abrir su orden con 100.000 km → «…no puede ser menor que el de la visita anterior (120.400 km)». Si spec-002 aún no está, dejar esta segunda parte anotada como pendiente en la tarea, sin marcarla como hecha. **Cubre:** FR-010 (efecto en spec-002, research D7).

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: T001 primero, con su propio commit y la revisión de B; T002 en paralelo.
- **Foundational (Phase 2)**: depende de T001 (el `POST` usa el nuevo `normalizar_matricula`).
  Bloquea todas las historias.
- **US1 (Phase 3)**: depende de la Phase 2. Es el MVP.
- **US2 (Phase 4)**: depende de la Phase 2. Comparte `citas.rs` y `cita/+page.svelte` con US1, así
  que va después de US1 o con cuidado de no pisar los mismos bloques.
- **US3 (Phase 5)**: depende de la Phase 2. Sus pruebas crean los vehículos por SQL, así que no
  dependen de US2. T029 se coloca entre T016 y el segundo hueco de T024 en el `POST`.
- **Polish (Phase 6)**: después de las tres historias, en el orden T031 → T032 → T033 → T034.

### Within Each User Story

- Las pruebas (T009–T014, T020–T022, T026–T027) se escriben y fallan **antes** de su
  implementación.
- Backend antes que la pantalla que lo usa; el orden del `POST` es siempre: validación de
  matrícula, fecha y motivo → primera búsqueda del vehículo y, si es nuevo, validación de sus datos
  → bloqueo de fecha → bloqueo de matrícula → segunda búsqueda (D4 si ya existe) → cita pendiente
  → cupo → alta del vehículo nuevo → cita.

### Parallel Opportunities

- T002 en paralelo con T001.
- T004 en paralelo con T003 y T005.
- T018 y T019 (frontend de US1) en paralelo entre sí y con T015–T017 (backend), cuando los tipos
  de T004 estén listos.
- T023 (funciones de validación) en paralelo con las pruebas de US2.

---

## Parallel Example: User Story 1

```bash
Task: "T015 GET /api/publico/vehiculos/{matricula} en backend/src/rutas/citas.rs"
Task: "T018 Formulario /cita en frontend/src/routes/cita/+page.svelte"
Task: "T019 Lista de citas en frontend/src/routes/citas/+page.svelte"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. T001 (commit propio, revisión de B) y T002.
2. Phase 2.
3. US1: los clientes habituales piden cita y recepción ve la lista. Se valida con los vehículos
   demo (quickstart §1, pasos 1, 4, 6–9).

### Incremental Delivery

1. US1 → clientes conocidos y lista de recepción (desbloquea spec-002).
2. US2 → clientes nuevos.
3. US3 → cupo y concurrencia.
4. Phase 6 → revisión de B, recorrido del quickstart y comprobación de §2.3.

---

## Notes

- [P] = otro fichero y sin dependencias pendientes.
- T001 va en un commit propio. El resto, un commit por tarea o por grupo lógico.
- Los ficheros compartidos (`matricula.rs`, `tipos.ts` y los documentos de `specs/000-base-comun/`)
  los revisa la Persona B antes de fusionar.

---

## Phase 7: Convergence

Tareas que salen de [converge-1.md](converge-1.md) (2026-10-29). **Responsable**: Persona A ·
**Revisor**: Persona B. Conviene hacerlas antes de T032, para que la prueba manual se haga sobre el
código definitivo.

- [ ] T035 [US1] En `frontend/src/routes/cita/+page.svelte`, con un 409 «Este vehículo ya tiene una cita pendiente.» mostrar siempre el aviso y desactivar Confirmar, también cuando el formulario había tratado la matrícula como nueva (`nuevo = true`, `vehiculo = null`); por ejemplo, volviendo a buscar el vehículo para que pase a «conocido con cita pendiente» y se oculten los campos de vehículo nuevo (converge-1 F1; Persona A, revisa Persona B) per FR-010, US1-4 (partial)
  - *Implementado el 2026-10-29; sin marcar hasta la prueba en el navegador (paso añadido a T032)*. Cambio: en `confirmar()`, con el 409 de cita pendiente y `vehiculo = null`, se llama a `buscarVehiculo()`. El `GET` devuelve el vehículo con `cita_pendiente: true` y el formulario pasa al estado «conocido con cita pendiente»: marca y modelo, el aviso, Confirmar desactivado y los campos del vehículo nuevo ocultos y vacíos. **Prueba reproducible (5.3.4)**. No puede ser automática: el frontend solo tiene `vitest` sin DOM, y añadir una librería de pruebas de componentes iría contra el plan («no se añade ninguna dependencia»). La parte del servidor que provoca el fallo (el 409 de cita pendiente con una matrícula que el formulario creía nueva) sí está cubierta por la prueba automática de T036. *Entrada*: dos pestañas con `/cita` y la misma matrícula nueva (p. ej. `7777XYZ`), cada una con sus datos. En la pestaña B se confirma (201) y después se confirma en la pestaña A. *Esperado*: en A se ven marca y modelo de B, el aviso «Este vehículo ya tiene una cita pendiente.» y Confirmar desactivado; los campos del vehículo nuevo desaparecen. *Antes* (según el código): A no cambia; no hay aviso ni error, los campos siguen visibles y Confirmar sigue activo, y cada intento repite el 409 en silencio. *Después*: pendiente de observar en T032.
- [X] T036 [US2] En `backend/tests/spec001_citas.rs`, añadir una prueba de concurrencia del alta: dos `POST` a la vez con la misma matrícula **nueva** y datos de cliente distintos → un 201 y un 409 «Este vehículo ya tiene una cita pendiente.», exactamente 1 cliente, 1 vehículo y 1 cita, y el cliente guardado es el de la petición que ha recibido el 201; repetir 20 veces y comprobar que falla si se quita temporalmente el bloqueo por matrícula (converge-1 F2; Persona A, revisa Persona B) per FR-004, FR-010, research D3 y D4 (partial)
  - *Hecho el 2026-10-29*: prueba `alta_simultanea_de_la_misma_matricula_nueva` (20 vueltas, días distintos para que el bloqueo por fecha no las serialice). **Prueba reproducible (5.3.4)**. El hallazgo era una falta de prueba, no un fallo del código, así que la prueba pasa con el código actual. Para demostrar que detecta el fallo, se quitó temporalmente el bloqueo por matrícula de `citas.rs`: *observado sin el bloqueo*, `[201, 500]` («Ha ocurrido un error interno»), porque las dos altas chocan en el `UNIQUE` de `vehiculo`. *Observado con el bloqueo*: `[201, 409]` con «Este vehículo ya tiene una cita pendiente.», 1 cliente, 1 vehículo y 1 cita, y el propietario es el de la petición ganadora. Código restaurado y comprobado con `cmp`.
- [ ] T037 [US1] En `frontend/src/routes/citas/+page.svelte`, ignorar las respuestas de `GET /api/citas` que no correspondan al día seleccionado en ese momento, para que la tabla nunca muestre las citas de un día distinto del del selector (converge-1 F3; Persona A, revisa Persona B) per FR-009, US1-8 (partial)
  - *Implementado el 2026-10-29; sin marcar hasta la prueba en el navegador (paso añadido a T032)*. Cambio: `cargar(elegido)` descarta la respuesta, o el error, si al llegar `elegido !== dia`, y solo pone `cargada = true` con la respuesta del día vigente. **Prueba reproducible (5.3.4)**. No puede ser automática, por la misma razón que T035. Tampoco se puede provocar de forma fiable a mano: hace falta que la respuesta de un día llegue después de la del día elegido a continuación, y la limitación de velocidad del navegador retrasa todas las peticiones por igual. *Entrada*: en `/citas`, elegir el día A y, antes de que llegue su respuesta, el día B; la respuesta de A llega la última. *Esperado*: el título y la tabla son los de B. *Antes* (según el código): título «Citas del <B>» con las citas de A, porque se asignaba la última respuesta en llegar. *Después* (según el código): la respuesta de A se descarta y se ven las de B. En T032 se comprueba que, cambiando de día varias veces seguidas, la tabla siempre corresponde al día del título.
- [X] T038 Alinear los contratos con lo implementado: en `contracts/api.md` de 001, la lista completa de textos propios de `/cita` («No conocemos este vehículo. Cuéntanos qué coche es y cómo contactarte.», «Buscando el vehículo…», «No se pudieron cargar los días», «Ha ocurrido un error inesperado», además de los dos que ya figuran); en el contrato común, `detalle.campo` en el 422 de `GET /api/publico/vehiculos/{matricula}` y el 400 «La fecha no es válida» de `GET /api/citas` (desviaciones de T015 y T017). Si la Persona B rechaza alguno, cambiar el código en su lugar (converge-1 F4; Persona A, revisa Persona B, porque el contrato común es compartido) per contracts/api.md de 001 y contrato común (contradicts)
  - *Hecho el 2026-10-29*. **Cambios del contrato común, pendientes de revisión de la Persona B (añadidos a T031)**: en `specs/000-base-comun/contracts/api.md`, (1) el 422 de `GET /api/publico/vehiculos/{matricula}` lleva `detalle.campo = "matricula"`, igual que en el `POST`; (2) `GET /api/citas` responde 400 «La fecha no es válida» si `fecha` no es YYYY-MM-DD. En `contracts/api.md` de 001 se completa la lista de textos propios de `/cita`. No se cambia código. **Prueba reproducible (5.3.4)**: *antes*, el contrato no recogía esas dos respuestas, que el servidor ya daba (`GET /api/publico/vehiculos/A` → `{"error":"La matrícula no es válida","detalle":{"campo":"matricula"}}`; `GET /api/citas?fecha=ayer` con sesión de recepción → 400). *Después*, el contrato las recoge y la nueva prueba `errores_documentados_en_el_contrato` comprueba las dos: falla si el código deja de cumplir lo que dice el contrato.
- [X] T039 Justificar en `plan.md` (Frontend `/cita` y `/citas`) los botones «Pedir otra cita» (tras la confirmación; recepción usa el mismo formulario, Assumptions de la spec) y «Hoy» (en `/citas`, vuelve al día con el que abre la lista, FR-009), o quitarlos del código (converge-1 F5; Persona A, revisa Persona B) per Constitución III (unrequested)
  - *Hecho el 2026-10-29*: los dos botones se mantienen y quedan justificados en `plan.md`, en la sección del frontend: «Pedir otra cita» en `/cita` (US1-4, US1-6 y el uso por recepción de las Assumptions) y «Hoy» en `/citas` (FR-009). No se cambia código. **Prueba reproducible (5.3.4)**: *antes*, ni la spec ni el plan nombraban los botones (`grep -n "Pedir otra cita\|«Hoy»" plan.md` no encontraba nada); *después*, el plan los nombra con su requisito. Su funcionamiento ya se comprobó en la prueba manual de US1 (T018, T019).

---

## Phase 8: Convergence

Tareas que salen de [converge-2.md](converge-2.md) (2026-10-29). **Responsable**: Persona A ·
**Revisor**: Persona B.

- [X] T040 En `contracts/api.md` de 001, completar la frase «Además de las etiquetas de los campos y los botones, los únicos textos propios del formulario son…» para que excluya también los títulos («Pedir cita», «Cita confirmada») y las etiquetas de los datos mostrados («Fecha:», «Motivo:», «Vehículo:»), o añadir esos textos a la lista; no cambia el código ni el contrato común (converge-2 G1; Persona A, revisa Persona B) per contracts/api.md de 001, texto de T038 (contradicts)
  - *Hecho el 2026-10-29*: se completa la lista en lugar de presentarla como parcial. El contrato define qué son **etiquetas**, que no se enumeran: los nombres de los campos, los textos de los botones («Confirmar», «Confirmando…», «Pedir otra cita»), la opción «Elige un día», el ejemplo «1234 BCD» del campo matrícula y los rótulos «Vehículo:», «Fecha:» y «Motivo:». Se añaden a la lista los títulos «Pedir cita» y «Cita confirmada», y el aviso «Este vehículo ya tiene una cita pendiente.» (FR-010), que el formulario muestra tras el `GET`. Al cruzar con la pantalla salieron dos textos más de los que nombraba la tarea: el ejemplo y el aviso. Comprobado contra `cita/+page.svelte`: todo texto visible está en la lista o es una etiqueta. Los mensajes de error del servidor salen del contrato común. No cambia el código ni el contrato común.
