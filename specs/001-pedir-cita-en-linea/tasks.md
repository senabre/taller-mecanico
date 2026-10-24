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

- [ ] T003 Crear `backend/src/rutas/citas.rs` con una función `rutas()` que devuelva un `Router<EstadoApp>` con las cuatro rutas del contrato (`GET /api/publico/vehiculos/{matricula}`, `GET /api/publico/dias-disponibles`, `POST /api/publico/citas`, `GET /api/citas`), que de momento responden 501, y registrarla en `backend/src/rutas/mod.rs` (`pub mod citas;` y `.merge(citas::rutas())` en `api()`). Añadir una función `error_campo(campo: &str, mensaje: &str) -> ErrorApi` que construya `ErrorApi::Validacion` con `detalle = {"campo": campo}` (contrato común, tabla de validación). **Cubre:** FR-001.
- [ ] T004 [P] Añadir a `frontend/src/lib/tipos.ts` (compartido) los tipos del contrato: `VehiculoPublico {matricula: string, marca: string, modelo: string, cita_pendiente: boolean}`, `PeticionCita {matricula, fecha: Fecha, motivo, vehiculo?: {marca, modelo}, cliente?: {nombre, telefono, email}}`, `CitaCreada {fecha: Fecha, motivo, matricula}` y `CitaDelDia {id: number, matricula, marca, modelo, motivo, cliente: {nombre: string, telefono: string}, orden_id: number | null}`, **sin ningún campo de correo** en `CitaDelDia`. **Cubre:** FR-003, FR-009.
- [ ] T005 Crear `backend/tests/spec001_citas.rs` con `mod comun;`, reutilizando `app_prueba`, `estado_prueba`, `peticion`, `crear_usuario` y `entrar`, y añadir solo las utilidades que faltan, por SQL directo: `insertar_vehiculo(pool, matricula, marca, modelo, nombre, telefono, email) -> i64`, `insertar_cita(pool, vehiculo_id, fecha, motivo)`, `laborable(n) -> NaiveDate` (el n-ésimo día de lunes a viernes a partir de mañana según `fechas::hoy()`), `contar(pool, tabla) -> i64` y `cliente_de(pool, matricula) -> (nombre, telefono, email)`. **Cubre:** infraestructura de pruebas, sin requisito propio (la usan las pruebas 1–12).
- [ ] T006 Escribir la **prueba 11** en `backend/tests/spec001_citas.rs`: un `POST /api/publico/citas` con una matrícula nueva y datos válidos para un sábado, para hoy y para hoy + 31 días → 422 «Elige uno de los días disponibles» con `detalle.campo = "fecha"`, y `cita` sigue vacía. Caso positivo en el límite: el último día de lunes a viernes que sea ≤ `hoy + 30` aparece en `GET /api/publico/dias-disponibles` y un `POST` con esa fecha **no** devuelve el 422 de `fecha`. Debe fallar. **Cubre:** FR-006.
- [ ] T007 En `backend/src/rutas/citas.rs`, escribir `dias_ofrecidos(hoy: NaiveDate) -> Vec<NaiveDate>`: de lunes a viernes, desde `hoy + 1` hasta `hoy + 30` incluidos (FR-006, «durante los próximos 30 días naturales»). Implementar `GET /api/publico/dias-disponibles` 🔓 para que devuelva de momento todos los días de `dias_ofrecidos(fechas::hoy())` como `["AAAA-MM-DD", …]`; el filtro por cupo lo añade T029. **Cubre:** FR-006.
- [ ] T008 En `backend/src/rutas/citas.rs`, implementar el esqueleto de `POST /api/publico/citas` 🔓 con el orden de la tabla de validación del contrato común (plan, reglas clave; research D2 y D3):
  1. Validar en este orden y devolver el primer error con `error_campo`: `matricula` con `normalizar_matricula` (`None` → «La matrícula no es válida», campo `matricula`); `fecha` incluida en `dias_ofrecidos(fechas::hoy())` («Elige uno de los días disponibles», campo `fecha`); `motivo` no vacío tras `trim` («El motivo es obligatorio», campo `motivo`).
  2. **Antes de los bloqueos**, buscar el vehículo por matrícula normalizada (sin transacción). Si no existe, dejar marcado el hueco donde T024 valida los datos del vehículo nuevo (422). Hasta entonces, con una matrícula nueva se responde 422.
  3. Abrir una transacción y tomar **los dos bloqueos en este orden (research D3)**. Primero la fecha, con la forma de **una clave de 64 bits**: `SELECT pg_advisory_xact_lock(($1::date - DATE '2000-01-01')::bigint)`. Después la matrícula normalizada, con la forma de **dos claves de 32 bits**: `SELECT pg_advisory_xact_lock(1, hashtext($1))`. PostgreSQL no solapa los espacios de claves de una y de dos claves, así que los dos bloqueos nunca coinciden (convención registrada en R6 del plan maestro). Respetar siempre el orden fecha → matrícula para que no haya interbloqueos. Documentarlo con un comentario que remita a R6 y D3.
  4. **Con los bloqueos tomados, volver a buscar el vehículo.** Si ahora existe (otra persona lo dio de alta mientras tanto), tratarlo como vehículo conocido: aplicar D4 e ignorar `vehiculo` y `cliente`. Dejar marcados los huecos de los pasos que añaden T016 (cita pendiente), T029 (cupo) y T024 (alta del vehículo nuevo).
  
  La prueba 11 (T006) debe pasar. **Cubre:** FR-002, FR-005, FR-006, FR-007, FR-010.

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

- [ ] T009 [US1] **Prueba 2** (vehículo reconocido): con `1234BCD` insertado, `GET /api/publico/vehiculos/1234-bcd` → 200 `{matricula: "1234BCD", marca, modelo, cita_pendiente: false}` y la respuesta **no** tiene `nombre`, `telefono`, `email` ni `cliente`; `POST` con `matricula: "1234 bcd"` → 201 `{fecha, motivo, matricula: "1234BCD"}` y una sola fila en `vehiculo`. **Cubre:** FR-002, FR-003, FR-008, US1-1, US1-2.
- [ ] T010 [US1] **Prueba 3** (matrícula inválida): `GET` y `POST` con `12#4`, `A` y `ABCDE123456` → 422 «La matrícula no es válida» (`detalle.campo = "matricula"` en el `POST`) y ninguna fila nueva en `cliente`, `vehiculo` ni `cita`; `GET /api/publico/vehiculos/AB` → 404 (es válida, pero no existe). **Cubre:** FR-002, US1-5.
- [ ] T011 [US1] **Prueba 6** (vehículo conocido): con `1234BCD` de «Ana Pérez», 600000001 y ana@ejemplo.test, un `POST` que trae `vehiculo: {marca: "Otra", modelo: "Otro"}` y `cliente: {nombre: "Intruso", telefono: "699999999", email: "otro@ejemplo.test"}` → 201; `cliente_de("1234BCD")` sigue igual, la marca y el modelo no cambian y `contar("cliente")` no aumenta. **Cubre:** FR-003, US1-7.
- [ ] T012 [US1] **Prueba 7** (cita pendiente): con una cita del vehículo para hoy (por SQL) → `GET` con `cita_pendiente: true` y sin ninguna fecha en la respuesta, y `POST` → 409 «Este vehículo ya tiene una cita pendiente.» (texto exacto); lo mismo con una cita para `laborable(3)`. Con la única cita en `hoy - 1` → `cita_pendiente: false` y `POST` → 201. **Cubre:** FR-010, US1-4, US1-6.
- [ ] T013 [US1] **Prueba 10** (concurrencia de la cita pendiente): con `1234BCD` sin citas, lanzar a la vez con `tokio::join!` dos `POST` de la misma matrícula para `laborable(1)` y `laborable(2)` (días **distintos**, así que el bloqueo por fecha no los serializa) → uno 201 y otro 409 «Este vehículo ya tiene una cita pendiente.», y una sola cita en la base de datos. Repetir 20 veces dentro de la prueba para no depender de la suerte. **Cubre:** FR-010 (research D3).
- [ ] T014 [US1] **Prueba 12** (lista de recepción): `GET /api/citas` sin sesión → 401; con `mecanico` y con `almacen` → 403; con `recepcion` y con `jefe` → 200. Crear antes por SQL, con `insertar_vehiculo`, los vehículos `1234BCD` y `5678CDF` (así la prueba no depende del alta de US2) y pedir por `POST`, primero para `1234BCD` y después para `5678CDF`, dos citas del mismo día; `GET /api/citas?fecha=…` las devuelve en ese orden con `matricula`, `marca`, `modelo`, `motivo`, `cliente.nombre`, `cliente.telefono` y `orden_id: null`, y **ningún objeto contiene la clave `email`**. Sin `fecha` → las de `fechas::hoy()`. Un día sin citas → `[]`. **Cubre:** FR-009, US1-3, US1-8, US1-9, US1-10.

### Implementation for User Story 1

- [ ] T015 [US1] En `backend/src/rutas/citas.rs`, implementar `GET /api/publico/vehiculos/{matricula}` 🔓: `normalizar_matricula` → `None` = 422 «La matrícula no es válida»; buscar el vehículo → 404 «No hay ningún vehículo con esa matrícula» (texto del contrato común); responder `{matricula, marca, modelo, cita_pendiente}`, con `cita_pendiente = EXISTS (SELECT 1 FROM cita WHERE vehiculo_id = $1 AND fecha >= $hoy)` y `$hoy = fechas::hoy()`. No seleccionar ningún campo de `cliente` ni la fecha de la cita. Hace pasar T009 (GET) y T010 (GET). **Cubre:** FR-002, FR-003, FR-010, US1-1, US1-4, US1-5.
- [ ] T016 [US1] En el `POST` de `backend/src/rutas/citas.rs`, después de los dos bloqueos de T008: si el vehículo existe y tiene una cita con `fecha >= fechas::hoy()` → 409 `ErrorApi::Conflicto("Este vehículo ya tiene una cita pendiente.")`. Si existe y no la tiene → **ignorar `vehiculo` y `cliente` de la petición, sin ningún `UPDATE` de `cliente` ni de `vehiculo`** (research D4), insertar la cita (`motivo` con `trim`) y responder 201 `{fecha, motivo, matricula}`. Usa el resultado de la **segunda** búsqueda del vehículo (paso 4 de T008). Esta comprobación va **antes** de la del cupo (research D3). Hace pasar T009, T010, T011, T012 y T013. **Cubre:** FR-003, FR-008, FR-010, US1-2, US1-4, US1-6, US1-7.
- [ ] T017 [US1] En `backend/src/rutas/citas.rs`, implementar `GET /api/citas?fecha=` con el extractor `Usuario` (401 sin sesión) y `usuario.exigir_rol(&[Rol::Recepcion])?` (el jefe pasa y el resto recibe 403). `fecha` es opcional (`Option<NaiveDate>`, por defecto `fechas::hoy()`). Consulta: `cita JOIN vehiculo JOIN cliente LEFT JOIN orden`, seleccionando solo `cita.id, matricula, marca, modelo, motivo, cliente.nombre, cliente.telefono, orden.id` (**nunca `cliente.email`**), con `ORDER BY cita.creada_en, cita.id`. Respuesta: `[{id, matricula, marca, modelo, motivo, cliente: {nombre, telefono}, orden_id}]`. Hace pasar T014. **Cubre:** FR-009, US1-3, US1-8, US1-9, US1-10.
- [ ] T018 [P] [US1] Crear `frontend/src/routes/cita/+page.svelte` (pública, usable desde el móvil). Es un formulario con `novalidate` (research D2) y campos de matrícula, día (selector con `GET /api/publico/dias-disponibles`, mostrado como dd/mm/aaaa con `formato.fecha`) y motivo. Comportamiento:
  - Al salir del campo matrícula, llamar a `GET /api/publico/vehiculos/{matricula}`. Con un 200 se muestran marca y modelo y no se piden datos de contacto ni se muestra ningún aviso para cambiarlos. Con `cita_pendiente: true` se muestra exactamente «Este vehículo ya tiene una cita pendiente.» y Confirmar queda desactivado. Con un 422 se muestra el mensaje bajo la matrícula y Confirmar queda desactivado.
  - Al confirmar, `POST /api/publico/citas`. Con un 201 se muestra la fecha (dd/mm/aaaa) y el motivo registrado. Con un 422, el mensaje se muestra bajo el campo de `error.detalle.campo`. Con un 409 de cita pendiente, se muestra el mismo aviso.
  - La página no pide sesión (ya está en `esRutaPublica`).
  
  El 404 (vehículo nuevo) lo completa T025 y el 409 de cupo, T030. **Cubre:** FR-001, FR-002, FR-003, FR-008, FR-010, US1-1, US1-2, US1-4, US1-5, US1-7.
- [ ] T019 [P] [US1] Crear `frontend/src/routes/citas/+page.svelte`:
  - Al abrir llama a `GET /api/citas` sin `fecha` y muestra las citas de hoy; tiene un selector de fecha para pedir otro día con `?fecha=`.
  - La tabla tiene las columnas Matrícula, Marca, Modelo, Motivo, Nombre y Teléfono, en el orden que devuelve la API, y **sin columna de correo**.
  - Con una lista vacía muestra «No hay citas para este día». Con un error (p. ej., 403) muestra `error.message` y ninguna tabla.
  - Dejar un hueco por fila para el botón «Abrir orden» de spec-002, sin implementarlo.
  
  **Cubre:** FR-009, US1-3, US1-8, US1-9, US1-10.

**Checkpoint**: US1 funciona sola con vehículos demo; las pruebas 2, 3, 6, 7, 10 y 12 pasan.

---

## Phase 4: User Story 2 - Cliente con vehículo nuevo pide cita (Priority: P1)

**Goal**: con una matrícula desconocida, el formulario pide marca, modelo, nombre, teléfono y
correo, los valida en el servidor y da de alta el vehículo, el cliente y la cita.

**Independent Test**: pedir cita con una matrícula inexistente, probar un teléfono y un correo
incorrectos, corregirlos y confirmar; al volver a escribir la matrícula, el vehículo se reconoce
(quickstart §1, pasos 2, 3 y 5).

### Tests for User Story 2 ⚠️

- [ ] T020 [US2] **Prueba 1** (alta completa): `POST` con `matricula: "5678CDF"`, `vehiculo` y `cliente` válidos → 201; existen un cliente, un vehículo con `matricula = "5678CDF"` y una cita; nombre, teléfono y correo están guardados tal como se escribieron, sin espacios al principio ni al final; después, `GET /api/publico/vehiculos/5678CDF` → 200. **Cubre:** FR-004, US2-1, US2-2.
- [ ] T021 [US2] **Prueba 4** (teléfono y correo): con una matrícula nueva y el resto válido, `telefono: "12345"` → 422 «El teléfono no es válido» (`detalle.campo = "cliente.telefono"`); `email: "ana@correo"` y `email: ""` → 422 «El correo no es válido» (`cliente.email`); `telefono: "+34 612 345 678"` y `telefono: "612-345-678"` con `email: "ana@correo.es"` → 201. Añadir casos límite de la regla («tras quitar espacios y guiones, «+» inicial opcional y de 9 a 15 cifras»): 8 cifras → 422, 15 cifras → 201, 16 → 422, `"6123+45678"` → 422; y del correo («una sola «@», texto delante y un dominio con un punto que tiene texto a ambos lados»): `"a@@b.es"`, `"@b.es"`, `"a@.es"` y `"a@b."` → 422. Orden de las comprobaciones: con el día ya lleno (dos citas insertadas por SQL) y una matrícula nueva con `telefono: "12345"` → 422 «El teléfono no es válido», **no** 409 de cupo, porque los datos del vehículo nuevo se validan antes de los bloqueos. **Cubre:** FR-004a, US2-3, US2-4.
- [ ] T022 [US2] **Prueba 5** (obligatorios): con una matrícula nueva, `motivo: "   "` → 422 «El motivo es obligatorio»; `vehiculo.marca` vacía → 422 «La marca es obligatoria» (`vehiculo.marca`); sin el objeto `cliente` → 422 «El nombre es obligatorio» (`cliente.nombre`). En ningún caso se crea nada. **Cubre:** FR-004, FR-005.

### Implementation for User Story 2

- [ ] T023 [P] [US2] En `backend/src/rutas/citas.rs`, escribir `telefono_valido(&str) -> bool`: quitar espacios y guiones; un «+» opcional **solo al principio**; el resto, de 9 a 15 dígitos ASCII. Escribir también `correo_valido(&str) -> bool`: exactamente una «@», al menos un carácter delante, y detrás un dominio que contiene un «.» con texto a ambos lados (no empieza ni termina por «.»). Solo se comprueban caracteres, sin el crate `regex` ni ninguna dependencia nueva (research D2). Incluir pruebas unitarias `#[cfg(test)]` con los mismos casos que T021. **Cubre:** FR-004a.
- [ ] T024 [US2] En el `POST` de `backend/src/rutas/citas.rs`, rellenar los dos huecos del vehículo nuevo que deja T008. **Primer hueco (paso 2, antes de los bloqueos)**: si la primera búsqueda no encuentra el vehículo, validar sus datos. Si falta un objeto, se trata como campos vacíos. Validar en este orden, con `trim` y `error_campo`:
  1. `vehiculo.marca` («La marca es obligatoria»)
  2. `vehiculo.modelo` («El modelo es obligatorio»)
  3. `cliente.nombre` («El nombre es obligatorio»)
  4. `cliente.telefono` con `telefono_valido` («El teléfono no es válido»)
  5. `cliente.email` con `correo_valido` («El correo no es válido»)
  
  **Segundo hueco (dentro de la transacción, después de la cita pendiente y del cupo)**: si la segunda búsqueda tampoco encuentra el vehículo, `INSERT` en `cliente` (valores con `trim`, tal como se escribieron), en `vehiculo` (matrícula normalizada) y en `cita`, y responder 201. Si la segunda búsqueda lo encuentra, no se inserta nada en `cliente` ni en `vehiculo` (D4, paso 4 de T008). Hace pasar T020, T021 y T022. **Cubre:** FR-004, FR-004a, FR-005, US2-1, US2-2, US2-3, US2-4.
- [ ] T025 [US2] En `frontend/src/routes/cita/+page.svelte`, con un 404 de la matrícula, mostrar los campos Marca, Modelo, Nombre, Teléfono y Correo electrónico. Enviarlos en `vehiculo` y `cliente` solo en ese caso, y ocultarlos y vaciarlos si después se escribe una matrícula conocida. Cada campo muestra bajo él el mensaje del 422 cuando `detalle.campo` coincide (`vehiculo.marca`, `cliente.telefono`, …). Usar `type="tel"` y `type="email"` solo por el teclado del móvil; no hay validación del navegador, porque el formulario tiene `novalidate`. **Cubre:** FR-004, FR-004a, US2-1, US2-3.

**Checkpoint**: US1 y US2 funcionan; las pruebas 1, 4 y 5 pasan.

---

## Phase 5: User Story 3 - Los días completos no se ofrecen (Priority: P1)

**Goal**: un día con tantas citas como el cupo diario deja de ofrecerse, y nunca se supera el
cupo, aunque dos clientes confirmen a la vez.

**Independent Test**: con cupo 2, pedir dos citas para el mismo día y comprobar que el día
desaparece del selector (quickstart §1, paso 5).

### Tests for User Story 3 ⚠️

- [ ] T026 [US3] **Prueba 8** (cupo): todos los vehículos se crean por SQL con `insertar_vehiculo` (la prueba no depende del alta de US2). Con `cupo_diario = 2` (de `estado_prueba`) y dos citas en `laborable(1)` insertadas con `insertar_cita` → `GET /api/publico/dias-disponibles` no incluye esa fecha, pero sí `laborable(2)`; un tercer `POST` para `laborable(1)` con otro vehículo ya insertado → 409 «Ese día ya está completo». Si el vehículo tiene además una cita pendiente → 409 de cita pendiente, no de cupo (orden de D3). **Cubre:** FR-007, US3-1.
- [ ] T027 [US3] **Prueba 9** (concurrencia del cupo): con tres vehículos creados por SQL con `insertar_vehiculo` (la prueba no depende del alta de US2) y una cita del primero en `laborable(1)`, lanzar a la vez con `tokio::join!` dos `POST` de los otros dos para ese día → uno 201 y otro 409 «Ese día ya está completo», y exactamente 2 citas ese día. Repetir 20 veces. **Cubre:** FR-007, US3-2, SC-003.

### Implementation for User Story 3

- [ ] T028 [US3] En `backend/src/rutas/citas.rs`, completar `GET /api/publico/dias-disponibles`: quitar de `dias_ofrecidos(fechas::hoy())` las fechas con `count(*) >= config.cupo_diario`, con una sola consulta `SELECT fecha, count(*) FROM cita WHERE fecha BETWEEN $1 AND $2 GROUP BY fecha`. **Cubre:** FR-007, US3-1.
- [ ] T029 [US3] En el `POST` de `backend/src/rutas/citas.rs`, entre la comprobación de cita pendiente (T016) y el alta del vehículo nuevo (T024), y siempre con los dos bloqueos ya tomados: contar las citas de la fecha y, si `>= config.cupo_diario`, responder 409 `ErrorApi::Conflicto("Ese día ya está completo")`. Hace pasar T026 y T027. **Cubre:** FR-007, US3-1, US3-2.
- [ ] T030 [US3] En `frontend/src/routes/cita/+page.svelte`: si `dias-disponibles` devuelve `[]`, mostrar en lugar del selector exactamente «No quedan fechas libres en los próximos 30 días. Llama al taller para pedir cita.», sin permitir confirmar. Con un 409 «Ese día ya está completo», volver a pedir los días, quitar la selección y mostrar «Ese día se acaba de completar; elige otro». **Cubre:** FR-006, FR-007, US3-1, US3-2.

**Checkpoint**: las 12 pruebas del plan pasan; las tres historias funcionan juntas.

---

## Phase 6: Polish & Cross-Cutting Concerns

- [ ] T031 Revisión de la **Persona B** de todo lo compartido antes de fusionar: el commit de T001 (`backend/src/matricula.rs`), `frontend/src/lib/tipos.ts` (T004) y los documentos comunes que cambió el plan: `specs/000-base-comun/contracts/api.md` (sección spec-001 y tabla de validación), `specs/000-base-comun/data-model.md` (`cliente`, `vehiculo`, `cita`) `specs/000-base-comun/quickstart.md` (§1, con el paso 10, y §2.3), `specs/000-base-comun/plan.md` (lista de ficheros compartidos, [hallazgos](hallazgos.md) I5) y `specs/000-base-comun/research.md` (R6, convención de claves, G1). Confirmar también que el historial de spec-007 sigue funcionando con matrículas válidas. **Cubre:** FR-002, FR-004a, FR-009, FR-010 (coherencia de los documentos compartidos).
- [ ] T032 **Verificación manual en el navegador** del [quickstart común §1](../000-base-comun/quickstart.md), pasos 1 a 10, con la base de datos recién creada con `backend/seed/demo.sql`. En el paso 10, cronometrar la cita de un vehículo conocido (< 1 min, SC-001) y la de uno nuevo (< 3 min, SC-002) y anotar los tiempos. Para cada paso, anotar si se cumple usando la correspondencia de [quickstart.md](quickstart.md). Probar `/cita` también con el ancho de un móvil (herramientas del navegador). Si un paso falla, la spec no está terminada (principio IV). **Cubre:** FR-001, FR-002, FR-003, FR-004, FR-004a, FR-005, FR-006, FR-007, FR-008, FR-009, FR-010, US1-1, US1-2, US1-3, US1-4, US1-5, US1-6, US1-7, US1-8, US1-9, US1-10, US2-1, US2-2, US2-3, US2-4, US3-1, SC-001, SC-002, SC-003, SC-004.
- [ ] T033 Ejecutar y dejar sin errores ni avisos: `cd backend && cargo test` (incluye `spec001_citas` y `matricula`), `cargo clippy --all-targets -- -D warnings`, `cd frontend && npm run check` y `npm test`. **Cubre:** las pruebas 1–12 y las unitarias de T001 y T023, es decir: FR-002 (pruebas 2, 3 y T001), FR-003 (pruebas 2 y 6), FR-004 (pruebas 1 y 5), FR-004a (prueba 4 y T023), FR-005 (prueba 5), FR-006 (prueba 11), FR-007 (pruebas 8 y 9), FR-008 (prueba 2), FR-009 (prueba 12), FR-010 (pruebas 7 y 10), SC-003 (prueba 9).
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
