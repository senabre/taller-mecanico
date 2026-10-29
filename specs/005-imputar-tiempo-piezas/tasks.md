---

description: "Tareas de spec-005 · Imputar tiempo y piezas durante la reparación"
---

# Tasks: spec-005 · Imputar tiempo y piezas durante la reparación

**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md) y los documentos
compartidos [data-model.md](../000-base-comun/data-model.md),
[contracts/api.md](../000-base-comun/contracts/api.md) y
[quickstart.md](../000-base-comun/quickstart.md).

**Tests**: se incluyen. El plan maestro (research R13) exige pruebas `#[sqlx::test]` para cada regla
de negocio, y el plan de esta spec enumera 9 grupos en `backend/tests/spec005_imputaciones.rs`.

**Organización**: una fase por historia de usuario de la spec. Todo texto visible y todo mensaje
de error, en español de España; importes con `formato.ts` (1.234,56 €) y horas con coma (1,25 h).

## Format: `[ID] [P?] [Story] Description`

- **[P]**: se puede hacer en paralelo (otro fichero, sin depender de tareas pendientes)
- **[Story]**: historia de usuario (US1…US4)

## Path Conventions

Aplicación web: `backend/src/`, `backend/tests/`, `backend/migrations/`, `frontend/src/`.

---

## Phase 1: Setup

**Purpose**: comprobar lo que ya existe en `main` y añadir la única migración propia.

- [x] T001 Comprobar que existen y se pueden usar: la transición `en_reparacion → pendiente_conformidad` y `en_reparacion → terminada` en `TRANSICIONES` de `backend/src/estados.rs` (y que `cambiar_estado` fija `terminada_en`), `Usuario::exigir_mecanico_de` en `backend/src/auth.rs`, `dinero::importe_linea` en `backend/src/dinero.rs`, los endpoints de presupuesto de spec-003 en `backend/src/rutas/presupuestos.rs` (crear un presupuesto `ampliacion` con la orden `pendiente_conformidad`) y la tabla `imputacion` de `backend/migrations/0001_esquema_inicial.sql` con su `CHECK` de horas en múltiplos de 0,25
- [x] T002 Crear `backend/migrations/0003_spec005_imputaciones.sql` (revisada por las dos personas) con `ALTER TABLE imputacion ADD CONSTRAINT imputacion_piezas_enteras CHECK (tipo <> 'pieza' OR cantidad = trunc(cantidad))` (research D2; data-model: «piezas en unidades enteras») y aplicarla con `cargo sqlx migrate run`
- [x] T003 [P] Comprobar en `backend/seed/demo.sql` que siguen cargándose sin error con `0003`, y que existen la orden 3 `en_reparacion` de `luis` (acumulado 40,00 de 110,00), la orden 7 `pendiente_conformidad` de `luis` (130,00 de 110,00) y la orden 2 `en_reparacion` (90,00 de 110,00)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: sacar a módulos compartidos lo que imputaciones reutiliza de spec-003, y el esqueleto
propio.

**⚠️ CRITICAL**: ninguna historia puede empezar sin esta fase.

- [x] T004 Crear `backend/src/lineas.rs` (y `pub mod lineas;` en `backend/src/lib.rs`) moviendo desde `backend/src/rutas/presupuestos.rs`, **sin cambiar ningún mensaje ni comportamiento**: el `struct Linea` del contrato (pasa a `pub`, con sus `#[serde(with = "rust_decimal::serde::str")]`), `struct NuevaLinea` (pasa a `pub`, `Deserialize`), la función `cantidad(&Option<Value>) -> Option<Decimal>` y la validación de `anadir_linea` convertida en `pub async fn validar(tx: &mut PgConnection, config: &Config, nueva: &NuevaLinea) -> Resultado<LineaValidada>`, con `LineaValidada {tipo: &'static str, concepto: String, pieza_id: Option<i64>, cantidad: Decimal, precio_unitario: Decimal, importe: Decimal}` (horas: «Indica las horas: un número mayor que 0», «Las horas van en múltiplos de 0,25 (p. ej. 1,5)», «Escribe el concepto de la mano de obra»; pieza: «Indica las unidades: un número mayor que 0», «Las unidades de pieza deben ser un número entero», «Elige una referencia de pieza dada de alta», concepto `format!("{referencia} {descripcion}")`; otro tipo: «El tipo de línea debe ser mano de obra o pieza») (research D1)
- [x] T005 Cambiar `backend/src/rutas/presupuestos.rs` para usar `lineas::{Linea, NuevaLinea, validar}` en `anadir_linea` y `presupuesto_por_id`, borrando el código movido, y comprobar con `cargo test --test spec003_presupuestos` que las 20 pruebas de spec-003 siguen pasando
- [x] T006 Crear `backend/src/rutas/imputaciones.rs` con `rutas()` que devuelva un `Router` vacío y registrarla en `backend/src/rutas/mod.rs` (`pub mod imputaciones;` y `.merge(imputaciones::rutas())` dentro de `api()`)
- [x] T007 En `backend/src/rutas/imputaciones.rs` añadir el objeto `ResumenImputaciones {orden_id, estado: Estado, coste_acumulado, total_presupuestado, imputaciones: Vec<Linea>}` (importes como string de 2 decimales) y la función pública `resumen_imputaciones(conexion, orden_id) -> Resultado<ResumenImputaciones>`: `coste_acumulado = Σ imputacion.importe` (0,00 si no hay), `total_presupuestado = Σ presupuesto.base` de los `estado = 'aceptado'` (inicial + ampliaciones), imputaciones de la orden ordenadas por `creada_en, id`; 404 «No existe la orden» (research D4, D5). La usa spec-002 para `GET /api/ordenes/{id}`
- [x] T008 [P] Añadir a `frontend/src/lib/tipos.ts` `ResumenImputaciones {orden_id: number, estado: EstadoOrden, coste_acumulado: Importe, total_presupuestado: Importe, imputaciones: Linea[]}`
- [x] T009 [P] Crear `backend/tests/spec005_imputaciones.rs` con `mod comun;` y reutilizar sus utilidades (`estado_prueba`, `peticion`, `crear_usuario`, `entrar`, `insertar_usuario`, `crear_orden`); añadir solo: `app_con_eventos` (como en `backend/tests/spec003_presupuestos.rs`), `insertar_pieza(referencia, descripcion, precio, stock)`, `presupuesto_aceptado(orden_id, base)` por SQL (estado `aceptado`, `iva_porcentaje` 21.00, `base`, `iva` y `total` coherentes, una línea de mano de obra por el importe), `stock_de` y `estado_de_orden`
- [x] T010 [P] Sacar de `frontend/src/routes/ordenes/[id]/Presupuesto.svelte` a `frontend/src/lib/componentes/FormularioLinea.svelte` el formulario «Añadir línea» (selector Mano de obra / Pieza, concepto y horas con coma convertidas a punto, buscador `GET /piezas?q=` y unidades enteras): props `titulo`, `textoBoton` y `onenviar: (linea) => Promise<void>` (el padre hace la llamada y el formulario se limpia si va bien y muestra el `error` si no); `Presupuesto.svelte` pasa a usarlo con el mismo aspecto y comportamiento (research D8)

**Checkpoint**: validación compartida, módulo registrado, migración aplicada, pruebas de spec-003 en verde y formulario compartido.

---

## Phase 3: User Story 1 - Imputar tiempo y piezas (Priority: P1) 🎯 MVP

**Goal**: el mecánico asignado imputa horas y piezas en una orden «en reparación»; cada pieza
descuenta el stock al momento y la orden muestra el acumulado frente a lo presupuestado
(FR-001…FR-004).

**Independent Test**: en la orden demo 3 (`en_reparacion`, 40,00 de 110,00), imputar PF-123 × 2
→ el stock baja 2 en `/almacen` y el acumulado pasa a 90,00 €; imputar más unidades de las que
hay → «Solo hay N unidades…».

### Tests for User Story 1

- [x] T011 [P] [US1] En `backend/tests/spec005_imputaciones.rs`: con 110,00 presupuestados y una pieza `PF-123 Pastillas freno` a 25,00 con stock 10, `POST /api/ordenes/{id}/imputaciones` de `{tipo:"pieza", pieza_id, cantidad:"2"}` → 201, stock 8, línea con concepto «PF-123 Pastillas freno», `importe "50.00"`; después `{tipo:"mano_obra", concepto:"Cambio de pastillas", cantidad:"1"}` → `coste_acumulado "90.00"`, `total_presupuestado "110.00"`, `estado "en_reparacion"`, dos imputaciones en orden; en la base de datos `usuario_id` es el mecánico y `creada_en` es de hoy
- [x] T012 [P] [US1] En `backend/tests/spec005_imputaciones.rs`: pieza con stock 1 e imputar 2 → 422 «Solo hay 1 unidad en stock de PF-123; el almacén puede pedirla al proveedor» (singular), el stock sigue en 1 y no se crea ninguna imputación; con stock 0 → 422 «Solo hay 0 unidades en stock de PF-123; el almacén puede pedirla al proveedor» (analyze I1, C1)
- [x] T013 [P] [US1] En `backend/tests/spec005_imputaciones.rs`: horas 1,3, 0, negativas o vacías, concepto vacío o solo espacios, pieza 2,5 o 0 uds, pieza inexistente y tipo desconocido → 422 con los mismos mensajes que spec-003; insertar por SQL una imputación de pieza de 2,5 uds falla por el `CHECK` de `0003`; una pieza que no estaba en el presupuesto se admite
- [x] T014 [P] [US1] En `backend/tests/spec005_imputaciones.rs`: con la orden `en_espera_piezas`, `pendiente_conformidad`, `terminada` o `en_diagnostico` → 409 «La orden está {estado}; solo se puede imputar en órdenes en reparación»; `en_reparacion` sin presupuesto aceptado → 409 «La orden no tiene un presupuesto aceptado»; otro mecánico → 403; el jefe → 201; sin sesión → 401; orden inexistente → 404
- [x] T015 [P] [US1] En `backend/tests/spec005_imputaciones.rs`: `resumen_imputaciones(conexion, orden_id)` devuelve solo las imputaciones de esa orden y suma como presupuestado solo los `aceptado` (no los `emitido`, `rechazado` ni `borrador`)

### Implementation for User Story 1

- [x] T016 [US1] En `backend/src/rutas/imputaciones.rs` implementar `POST /api/ordenes/{id}/imputaciones` en una transacción (research D3): `SELECT estado, mecanico_id FROM orden WHERE id = $1 FOR UPDATE` (404 «No existe la orden»); `exigir_mecanico_de`; si no está `en_reparacion` → 409 «La orden está {estado.texto()}; solo se puede imputar en órdenes en reparación»; si no hay ningún presupuesto `aceptado` → 409 «La orden no tiene un presupuesto aceptado»; `lineas::validar` (422); si es pieza, `UPDATE pieza SET stock = stock - $2 WHERE id = $1 AND stock >= $2` y, si no actualiza ninguna fila, leer el stock y responder 422 «Solo hay {stock} {unidad|unidades} en stock de {referencia}; el almacén puede pedirla al proveedor» («unidad» si el stock es 1); `INSERT INTO imputacion (orden_id, tipo, concepto, pieza_id, cantidad, precio_unitario, importe, usuario_id)`; `commit`; responder 201 con `resumen_imputaciones` (la regla del 10 % llega en US2)
- [x] T017 [US1] Crear `frontend/src/routes/ordenes/[id]/Imputaciones.svelte` (sección de la ficha de spec-002): props `orden: {id, estado}`, `puedeEditar` y `onCambio`; carga el resumen (lo recibe de la ficha o lo devuelve cada imputación); muestra «Acumulado {euros(coste_acumulado)} de {euros(total_presupuestado)} presupuestados» (destacado si el acumulado supera lo presupuestado), `TablaLineas` sin «Quitar» y, si `puedeEditar` y la orden está «en reparación», `FormularioLinea` con el título «Nueva imputación» que llama a `POST /ordenes/{id}/imputaciones`; si la orden no está en reparación, un texto con el motivo («Pendiente de conformidad: hay que emitir una ampliación del presupuesto», «En espera de piezas»…)

**Checkpoint**: US1 funciona sola y pasan T011–T015.

---

## Phase 4: User Story 2 - Bloqueo por desviación de más del 10 % (Priority: P1)

**Goal**: si una imputación hace que el acumulado supere el 110 % de lo presupuestado, se guarda y
la orden pasa a «pendiente de conformidad»; las siguientes se rechazan (FR-005).

**Independent Test**: con 100,00 € presupuestados, imputar hasta 110,00 € → sigue «en
reparación»; imputar hasta 115,00 € → la imputación se guarda y la orden queda «pendiente de
conformidad»; la siguiente imputación se rechaza.

### Tests for User Story 2

- [x] T018 [P] [US2] En `backend/tests/spec005_imputaciones.rs`: con 100,00 presupuestados, imputar 2,75 h a 40,00 (110,00) → `estado "en_reparacion"` y ningún evento; imputar 0,25 h más (120,00 > 110,00) → 201, la imputación está guardada, `estado "pendiente_conformidad"` y llega al tablero `{id, estado: "pendiente_conformidad"}`; la siguiente imputación → 409 y el stock no cambia si era una pieza
- [x] T019 [P] [US2] En `backend/tests/spec005_imputaciones.rs` (sin floats, R4): con 100,00 presupuestados y una pieza a 110,01 € → la orden se bloquea; con 0,01 € menos (110,00) → no se bloquea

### Implementation for User Story 2

- [x] T020 [US2] En `backend/src/rutas/imputaciones.rs`, dentro de la transacción de imputar y después del `INSERT` (research D4): calcular `coste_acumulado` y `total_presupuestado` como en `resumen_imputaciones`; si `coste_acumulado > total_presupuestado * Decimal::new(110, 2)` → `cambiar_estado(tx, id, &[EnReparacion], PendienteConformidad)`; `commit` y, **después del `commit`**, `cambio.publicar(&estado.eventos)` (R14)
- [x] T021 [US2] En `frontend/src/routes/ordenes/[id]/Imputaciones.svelte`, cuando la respuesta trae `estado "pendiente_conformidad"`, ocultar el formulario, mostrar el aviso «El coste supera en más de un 10 % lo presupuestado: prepara una ampliación del presupuesto» y llamar a `onCambio` para que la ficha recargue (la sección Presupuesto muestra entonces «Nuevo presupuesto»)

**Checkpoint**: US1 y US2 funcionan juntas y pasan T018–T019.

---

## Phase 5: User Story 3 - Ampliación del presupuesto y reanudación (Priority: P1)

**Goal**: con la orden bloqueada, el mecánico prepara y emite una ampliación (endpoints de
spec-003); cuando el cliente la acepta (spec-004), la orden vuelve a «en reparación» y lo
presupuestado es la suma de lo aceptado (FR-006, FR-007).

**Independent Test**: bloquear una orden con 100,00 €, crear y emitir una ampliación de 50,00 €,
aceptarla (SQL en la prueba; spec-004 en la app) → lo presupuestado pasa a 150,00 € y se puede
volver a imputar.

### Tests for User Story 3

- [x] T022 [P] [US3] En `backend/tests/spec005_imputaciones.rs`: con la orden `pendiente_conformidad` y 100,00 aceptados, `POST /api/ordenes/{id}/presupuestos` → 201 `tipo "ampliacion"`; añadir 1,25 h (50,00) y emitir → 200; mientras está emitida sin respuesta, imputar → 409; simular la aceptación de spec-004 con SQL (`UPDATE presupuesto SET estado = 'aceptado', respondido_en = now()` y `UPDATE orden SET estado = 'en_reparacion'`) → `resumen_imputaciones` da `total_presupuestado "150.00"`, y se puede imputar hasta 165,00 sin bloqueo (165,01 bloquea)
- [x] T023 [P] [US3] En `backend/tests/spec005_imputaciones.rs`: con una ampliación `rechazada` la orden sigue `pendiente_conformidad`, no se puede imputar (409) y se puede crear otra ampliación (201) (límite conocido de la spec)

### Implementation for User Story 3

- [x] T024 [US3] Comprobar que T022–T023 pasan sin código nuevo en `backend/src/rutas/presupuestos.rs` (las ampliaciones ya las crea y emite spec-003; la suma la hace `resumen_imputaciones`); si alguna falla, corregirlo en `backend/src/rutas/imputaciones.rs` sin tocar el comportamiento de spec-003 (research D7)

**Checkpoint**: el ciclo bloqueo → ampliación → reanudación funciona en el backend.

---

## Phase 6: User Story 4 - Terminar la reparación (Priority: P1)

**Goal**: el mecánico marca como «terminada» una orden «en reparación» con al menos una imputación
(FR-008).

**Independent Test**: en la orden demo 3 (con imputaciones), pulsar «Terminar reparación» → la
orden queda «Terminada» y ya no admite imputaciones; en una orden sin imputaciones → «Primero hay
que imputar el trabajo realizado».

### Tests for User Story 4

- [x] T025 [P] [US4] En `backend/tests/spec005_imputaciones.rs`: `POST /api/ordenes/{id}/terminar` con la orden `en_reparacion` y una imputación → 200 `estado "terminada"`, `terminada_en` puesto en la base de datos, evento `{id, estado: "terminada"}` en el tablero; después imputar → 409
- [x] T026 [P] [US4] En `backend/tests/spec005_imputaciones.rs`: terminar sin imputaciones → 422 «Primero hay que imputar el trabajo realizado» y sigue `en_reparacion`; con la orden `pendiente_conformidad` → 409 «La orden está pendiente de conformidad; solo se puede terminar una orden en reparación»; `en_espera_piezas` → 409; otro mecánico → 403; el jefe → 200
- [x] T027 [US4] Quitar el `#[ignore]` de `orden_en_espera_no_se_puede_terminar_ni_imputar` en `backend/tests/spec006_pedidos.rs` (spec-006 T016) y comprobar que pasa con los endpoints de spec-005

### Implementation for User Story 4

- [x] T028 [US4] En `backend/src/rutas/imputaciones.rs` implementar `POST /api/ordenes/{id}/terminar` en una transacción (research D6): bloquear la orden (404); `exigir_mecanico_de`; si no está `en_reparacion` → 409 «La orden está {estado.texto()}; solo se puede terminar una orden en reparación»; si no tiene imputaciones → 422 «Primero hay que imputar el trabajo realizado»; `cambiar_estado(tx, id, &[EnReparacion], Terminada)`; `commit`, publicar el evento después del `commit` y responder 200 con `resumen_imputaciones`
- [x] T029 [US4] En `frontend/src/routes/ordenes/[id]/Imputaciones.svelte` añadir el botón «Terminar reparación» (solo si `puedeEditar`, la orden está «en reparación» y hay al menos una imputación), con confirmación «¿Marcar la reparación como terminada? Ya no se podrán imputar más horas ni piezas», que llama a `terminar`, muestra el `error` tal cual y llama a `onCambio`
- [ ] T030 *(pendiente de spec-002: `Imputaciones.svelte` y `resumen_imputaciones(conexion, orden_id)` ya existen y están probados)* [US4] Integrar en la ficha de la orden: en `frontend/src/routes/ordenes/[id]/+page.svelte` (spec-002, Persona A) incluir `<Imputaciones>` y en `GET /api/ordenes/{id}` (`backend/src/rutas/ordenes.rs`) llenar `coste_acumulado`, `total_presupuestado` e `imputaciones` con `resumen_imputaciones` (punto de contacto con la Persona A: solo se añaden esas llamadas)

**Checkpoint**: todas las historias funcionan en el backend (quickstart §5 pasos 1–6).

---

## Phase 7: Polish & Cross-Cutting Concerns

- [x] T031 [P] Revisar que todos los mensajes de `backend/src/rutas/imputaciones.rs`, `backend/src/lineas.rs` y las pantallas están en español de España y que importes y horas usan `frontend/src/lib/formato.ts` («1,25 h», «90,00 €»)
- [x] T032 [P] Revisar que no hay forma de anular ni corregir imputaciones (ni endpoint `DELETE`/`PUT` ni botón «Quitar» en `Imputaciones.svelte`), ni salida de `pendiente_conformidad` distinta de la aceptación de spec-004 (`grep` de `cambiar_estado` en `backend/src/rutas/imputaciones.rs`: solo `→ pendiente_conformidad` y `→ terminada`) (principio III, límite conocido)
- [x] T033 Ejecutar `cd backend && cargo test` (todas las specs) y `cd frontend && npm run check && npm test && npm run build`
- [ ] T034 *(29/10/2026: pasos 2–6 comprobados contra la API con la orden demo 3, con la aceptación de spec-004 simulada por SQL; el paso 1 lo cubre la prueba `no_se_termina_sin_imputaciones_ni_fuera_de_reparacion`. Falta el recorrido en pantalla, que necesita spec-002 y spec-004)* Recorrer a mano [quickstart §5](../000-base-comun/quickstart.md) como `luis`, mirar el stock en `/almacen` como `pepe` y anotar que se cumplen SC-001…SC-004 (necesita T030 y spec-004 para aceptar la ampliación desde Mailpit)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: depende de la base común, spec-006 y spec-003 (todo en `main`).
- **Foundational (Phase 2)**: depende de Setup y bloquea todas las historias. T005 depende de T004;
  T010 toca `Presupuesto.svelte` de spec-003.
- **US1 → US2**: US2 añade la regla del 10 % al mismo endpoint de US1.
- **US3**: depende de US2 (orden bloqueada) y de los endpoints de spec-003.
- **US4**: depende de US1 (necesita imputaciones); independiente de US2 y US3.
- **T030**: necesita la ficha y `GET /api/ordenes/{id}` de spec-002 (Persona A).
- **T034**: necesita T030 y la aceptación de spec-004 (Persona A).

### Within Each User Story

- Pruebas primero, y deben fallar antes de implementar.
- Backend antes que pantalla.
- Todas las tareas de backend de spec-005 tocan `imputaciones.rs`, así que van en secuencia.

### Parallel Opportunities

- T003 en paralelo con T002; T008, T009 y T010 en paralelo (tras T004–T007 en el backend).
- Las pruebas de cada historia (marcadas [P]) se escriben en paralelo entre sí.
- La pantalla de una historia puede hacerse en paralelo con el backend de la siguiente.

---

## Parallel Example: User Story 1

```bash
Task: "T011 Pruebas de imputar pieza y horas en backend/tests/spec005_imputaciones.rs"
Task: "T012 Pruebas de stock insuficiente en backend/tests/spec005_imputaciones.rs"
Task: "T013 Pruebas de validación compartida con 003 en backend/tests/spec005_imputaciones.rs"
Task: "T014 Pruebas de estados y permisos en backend/tests/spec005_imputaciones.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 y Phase 2 (incluida la refactorización compartida con spec-003).
2. US1 (imputar con descuento de stock y acumulado).
3. **Validar** con T011–T015.

### Incremental Delivery

1. US1 → imputaciones.
2. US2 → bloqueo del 10 % (sin él, US1 no cumple la HU-05).
3. US4 → terminar (desbloquea spec-007, facturar).
4. US3 → reanudación tras la ampliación (sobre todo pruebas; el código ya está en spec-003).

---

## Notes

- [P] = otro fichero y sin dependencias pendientes.
- Hacer commit al terminar cada tarea o grupo lógico.
- Los cambios en ficheros compartidos (migración `0003`, `lineas.rs`, `presupuestos.rs`,
  `tipos.ts`, `componentes/FormularioLinea.svelte`, `Presupuesto.svelte`, `ordenes/[id]/+page.svelte`,
  `ordenes.rs`) los revisa también la Persona A.
