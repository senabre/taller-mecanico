---

description: "Tareas de spec-002 · Recibir el vehículo y abrir la orden de reparación (HU-02)"
---

# Tasks: spec-002 · Recibir el vehículo y abrir la orden de reparación (HU-02)

**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md) (D1–D8),
[data-model.md](data-model.md), [contracts/api.md](contracts/api.md) y
[quickstart.md](quickstart.md), que remiten a los documentos comunes de
[000-base-comun](../000-base-comun/).

**Responsable**: Persona A · **Revisor**: Persona B

**Tests**: se incluyen. El plan maestro (research R13) exige pruebas `#[sqlx::test]` para cada regla
de negocio, y el plan de esta spec enumera 12 en `backend/tests/spec002_ordenes.rs` (aquí «prueba
1»…«prueba 12»). Cada prueba se escribe **antes** de la tarea que la hace pasar y debe fallar
hasta entonces.

**Trazabilidad**: cada tarea termina con «**Cubre:**», que indica los requisitos (FR-xxx), los
escenarios (US1-n, US2-n) y los criterios de éxito (SC-xxx) de la spec que atiende. Sirve para
construir la matriz de trazabilidad. **US1-6** («cliente sin cita») no tiene prueba en spec-002:
lo cumplen spec-001 FR-006 (su prueba 11) y el paso 1 del quickstart común §2, que comprueba T029
(hallazgo E2). La lista de órdenes (T016, T021, T025) no viene de un FR de spec-002, sino del menú
común y del quickstart §3 (hallazgo E1).

**Cambios compartidos**: las tareas marcadas «**cambio compartido · revisa Persona B**» tocan
ficheros de la base común o de la Persona B. Cada una va en **su propio commit**, sin mezclarse con
el resto de spec-002, y la Persona B la revisa antes de fusionar (plan maestro, ficheros
compartidos).

**Organización**: una fase por historia de usuario. Todo texto visible y todo mensaje de error,
en español de España, con los textos exactos de la tabla de validación del
[contrato común](../000-base-comun/contracts/api.md#spec-002--orden-de-reparación); km con
`formato::numero` («85.000 km»); fechas dd/mm/aaaa con `formato.ts`; «hoy» siempre con
`fechas::hoy()` (Europe/Madrid, R12). Las pruebas crean sus propios datos y no usan valores demo
(principio V).

## Format: `[ID] [P?] [Story] Description`

- **[P]**: se puede hacer en paralelo (otro fichero, sin depender de tareas pendientes)
- **[Story]**: historia de usuario (US1, US2)

## Path Conventions

Aplicación web: `backend/src/`, `backend/tests/`, `backend/seed/`, `frontend/src/`. No hay
migraciones nuevas: `orden` y `contador` ya existen en `0001_esquema_inicial.sql`.

---

## Phase 1: Setup (cambios compartidos)

**Purpose**: aplicar, cada uno en su commit, los cambios en ficheros compartidos que necesitan las
historias.

- [X] T001 **Cambio compartido · revisa Persona B · commit propio**. En `backend/seed/demo.sql`, añadir una **parte 5** al final, antes del `COMMIT`, sin tocar las partes 1–4 ni las matrículas del quickstart §1 (`1111BBB`, `2222CCC`, `9999LLL`, `1234BCD`, `5678CDF`) (research D8): (a) vehículo `1313MMM` (Seat León) de «Lucía Fernández» y vehículo `1414NNN` (Renault Mégane) de «Andrés Gil», sin órdenes, cada uno con una cita de **hoy** («Revisión general» y «Cambio de aceite»); (b) una cita de **hoy** para `8888KKK` («Revisión de frenos»), cuya orden 8 tiene 39.800 km; (c) vehículo `1515PPP` (Ford Fiesta) de «Jorge Martín» con una cita de **ayer** y sin orden; (d) usuario `Pedro` / `pedro`, rol `mecanico`, `activo = false`, con el mismo hash ficticio de los otros cinco. Las fechas, con `(now() AT TIME ZONE 'Europe/Madrid')::date` (y `- 1` para ayer), no con `current_date`. No crear órdenes ni tocar `contador`. Comprobar sobre una base recién migrada que el fichero carga sin errores, que `SELECT ultimo FROM contador WHERE nombre = 'orden'` sigue dando 9 y que `cargo test` sigue pasando (las pruebas no usan los datos demo). Añadir al comentario de cabecera de la parte 5 que los datos demo se cargan el mismo día en que se recorre el quickstart. **Además (hallazgo I3)**: corregir el comentario de la parte 3 (`-- Las citas son de días pasados para no gastar el cupo de los próximos días`) para que diga que las citas de las partes 1–4 son de días pasados y que la parte 5 añade citas de hoy y de ayer, que no gastan el cupo de los próximos días; y, en `specs/001-pedir-cita-en-linea/quickstart.md` («Antes de empezar»), cambiar «Las citas demo son de días pasados … los vehículos demo no tienen ninguna cita pendiente» por: las citas demo de días pasados no gastan el cupo; `1313MMM`, `1414NNN` y `8888KKK` tienen una cita de hoy (cuentan como pendientes) y el recorrido del §1 no las usa. **Cubre:** FR-001, FR-004, FR-005, FR-008, US1-4, US1-5, US1-6, US1-7, US2-1, US2-3, SC-001 (datos para verificarlos en el quickstart común §2).
  - *Hecho el 2026-11-03*. **Commit propio**: `backend/seed/demo.sql` y `specs/001-pedir-cita-en-linea/quickstart.md` (la parte de I3 que pide esta tarea). Base recreada con `cargo sqlx database reset -y` (migraciones 0001–0003) y `demo.sql` cargado con el `psql` del contenedor, sin errores. Comprobado con consultas (hoy en Madrid = 2026-11-03): citas de hoy de `1313MMM`, `1414NNN` y `8888KKK` sin orden; cita de ayer de `1515PPP` sin orden; `pedro` con rol `mecanico` y `activo = false`; los mecánicos activos (lo que da `GET /api/mecanicos`) son solo Luis y Marta; contador de órdenes en **9** y 9 órdenes (sin cambios); la orden 8 de `8888KKK` tiene 39.800 km. `cargo test` en verde. **Desviaciones**: (1) el hash de `pedro` se copia del de `luis` con `INSERT … SELECT`, en lugar de repetir el literal; es la misma contraseña ficticia (demo1234). (2) Motivo de la cita de `1515PPP`: «Ruido en la suspensión» (la tarea no lo fijaba). (3) La cabecera de la parte 5 lleva la tabla de qué paso del quickstart usa cada dato. (4) En la lista de ayer sale también `1111BBB`, cuya cita demo es de ayer y tiene la orden Nº 1; en el §2.1 se verá su «Orden Nº 1» como enlace, que confirma Q5 en otro día. (5) El texto nuevo del quickstart de spec-001 dice que `8888KKK` es el único vehículo de las órdenes 1–9 con cita pendiente.
- [X] T002 [P] **Cambio compartido · revisa Persona B · commit propio**. En `backend/src/rutas/imputaciones.rs` (de la Persona B, spec-005), hacer `pub` los campos `coste_acumulado`, `total_presupuestado` e `imputaciones` de `ResumenImputaciones`, sin cambiar nada más (ni el `Serialize` ni lo que devuelve `resumen_imputaciones`), para que la ficha de spec-002 copie esos tres campos al objeto `Orden` sin duplicar `estado` (research D6). `cargo test --test spec005_imputaciones` debe seguir pasando sin cambios. **Cubre:** US1-3, SC-004 (la ficha muestra lo imputado y lo presupuestado).
  - *Hecho el 2026-11-03*. **Commit propio**: solo `backend/src/rutas/imputaciones.rs`. `coste_acumulado`, `total_presupuestado` e `imputaciones` pasan a `pub`; `orden_id` y `estado` siguen privados, y el `Serialize` y lo que devuelve `resumen_imputaciones` no cambian. `cargo test --test spec005_imputaciones`: 12/12, sin cambios. **Desviación**: se añade al comentario del tipo por qué esos tres campos son públicos.
- [X] T003 [P] **Cambio compartido · revisa Persona B**. En `frontend/src/lib/tipos.ts`, añadir en una sección «spec-002 · Órdenes», sin tocar las demás: `PeticionOrden {cita_id: number, km: number | string, danos: string, mecanico_id: number, fecha_entrega_prometida: Fecha}` y `Orden` = `OrdenResumen` + `{cita: {fecha: Fecha, motivo: string}, vehiculo: {marca: string, modelo: string}, km: number, danos: string, diagnostico: string | null, total_presupuestado: Importe, coste_acumulado: Importe, presupuestos: PresupuestoResumen[], imputaciones: Linea[], pedidos: Pedido[], factura_id: number | null}`; y en `CitaDelDia` (spec-001), `orden_numero: number | null` junto a `orden_id`. Reutilizar los tipos que ya existen (`OrdenResumen`, `PresupuestoResumen`, `Linea`, `Pedido`). `npm run check`: 0 errores. **Cubre:** FR-008, US1-3, US1-4.
  - *Hecho el 2026-11-03*. **Commit propio** (cambio compartido): solo `frontend/src/lib/tipos.ts`. Sección «spec-002 · Órdenes» después de la de spec-001, sin tocar las demás; `orden_numero: number | null` en `CitaDelDia`. `npm run check`: 0 errores, 0 avisos. **Desviación**: `Orden` se declara como `interface Orden extends OrdenResumen { … }` en lugar de un tipo «=», con un comentario que dice qué spec da cada campo; los tipos de las secciones (`PresupuestoResumen`, `Linea`, `Pedido`) se reutilizan sin cambios.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: el módulo de rutas y la base de las pruebas, que necesitan las dos historias.

**⚠️ CRITICAL**: ninguna historia puede empezar hasta que esta fase esté terminada.

- [X] T004 Crear `backend/src/rutas/ordenes.rs` con `rutas()`, que registra `POST /ordenes`, `GET /ordenes` y `GET /ordenes/{id}`, los tres respondiendo provisionalmente 501. Registrarlo en `backend/src/rutas/mod.rs` con una sola línea `.merge(ordenes::rutas())` después de las que ya hay, sin tocar las demás (**cambio compartido · revisa Persona B**, en el mismo commit que el módulo). Comprobar que no hay rutas solapadas con `presupuestos.rs` e `imputaciones.rs` (`/ordenes/{id}/…`). **Cubre:** FR-001 (infraestructura).
  - *Hecho el 2026-11-03*. **Commit propio** (con revisión de la Persona B por `mod.rs`): `backend/src/rutas/ordenes.rs` (nuevo) y `backend/src/rutas/mod.rs`. Los tres endpoints responden 501 con `StatusCode::NOT_IMPLEMENTED`, como hizo spec-001 en su esqueleto (`ErrorApi` no tiene 501). No hay rutas solapadas: axum lo comprobaría al montar el router, y `cargo test` lo monta en cada prueba sin error; `presupuestos.rs` e `imputaciones.rs` usan `/ordenes/{id}/…` y spec-002 solo `/ordenes` y `/ordenes/{id}`. **Desviación**: en `mod.rs` son **dos** líneas, no una: además de `.merge(ordenes::rutas())`, la declaración `pub mod ordenes;`, sin la cual el módulo no existe. Las dos van después de las de `imputaciones`, sin tocar las demás.
- [X] T005 Crear `backend/tests/spec002_ordenes.rs` con `mod comun;` y utilidades propias: `cita(pool, matricula, fecha) -> (cita_id, vehiculo_id)` (cliente, vehículo y cita por SQL; para hoy, `fechas::hoy()`); `orden_previa(pool, cita_id, vehiculo_id, mecanico_id, km, numero)` (una orden anterior por SQL, para el km); `mecanico(pool, login, activo) -> i64`; `peticion(cita_id, mecanico_id) -> Value` (una petición válida: 85.000 km, «Rayón puerta trasera izquierda», entrega `hoy + 3`); `contador(pool) -> i64` (`SELECT ultimo FROM contador WHERE nombre = 'orden'`); y `ordenes(pool) -> i64`. Reutilizar `comun::app_prueba`, `estado_prueba`, `peticion`, `crear_usuario` y `entrar`. Las utilidades que aún no se usen llevan `#[allow(dead_code)]`, que se quita al usarlas. **Cubre:** infraestructura de pruebas, sin requisito propio.
  - *Hecho el 2026-11-03*: `backend/tests/spec002_ordenes.rs` con `mod comun;` y las seis utilidades, todas con `#[allow(dead_code)]` hasta que las usen las pruebas de las fases 3 y 4. Compila y `cargo test --test spec002_ordenes` da 0 pruebas, como corresponde. **Desviaciones**: (1) `cita` crea siempre un vehículo «Seat Ibiza» y un cliente genérico («Cliente de prueba»), porque las pruebas de spec-002 solo necesitan la matrícula; (2) `orden_previa` deja `abierta_en` por defecto (`now()`): cada consulta va en su propia transacción, así que las órdenes insertadas después quedan después, y la de km usa además `id` para desempatar; su entrega prometida es hoy; (3) la utilidad local se llama `peticion` y la de `comun` se usa siempre como `comun::peticion`, para que no se confundan.

**Checkpoint**: el backend compila con el módulo registrado y el fichero de pruebas existe.

---

## Phase 3: User Story 1 - Abrir la orden al recibir el coche (Priority: P1) 🎯 MVP

**Goal**: el día de la cita, recepción abre la orden desde `/citas`; la orden recibe su número
correlativo, queda en «recibida» con km, daños, mecánico y entrega, aparece en el tablero, y la
cita pasa a mostrar «Orden Nº N».

**Independent Test**: con una cita de hoy, abrir la orden y comprobar el número, los datos y el
estado; intentar abrirla otra vez desde otra pestaña (quickstart común §2, pasos 1–5 y 7).

### Tests for User Story 1 ⚠️

- [X] T006 [US1] **Prueba 9** (acceso y existencia): `POST /api/ordenes` sin sesión → 401; con `mecanico` y `almacen` → 403; con `recepcion` y con `jefe` → 201; con un `cita_id` que no existe → 404 «No existe la cita». **Cubre:** FR-001.
  - *Hecho el 2026-11-03*: escrita antes de la implementación junto con las otras diez pruebas de US1; las once fallaron en rojo (el endpoint respondía 501) y pasan tras T017–T022.
- [X] T007 [US1] **Prueba 1** (abrir y publicar el evento): con `let estado = comun::estado_prueba(pool.clone()); let mut tablero = estado.eventos.suscribir(); let app = taller::app(estado);` y una cita de hoy, `POST /api/ordenes` → 201 con el objeto `Orden`: `estado = "recibida"`, `numero` = contador anterior + 1, `km`, `danos`, `mecanico`, `fecha_entrega_prometida`, `matricula` y `vehiculo` de la cita; en la tabla, `vehiculo_id` es el de la cita aunque la petición no lo envíe. `tablero.try_recv()` devuelve `CambioEstado { id, estado: Estado::Recibida }` con el `id` de la orden. Después de un 422, `tablero.try_recv()` está vacío: si la transacción no se confirma, no se publica nada. **Cubre:** FR-005, FR-006, FR-007, US1-1 (y research D5, hallazgo I4 de la base común).
  - *Hecho el 2026-11-03*: escrita antes de la implementación junto con las otras diez pruebas de US1; las once fallaron en rojo (el endpoint respondía 501) y pasan tras T017–T022. El suscriptor se toma antes de montar la app (`estado.eventos.suscribir()`), y se comprueba que un 422 no publica nada.
- [X] T008 [US1] **Prueba 2** (correlativo sin huecos): con el contador en 41 (`UPDATE contador`), provocar un 422 de cada campo de la tabla de validación (km, daños, mecánico, fecha de entrega) y los dos 409 (cita de mañana; cita que ya tiene orden), y después abrir una orden válida → recibe la **42**; el contador avanza exactamente 1 en total. A continuación, abrir la orden de otra cita de hoy y repetir el `POST` de esa cita (segunda pestaña, 409) → la nueva orden es la **43** y el contador queda en 43 (escenario 9 de la US1). **Cubre:** FR-002, US1-2, US1-9, SC-002, caso límite «no se consume ningún número».
  - *Hecho el 2026-11-03*: escrita antes de la implementación junto con las otras diez pruebas de US1; las once fallaron en rojo (el endpoint respondía 501) y pasan tras T017–T022. **Desviación**: la orden que ya existe para el 409 «ya tiene orden» se crea con `orden_previa` y el número 1000, que no toca el contador.
- [X] T009 [US1] **Prueba 3** (concurrencia del número): diez citas de hoy distintas y diez `POST` a la vez (`futures_util::future::join_all`) → diez 201 con los números `n+1 … n+10`, sin huecos ni repetidos, y el contador en `n+10`. **Cubre:** FR-002, caso límite «dos recepcionistas a la vez» (la apertura simultánea; SC-002 se comprueba con T008 y T029, Q6).
  - *Hecho el 2026-11-03*: escrita antes de la implementación junto con las otras diez pruebas de US1; las once fallaron en rojo (el endpoint respondía 501) y pasan tras T017–T022.
- [X] T010 [US1] **Prueba 4** (concurrencia de la misma cita): con una cita de hoy, lanzar a la vez con `tokio::join!` dos `POST` para esa cita → un 201 y un 409 «Esta cita ya tiene una orden (Nº N)», con `detalle.orden_id` y `detalle.numero` iguales a los de la orden creada; exactamente 1 orden de esa cita y el contador avanzado en 1. Repetir 20 veces, con **una cita de hoy nueva en cada vuelta** (sin borrar órdenes ni tocar el contador a mano), y comprobar en cada vuelta que el contador avanza exactamente 1. **Comprobación obligatoria**: una vez hecha T017, quitar temporalmente el `FOR UPDATE` y comprobar que esta prueba **falla** (se espera `[201, 500]`, porque el segundo `INSERT` choca con el `UNIQUE (cita_id)`); restaurar el código, comprobarlo con `cmp` y anotar el resultado en esta tarea. **Cubre:** FR-008, FR-002, US1-4 (Q5).
  - *Hecho el 2026-11-03*: escrita antes de la implementación junto con las otras diez pruebas de US1; las once fallaron en rojo (el endpoint respondía 501) y pasan tras T017–T022. **Comprobación obligatoria hecha**: quitando temporalmente el `SELECT … FOR UPDATE`, la prueba **falla en la vuelta 2** con `[201, 500]` («Ha ocurrido un error interno»: el segundo `INSERT` choca con el `UNIQUE (cita_id)`); las vueltas 0 y 1 pasaron por azar, lo que confirma que la carrera es intermitente y que hacen falta las 20 vueltas. Código restaurado, comprobado idéntico con `cmp`, y la prueba vuelve a pasar.
- [X] T011 [US1] **Prueba 5** (día de la cita): con una cita de **mañana** y otra de **ayer** sin orden, `POST` con datos válidos → 409 «Solo se puede abrir la orden el día de la cita» en las dos, sin crear órdenes y sin mover el contador. Con datos válidos **y** un km negativo, la cita de mañana da el 422 del km, no el 409 (todos los 422 antes que cualquier 409). **Cubre:** FR-001, US1-5 (Q2).
  - *Hecho el 2026-11-03*: escrita antes de la implementación junto con las otras diez pruebas de US1; las once fallaron en rojo (el endpoint respondía 501) y pasan tras T017–T022.
- [X] T012 [US1] **Prueba 7** (campos de la tabla de validación): con una cita de hoy, `km: "85000"` (texto con cifras) → 201; `km` ausente, `null`, `-1`, `1.5`, `""` y `"abc"` → 422 «El kilometraje debe ser un número entero igual o mayor que 0» (`detalle.campo = "km"`); `danos` ausente, `""` y `"   "` → 422 «Describe los daños visibles o escribe “Sin daños visibles”» (`danos`); `mecanico_id` ausente → 422 «Elige un mecánico de la lista»; sin `cita_id` → 404 «No existe la cita»; `fecha_entrega_prometida` ausente o `"ayer"` → 422 «Indica la fecha de entrega prometida»; ayer → 422 «La fecha de entrega prometida no puede ser anterior a hoy» (`fecha_entrega_prometida`); hoy → 201; `danos` `"  Rayón  "` → se guarda «Rayón»; «Sin daños visibles» → 201. Con varios errores a la vez, el primero según el orden de la tabla. En ningún 422 se crea orden ni se mueve el contador. **Cubre:** FR-003, FR-006, US1-8 (Q4), casos límite de km y de entrega.
  - *Hecho el 2026-11-03*: escrita antes de la implementación junto con las otras diez pruebas de US1; las once fallaron en rojo (el endpoint respondía 501) y pasan tras T017–T022. **Desviación**: se añade el caso `km: ""` (texto vacío), que la tabla de validación también rechaza.
- [X] T013 [US1] **Prueba 8** (mecánico): `mecanico_id` de un mecánico **de baja**, de un usuario con rol `recepcion`, del `jefe` y un id que no existe → 422 «Elige un mecánico de la lista» (`detalle.campo = "mecanico_id"`); con un mecánico activo → 201. **Cubre:** FR-005, US1-7 (Q3).
  - *Hecho el 2026-11-03*: escrita antes de la implementación junto con las otras diez pruebas de US1; las once fallaron en rojo (el endpoint respondía 501) y pasan tras T017–T022.
- [X] T014 [US1] **Prueba 10** (ficha): tras abrir una orden, `GET /api/ordenes/{id}` con `recepcion`, `mecanico`, `almacen` y `jefe` → 200 con `id`, `numero`, `matricula`, `estado`, `mecanico: {id, nombre}`, `fecha_entrega_prometida`, `retrasada: false`, `cita: {fecha, motivo}`, `vehiculo: {marca, modelo}`, `km`, `danos`, `diagnostico: null`, `presupuestos: []`, `imputaciones: []`, `pedidos: []`, `total_presupuestado: "0.00"`, `coste_acumulado: "0.00"` y `factura_id: null`; ningún campo de datos del cliente; sin sesión → 401; id inexistente → 404 «No existe la orden». **Cubre:** FR-005, US1-3, SC-004.
  - *Hecho el 2026-11-03*: escrita antes de la implementación junto con las otras diez pruebas de US1; las once fallaron en rojo (el endpoint respondía 501) y pasan tras T017–T022. **Desviación**: compara el objeto entero con el esperado, en vez de campo a campo, y comprueba además que la respuesta 201 del `POST` es la misma ficha; así ningún campo de datos del cliente puede colarse.
- [X] T015 [US1] **Prueba 11** (lista de citas): tras abrir la orden de una cita de hoy, `GET /api/citas?fecha=<hoy>` como recepción → esa cita trae `orden_id` y `orden_numero` de la orden; otra cita sin orden trae los dos a `null`. **Cubre:** FR-008, US1-4 (Q5).
  - *Hecho el 2026-11-03*: escrita antes de la implementación junto con las otras diez pruebas de US1; las once fallaron en rojo (el endpoint respondía 501) y pasan tras T017–T022. **Desviación**: en la primera ejecución en rojo la prueba **pasó sin comprobar nada**: con el 501 la apertura no devolvía orden y tanto `orden["numero"]` como el `orden_numero` inexistente valían `null`. Se reforzó (exige el 201 de la apertura y que `orden_numero` sea un número, o `null` presente) y entonces falló en rojo como las demás.
- [X] T016 [US1] **Prueba 12** (lista de órdenes): con órdenes de dos mecánicos, `GET /api/ordenes` como recepción → todas, de la más reciente a la más antigua, con los campos de `OrdenResumen`; `GET /api/ordenes?mias=true` como `luis` → solo las suyas; `almacen` → 403; `retrasada` es `true` con la entrega de ayer y `false` con la de hoy (data-model común: «estado ≠ `facturada` y `fecha_entrega_prometida` < hoy»). **Justificación (hallazgo E1)**: la lista de órdenes no sale de un FR de spec-002; la exigen el menú común (escenario 1 de la US1 de la base común, «Mis órdenes»; quickstart §0.4, «Órdenes»; el jefe, por FR-002 común) y el quickstart §3 («Como `luis`, abrir la orden…»), y se hace aquí porque spec-002 es la que crea las órdenes. **Cubre:** menú común (escenario 1 de la US1 de la base común), quickstart §3; y lo que usa spec-008.
  - *Hecho el 2026-11-03*: escrita antes de la implementación junto con las otras diez pruebas de US1; las once fallaron en rojo (el endpoint respondía 501) y pasan tras T017–T022. No se prueba la lista de un mecánico sin `mias`, que devuelve todas las órdenes, como dice el contrato.

### Implementation for User Story 1

- [X] T017 [US1] En `backend/src/rutas/ordenes.rs`, implementar `POST /api/ordenes` (`Usuario` + `exigir_rol(&[Rol::Recepcion])`; el jefe pasa) en el orden de research D2 y de la tabla de validación del contrato común. Los campos de la petición se leen como valores JSON libres (`Option<serde_json::Value>`), para que un tipo incorrecto dé su 422 y no el 400 de «JSON mal formado»; `km` se acepta como número o como texto con cifras (contrato común): (1) la cita existe, si no (o si falta `cita_id`) 404 «No existe la cita»; (2) **sin transacción**, los campos en el orden de la tabla, con `error_campo` como en `citas.rs`: `km` «número entero de 0 a 2.147.483.647» (data-model común: «≥ 0»), `danos` «obligatorio, no vacío tras quitar espacios» (con `trim`, se guarda sin los espacios del principio y del final), `mecanico_id` «el usuario debe tener rol `mecanico` y estar activo al abrir la orden» (la misma condición que `GET /api/mecanicos`), `fecha_entrega_prometida` presente y «≥ fecha de apertura» (`fechas::hoy()`); (3) `cita.fecha == fechas::hoy()`, si no 409 «Solo se puede abrir la orden el día de la cita»; (4) transacción: `SELECT … FROM cita WHERE id = $1 FOR UPDATE` y, si ya hay una orden con ese `cita_id`, 409 «Esta cita ya tiene una orden (Nº N)» con `detalle: {orden_id, numero}`; (5) **solo entonces** `contador::siguiente(&mut tx, contador::ORDEN)` e `INSERT` en `orden` con el `vehiculo_id` de la cita y `estado = 'recibida'`; `commit`; responder 201. **Hasta que exista T019**, la respuesta es `{id, numero, estado}`; T019 la sustituye por el objeto `Orden` completo, y la parte de la respuesta de T007 pasa con T019 (hallazgo O1). Deja el hueco del km anterior (T029) al final del paso 2. Hace pasar T006, T008, T009, T010, T011, T012 y T013. **Cubre:** FR-001, FR-002, FR-003, FR-005, FR-006, FR-007, FR-008, US1-1, US1-2, US1-4, US1-5, US1-7, US1-8.
  - *Hecho el 2026-11-03*: `POST /api/ordenes` en `backend/src/rutas/ordenes.rs`, en el orden de research D2: 404 de la cita → 422 de km, daños, mecánico y fecha (sin transacción) → 409 del día → transacción con `SELECT id FROM cita WHERE id = $1 FOR UPDATE` → 409 si ya tiene orden → **solo entonces** `contador::siguiente` → `INSERT` → `commit`. El hueco del km anterior (T028) queda señalado con un comentario al final del paso 2. **Desviaciones**: (1) el 409 «Esta cita ya tiene una orden (Nº N)» con `detalle: {orden_id, numero}` se construye a mano con la misma forma `{error, detalle}`, porque `ErrorApi::Conflicto` no admite `detalle` y `error.rs` es compartido; (2) T017 y T019 se hicieron a la vez, así que el 201 devuelve el objeto `Orden` completo desde el principio y la respuesta provisional `{id, numero, estado}` de O1 no llegó a hacer falta.
- [X] T018 [US1] En `POST /api/ordenes` de `backend/src/rutas/ordenes.rs`, **después del `commit`** (nunca antes ni dentro de la transacción): `CambioEstado { id, estado: Estado::Recibida }.publicar(&estado_app.eventos)` (research D5). Si algo falla antes, no se publica nada. Hace pasar la parte del evento de T007. **Cubre:** FR-007 (el tablero de spec-008 ve la orden nueva; hallazgo I4 de la base común).
  - *Hecho el 2026-11-03*: `CambioEstado { id, estado: Estado::Recibida }.publicar(&estado.eventos)` justo después de `tx.commit()`; los errores salen antes, así que no publican nada (comprobado en T007).
- [X] T019 [US1] En `backend/src/rutas/ordenes.rs`, implementar `GET /api/ordenes/{id}` (todos los roles con sesión; 404 «No existe la orden») con la composición de research D6: una consulta propia para los campos de `OrdenResumen` (`retrasada` = estado ≠ `facturada` y `fecha_entrega_prometida < fechas::hoy()`), `cita: {fecha, motivo}`, `vehiculo: {marca, modelo}`, `km`, `danos`, `diagnostico` y `factura_id` (`SELECT id FROM factura WHERE orden_id = …`); y **una llamada por sección de B**: `presupuestos::presupuestos_de_orden(&mut conn, id, config.iva_porcentaje)`, `imputaciones::resumen_imputaciones(&mut conn, id)` (de la que se copian `total_presupuestado`, `coste_acumulado` e `imputaciones`, T002) y `almacen::pedidos_de_orden(&mut conn, id)`. No modificar las funciones de B. Definir `OrdenResumen` en este módulo como `pub`, para que la reutilicen `GET /api/ordenes` y spec-008. La misma función construye la respuesta 201 de T017. Hace pasar T014 y la respuesta de T007. **Cubre:** FR-005, US1-1, US1-3, SC-004.
  - *Hecho el 2026-11-03*: `pub async fn orden_completa(conexion, id, iva_porcentaje)` con una consulta propia (campos de `OrdenResumen`, cita, vehículo, km, daños, diagnóstico y `factura_id` por subconsulta) y una llamada por sección de B: `presupuestos_de_orden`, `resumen_imputaciones` (de la que se copian los tres campos públicos de T002) y `pedidos_de_orden`, sin modificarlas. `GET /api/ordenes/{id}` con cualquier sesión; 404 «No existe la orden». **Desviaciones**: `OrdenResumen` y `MecanicoDeOrden` son `pub`, y se expone `pub async fn resumenes(conexion, mecanico_id)` para que la reutilicen la lista (T021) y el tablero de spec-008; `Orden` usa `#[serde(flatten)]` para los campos de `OrdenResumen`.
- [X] T020 [US1] **Cambio compartido · revisa Persona B · commit propio**. En `backend/tests/spec006_pedidos.rs` (de la Persona B), quitar el `#[ignore = "necesita GET /api/ordenes/{id} de spec-002"]` de `la_ficha_de_la_orden_trae_sus_pedidos`, sin cambiar la prueba. Debe pasar con T019; si no pasa, no se toca la prueba: se avisa a la Persona B. **Cubre:** US1-3, SC-004 (la ficha trae los pedidos de spec-006).
  - *Hecho el 2026-11-03*. **Commit propio** (cambio compartido, revisa Persona B): solo `backend/tests/spec006_pedidos.rs`, quitando la línea `#[ignore = …]`. La prueba pasa sin cambios y `spec006_pedidos` queda en 23/23, sin ignoradas. **Observación para B**: el comentario de la prueba aún dice «quitar el `#[ignore]` cuando exista (T031)»; no se ha cambiado porque la tarea pide no tocar la prueba.
- [X] T021 [US1] En `backend/src/rutas/ordenes.rs`, implementar `GET /api/ordenes?mias=true` (`exigir_rol(&[Rol::Recepcion, Rol::Mecanico])`; el jefe pasa): `[OrdenResumen]` de la más reciente a la más antigua; con `mias=true`, solo las órdenes con `mecanico_id` = usuario que ha entrado. Reutiliza la consulta de `OrdenResumen` de T019. Hace pasar T016. **Justificación (hallazgo E1)**: la lista de órdenes no sale de un FR de spec-002; la exigen el menú común (escenario 1 de la US1 de la base común, «Mis órdenes»; quickstart §0.4, «Órdenes»; el jefe, por FR-002 común) y el quickstart §3 («Como `luis`, abrir la orden…»), y se hace aquí porque spec-002 es la que crea las órdenes. **Cubre:** menú común (escenario 1 de la US1 de la base común), quickstart §3.
  - *Hecho el 2026-11-03*: `GET /api/ordenes` con `exigir_rol(&[Rol::Recepcion, Rol::Mecanico])`; `mias=true` filtra por el mecánico que ha entrado. Usa `resumenes` de T019 (de la más reciente a la más antigua, por `abierta_en` y después `id`).
- [X] T022 [US1] En `backend/src/rutas/citas.rs` (spec-001, Persona A), añadir `orden_numero` a `GET /api/citas`: `o.numero AS orden_numero` en el `LEFT JOIN orden` que ya existe, y el campo en `FilaCita` y `CitaDelDia`. Hace pasar T015; `cargo test --test spec001_citas` debe seguir pasando (15/15). **Además (hallazgo I3)**: en `specs/001-pedir-cita-en-linea/contracts/api.md`, sustituir «`orden_id` no se muestra como dato. Lo usa spec-002 para el botón «Abrir orden».» por: `orden_id` y `orden_numero` los añade spec-002 para la columna «Orden» («Orden Nº N» o «Abrir orden»), y remitir al contrato común. **Cubre:** FR-008, US1-4 (Q5).
  - *Hecho el 2026-11-03*: `o.numero AS orden_numero` en el `LEFT JOIN orden` de `GET /api/citas` y el campo en `FilaCita` y `CitaDelDia` de `backend/src/rutas/citas.rs`; `spec001_citas` sigue en 15/15. **Hallazgo I3**: actualizado `specs/001-pedir-cita-en-linea/contracts/api.md` (la nota de `orden_id` dice ahora que spec-002 añade `orden_id` y `orden_numero` para la columna «Orden»).
- [X] T023 [P] [US1] En `frontend/src/routes/citas/+page.svelte` (spec-001), añadir una columna «Orden»: si la cita tiene `orden_id`, un enlace «Orden Nº {orden_numero}» a `/ordenes/{orden_id}`; si no la tiene **y** el día elegido es hoy (`dia === hoy`, que la página ya calcula), un botón «Abrir orden» que lleva a `/ordenes/nueva?cita={id}`; en otro caso, la celda vacía. Sustituye el comentario de hueco que dejó spec-001. **Además (hallazgo I3)**: en la tabla «`/citas`» de `specs/001-pedir-cita-en-linea/contracts/api.md`, añadir que la lista tiene una columna «Orden» que pone spec-002 (FR-008 de spec-002). **Cubre:** FR-001, FR-008, US1-4, US1-5 (Q2, Q5).
  - *Hecho el 2026-11-04. Verificado con navegador automatizado ([informe](evidencias/T029/verificacion.md), 23/23; filas 1–3 y 11)*. `npm run check` y `npm test` correctos. **Hallazgo I3**: añadida a la tabla «`/citas`» de `specs/001-pedir-cita-en-linea/contracts/api.md` la fila de la columna «Orden». **Desviación**: «Abrir orden» es un enlace con aspecto de botón (`<a class="btn">`), no un `<button>`, porque solo navega.
- [X] T024 [P] [US1] Crear `frontend/src/routes/ordenes/nueva/+page.svelte` (recepción y jefe; título «Abrir orden»), con `novalidate`. Al abrir, `GET /api/citas?fecha=<hoy en Madrid>` y `GET /api/mecanicos`. Si la cita de `?cita=` no está en la lista de hoy, mostrar solo «Solo se puede abrir la orden el día de la cita». Si está, mostrar su matrícula, marca, modelo y motivo, y los campos Kilómetros (`inputmode="numeric"`), Daños visibles (textarea), Mecánico (selector solo con la lista de `GET /api/mecanicos`) y Fecha de entrega prometida (`type="date"`). Al guardar, `POST /api/ordenes`: 201 → `goto('/ordenes/{id}')`; 422 → mensaje bajo el campo de `detalle.campo`; 409 con `detalle.orden_id` → el mensaje y un enlace «Ver la orden» a `/ordenes/{detalle.orden_id}`; otro 409 → error general. **Cubre:** FR-001, FR-003, FR-005, FR-006, FR-008, US1-1, US1-4, US1-7, US1-8, SC-001.
  - *Hecho el 2026-11-04. Verificado con navegador automatizado ([informe](evidencias/T029/verificacion.md), 23/23; filas 4–8, 12 y 13)*. **Desviaciones**: (1) la cita se busca con `GET /api/citas` **sin `fecha`**, no con `?fecha=<hoy en Madrid>`: el servidor da las de hoy con `fechas::hoy()`, y así no se copia el cálculo de «hoy» de `/citas` a otra pantalla; `contracts/api.md` de 002 y research D7 se han alineado con esta llamada, que es correcta porque solo se abren citas de hoy. (2) Kilómetros es un campo de texto con `inputmode="numeric"`, no `type="number"`, para que el texto llegue tal cual al servidor y los mensajes sean los suyos; por eso `km` va como texto. (3) `peticion` se convierte con `as PeticionOrden` porque el mecánico y la fecha pueden ir vacíos y es el servidor quien responde el 422. (4) Si se abre la URL de una cita que ya tiene orden, se muestra el formulario y el 409 sale al guardar, igual que en una segunda pestaña. (5) **Textos propios que no están en el contrato de 002**, que dice que los únicos son «Abrir orden» y el mensaje de otro día: «No se pudo cargar la cita», «Ha ocurrido un error inesperado», «Elige un mecánico», «Ver la orden», «Guardando…» y el rótulo «Motivo:». El contrato de 002 ya los recoge, con la misma definición de «etiquetas» que el de spec-001 (T040).
- [X] T025 [P] [US1] Crear `frontend/src/routes/ordenes/+page.svelte`: con rol `mecanico`, `GET /api/ordenes?mias=true` y título «Mis órdenes»; con recepción o jefe, `GET /api/ordenes` y título «Órdenes». Tabla con Número (enlace a `/ordenes/{id}`), Matrícula, Estado (`NOMBRE_ESTADO` de `formato.ts`), Mecánico y Entrega prometida (dd/mm/aaaa, con «Retrasada» si `retrasada`); con la lista vacía, «No hay órdenes». **Justificación (hallazgo E1)**: la lista de órdenes no sale de un FR de spec-002; la exigen el menú común (escenario 1 de la US1 de la base común, «Mis órdenes»; quickstart §0.4, «Órdenes»; el jefe, por FR-002 común) y el quickstart §3 («Como `luis`, abrir la orden…»), y se hace aquí porque spec-002 es la que crea las órdenes. **Cubre:** menú común (escenario 1 de la US1 de la base común), quickstart §0.1, §0.4 y §3.
  - *Hecho el 2026-11-04. Verificado con navegador automatizado ([informe](evidencias/T029/verificacion.md), 23/23; filas 14–16)*. **Desviaciones**: el rol se lee de `page.data.yo` (`$app/state`), que da el layout; con error de carga se muestra «No se pudo cargar la lista de órdenes». «Retrasada» se muestra en rojo junto a la fecha.
- [X] T026 [US1] **Cambio compartido · revisa Persona B**. Crear `frontend/src/routes/ordenes/[id]/+page.svelte` (contenedor de secciones, plan maestro) y `frontend/src/routes/ordenes/[id]/Datos.svelte`. El contenedor carga `GET /api/ordenes/{id}` una vez (404 → «No existe la orden») y pinta, en este orden: `Datos` (número, estado, matrícula, marca, modelo, fecha dd/mm/aaaa y motivo de la cita, km con separador de miles «85.000 km», daños, mecánico, entrega prometida y «Retrasada» si lo está); `Diagnostico` (`orden`, `puedeEditar`, `onguardado`); `Presupuesto` (`orden`, `presupuestos`, `puedeEditar`, `onCambio`); `Imputaciones` (`resumen` = `{orden_id: orden.id, estado, coste_acumulado, total_presupuestado, imputaciones}`, `puedeEditar`, `onCambio`); y `Pedidos` (`pedidos`). `puedeEditar` = el usuario es el mecánico asignado o el jefe (para las secciones del mecánico). El contenedor tiene también el rol del usuario (`yo` del layout) y lo pasará a la sección «Factura» de spec-007 (Persona B), que se activa para **recepción** y el jefe (hallazgo I4); spec-007 la añade con una línea, con la revisión de la Persona A. `onCambio` y `onguardado` vuelven a pedir la orden. No modificar los componentes de B; añadir una sección nueva tiene que ser una línea. **Cubre:** US1-3, SC-004.
  - *Hecho el 2026-11-04. Verificado con navegador automatizado ([informe](evidencias/T029/verificacion.md), 23/23; filas 9 y 17–23)*. **Cambio compartido · revisa Persona B**. **Commit**: `frontend/src/routes/ordenes/[id]/+page.svelte` y `Datos.svelte`. Los componentes de B no se han tocado: `Diagnostico` (`orden`, `puedeEditar`, `onguardado`), `Presupuesto` (`orden`, `presupuestos`, `puedeEditar`, `onCambio`), `Imputaciones` (`resumen`, `puedeEditar`, `onCambio`) y `Pedidos` (`pedidos`). **Desviaciones**: (1) «Factura» aún no existe: el contenedor tiene `yo` (con su rol) y un comentario que dice que spec-007 la añade con una línea y la activa para recepción y el jefe (I4). (2) El número de la orden va en el título de la página («Orden Nº N»), no en la sección `Datos`. (3) Las secciones de B se muestran a todos los roles; cada una decide con `puedeEditar` y el estado qué se puede hacer.

**Checkpoint**: las pruebas 1–5 y 7–12 pasan, también la de spec-006; desde `/citas` se abre una
orden y se ve su ficha.

---

## Phase 4: User Story 2 - Control del kilometraje (Priority: P1)

**Goal**: un kilometraje menor que el de la visita anterior del vehículo se rechaza mostrando el
valor previo.

**Independent Test**: con una visita anterior de 85.000 km, intentar abrir la orden con 80.000 km y
comprobar el rechazo con «85.000 km» (quickstart común §2, paso 6, con 39.800 km de `8888KKK`).

### Tests for User Story 2 ⚠️

- [X] T027 [US2] **Prueba 6** (kilometraje): con dos órdenes anteriores del mismo vehículo (70.000 km y después 85.000 km, por SQL con `abierta_en` distinto) y una cita de hoy, `km: 80000` → 422 «El kilometraje no puede ser menor que el de la visita anterior (85.000 km)» con `detalle: {campo: "km", km_anterior: 85000}`, sin crear orden ni mover el contador; `km: 85000` → 201. Un vehículo sin órdenes con `km: 0` → 201. Una segunda pestaña: abierta la orden de una cita con 85.000 km, un segundo `POST` para **la misma cita** con 80.000 km → 409 «Esta cita ya tiene una orden (Nº N)», no el 422 del km (research D3). Con la cita de mañana y un km menor → el 422 del km, no el 409 del día. **Cubre:** FR-004, US2-1, US2-2, US2-3, SC-003.
  - *Hecho el 2026-11-04*: falló en rojo (80.000 km daba 201 en lugar de 422) y pasa con T028. **Desviaciones**: (1) el caso de la segunda pestaña usa un vehículo **sin** otras visitas: si el vehículo tiene una visita anterior de más km, el 422 es correcto aunque la cita ya tenga orden, porque los 422 van antes que los 409. (2) Casos que la tarea no nombra: con la visita anterior de 90.000 km y la más reciente de 85.000 km, 86.000 km → 201 (la visita anterior es la más reciente, no la de más km; con 70.000 y 85.000 no se distinguirían), y con un km menor y además los daños vacíos sale el error de daños (el km anterior es la última fila de la tabla). (3) Utilidades nuevas: `otra_cita`, para dar varias citas a un vehículo, y `con_km`. (4) Las órdenes previas usan números altos (1001–1004) para no chocar con los del contador.

### Implementation for User Story 2

- [X] T028 [US2] En `POST /api/ordenes` de `backend/src/rutas/ordenes.rs`, rellenar el hueco que deja T017 al final del paso 2 (última fila de la tabla de validación, antes de cualquier 409): `SELECT km FROM orden WHERE vehiculo_id = $1 AND cita_id <> $2 ORDER BY abierta_en DESC, id DESC LIMIT 1` (data-model común: «≥ km de la visita anterior: la orden más reciente del mismo vehículo para otra cita»); si existe y el km nuevo es menor → 422 con el texto del contrato, el km escrito con `formato::numero` («85.000») y `detalle: {campo: "km", km_anterior}`. Sin órdenes anteriores no se comprueba nada más. El formulario de T024 ya lo muestra bajo Kilómetros. Hace pasar T027. **Cubre:** FR-004, US2-1, US2-2, US2-3, SC-003.
  - *Hecho el 2026-11-04*: en `POST /api/ordenes` de `backend/src/rutas/ordenes.rs`, en el hueco de T017: última comprobación de los 422, sin transacción y antes del 409 del día (research D2 y D3). `formato::numero` da «85.000». Pasa T027 y `spec002_ordenes` queda en 12/12. **Comprobación adicional**: quitando temporalmente `cita_id <> $2`, la prueba falla (422 en lugar de 409 en la segunda pestaña), así que la detecta; código restaurado y comprobado idéntico con `cmp`. **Sin desviaciones.** El formulario de T024 lo muestra bajo Kilómetros por `detalle.campo`; falta verlo en el navegador (paso 6 del quickstart común §2, en T029).

**Checkpoint**: las 12 pruebas del plan pasan; las dos historias funcionan juntas.

---

## Phase 5: Polish & Cross-Cutting Concerns

- [X] T029 **Verificación en el navegador** del [quickstart común §2](../000-base-comun/quickstart.md), pasos 1–7, con la base de datos recién creada y cargada **ese mismo día** con `backend/seed/demo.sql`, que ya incluye la parte 5 de T001 (citas de hoy de `1313MMM`, `1414NNN` y `8888KKK`, cita de ayer de `1515PPP` y el mecánico de baja `pedro`), después de recorrer el §1. En el paso 4, **cronometrar** desde la lista de citas hasta ver la ficha (< 2 min, SC-001) y anotar el tiempo. Para cada paso, anotar si se cumple usando la correspondencia de [quickstart.md](quickstart.md); guardar el informe y las capturas en `specs/002-abrir-orden-reparacion/evidencias/T029/`. Probar `/ordenes/nueva` también con el ancho de un móvil. Si un paso falla, la spec no está terminada (principio IV): se registra el hallazgo y una tarea correctora. **Cubre:** FR-001, FR-002, FR-003, FR-004, FR-005, FR-006, FR-007, FR-008, US1-1, US1-2, US1-3, US1-4, US1-5, US1-6 (paso 1; ver E2), US1-7, US1-8, US1-9 (pasos 3–5), US2-1, US2-3, SC-001, SC-002 (pasos 3–6, Q6), SC-003, SC-004.
  - *Hecho el 2026-11-04 con **verificación automatizada***: un navegador automático (Chromium sin interfaz, Playwright 1.56) siguió los pasos como una persona, con la base de datos recién cargada con `demo.sql` ese mismo día. Informe, capturas y guiones reproducibles (`t029.mjs`, `t029b.mjs`) en [informe](evidencias/T029/verificacion.md). **US1: 23/23** (pasos 1–5 y 7 y la ficha compartida). **US2 y móvil: 3/3** (paso 6: 30.000 km en `8888KKK` da «…(39.800 km)» bajo Kilómetros; corregido a 40.000 km se guarda como Nº 10, sin que el rechazo gaste número; `/ordenes/nueva` a 390 px sin desplazamiento horizontal). **Regresión de US1 con T028: 23/23.** **Pendiente, solo esto**: (1) **el cronometraje humano de SC-001** (menos de 2 minutos desde la lista de citas hasta la ficha): los tiempos medidos son de máquina y no valen; hasta entonces, el principio IV no está cumplido del todo para SC-001; (2) **comprobar en un navegador en español que el campo de entrega sale en dd/mm/aaaa** (principio II): el navegador automático va en inglés y lo muestra como mm/dd/aaaa. **Desviaciones**: (a) la cita de `1234BCD` del paso 1 se creó con `POST /api/publico/citas`, sin recorrer el §1 en la pantalla; (b) guardar `8888KKK` con 40.000 km no está en el §2.6 y se hizo en una base de datos aparte, porque crearía una orden más y cambiaría la numeración del recorrido; (c) los guiones se ejecutaron con parte del código aún sin commit, como indica el informe.
- [X] T030 Ejecutar y dejar sin errores ni avisos: `cd backend && cargo test` (incluye `spec002_ordenes`, `spec001_citas` con `orden_numero` y `spec006_pedidos` sin ninguna prueba ignorada), `cargo clippy --all-targets -- -D warnings`, `cd frontend && npm run check` y `npm test`. Anotar los resultados en esta tarea. **Cubre:** las pruebas 1–12, es decir: FR-001 (pruebas 5 y 9), FR-002 (2, 3 y 4), FR-003 (7), FR-004 (6), FR-005 (1, 8 y 10), FR-006 (1 y 7), FR-007 (1), FR-008 (4 y 11), SC-002 (2; la apertura simultánea, 3), SC-003 (6), SC-004 (10 y 12).
  - *Hecho el 2026-11-04, sobre el commit `c9d868e` más T027 y T028 sin commit*. Las cuatro comprobaciones terminan con código 0, **sin errores ni avisos**. (1) `cargo test`: 118 pruebas, 0 avisos de compilación y **ninguna ignorada**: unitarias 15, `spec000_contador` 3, `spec000_estados` 6, `spec000_sesion` 12, `spec001_citas` **15** (con `orden_numero`), `spec002_ordenes` **12/12**, `spec003_presupuestos` 20, `spec005_imputaciones` 12 y `spec006_pedidos` **23** (con `la_ficha_de_la_orden_trae_sus_pedidos` activa). (2) `cargo clippy --all-targets -- -D warnings`: sin avisos; se forzó la revisión tocando `src/lib.rs`. (3) `npm run check`: 229 ficheros, 0 errores y 0 avisos. (4) `npm test`: 6/6. Sin desviaciones.
- [ ] T031 **Revisión de la Persona B** de todo lo compartido antes de fusionar. **Código**: los commits de T001 (`backend/seed/demo.sql`, parte 5), T002 (`imputaciones.rs`), T020 (`spec006_pedidos.rs`), T003 (`tipos.ts`), T004 (`rutas/mod.rs`) y T026 (`ordenes/[id]/+page.svelte`). **Documentos comunes** que cambió el plan de spec-002: `specs/000-base-comun/contracts/api.md` (sección spec-002 con su tabla de validación y los tipos y campos ausentes de A1, la parte de `Orden` que da cada spec y `orden_numero` en `GET /api/citas`), `specs/000-base-comun/plan.md` (`backend/seed/demo.sql` en la lista de ficheros compartidos, G1), `specs/000-base-comun/data-model.md` (fila `orden`: día de la cita, mecánico activo, visita anterior, daños) y `specs/000-base-comun/quickstart.md`: §0 (`pedro` y las citas de hoy), §2 reescrito y **sus secciones** §3, §4, §5 y §7, donde «la orden de 1234BCD» pasa a ser «la orden de 1313MMM» y «la otra orden», la de `1414NNN`, y §9 (recorrido en dos días). Confirmar también que sus pruebas de spec-003, 005 y 006 siguen pasando con la ficha y que sus secciones se ven bien en `/ordenes/[id]`. **Cubre:** FR-001, FR-003, FR-004, FR-005, FR-008, SC-004 (coherencia de los documentos y ficheros compartidos).

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: T001, T002 y T003 en paralelo, cada uno en su commit. T001 solo bloquea la
  verificación en el navegador (T029); T002 bloquea T019; T003 bloquea el frontend (T023–T026).
- **Foundational (Phase 2)**: T004 y T005. Bloquea las dos historias.
- **US1 (Phase 3)**: depende de la Phase 2. Es el MVP.
- **US2 (Phase 4)**: depende de T017 (rellena su hueco del km). Sus pruebas se pueden escribir en
  cuanto exista T005.
- **Polish (Phase 5)**: después de las dos historias, en el orden T029 → T030 → T031.

### Within Each User Story

- Las pruebas (T006–T016, T027) se escriben y fallan **antes** de su implementación.
- T017 responde provisionalmente `{id, numero, estado}`; T019 pone el objeto `Orden` completo, y la
  comprobación de la respuesta de T007 pasa con T019 (hallazgo O1).
- La comprobación de T010 sin el `FOR UPDATE` se hace después de T017, con el código restaurado al
  terminar.
- Backend antes que la pantalla que lo usa. El orden del `POST` es siempre: cita existe (404) →
  campos de la tabla, con el km anterior al final (422) → día de la cita (409) → transacción con la
  cita bloqueada → cita con orden (409) → número del contador → `INSERT` → `commit` → evento.
- T020 (quitar el `#[ignore]`) va después de T019 y en su propio commit.

### Parallel Opportunities

- T001, T002 y T003 entre sí.
- Las pruebas T006–T016 son del mismo fichero: se escriben una detrás de otra, pero en paralelo
  con T004.
- T023, T024 y T025 (frontend, ficheros distintos) en paralelo entre sí y con T019–T022, cuando
  T003 esté hecha.
- T027 (prueba de US2) en paralelo con la implementación de US1.

---

## Parallel Example: User Story 1

```bash
Task: "T023 Columna «Orden» en frontend/src/routes/citas/+page.svelte"
Task: "T024 Formulario en frontend/src/routes/ordenes/nueva/+page.svelte"
Task: "T025 Lista en frontend/src/routes/ordenes/+page.svelte"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. T001, T002 y T003, cada uno en su commit y con la revisión de B.
2. Phase 2.
3. US1: recepción abre órdenes numeradas sin huecos, una por cita y solo el día de la cita, y el
   tablero las ve. Se valida con el quickstart común §2, pasos 1–5 y 7.

### Incremental Delivery

1. US1 → órdenes abiertas, ficha y listas (desbloquea el uso real de spec-003, 005 y 006 desde la
   app y el tablero de spec-008).
2. US2 → control del kilometraje (paso 6).
3. Polish → verificación en el navegador, comprobaciones automáticas y revisión de B.

---

## Notes

- Cada tarea se marca `[X]` al terminarla, con una nota de lo hecho y de las desviaciones.
- No se hace commit de varios cambios compartidos juntos: cada «cambio compartido · revisa Persona
  B» va en su propio commit.
- Si al implementar aparece algo que no está en la spec, se anota y se propone; no se construye
  (principio III).
