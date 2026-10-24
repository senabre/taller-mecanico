---

description: "Tareas de spec-003 · Registrar diagnóstico y preparar presupuesto"
---

# Tasks: spec-003 · Registrar diagnóstico y preparar presupuesto

**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md) y los documentos
compartidos [data-model.md](../000-base-comun/data-model.md),
[contracts/api.md](../000-base-comun/contracts/api.md) y
[quickstart.md](../000-base-comun/quickstart.md).

**Tests**: se incluyen. El plan maestro (research R13) exige pruebas `#[sqlx::test]` para cada regla
de negocio, y el plan de esta spec enumera 7 grupos en `backend/tests/spec003_presupuestos.rs`.

**Organización**: una fase por historia de usuario de la spec. Todo texto visible y todo mensaje
de error, en español de España; importes con `formato.ts` (1.234,56 €) y fechas dd/mm/aaaa.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: se puede hacer en paralelo (otro fichero, sin depender de tareas pendientes)
- **[Story]**: historia de usuario (US1…US3)

## Path Conventions

Aplicación web: `backend/src/`, `backend/tests/`, `backend/migrations/`, `frontend/src/`.

---

## Phase 1: Setup (base común)

**Purpose**: comprobar que la base común y spec-006 dan lo que necesita esta spec, y añadir la
única migración propia.

- [x] T001 Comprobar que existen y se pueden usar: `backend/src/estados.rs` (transición `recibida → en_diagnostico` en `TRANSICIONES`), `Usuario::exigir_mecanico_de` en `backend/src/auth.rs` (el jefe pasa siempre), `dinero::importe_linea` y `dinero::totales` en `backend/src/dinero.rs`, `Correo::enviar(para, asunto, cuerpo) -> bool` en `backend/src/correo.rs`, `config.precio_hora`, `config.iva_porcentaje` y `config.url_publica` en `backend/src/config.rs`, `auth::nuevo_token()` y `fechas::hoy()`, y `GET /api/piezas?q=` de spec-006 en `backend/src/rutas/almacen.rs`
- [x] T002 Crear `backend/migrations/0002_spec003_lineas.sql` (revisada por las dos personas) con `ALTER TABLE linea_presupuesto ADD CONSTRAINT linea_presupuesto_horas_cuarto CHECK (tipo <> 'mano_obra' OR cantidad * 4 = trunc(cantidad * 4))` y `ADD CONSTRAINT linea_presupuesto_piezas_enteras CHECK (tipo <> 'pieza' OR cantidad = trunc(cantidad))` (research D1; data-model: «mano de obra en múltiplos de 0,25 h; pieza en unidades enteras»), y aplicarla con `cargo sqlx migrate run`
- [x] T003 [P] Comprobar en `backend/seed/demo.sql` que existen la orden 1 `recibida` de `luis` (diagnóstico), la orden 4 `en_diagnostico` de `luis` sin presupuesto (presupuesto), la orden 5 con el presupuesto `emitido` de `marta`, la orden 7 `pendiente_conformidad` (ampliación) y la pieza `PF-123` a 25,00 €, y que los datos demo se siguen cargando sin error con la migración `0002` aplicada

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: esqueleto propio de spec-003 que usan todas sus historias.

**⚠️ CRITICAL**: ninguna historia puede empezar sin esta fase.

- [x] T004 Crear `backend/src/rutas/presupuestos.rs` con una función `rutas()` que devuelva un `Router` vacío del estado de la app, y registrarla en `backend/src/rutas/mod.rs` (`pub mod presupuestos;` y `.merge(presupuestos::rutas())` dentro de `api()`); `main.rs` y `lib.rs` no se tocan
- [x] T005 En `backend/src/rutas/presupuestos.rs` añadir lo común a todas las historias: (a) `async fn orden_bloqueada(tx, orden_id) -> Resultado<(Estado, i64 /*mecanico_id*/, i64 /*numero*/)>` con `SELECT estado, mecanico_id, numero FROM orden WHERE id = $1 FOR UPDATE` y 404 «No existe la orden»; (b) el objeto `Presupuesto` del contrato `{id, orden_id, tipo, estado, caducado, lineas:[Linea], base, iva_porcentaje, iva, total, emitido_en, caduca_el, respondido_en, enlace}` con importes como string de 2 decimales (`rust_decimal::serde::str`) y `async fn presupuesto_por_id(conexion, id, config) -> Resultado<Presupuesto>`: en `borrador` los totales se calculan con `dinero::totales(importes, config.iva_porcentaje)`; si no, se leen los guardados; `caducado = estado == 'emitido' && caduca_el < fechas::hoy()`; `enlace = Some(format!("{url_publica}/presupuesto/{token}"))` si hay token (research D4, D6)
- [x] T006 [P] Añadir a `frontend/src/lib/tipos.ts` los tipos del contrato: `EstadoPresupuesto = 'borrador'|'emitido'|'aceptado'|'rechazado'`, `TipoPresupuesto = 'inicial'|'ampliacion'`, `Presupuesto {id, orden_id, tipo, estado, caducado: boolean, lineas: Linea[], base, iva_porcentaje, iva, total: Importe, emitido_en: Instante|null, caduca_el: Fecha|null, respondido_en: Instante|null, enlace: string|null, aviso_enviado?: boolean}` y `PresupuestoResumen {id, tipo, estado, caducado, total: Importe, emitido_en: Instante|null, caduca_el: Fecha|null}`
- [x] T007 [P] Crear `backend/tests/spec003_presupuestos.rs` con `mod comun;` y reutilizar sus utilidades (`app_prueba`, `estado_prueba`, `peticion`, `crear_usuario`, `entrar`, `insertar_usuario`, `crear_orden`); añadir solo las que faltan: `insertar_pieza(referencia, descripcion, precio, stock)`, `estado_de_orden`, `diagnostico_de`, `insertar_presupuesto(orden_id, estado, caduca_el)` por SQL directo, y una `app_con_eventos` como la de `backend/tests/spec006_pedidos.rs`
- [x] T008 [P] Crear `frontend/src/lib/componentes/TablaLineas.svelte` (compartido con 004, 005 y 007): props `lineas: Linea[]` y opcional `onquitar?: (linea) => void`; tabla con concepto, cantidad (horas «1,5 h» con `numero()` para mano de obra, «2 uds» para pieza), precio unitario e importe con `euros()`; si llega `onquitar`, cada fila tiene un botón «Quitar»; sin líneas, «Todavía no hay líneas»
- [x] T009 [P] Crear `frontend/src/lib/componentes/Totales.svelte` (compartido con 004, 005 y 007): prop `totales: Totales`; muestra «Base imponible», «IVA (21 %)» con el porcentaje de `iva_porcentaje` sin decimales sobrantes y «Total» en negrita, todo con `euros()`

**Checkpoint**: módulo registrado, migración aplicada, tipos y componentes listos y pruebas compilando.

---

## Phase 3: User Story 1 - Registrar el diagnóstico (Priority: P1) 🎯 MVP

**Goal**: el mecánico asignado guarda el diagnóstico de una orden `recibida` (pasa a «en
diagnóstico») y puede cambiarlo hasta que se emite un presupuesto (FR-001, FR-001a).

**Independent Test**: con la orden demo 1 (`recibida`, de `luis`), guardar «Pastillas delanteras
desgastadas» → la orden queda «en diagnóstico» con ese texto; cambiarlo → se guarda y sigue «en
diagnóstico».

### Tests for User Story 1

- [x] T010 [P] [US1] En `backend/tests/spec003_presupuestos.rs`: `PUT /api/ordenes/{id}/diagnostico` con la orden `recibida` → 200 `{id, numero, estado: "en_diagnostico", diagnostico}`, el diagnóstico guardado sin espacios iniciales ni finales y llega al tablero el evento `{id, estado: "en_diagnostico"}` (suscribirse con `estado.eventos.suscribir()` como en spec-006); cambiarlo con la orden `en_diagnostico` y solo un borrador → 200, texto nuevo, y **no** llega ningún evento
- [x] T011 [P] [US1] En `backend/tests/spec003_presupuestos.rs`: con un presupuesto `emitido` (también `rechazado`) en la orden → 409 «Ya se ha emitido un presupuesto para esta orden; el diagnóstico no se puede cambiar» y el diagnóstico no cambia; orden `en_reparacion` → 409 con el estado en el mensaje; diagnóstico vacío o solo espacios → 422; otro mecánico → 403; el jefe → 200; sin sesión → 401

### Implementation for User Story 1

- [x] T012 [US1] En `backend/src/rutas/presupuestos.rs` implementar `PUT /api/ordenes/{id}/diagnostico` en una transacción (research D2): recibir `{diagnostico}`, quitar espacios y rechazar vacío con 422 «Escribe el diagnóstico»; `orden_bloqueada`; `exigir_mecanico_de(mecanico_id)`; si `recibida` → `UPDATE orden SET diagnostico` y `cambiar_estado(tx, id, &[Recibida], EnDiagnostico)`; si `en_diagnostico` → 409 si `EXISTS (SELECT 1 FROM presupuesto WHERE orden_id = $1 AND estado <> 'borrador')`, si no solo `UPDATE orden SET diagnostico`; otro estado → 409 «La orden está {estado.texto()}; el diagnóstico solo se registra en órdenes recibidas o en diagnóstico»; `commit` y, **después del `commit`**, `cambio.publicar(&estado.eventos)` si hubo cambio de estado (R14); responder 200 `{id, numero, estado, diagnostico}`
- [x] T013 [US1] Crear `frontend/src/routes/ordenes/[id]/Diagnostico.svelte` (sección de la ficha de spec-002): props `orden` (`{id, estado, diagnostico}`), `puedeEditar: boolean` y `onguardado: () => void`; muestra el diagnóstico; si `puedeEditar` y la orden está «recibida» o «en diagnóstico», un `textarea` con el texto actual y el botón «Guardar diagnóstico» que llama a `PUT /ordenes/{id}/diagnostico`, muestra el `error` del servidor tal cual y, si va bien, llama a `onguardado` para que la ficha recargue

**Checkpoint**: US1 funciona sola y `cargo test spec003` pasa T010–T011.

---

## Phase 4: User Story 2 - Preparar el presupuesto línea a línea (Priority: P1)

**Goal**: crear un presupuesto en borrador, añadir líneas de mano de obra (0,25 h, concepto
escrito, precio por hora) y de piezas (unidades enteras, concepto «referencia descripción», precio
de venta), quitar líneas y ver base, IVA y total (FR-002…FR-006, FR-002a, FR-008).

**Independent Test**: en la orden demo 4, crear un presupuesto con «Cambio de pastillas» 1,5 h y
PF-123 × 2 → 60,00 € + 50,00 €; base 110,00 €, IVA 23,10 €, total 133,10 €; quitar la mano de
obra → 50,00 / 10,50 / 60,50.

### Tests for User Story 2

- [x] T014 [P] [US2] En `backend/tests/spec003_presupuestos.rs`: `POST /api/ordenes/{id}/presupuestos` con la orden `en_diagnostico` → 201 `tipo: "inicial"`, `estado: "borrador"`, sin líneas, `enlace: null`; con la orden `pendiente_conformidad` → `tipo: "ampliacion"`; con la orden `recibida` → 409; con un borrador ya creado → 409 «Ya hay un presupuesto en borrador para esta orden»; con uno `emitido` y `caduca_el` = hoy → 409 «Ya hay un presupuesto emitido sin caducar para esta orden»; con uno `emitido` y `caduca_el` = ayer → 201; otro mecánico → 403
- [x] T015 [P] [US2] En `backend/tests/spec003_presupuestos.rs`: con `TALLER_PRECIO_HORA` 40,00 (config de prueba) y PF-123 a 25,00 € con stock 0, añadir `{tipo:"mano_obra", concepto:"Cambio de pastillas", cantidad:"1.5"}` → línea con `cantidad "1.50"`, `precio_unitario "40.00"`, `importe "60.00"`; añadir `{tipo:"pieza", pieza_id, cantidad:"2", concepto:"otro"}` → concepto «PF-123 Pastillas freno» (se ignora el enviado), `precio_unitario "25.00"`, `importe "50.00"`; el presupuesto tiene `base "110.00"`, `iva_porcentaje "21.00"`, `iva "23.10"`, `total "133.10"`
- [x] T016 [P] [US2] En `backend/tests/spec003_presupuestos.rs`: mano de obra con `cantidad` "1.3", "0", "-1", "abc" o vacía → 422; concepto vacío o solo espacios → 422; pieza con `cantidad` "2.5" o "0" → 422; `pieza_id` inexistente → 422; `tipo` desconocido → 422; ninguna de ellas crea líneas. Además, insertar por SQL directo una línea de mano de obra de 1,30 h o de pieza de 2,5 uds falla por los `CHECK` de la migración `0002`
- [x] T017 [P] [US2] En `backend/tests/spec003_presupuestos.rs`: `DELETE /api/presupuestos/{id}/lineas/{linea_id}` de la mano de obra → 200 con solo la pieza y totales `50.00` / `10.50` / `60.50`; una línea de otro presupuesto → 404; otro mecánico → 403
- [x] T018 [P] [US2] En `backend/tests/spec003_presupuestos.rs` (redondeo del IVA, SC-002; con 0,25 h o unidades enteras y precios de 2 decimales los importes de línea son exactos, solo se redondea el IVA): (a) 0,25 h (10,00) + 3 uds a 0,35 € (1,05) + 7 uds a 1,15 € (8,05) → `base "19.10"` = suma de los `importe`, `iva "4.01"` (4,011), `total "23.11"`; (b) 1 ud a 2,50 € → `iva "0.53"` (0,525, mitad hacia arriba, R4), `total "3.03"`
- [x] T019 [P] [US2] En `backend/tests/spec003_presupuestos.rs`: `GET /api/presupuestos/{id}` lo puede leer cualquier rol con sesión (p. ej. `almacen`) → 200; id inexistente → 404

### Implementation for User Story 2

- [x] T020 [US2] En `backend/src/rutas/presupuestos.rs` implementar `POST /api/ordenes/{id}/presupuestos` en una transacción (research D5): `orden_bloqueada`; `exigir_mecanico_de`; tipo `inicial` si `en_diagnostico`, `ampliacion` si `pendiente_conformidad`, si no 409 «La orden está {estado.texto()}; no se puede preparar un presupuesto»; 409 «Ya hay un presupuesto emitido sin caducar para esta orden» si existe uno `emitido` con `caduca_el >= fechas::hoy()`; `INSERT` (estado `borrador` por defecto) y, si choca con `presupuesto_borrador_unico`, 409 «Ya hay un presupuesto en borrador para esta orden»; responder 201 con `presupuesto_por_id`
- [x] T021 [US2] En `backend/src/rutas/presupuestos.rs` implementar `GET /api/presupuestos/{id}` (cualquier rol con sesión) con `presupuesto_por_id`; 404 «No existe el presupuesto»
- [x] T022 [US2] En `backend/src/rutas/presupuestos.rs` implementar `POST /api/presupuestos/{id}/lineas` en una transacción (research D1, D3, D4): bloquear el presupuesto (`SELECT p.estado, o.mecanico_id FROM presupuesto p JOIN orden o ON o.id = p.orden_id WHERE p.id = $1 FOR UPDATE OF p`, 404 si no existe); `exigir_mecanico_de`; 409 «El presupuesto ya está emitido; no se puede modificar» si no está en `borrador`; validar `cantidad` (string o número → `Decimal`, > 0, ≤ 2 decimales) y según `tipo`: `mano_obra` → múltiplo de 0,25 o 422 «Las horas van en múltiplos de 0,25 (p. ej. 1,5)», concepto sin espacios y no vacío o 422 «Escribe el concepto de la mano de obra», `precio_unitario = config.precio_hora`; `pieza` → entero o 422 «Las unidades de pieza deben ser un número entero», pieza existente o 422 «Elige una referencia de pieza dada de alta», `concepto = format!("{referencia} {descripcion}")`, `precio_unitario = pieza.precio_venta`; otro `tipo` → 422; `importe = dinero::importe_linea(cantidad, precio_unitario)`; `INSERT` y responder 201 con `presupuesto_por_id`. No se comprueba el stock (caso límite de la spec)
- [x] T023 [US2] En `backend/src/rutas/presupuestos.rs` implementar `DELETE /api/presupuestos/{id}/lineas/{linea_id}` en una transacción: bloquear el presupuesto como en T022 (404, 403, 409 iguales); `DELETE FROM linea_presupuesto WHERE id = $2 AND presupuesto_id = $1`, 404 «No existe esa línea en el presupuesto» si no borra ninguna; responder 200 con `presupuesto_por_id`
- [x] T024 [US2] Crear `frontend/src/routes/ordenes/[id]/Presupuesto.svelte` (sección de la ficha de spec-002): props `ordenId`, `presupuestos: PresupuestoResumen[]`, `puedeEditar` y `onCambio`; carga con `GET /presupuestos/{id}` el más reciente; si no hay ninguno abierto y `puedeEditar`, botón «Nuevo presupuesto»; en borrador, formulario «Añadir línea» con selector Mano de obra / Pieza: mano de obra → concepto y horas (`step="0.25"`, escritas con coma y enviadas con punto); pieza → buscador `GET /piezas?q=` como el de `frontend/src/routes/pedidos/+page.svelte` y unidades (`step="1"`); muestra `TablaLineas` (con `onquitar` solo en borrador) y `Totales`; cada error del servidor se muestra tal cual junto al formulario

**Checkpoint**: US2 funciona (quickstart §3 pasos 2–5) y pasan T014–T019.

---

## Phase 5: User Story 3 - Emitir el presupuesto (Priority: P1)

**Goal**: emitir un presupuesto con líneas: se fijan totales, `caduca_el` (+15 días) y el token,
queda bloqueado y se envía el correo con el enlace (FR-007, SC-003).

**Independent Test**: emitir el presupuesto de la orden 4 → «Válido hasta» = hoy + 15 días, ya no
se pueden añadir ni quitar líneas y en Mailpit llega el correo con el enlace.

### Tests for User Story 3

- [x] T025 [P] [US3] En `backend/tests/spec003_presupuestos.rs`: `POST /api/presupuestos/{id}/emitir` con líneas → 200, `estado: "emitido"`, `caduca_el` = `fechas::hoy() + 15` (formato `YYYY-MM-DD`), `emitido_en` es hoy en Madrid, `enlace` = `{url_publica}/presupuesto/{token}` con un token de 43 caracteres base64url, y en la base de datos `base`, `iva`, `total` e `iva_porcentaje` guardados (110,00 / 23,10 / 133,10 / 21,00); `aviso_enviado` es un booleano
- [x] T026 [P] [US3] En `backend/tests/spec003_presupuestos.rs`: emitir sin líneas → 422 «El presupuesto no tiene líneas»; emitir dos veces → 409; tras emitir, añadir o quitar líneas → 409 «El presupuesto ya está emitido; no se puede modificar»; otro mecánico → 403; con un `EstadoApp` cuyo `smtp_url` apunta a un puerto cerrado → 200 igualmente, `estado: "emitido"` y `aviso_enviado: false` (el correo se envía después del `commit` y su fallo no deshace nada, R10)
- [x] T027 [P] [US3] En `backend/tests/spec003_presupuestos.rs`: `presupuestos_de_orden(conexion, orden_id)` devuelve solo los de esa orden, del más antiguo al más reciente, con `caducado: true` en un `emitido` con `caduca_el` = ayer y `false` en uno con `caduca_el` = hoy

### Implementation for User Story 3

- [x] T028 [US3] En `backend/src/rutas/presupuestos.rs` implementar `POST /api/presupuestos/{id}/emitir` (research D6): en una transacción, bloquear el presupuesto **y su orden** (`FOR UPDATE OF p, o`, para que no se cuele un cambio de diagnóstico a la vez; analyze C1) con 404 y 403 como en T022; 409 «El presupuesto ya está emitido» si no está en `borrador`; leer los importes de sus líneas y, si no hay, 422 «El presupuesto no tiene líneas»; `t = dinero::totales(importes, config.iva_porcentaje)`; `UPDATE presupuesto SET estado = 'emitido', base, iva, total, iva_porcentaje, emitido_en = now(), caduca_el = fechas::hoy() + 15 días, token = auth::nuevo_token()`; leer el correo del cliente y la matrícula (`orden → vehiculo → cliente`); `commit`; **después del `commit`**, `estado.correo.enviar(email, "Presupuesto de su vehículo {matricula}", cuerpo)` con el cuerpo en español: saludo con el nombre del cliente, la matrícula, el enlace, «Válido hasta dd/mm/aaaa» (chrono `%d/%m/%Y`) y el nombre del taller, sin importes (spec-004 FR-001 solo pide el enlace; analyze U1); responder 200 con `presupuesto_por_id` + `aviso_enviado` (resultado de `enviar`)
- [x] T029 [US3] En `backend/src/rutas/presupuestos.rs` añadir la función pública `presupuestos_de_orden(conexion, orden_id, iva_porcentaje)` (el IVA de la configuración, para el total de un borrador) -> Resultado<Vec<PresupuestoResumen>>` con `{id, tipo, estado, caducado, total, emitido_en, caduca_el}` (en borrador, `total` calculado como en `presupuesto_por_id`), del más antiguo al más reciente, para que spec-002 llene `presupuestos` de `GET /api/ordenes/{id}` (research D8)
- [x] T030 [US3] En `frontend/src/routes/ordenes/[id]/Presupuesto.svelte` añadir, en borrador con al menos una línea, el botón «Emitir presupuesto» (con confirmación «Una vez emitido no se podrá modificar») que llama a `emitir`; ya emitido: «Emitido el dd/mm/aaaa» y «Válido hasta dd/mm/aaaa» (o «Caducado el dd/mm/aaaa» si `caducado`), sin formulario ni botones «Quitar»; si `aviso_enviado` es `false`, el aviso «No se pudo enviar el correo al cliente». El enlace y el botón «Copiar enlace» **no** se hacen aquí: son de spec-004 FR-008 (Persona A), que los añade en esta misma sección (analyze I1)
- [ ] T031 *(pendiente de spec-002: `Diagnostico.svelte`, `Presupuesto.svelte` y `presupuestos_de_orden(conexion, orden_id, estado.config.iva_porcentaje)` ya existen y están probados)* [US3] Integrar las secciones en la ficha de la orden: en `frontend/src/routes/ordenes/[id]/+page.svelte` (spec-002, Persona A) incluir `<Diagnostico>` y `<Presupuesto>` con `puedeEditar` = el usuario es el mecánico asignado o el jefe, y en `GET /api/ordenes/{id}` (`backend/src/rutas/ordenes.rs`) llenar `presupuestos` con `presupuestos_de_orden` (punto de contacto con la Persona A: solo se añaden esas llamadas)

**Checkpoint**: todas las historias funcionan (quickstart §3 pasos 6 y 7).

---

## Phase 6: Polish & Cross-Cutting Concerns

- [x] T032 [P] Revisar que todos los mensajes de error de `backend/src/rutas/presupuestos.rs`, el correo y las pantallas están en español de España, que importes y fechas usan `frontend/src/lib/formato.ts` (y en el correo, formato español) y que el porcentaje de IVA se ve como «21 %»
- [x] T033 [P] Revisar que no hay descuentos, edición de líneas, plantillas ni catálogos, y que ningún endpoint de esta spec cambia el estado de la orden salvo `recibida → en_diagnostico` (`grep` de `cambiar_estado` en `backend/src/rutas/presupuestos.rs`) (principio III)
- [x] T034 Ejecutar `cd backend && cargo test` (todas las specs) y `cd frontend && npm run check && npm test && npm run build`
- [ ] T035 *(22/10/2026: pasos 1–7 comprobados contra la API con los datos demo (órdenes 1 y 4), correo recibido en Mailpit; falta el recorrido en pantalla, que necesita la ficha de spec-002)* Recorrer a mano [quickstart §3](../000-base-comun/quickstart.md) completo como `luis` (orden demo 1 y 4) y como `paco`, mirar el correo en Mailpit (http://localhost:8025) y anotar que se cumplen SC-001…SC-004

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: depende de la base común y de spec-006 (ya en `main`).
- **Foundational (Phase 2)**: depende de Setup y bloquea todas las historias.
- **US1, US2, US3**: después de Phase 2. US2 usa órdenes `en_diagnostico` (datos de prueba por SQL),
  así que no espera a US1. US3 necesita presupuestos con líneas, que se crean con US2 o por SQL.
- **T031**: necesita la ficha de la orden y `GET /api/ordenes/{id}` de spec-002 (Persona A).
- **Polish**: al final; T035 necesita T031.

### User Story Dependencies

- **US1 (P1)**: independiente.
- **US2 (P1)**: independiente en el backend (órdenes por SQL); en la interfaz, tras US1 (la orden
  debe estar «en diagnóstico»).
- **US3 (P1)**: depende de US2 (emitir un presupuesto con líneas).

### Within Each User Story

- Pruebas primero, y deben fallar antes de implementar.
- Backend antes que pantalla.
- Todas las tareas de backend tocan `presupuestos.rs`, así que van en secuencia.

### Parallel Opportunities

- T003 en paralelo con T002; T006, T007, T008 y T009 en paralelo.
- Las pruebas de cada historia (marcadas [P]) se escriben en paralelo entre sí.
- Los componentes `TablaLineas` y `Totales` (T008, T009) en paralelo con el backend de US1.
- La pantalla de una historia puede hacerse en paralelo con el backend de la siguiente.

---

## Parallel Example: User Story 2

```bash
Task: "T014 Pruebas de crear presupuesto (tipo, uno abierto) en backend/tests/spec003_presupuestos.rs"
Task: "T015 Pruebas de líneas e importes en backend/tests/spec003_presupuestos.rs"
Task: "T016 Pruebas de cantidades y conceptos no válidos en backend/tests/spec003_presupuestos.rs"
Task: "T018 Prueba de redondeo en backend/tests/spec003_presupuestos.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 y Phase 2.
2. US1 (diagnóstico).
3. **Validar** con las pruebas T010–T011 (en pantalla, cuando exista la ficha de spec-002).

### Incremental Delivery

1. US1 → diagnóstico.
2. US2 → presupuesto en borrador con totales (el núcleo de HU-03).
3. US3 → emitir; desbloquea spec-004 (Persona A) con presupuestos reales y spec-005 (ampliaciones).

---

## Notes

- [P] = otro fichero y sin dependencias pendientes.
- Hacer commit al terminar cada tarea o grupo lógico.
- Los cambios en ficheros compartidos (migración `0002`, `tipos.ts`, `componentes/`,
  `ordenes/[id]/+page.svelte`, `ordenes.rs`) los revisa también la Persona A.
