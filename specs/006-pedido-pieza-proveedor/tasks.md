---

description: "Tareas de spec-006 · Pedir al proveedor una pieza sin stock"
---

# Tasks: spec-006 · Pedir al proveedor una pieza sin stock

**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md) y los documentos
compartidos [data-model.md](../000-base-comun/data-model.md),
[contracts/api.md](../000-base-comun/contracts/api.md) y
[quickstart.md](../000-base-comun/quickstart.md).

**Tests**: se incluyen. El plan maestro (research R13) exige pruebas `#[sqlx::test]` para cada regla
de negocio, y el plan de esta spec enumera 10 en `backend/tests/spec006_pedidos.rs`.

**Organización**: una fase por historia de usuario de la spec. Todo texto visible y todo mensaje
de error, en español de España; importes con `formato.ts` (1.234,56 €) y fechas dd/mm/aaaa.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: se puede hacer en paralelo (otro fichero, sin depender de tareas pendientes)
- **[Story]**: historia de usuario (US1…US4)

## Path Conventions

Aplicación web: `backend/src/`, `backend/tests/`, `backend/migrations/`, `backend/seed/`,
`frontend/src/`.

---

## Phase 1: Setup (base común)

**Purpose**: comprobar que la base común (fase 0 del plan maestro, A + B) da lo que necesita esta
spec. Si algo falta, se añade en la base común con revisión de la Persona A.

- [x] T001 Comprobar que el quickstart §0 funciona (entrar como `pepe`/almacén y como `paco`/jefe) y que existen `backend/src/estados.rs::cambiar_estado`, `backend/src/auth.rs` (método `Usuario::exigir_rol`, que ya deja pasar siempre al jefe), `backend/src/eventos.rs` (`CambioEstado::publicar`), `backend/src/error.rs` y `backend/src/fechas.rs`
- [x] T002 *(ya cubierta por la base común: la migración `0001` cumple todo lo siguiente y además exige `referencia` sin espacios iniciales ni finales y no vacía; no hace falta migración nueva)* En `backend/migrations/0001_esquema_inicial.sql` asegurar exactamente: tabla `pieza` con `referencia TEXT NOT NULL UNIQUE`, `descripcion TEXT NOT NULL`, `precio_venta NUMERIC(12,2) NOT NULL CHECK (precio_venta >= 0)`, `stock INTEGER NOT NULL DEFAULT 0 CHECK (stock >= 0)`; tabla `proveedor` con `nombre TEXT NOT NULL UNIQUE`; tabla `pedido` con `proveedor_id` FK, `pieza_id` FK, `cantidad INTEGER NOT NULL CHECK (cantidad > 0)`, `orden_id` FK, `estado estado_pedido NOT NULL DEFAULT 'pendiente'`, `pedido_en TIMESTAMPTZ NOT NULL DEFAULT now()`, `recibido_en TIMESTAMPTZ NULL`; e índice `CREATE UNIQUE INDEX pedido_pendiente_unico ON pedido (orden_id, pieza_id) WHERE estado = 'pendiente'` (research D2)
- [x] T003 [P] *(ya cubierta por la base común: proveedores, PF-123, órdenes 2 y 3 `en_reparacion` y 4 `en_diagnostico` de `luis`; además BJ-7 y un pedido pendiente para la orden 6 `en_espera_piezas`)* En `backend/seed/demo.sql` añadir los proveedores precargados «Recambios Sur» y «Autopartes Norte», y comprobar que existen la pieza `PF-123 Pastillas freno` (25,00 €, stock 4) y al menos dos órdenes `en_reparacion` y una `en_diagnostico` asignadas a `luis`
- [x] T004 [P] *(ya cubierta por la base común)* Comprobar que en `backend/src/estados.rs` la tabla de transiciones permitidas incluye `en_reparacion → en_espera_piezas` y `en_espera_piezas → en_reparacion`, y que no hay ninguna otra salida de `en_espera_piezas` (data-model, máquina de estados; research D4)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: esqueleto propio de spec-006 que usan todas sus historias.

**⚠️ CRITICAL**: ninguna historia puede empezar sin esta fase.

- [x] T005 Crear `backend/src/rutas/almacen.rs` con una función `rutas()` que devuelva un `Router` vacío del estado de la app, y registrarla en `backend/src/rutas/mod.rs` (`pub mod almacen;` y `.merge(almacen::rutas())` dentro de `api()`); `main.rs` y `lib.rs` no se tocan
- [x] T006 [P] Añadir a `frontend/src/lib/tipos.ts` los tipos del contrato: `Pieza {id, referencia, descripcion, precio_venta: string, stock: number}`, `Proveedor {id, nombre}` y `Pedido {id, proveedor:{id,nombre}, pieza:{id,referencia,descripcion}, cantidad: number, orden:{id,numero}, estado: 'pendiente'|'recibido', pedido_en: string, recibido_en: string|null}`
- [x] T007 [P] Crear `backend/tests/spec006_pedidos.rs` con `mod comun;` y reutilizar sus utilidades (`app_prueba`, `estado_prueba`, `peticion`, `crear_usuario`, `entrar`, `crear_orden`); añadir solo las que faltan: `insertar_pieza`, `insertar_proveedor`, `estado_de_orden` y `stock_de` (por SQL directo, sin pasar por spec-002)
- [x] T008 [P] *(ya cubierta por la base común)* Comprobar que `frontend/src/routes/+layout.svelte` muestra «Almacén» (`/almacen`) y «Pedidos» (`/pedidos`) solo a los roles `almacen` y `jefe`; si no, añadir los dos enlaces

**Checkpoint**: módulo registrado, tipos listos y pruebas compilando.

---

## Phase 3: User Story 1 - Dar de alta referencias de piezas (Priority: P1) 🎯 MVP

**Goal**: el almacén da de alta referencias con código único, descripción, precio de venta y stock
inicial opcional (FR-001, FR-002).

**Independent Test**: dar de alta «PF-124 Pastillas freno traseras», 25,00 €, stock 4 → aparece en
`/almacen`; dar de alta «FA-77 Filtro aire», 12,50 €, sin stock → aparece con stock 0; repetir
«PF-124» → «Ya existe la referencia PF-124».

### Tests for User Story 1

- [x] T009 [P] [US1] En `backend/tests/spec006_pedidos.rs`: `POST /api/piezas` con `stock: 4` → 201 con stock 4; sin `stock` → 201 con stock 0; misma referencia otra vez → 409 «Ya existe la referencia PF-123»; `stock: -1`, `stock: 1.5`, `precio_venta: "-1.00"`, referencia o descripción vacías (o solo espacios) → 422; usuario `mecanico` → 403
- [x] T010 [P] [US1] En `backend/tests/spec006_pedidos.rs`: `GET /api/piezas` devuelve todas las referencias ordenadas por referencia, y con `?q=filtro` solo las que contienen el texto en la referencia o en la descripción (sin distinguir mayúsculas)

### Implementation for User Story 1

- [x] T011 [US1] En `backend/src/rutas/almacen.rs` implementar `POST /api/piezas` (`exigir_rol(&[Rol::Almacen])`, que ya deja pasar al jefe): recibir `{referencia, descripcion, precio_venta, stock?}`; quitar espacios iniciales y finales a `referencia` y `descripcion` y rechazar vacías con 422; `precio_venta` decimal ≥ 0 con 2 decimales; `stock` opcional, entero ≥ 0, 0 por defecto; insertar y responder 201 `{id, referencia, descripcion, precio_venta, stock}`; si se viola el `UNIQUE` de `referencia` → 409 «Ya existe la referencia {referencia}» (research D6)
- [x] T012 [US1] En `backend/src/rutas/almacen.rs` implementar `GET /api/piezas?q=texto` (todos los roles con sesión): `[{id, referencia, descripcion, precio_venta, stock}]` ordenado por `referencia`, con filtro opcional `ILIKE` sobre referencia o descripción (lo reutilizan spec-003 y spec-005)
- [x] T013 [US1] Crear `frontend/src/routes/almacen/+page.svelte`: tabla de referencias (referencia, descripción, precio en € con `formato.ts`, stock) y formulario de alta (referencia, descripción, precio, stock opcional) que, al guardar, añade la fila o muestra el `error` del servidor junto al formulario

**Checkpoint**: US1 funciona sola (quickstart §6 paso 1) y `cargo test spec006` pasa T009–T010.

---

## Phase 4: User Story 2 - Pedir una pieza vinculada a una orden (Priority: P1)

**Goal**: registrar un pedido (proveedor, referencia, cantidad y nº de orden) que queda «pendiente»
con la fecha de hoy y pone la orden «en espera de piezas» (FR-003…FR-006, FR-004a).

**Independent Test**: con la orden R «En reparación» y la pieza demo PF-123, registrar Recambios
Sur, PF-123, 2 uds, orden R → pedido pendiente con fecha de hoy y orden R «En espera de piezas».

### Tests for User Story 2

- [x] T014 [P] [US2] En `backend/tests/spec006_pedidos.rs`: pedido para orden `en_reparacion` → 201, `estado = pendiente`, `pedido_en` es hoy (Europe/Madrid) y la orden pasa a `en_espera_piezas`; otro pedido de **otra** referencia con la orden ya en espera → 201 y sigue en espera
- [x] T015 [P] [US2] En `backend/tests/spec006_pedidos.rs`: misma referencia y misma orden con el primero pendiente → 409 «Ya hay un pedido pendiente de PF-123 para la orden 42», con el texto exacto; orden `en_diagnostico` → 409 con el motivo y la orden no cambia; `orden_numero` inexistente → 422 «No existe la orden 99»; `cantidad` 0, negativa o no entera → 422; proveedor o pieza inexistentes → 422; usuario `mecanico` → 403
- [x] T016 [P] [US2] En `backend/tests/spec006_pedidos.rs`: con la orden en `en_espera_piezas`, `POST /api/ordenes/{id}/terminar` e imputar (`POST /api/ordenes/{id}/imputaciones`) → 409 (FR-006, FR-008a). Marcar con `#[ignore]` hasta que existan los endpoints de spec-005

### Implementation for User Story 2

- [x] T017 [P] [US2] En `backend/src/rutas/almacen.rs` implementar `GET /api/proveedores` (rol `almacen`): `[{id, nombre}]` ordenado por nombre. **No** crear endpoints para dar de alta, editar ni borrar proveedores (FR-010, research D1)
- [x] T018 [US2] En `backend/src/rutas/almacen.rs` implementar `POST /api/pedidos` (rol `almacen`) en una transacción: (1) validar `cantidad` entero > 0 y que existen `proveedor_id` y `pieza_id`, con 422 si no; (2) `SELECT id, estado FROM orden WHERE numero = $1 FOR UPDATE`, con 422 «No existe la orden {n}» si no hay fila, y 409 «La orden {n} está {estado en texto}; solo se puede pedir para órdenes en reparación o en espera de piezas» si el estado no es `en_reparacion` ni `en_espera_piezas`; (3) insertar el pedido y, si choca con `pedido_pendiente_unico`, 409 «Ya hay un pedido pendiente de {referencia} para la orden {n}»; (4) si la orden estaba `en_reparacion`, `cambiar_estado(tx, orden_id, &[EnReparacion], EnEsperaPiezas)` y guardar el `CambioEstado`; (5) `commit` y, **después del `commit`**, `cambio.publicar(&estado.eventos)` para que el tablero se entere (R11, R14); (6) responder 201 con el `Pedido` del contrato (research D3, D5)
- [x] T019 [US2] Crear `frontend/src/routes/pedidos/+page.svelte` con el formulario «Nuevo pedido»: desplegable de proveedores (`GET /api/proveedores`), buscador de referencia (`GET /api/piezas?q=`), cantidad (entero ≥ 1) y nº de orden; al guardar muestra «Pedido registrado» o el `error` del servidor tal cual

**Checkpoint**: US2 funciona con la pieza demo (quickstart §6 pasos 2–6) y pasan T014–T015.

---

## Phase 5: User Story 3 - Recibir el pedido (Priority: P1)

**Goal**: marcar un pedido como recibido: guarda la fecha, suma el stock y, si era el último
pendiente, la orden vuelve sola a «en reparación» (FR-007, FR-008, FR-009 lista de pendientes).

**Independent Test**: con PF-123 y un pedido pendiente de 2 uds para la orden R, pulsar «Marcar
recibido» → el stock sube en 2 y la orden R vuelve a «En reparación» sin más acciones.

### Tests for User Story 3

- [x] T020 [P] [US3] En `backend/tests/spec006_pedidos.rs`: recibir → 200, `estado = recibido`, `recibido_en` es hoy, stock + cantidad y orden `en_reparacion`; con dos pedidos pendientes, recibir uno → la orden sigue `en_espera_piezas` y al recibir el otro vuelve a `en_reparacion`; tras recibir, volver a pedir la misma referencia para la misma orden → 201. Para los eventos, crear `let estado = comun::estado_prueba(pool)`, suscribirse con `estado.eventos.suscribir()` y montar la app con `app(estado.clone())` (no `app_prueba`, que crea su propio canal; ejemplo en `backend/tests/spec000_estados.rs`); suscribirse antes de recibir el último pedido y comprobar que llega `{id: orden_id, estado: en_reparacion}`; al registrar un pedido sobre una orden `en_reparacion` llega `{id, estado: en_espera_piezas}`
- [x] T021 [P] [US3] En `backend/tests/spec006_pedidos.rs`: recibir dos veces → 409 «Este pedido ya está recibido» y el stock no cambia; id inexistente → 404; usuario `mecanico` → 403
- [x] T022 [P] [US3] En `backend/tests/spec006_pedidos.rs`: `GET /api/pedidos?estado=pendiente` devuelve solo los pendientes de todas las órdenes, cada uno con `orden.numero`; sin `estado` devuelve todos

### Implementation for User Story 3

- [x] T023 [US3] En `backend/src/rutas/almacen.rs` implementar `POST /api/pedidos/{id}/recibir` (rol `almacen`) en una transacción: (1) leer `orden_id` y `pieza_id` del pedido, con 404 si no existe; (2) `SELECT ... FROM orden WHERE id = $1 FOR UPDATE`; (3) `UPDATE pedido SET estado = 'recibido', recibido_en = now() WHERE id = $1 AND estado = 'pendiente' RETURNING cantidad`, con 409 «Este pedido ya está recibido» si no hay fila; (4) `UPDATE pieza SET stock = stock + cantidad`; (5) si no quedan pedidos `pendiente` de esa orden, `cambiar_estado(tx, orden_id, &[EnEsperaPiezas], EnReparacion)` y guardar el `CambioEstado`; (6) `commit` y, **después del `commit`**, `cambio.publicar(&estado.eventos)` si hubo cambio (R11, R14); responder 200 con el `Pedido`. Sin correo ni aviso al mecánico (research D8)
- [x] T024 [US3] En `backend/src/rutas/almacen.rs` implementar `GET /api/pedidos?estado=pendiente|recibido` (rol `almacen`): `[Pedido]` con proveedor, pieza, orden `{id, numero}`, cantidad, estado, `pedido_en` y `recibido_en`, del más antiguo al más reciente
- [x] T025 [US3] En `frontend/src/routes/pedidos/+page.svelte` añadir la lista «Pedidos pendientes» de todas las órdenes (nº de orden, proveedor, referencia y descripción, cantidad, fecha dd/mm/aaaa) con el botón «Marcar recibido», que llama a `recibir`, quita la fila y muestra el `error` si lo hay; recargar la lista al registrar un pedido nuevo

**Checkpoint**: el ciclo completo pedir → recibir funciona (quickstart §6 pasos 3, 8 y 9).

---

## Phase 6: User Story 4 - Ver los pedidos de una orden (Priority: P2)

**Goal**: la ficha de la orden muestra todos sus pedidos con proveedor, referencia, cantidad, fecha y
estado (FR-009).

**Independent Test**: con la orden R con un pedido recibido y otro pendiente, abrir `/ordenes/R`
como `luis` → aparecen los dos con su estado.

### Tests for User Story 4

- [x] T026 [P] [US4] En `backend/tests/spec006_pedidos.rs`: `GET /api/ordenes/{id}` devuelve en `pedidos` todos los pedidos de esa orden (pendientes y recibidos) y ninguno de otras órdenes. Marcar con `#[ignore]` hasta que exista el endpoint de spec-002

### Implementation for User Story 4

- [ ] T027 *(hecha la parte de spec-006: `pedidos_de_orden` ya existe y está probada; falta la llamada desde `ordenes.rs` cuando exista spec-002)* [US4] Añadir a `backend/src/rutas/almacen.rs` una función pública `pedidos_de_orden(tx_o_pool, orden_id) -> Vec<Pedido>`, y llamarla desde `GET /api/ordenes/{id}` en `backend/src/rutas/ordenes.rs` para llenar el campo `pedidos` del contrato (punto de contacto con la Persona A: solo se añade esa llamada)
- [ ] T028 *(componente creado; falta incluirlo en `ordenes/[id]/+page.svelte` cuando exista spec-002)* [US4] Crear `frontend/src/routes/ordenes/[id]/Pedidos.svelte` (solo lectura): tabla con proveedor, referencia y descripción, cantidad, fecha de pedido, estado («Pendiente»/«Recibido») y fecha de recepción en dd/mm/aaaa; si no hay pedidos, «Esta orden no tiene pedidos». Incluirla en `frontend/src/routes/ordenes/[id]/+page.svelte` como sección «Pedidos»

**Checkpoint**: todas las historias funcionan (quickstart §6 paso 7).

---

## Phase 7: Polish & Cross-Cutting Concerns

- [x] T029 [P] Revisar que todos los mensajes de error de `backend/src/rutas/almacen.rs` y de las pantallas están en español de España y que precios y fechas usan `frontend/src/lib/formato.ts`
- [x] T030 [P] Revisar que no existe ningún endpoint ni control de interfaz para crear proveedores ni para cambiar a mano el estado de una orden (`grep` de `proveedores` y `estado` en `backend/src/rutas/` y `frontend/src/routes/`), y que no hay ninguna conexión con catálogos de fabricantes ni dependencia nueva para ello (FR-008a, FR-010, FR-011)
- [ ] T031 *(ejecutado: 21 pruebas en verde y `npm run check` sin errores; queda quitar los `#[ignore]` cuando existan spec-002 y spec-005)* Ejecutar `cd backend && cargo test spec006` y `cd frontend && npm run check`; quitar el `#[ignore]` de T016 y T026 en cuanto existan spec-002 y spec-005
- [ ] T032 *(pasos 1–6, 8 y 9 comprobados contra la app con los datos demo; el paso 7 necesita la ficha de la orden de spec-002 y falta el recorrido en pantalla por una persona)* Recorrer a mano [quickstart §6](../000-base-comun/quickstart.md) completo como `pepe`, `luis` y `paco`, y anotar que se cumplen SC-001…SC-006

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: depende de la base común (fase 0 del plan maestro).
- **Foundational (Phase 2)**: depende de Setup y bloquea todas las historias.
- **US1, US2, US3**: después de Phase 2. US2 usa la pieza demo PF-123, así que no espera a US1.
  US3 necesita pedidos pendientes, que se pueden crear con US2 o insertar por SQL en las pruebas.
- **US4**: después de Phase 2, pero la integración T027 necesita `GET /api/ordenes/{id}` de
  spec-002 (Persona A).
- **Polish**: al final.

### User Story Dependencies

- **US1 (P1)**: independiente.
- **US2 (P1)**: independiente (pieza y proveedores demo).
- **US3 (P1)**: en la interfaz depende de US2 para crear pedidos; en el backend es independiente.
- **US4 (P2)**: depende de spec-002 para la ficha de la orden.

### Within Each User Story

- Pruebas primero, y deben fallar antes de implementar.
- Backend antes que pantalla.
- Todas las tareas de backend de una historia tocan `almacen.rs`, así que van en secuencia.

### Parallel Opportunities

- T003 y T004 en paralelo; T006, T007 y T008 en paralelo.
- Las pruebas de cada historia (marcadas [P]) se escriben en paralelo entre sí.
- La pantalla de una historia puede hacerse en paralelo con el backend de la siguiente.
- T017 (proveedores) en paralelo con las pruebas de US2.

---

## Parallel Example: User Story 2

```bash
Task: "T014 Pruebas de pedido → espera en backend/tests/spec006_pedidos.rs"
Task: "T015 Pruebas de rechazos (duplicado, estado, validación) en backend/tests/spec006_pedidos.rs"
Task: "T017 GET /api/proveedores en backend/src/rutas/almacen.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 y Phase 2.
2. US1 (alta de referencias). Desbloquea spec-003 y spec-005, que necesitan piezas.
3. **Validar** con quickstart §6 paso 1.

### Incremental Delivery

1. US1 → referencias (desbloquea a la Persona B en 003).
2. US2 + US3 juntas → el núcleo de HU-06: la orden se para y vuelve sola.
3. US4 → visibilidad en la ficha de la orden, cuando spec-002 esté lista.

---

## Notes

- [P] = otro fichero y sin dependencias pendientes.
- Hacer commit al terminar cada tarea o grupo lógico.
- Los cambios en ficheros compartidos (migración, `demo.sql`, `tipos.ts`, `+layout.svelte`,
  `ordenes.rs`, `ordenes/[id]/+page.svelte`) los revisa también la Persona A.
