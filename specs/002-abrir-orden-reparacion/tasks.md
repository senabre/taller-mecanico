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
construir la matriz de trazabilidad.

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

- [ ] T001 **Cambio compartido · revisa Persona B · commit propio**. En `backend/seed/demo.sql`, añadir una **parte 5** al final, antes del `COMMIT`, sin tocar las partes 1–4 ni las matrículas del quickstart §1 (`1111BBB`, `2222CCC`, `9999LLL`, `1234BCD`, `5678CDF`) (research D8): (a) vehículo `1313MMM` (Seat León) de «Lucía Fernández» y vehículo `1414NNN` (Renault Mégane) de «Andrés Gil», sin órdenes, cada uno con una cita de **hoy** («Revisión general» y «Cambio de aceite»); (b) una cita de **hoy** para `8888KKK` («Revisión de frenos»), cuya orden 8 tiene 39.800 km; (c) vehículo `1515PPP` (Ford Fiesta) de «Jorge Martín» con una cita de **ayer** y sin orden; (d) usuario `Pedro` / `pedro`, rol `mecanico`, `activo = false`, con el mismo hash ficticio de los otros cinco. Las fechas, con `(now() AT TIME ZONE 'Europe/Madrid')::date` (y `- 1` para ayer), no con `current_date`. No crear órdenes ni tocar `contador`. Comprobar sobre una base recién migrada que el fichero carga sin errores, que `SELECT ultimo FROM contador WHERE nombre = 'orden'` sigue dando 9 y que `cargo test` sigue pasando (las pruebas no usan los datos demo). Añadir al comentario de cabecera de la parte 5 que los datos demo se cargan el mismo día en que se recorre el quickstart. **Cubre:** FR-001, FR-004, FR-005, FR-008, US1-4, US1-5, US1-6, US1-7, US2-1, US2-3, SC-001 (datos para verificarlos en el quickstart común §2).
- [ ] T002 [P] **Cambio compartido · revisa Persona B · commit propio**. En `backend/src/rutas/imputaciones.rs` (de la Persona B, spec-005), hacer `pub` los campos `coste_acumulado`, `total_presupuestado` e `imputaciones` de `ResumenImputaciones`, sin cambiar nada más (ni el `Serialize` ni lo que devuelve `resumen_imputaciones`), para que la ficha de spec-002 copie esos tres campos al objeto `Orden` sin duplicar `estado` (research D6). `cargo test --test spec005_imputaciones` debe seguir pasando sin cambios. **Cubre:** US1-3, SC-004 (la ficha muestra lo imputado y lo presupuestado).
- [ ] T003 [P] **Cambio compartido · revisa Persona B**. En `frontend/src/lib/tipos.ts`, añadir en una sección «spec-002 · Órdenes», sin tocar las demás: `PeticionOrden {cita_id: number, km: number | string, danos: string, mecanico_id: number, fecha_entrega_prometida: Fecha}` y `Orden` = `OrdenResumen` + `{cita: {fecha: Fecha, motivo: string}, vehiculo: {marca: string, modelo: string}, km: number, danos: string, diagnostico: string | null, total_presupuestado: Importe, coste_acumulado: Importe, presupuestos: PresupuestoResumen[], imputaciones: Linea[], pedidos: Pedido[], factura_id: number | null}`; y en `CitaDelDia` (spec-001), `orden_numero: number | null` junto a `orden_id`. Reutilizar los tipos que ya existen (`OrdenResumen`, `PresupuestoResumen`, `Linea`, `Pedido`). `npm run check`: 0 errores. **Cubre:** FR-008, US1-3, US1-4.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: el módulo de rutas y la base de las pruebas, que necesitan las dos historias.

**⚠️ CRITICAL**: ninguna historia puede empezar hasta que esta fase esté terminada.

- [ ] T004 Crear `backend/src/rutas/ordenes.rs` con `rutas()`, que registra `POST /ordenes`, `GET /ordenes` y `GET /ordenes/{id}`, los tres respondiendo provisionalmente 501. Registrarlo en `backend/src/rutas/mod.rs` con una sola línea `.merge(ordenes::rutas())` después de las que ya hay, sin tocar las demás (**cambio compartido · revisa Persona B**, en el mismo commit que el módulo). Comprobar que no hay rutas solapadas con `presupuestos.rs` e `imputaciones.rs` (`/ordenes/{id}/…`). **Cubre:** FR-001 (infraestructura).
- [ ] T005 Crear `backend/tests/spec002_ordenes.rs` con `mod comun;` y utilidades propias: `cita(pool, matricula, fecha) -> (cita_id, vehiculo_id)` (cliente, vehículo y cita por SQL; para hoy, `fechas::hoy()`); `orden_previa(pool, cita_id, vehiculo_id, mecanico_id, km, numero)` (una orden anterior por SQL, para el km); `mecanico(pool, login, activo) -> i64`; `peticion(cita_id, mecanico_id) -> Value` (una petición válida: 85.000 km, «Rayón puerta trasera izquierda», entrega `hoy + 3`); `contador(pool) -> i64` (`SELECT ultimo FROM contador WHERE nombre = 'orden'`); y `ordenes(pool) -> i64`. Reutilizar `comun::app_prueba`, `estado_prueba`, `peticion`, `crear_usuario` y `entrar`. Las utilidades que aún no se usen llevan `#[allow(dead_code)]`, que se quita al usarlas. **Cubre:** infraestructura de pruebas, sin requisito propio.

**Checkpoint**: el backend compila con el módulo registrado y el fichero de pruebas existe.

---

## Phase 3: User Story 1 - Abrir la orden al recibir el coche (Priority: P1) 🎯 MVP

**Goal**: el día de la cita, recepción abre la orden desde `/citas`; la orden recibe su número
correlativo, queda en «recibida» con km, daños, mecánico y entrega, aparece en el tablero, y la
cita pasa a mostrar «Orden Nº N».

**Independent Test**: con una cita de hoy, abrir la orden y comprobar el número, los datos y el
estado; intentar abrirla otra vez desde otra pestaña (quickstart común §2, pasos 1–5 y 7).

### Tests for User Story 1 ⚠️

- [ ] T006 [US1] **Prueba 9** (acceso y existencia): `POST /api/ordenes` sin sesión → 401; con `mecanico` y `almacen` → 403; con `recepcion` y con `jefe` → 201; con un `cita_id` que no existe → 404 «No existe la cita». **Cubre:** FR-001.
- [ ] T007 [US1] **Prueba 1** (abrir y publicar el evento): con `let estado = comun::estado_prueba(pool.clone()); let mut tablero = estado.eventos.suscribir(); let app = taller::app(estado);` y una cita de hoy, `POST /api/ordenes` → 201 con el objeto `Orden`: `estado = "recibida"`, `numero` = contador anterior + 1, `km`, `danos`, `mecanico`, `fecha_entrega_prometida`, `matricula` y `vehiculo` de la cita; en la tabla, `vehiculo_id` es el de la cita aunque la petición no lo envíe. `tablero.try_recv()` devuelve `CambioEstado { id, estado: Estado::Recibida }` con el `id` de la orden. Después de un 422, `tablero.try_recv()` está vacío: si la transacción no se confirma, no se publica nada. **Cubre:** FR-005, FR-006, FR-007, US1-1 (y research D5, hallazgo I4 de la base común).
- [ ] T008 [US1] **Prueba 2** (correlativo sin huecos): con el contador en 41 (`UPDATE contador`), provocar un 422 de cada campo de la tabla de validación (km, daños, mecánico, fecha de entrega) y los dos 409 (cita de mañana; cita que ya tiene orden), y después abrir una orden válida → recibe la **42**; el contador avanza exactamente 1 en total. **Cubre:** FR-002, US1-2, caso límite «no se consume ningún número».
- [ ] T009 [US1] **Prueba 3** (concurrencia del número): diez citas de hoy distintas y diez `POST` a la vez (`futures_util::future::join_all`) → diez 201 con los números `n+1 … n+10`, sin huecos ni repetidos, y el contador en `n+10`. **Cubre:** FR-002, SC-002, caso límite «dos recepcionistas a la vez».
- [ ] T010 [US1] **Prueba 4** (concurrencia de la misma cita): con una cita de hoy, lanzar a la vez con `tokio::join!` dos `POST` para esa cita → un 201 y un 409 «Esta cita ya tiene una orden (Nº N)», con `detalle.orden_id` y `detalle.numero` iguales a los de la orden creada; exactamente 1 orden de esa cita y el contador avanzado en 1. Repetir 20 veces (borrando la orden y restando 1 al contador entre vueltas, o con una cita nueva por vuelta). **Comprobación obligatoria**: una vez hecha T017, quitar temporalmente el `FOR UPDATE` y comprobar que esta prueba **falla** (se espera `[201, 500]`, porque el segundo `INSERT` choca con el `UNIQUE (cita_id)`); restaurar el código, comprobarlo con `cmp` y anotar el resultado en esta tarea. **Cubre:** FR-008, FR-002, US1-4 (Q5).
- [ ] T011 [US1] **Prueba 5** (día de la cita): con una cita de **mañana** y otra de **ayer** sin orden, `POST` con datos válidos → 409 «Solo se puede abrir la orden el día de la cita» en las dos, sin crear órdenes y sin mover el contador. Con datos válidos **y** un km negativo, la cita de mañana da el 422 del km, no el 409 (todos los 422 antes que cualquier 409). **Cubre:** FR-001, US1-5 (Q2), US1-6.
- [ ] T012 [US1] **Prueba 7** (campos de la tabla de validación): con una cita de hoy, `km` ausente, `null`, `-1`, `1.5` y `"abc"` → 422 «El kilometraje debe ser un número entero igual o mayor que 0» (`detalle.campo = "km"`); `danos` `""` y `"   "` → 422 «Describe los daños visibles o escribe “Sin daños visibles”» (`danos`); `fecha_entrega_prometida` ausente o `"ayer"` → 422 «Indica la fecha de entrega prometida»; ayer → 422 «La fecha de entrega prometida no puede ser anterior a hoy» (`fecha_entrega_prometida`); hoy → 201; `danos` `"  Rayón  "` → se guarda «Rayón»; «Sin daños visibles» → 201. Con varios errores a la vez, el primero según el orden de la tabla. En ningún 422 se crea orden ni se mueve el contador. **Cubre:** FR-003, FR-006, US1-8 (Q4), casos límite de km y de entrega.
- [ ] T013 [US1] **Prueba 8** (mecánico): `mecanico_id` de un mecánico **de baja**, de un usuario con rol `recepcion`, del `jefe` y un id que no existe → 422 «Elige un mecánico de la lista» (`detalle.campo = "mecanico_id"`); con un mecánico activo → 201. **Cubre:** FR-005, US1-7 (Q3).
- [ ] T014 [US1] **Prueba 10** (ficha): tras abrir una orden, `GET /api/ordenes/{id}` con `recepcion`, `mecanico`, `almacen` y `jefe` → 200 con `id`, `numero`, `matricula`, `estado`, `mecanico: {id, nombre}`, `fecha_entrega_prometida`, `retrasada: false`, `cita: {fecha, motivo}`, `vehiculo: {marca, modelo}`, `km`, `danos`, `diagnostico: null`, `presupuestos: []`, `imputaciones: []`, `pedidos: []`, `total_presupuestado: "0.00"`, `coste_acumulado: "0.00"` y `factura_id: null`; ningún campo de datos del cliente; sin sesión → 401; id inexistente → 404 «No existe la orden». **Cubre:** FR-005, US1-3, SC-004.
- [ ] T015 [US1] **Prueba 11** (lista de citas): tras abrir la orden de una cita de hoy, `GET /api/citas?fecha=<hoy>` como recepción → esa cita trae `orden_id` y `orden_numero` de la orden; otra cita sin orden trae los dos a `null`. **Cubre:** FR-008, US1-4 (Q5).
- [ ] T016 [US1] **Prueba 12** (lista de órdenes): con órdenes de dos mecánicos, `GET /api/ordenes` como recepción → todas, de la más reciente a la más antigua, con los campos de `OrdenResumen`; `GET /api/ordenes?mias=true` como `luis` → solo las suyas; `almacen` → 403; `retrasada` es `true` con la entrega de ayer y `false` con la de hoy (data-model común: «estado ≠ `facturada` y `fecha_entrega_prometida` < hoy»). **Cubre:** US1-3, SC-004 (y lo que usa spec-008).

### Implementation for User Story 1

- [ ] T017 [US1] En `backend/src/rutas/ordenes.rs`, implementar `POST /api/ordenes` (`Usuario` + `exigir_rol(&[Rol::Recepcion])`; el jefe pasa) en el orden de research D2 y de la tabla de validación del contrato común: (1) la cita existe, si no 404 «No existe la cita»; (2) **sin transacción**, los campos en el orden de la tabla, con `error_campo` como en `citas.rs`: `km` «número entero de 0 a 2.147.483.647» (data-model común: «≥ 0»), `danos` «obligatorio, no vacío tras quitar espacios» (con `trim`, se guarda sin los espacios del principio y del final), `mecanico_id` «el usuario debe tener rol `mecanico` y estar activo al abrir la orden» (la misma condición que `GET /api/mecanicos`), `fecha_entrega_prometida` presente y «≥ fecha de apertura» (`fechas::hoy()`); (3) `cita.fecha == fechas::hoy()`, si no 409 «Solo se puede abrir la orden el día de la cita»; (4) transacción: `SELECT … FROM cita WHERE id = $1 FOR UPDATE` y, si ya hay una orden con ese `cita_id`, 409 «Esta cita ya tiene una orden (Nº N)» con `detalle: {orden_id, numero}`; (5) **solo entonces** `contador::siguiente(&mut tx, contador::ORDEN)` e `INSERT` en `orden` con el `vehiculo_id` de la cita y `estado = 'recibida'`; `commit`; responder 201 con el objeto `Orden` (T019). Deja el hueco del km anterior (T029) al final del paso 2. Hace pasar T006, T008, T009, T010, T011, T012 y T013. **Cubre:** FR-001, FR-002, FR-003, FR-005, FR-006, FR-007, FR-008, US1-1, US1-2, US1-4, US1-5, US1-7, US1-8.
- [ ] T018 [US1] En `POST /api/ordenes` de `backend/src/rutas/ordenes.rs`, **después del `commit`** (nunca antes ni dentro de la transacción): `CambioEstado { id, estado: Estado::Recibida }.publicar(&estado_app.eventos)` (research D5). Si algo falla antes, no se publica nada. Hace pasar la parte del evento de T007. **Cubre:** FR-007 (el tablero de spec-008 ve la orden nueva; hallazgo I4 de la base común).
- [ ] T019 [US1] En `backend/src/rutas/ordenes.rs`, implementar `GET /api/ordenes/{id}` (todos los roles con sesión; 404 «No existe la orden») con la composición de research D6: una consulta propia para los campos de `OrdenResumen` (`retrasada` = estado ≠ `facturada` y `fecha_entrega_prometida < fechas::hoy()`), `cita: {fecha, motivo}`, `vehiculo: {marca, modelo}`, `km`, `danos`, `diagnostico` y `factura_id` (`SELECT id FROM factura WHERE orden_id = …`); y **una llamada por sección de B**: `presupuestos::presupuestos_de_orden(&mut conn, id, config.iva_porcentaje)`, `imputaciones::resumen_imputaciones(&mut conn, id)` (de la que se copian `total_presupuestado`, `coste_acumulado` e `imputaciones`, T002) y `almacen::pedidos_de_orden(&mut conn, id)`. No modificar las funciones de B. Definir `OrdenResumen` en este módulo como `pub`, para que la reutilicen `GET /api/ordenes` y spec-008. La misma función construye la respuesta 201 de T017. Hace pasar T014 y la respuesta de T007. **Cubre:** FR-005, US1-1, US1-3, SC-004.
- [ ] T020 [US1] **Cambio compartido · revisa Persona B · commit propio**. En `backend/tests/spec006_pedidos.rs` (de la Persona B), quitar el `#[ignore = "necesita GET /api/ordenes/{id} de spec-002"]` de `la_ficha_de_la_orden_trae_sus_pedidos`, sin cambiar la prueba. Debe pasar con T019; si no pasa, no se toca la prueba: se avisa a la Persona B. **Cubre:** US1-3, SC-004 (la ficha trae los pedidos de spec-006).
- [ ] T021 [US1] En `backend/src/rutas/ordenes.rs`, implementar `GET /api/ordenes?mias=true` (`exigir_rol(&[Rol::Recepcion, Rol::Mecanico])`; el jefe pasa): `[OrdenResumen]` de la más reciente a la más antigua; con `mias=true`, solo las órdenes con `mecanico_id` = usuario que ha entrado. Reutiliza la consulta de `OrdenResumen` de T019. Hace pasar T016. **Cubre:** US1-3, SC-004.
- [ ] T022 [US1] En `backend/src/rutas/citas.rs` (spec-001, Persona A), añadir `orden_numero` a `GET /api/citas`: `o.numero AS orden_numero` en el `LEFT JOIN orden` que ya existe, y el campo en `FilaCita` y `CitaDelDia`. Hace pasar T015; `cargo test --test spec001_citas` debe seguir pasando (15/15). **Cubre:** FR-008, US1-4 (Q5).
- [ ] T023 [P] [US1] En `frontend/src/routes/citas/+page.svelte` (spec-001), añadir una columna «Orden»: si la cita tiene `orden_id`, un enlace «Orden Nº {orden_numero}» a `/ordenes/{orden_id}`; si no la tiene **y** el día elegido es hoy (`dia === hoy`, que la página ya calcula), un botón «Abrir orden» que lleva a `/ordenes/nueva?cita={id}`; en otro caso, la celda vacía. Sustituye el comentario de hueco que dejó spec-001. **Cubre:** FR-001, FR-008, US1-4, US1-5 (Q2, Q5).
- [ ] T024 [P] [US1] Crear `frontend/src/routes/ordenes/nueva/+page.svelte` (recepción y jefe; título «Abrir orden»), con `novalidate`. Al abrir, `GET /api/citas?fecha=<hoy en Madrid>` y `GET /api/mecanicos`. Si la cita de `?cita=` no está en la lista de hoy, mostrar solo «Solo se puede abrir la orden el día de la cita». Si está, mostrar su matrícula, marca, modelo y motivo, y los campos Kilómetros (`inputmode="numeric"`), Daños visibles (textarea), Mecánico (selector solo con la lista de `GET /api/mecanicos`) y Fecha de entrega prometida (`type="date"`). Al guardar, `POST /api/ordenes`: 201 → `goto('/ordenes/{id}')`; 422 → mensaje bajo el campo de `detalle.campo`; 409 con `detalle.orden_id` → el mensaje y un enlace «Ver la orden» a `/ordenes/{detalle.orden_id}`; otro 409 → error general. **Cubre:** FR-001, FR-003, FR-005, FR-006, FR-008, US1-1, US1-4, US1-7, US1-8, SC-001.
- [ ] T025 [P] [US1] Crear `frontend/src/routes/ordenes/+page.svelte`: con rol `mecanico`, `GET /api/ordenes?mias=true` y título «Mis órdenes»; con recepción o jefe, `GET /api/ordenes` y título «Órdenes». Tabla con Número (enlace a `/ordenes/{id}`), Matrícula, Estado (`NOMBRE_ESTADO` de `formato.ts`), Mecánico y Entrega prometida (dd/mm/aaaa, con «Retrasada» si `retrasada`); con la lista vacía, «No hay órdenes». **Cubre:** US1-3, SC-004.
- [ ] T026 [US1] **Cambio compartido · revisa Persona B**. Crear `frontend/src/routes/ordenes/[id]/+page.svelte` (contenedor de secciones, plan maestro) y `frontend/src/routes/ordenes/[id]/Datos.svelte`. El contenedor carga `GET /api/ordenes/{id}` una vez (404 → «No existe la orden») y pinta, en este orden: `Datos` (número, estado, matrícula, marca, modelo, fecha dd/mm/aaaa y motivo de la cita, km con separador de miles «85.000 km», daños, mecánico, entrega prometida y «Retrasada» si lo está); `Diagnostico` (`orden`, `puedeEditar`, `onguardado`); `Presupuesto` (`orden`, `presupuestos`, `puedeEditar`, `onCambio`); `Imputaciones` (`resumen` = `{orden_id: orden.id, estado, coste_acumulado, total_presupuestado, imputaciones}`, `puedeEditar`, `onCambio`); y `Pedidos` (`pedidos`). `puedeEditar` = el usuario es el mecánico asignado o el jefe. `onCambio` y `onguardado` vuelven a pedir la orden. No modificar los componentes de B; añadir una sección nueva tiene que ser una línea. **Cubre:** US1-3, SC-004.

**Checkpoint**: las pruebas 1–5 y 7–12 pasan, también la de spec-006; desde `/citas` se abre una
orden y se ve su ficha.

---

## Phase 4: User Story 2 - Control del kilometraje (Priority: P1)

**Goal**: un kilometraje menor que el de la visita anterior del vehículo se rechaza mostrando el
valor previo.

**Independent Test**: con una visita anterior de 85.000 km, intentar abrir la orden con 80.000 km y
comprobar el rechazo con «85.000 km» (quickstart común §2, paso 6, con 39.800 km de `8888KKK`).

### Tests for User Story 2 ⚠️

- [ ] T027 [US2] **Prueba 6** (kilometraje): con dos órdenes anteriores del mismo vehículo (70.000 km y después 85.000 km, por SQL con `abierta_en` distinto) y una cita de hoy, `km: 80000` → 422 «El kilometraje no puede ser menor que el de la visita anterior (85.000 km)» con `detalle: {campo: "km", km_anterior: 85000}`, sin crear orden ni mover el contador; `km: 85000` → 201. Un vehículo sin órdenes con `km: 0` → 201. Una segunda pestaña: abierta la orden de una cita con 85.000 km, un segundo `POST` para **la misma cita** con 80.000 km → 409 «Esta cita ya tiene una orden (Nº N)», no el 422 del km (research D3). Con la cita de mañana y un km menor → el 422 del km, no el 409 del día. **Cubre:** FR-004, US2-1, US2-2, US2-3, SC-003.

### Implementation for User Story 2

- [ ] T028 [US2] En `POST /api/ordenes` de `backend/src/rutas/ordenes.rs`, rellenar el hueco que deja T017 al final del paso 2 (última fila de la tabla de validación, antes de cualquier 409): `SELECT km FROM orden WHERE vehiculo_id = $1 AND cita_id <> $2 ORDER BY abierta_en DESC, id DESC LIMIT 1` (data-model común: «≥ km de la visita anterior: la orden más reciente del mismo vehículo para otra cita»); si existe y el km nuevo es menor → 422 con el texto del contrato, el km escrito con `formato::numero` («85.000») y `detalle: {campo: "km", km_anterior}`. Sin órdenes anteriores no se comprueba nada más. El formulario de T024 ya lo muestra bajo Kilómetros. Hace pasar T027. **Cubre:** FR-004, US2-1, US2-2, US2-3, SC-003.

**Checkpoint**: las 12 pruebas del plan pasan; las dos historias funcionan juntas.

---

## Phase 5: Polish & Cross-Cutting Concerns

- [ ] T029 **Verificación en el navegador** del [quickstart común §2](../000-base-comun/quickstart.md), pasos 1–7, con la base de datos recién creada y cargada **ese mismo día** con `backend/seed/demo.sql`, que ya incluye la parte 5 de T001 (citas de hoy de `1313MMM`, `1414NNN` y `8888KKK`, cita de ayer de `1515PPP` y el mecánico de baja `pedro`), después de recorrer el §1. En el paso 4, **cronometrar** desde la lista de citas hasta ver la ficha (< 2 min, SC-001) y anotar el tiempo. Para cada paso, anotar si se cumple usando la correspondencia de [quickstart.md](quickstart.md); guardar el informe y las capturas en `specs/002-abrir-orden-reparacion/evidencias/T029/`. Probar `/ordenes/nueva` también con el ancho de un móvil. Si un paso falla, la spec no está terminada (principio IV): se registra el hallazgo y una tarea correctora. **Cubre:** FR-001, FR-002, FR-003, FR-004, FR-005, FR-006, FR-007, FR-008, US1-1, US1-2, US1-3, US1-4, US1-5, US1-6, US1-7, US1-8, US2-1, US2-3, SC-001, SC-002, SC-003, SC-004.
- [ ] T030 Ejecutar y dejar sin errores ni avisos: `cd backend && cargo test` (incluye `spec002_ordenes`, `spec001_citas` con `orden_numero` y `spec006_pedidos` sin ninguna prueba ignorada), `cargo clippy --all-targets -- -D warnings`, `cd frontend && npm run check` y `npm test`. Anotar los resultados en esta tarea. **Cubre:** las pruebas 1–12, es decir: FR-001 (pruebas 5 y 9), FR-002 (2, 3 y 4), FR-003 (7), FR-004 (6), FR-005 (1, 8 y 10), FR-006 (1 y 7), FR-007 (1), FR-008 (4 y 11), SC-002 (3), SC-003 (6), SC-004 (10 y 12).
- [ ] T031 **Revisión de la Persona B** de todo lo compartido antes de fusionar. **Código**: los commits de T001 (`backend/seed/demo.sql`, parte 5), T002 (`imputaciones.rs`), T020 (`spec006_pedidos.rs`), T003 (`tipos.ts`), T004 (`rutas/mod.rs`) y T026 (`ordenes/[id]/+page.svelte`). **Documentos comunes** que cambió el plan de spec-002: `specs/000-base-comun/contracts/api.md` (sección spec-002 con su tabla de validación, la parte de `Orden` que da cada spec y `orden_numero` en `GET /api/citas`), `specs/000-base-comun/data-model.md` (fila `orden`: día de la cita, mecánico activo, visita anterior, daños) y `specs/000-base-comun/quickstart.md`: §0 (`pedro` y las citas de hoy), §2 reescrito y **sus secciones** §3, §4, §5 y §7, donde «la orden de 1234BCD» pasa a ser «la orden de 1313MMM» y «la otra orden», la de `1414NNN`, y §9 (recorrido en dos días). Confirmar también que sus pruebas de spec-003, 005 y 006 siguen pasando con la ficha y que sus secciones se ven bien en `/ordenes/[id]`. **Cubre:** FR-001, FR-003, FR-004, FR-005, FR-008, SC-004 (coherencia de los documentos y ficheros compartidos).

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
