---

description: "Tareas de spec-007 · Facturar la orden y avisar al cliente"
---

# Tasks: spec-007 · Facturar la orden y avisar al cliente

**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md) y los documentos
compartidos [data-model.md](../000-base-comun/data-model.md),
[contracts/api.md](../000-base-comun/contracts/api.md) y
[quickstart.md](../000-base-comun/quickstart.md).

**Tests**: se incluyen. El plan maestro (research R13) exige pruebas `#[sqlx::test]` para cada regla
de negocio, y el plan de esta spec enumera 8 grupos en `backend/tests/spec007_facturas.rs`.

**Organización**: una fase por historia de usuario de la spec. Todo texto visible y todo mensaje
de error, en español de España; importes «1.234,56 €» y fechas dd/mm/aaaa.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: se puede hacer en paralelo (otro fichero, sin depender de tareas pendientes)
- **[Story]**: historia de usuario (US1…US3)

## Path Conventions

Aplicación web: `backend/src/`, `backend/tests/`, `frontend/src/`.

---

## Phase 1: Setup

**Purpose**: comprobar lo que ya existe en `main` (esta spec no necesita migraciones).

- [x] T001 Comprobar que existen y se pueden usar: las tablas `factura` (con `numero BIGINT NOT NULL UNIQUE`, `orden_id … UNIQUE`, `aviso_enviado BOOLEAN NOT NULL DEFAULT false`) y `linea_factura` de `backend/migrations/0001_esquema_inicial.sql`, `contador::siguiente(tx, contador::FACTURA)` en `backend/src/contador.rs`, la transición `terminada → facturada` en `TRANSICIONES` de `backend/src/estados.rs`, `matricula::normalizar_matricula`, `dinero::totales`, `Correo::enviar`, `config.taller_nombre`, `config.taller_nif` y `config.taller_direccion`, y `Usuario::exigir_rol`
- [x] T002 [P] Comprobar en `backend/seed/demo.sql` que existen la orden 8 `terminada` de `marta` con dos imputaciones (110,00), la orden 9 `facturada` con su factura nº 1 (matrícula `9999LLL`) y que el contador `factura` queda en 1

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: esqueleto propio de spec-007 y lo compartido que necesitan sus historias.

**⚠️ CRITICAL**: ninguna historia puede empezar sin esta fase.

- [x] T003 Crear `backend/src/rutas/facturas.rs` con `rutas()` que devuelva un `Router` vacío y registrarla en `backend/src/rutas/mod.rs` (`pub mod facturas;` y `.merge(facturas::rutas())` dentro de `api()`)
- [x] T004 En `backend/src/rutas/facturas.rs` añadir los objetos del contrato: `LineaFactura {concepto, cantidad, precio_unitario, importe}` (importes y cantidad como string de 2 decimales, `rust_decimal::serde::str`) y `Factura {id, numero, orden_id, fecha: NaiveDate, taller: {nombre, nif, direccion}, cliente_nombre, matricula, lineas: Vec<LineaFactura>, base, iva_porcentaje, iva, total, aviso_enviado}`, y `async fn factura_por_id(conexion, id, config) -> Resultado<Factura>` (líneas por `id`; 404 «No existe la factura»)
- [x] T005 [P] Añadir a `backend/src/formato.rs` `pub fn euros(importe: Decimal) -> String` con formato español y dos decimales («157,30 €», «1.234,56 €», «0,00 €», reutilizando `numero()` para los miles) y su prueba en el `mod pruebas` del mismo fichero (research D2)
- [x] T006 [P] Añadir a `frontend/src/lib/tipos.ts` `LineaFactura {concepto, cantidad: string, precio_unitario: Importe, importe: Importe}`, `Factura extends Totales {id, numero, orden_id, fecha: Fecha, taller: {nombre, nif, direccion}, cliente_nombre, matricula, lineas: LineaFactura[], aviso_enviado: boolean}` y `Historial {matricula, marca, modelo, reparaciones: {orden_numero, fecha: Fecha, km, diagnostico: string|null, factura_numero, lineas: LineaFactura[], total: Importe}[]}`
- [x] T007 [P] Cambiar `frontend/src/lib/componentes/TablaLineas.svelte` para aceptar líneas sin `tipo` ni `id` (`LineaFactura`): con `tipo`, la cantidad sigue saliendo «1,5 h» / «2 uds»; sin `tipo`, solo el número («1,5»); la clave del `each` pasa a ser el índice si no hay `id`. Presupuesto e Imputaciones no cambian de aspecto (research D6)
- [x] T008 [P] Crear `backend/tests/spec007_facturas.rs` con `mod comun;` y reutilizar sus utilidades (`estado_prueba`, `peticion`, `crear_usuario`, `entrar`, `insertar_usuario`, `crear_orden`); añadir solo: `app_con_eventos`, `app_sin_correo` (SMTP a un puerto cerrado, como en `backend/tests/spec003_presupuestos.rs`), `imputar_sql(orden_id, tipo, concepto, cantidad, precio)` (inserta la imputación con su importe), `estado_de_orden` y `contador_factura`

**Checkpoint**: módulo registrado, objetos y utilidades listos, `formato::euros` probado.

---

## Phase 3: User Story 1 - Generar la factura de una orden terminada (Priority: P1) 🎯 MVP

**Goal**: recepción factura una orden `terminada`; la factura copia las imputaciones, lleva número
correlativo, fecha y totales, la orden pasa a «facturada» y la factura se ve e imprime (FR-001…FR-005,
FR-008).

**Independent Test**: facturar la orden demo 8 → «Factura nº 2» con sus dos líneas, base 110,00 €,
IVA 23,10 €, total 133,10 €; la orden queda «facturada»; la vista de la factura se imprime sola.

### Tests for User Story 1

- [x] T009 [P] [US1] En `backend/tests/spec007_facturas.rs`: orden `terminada` con 2 h a 40,00 y PF-123 × 2 a 25,00 → `POST /api/ordenes/{id}/factura` como recepción → 201 con las dos líneas copiadas en el orden en que se imputaron, `base "130.00"`, `iva_porcentaje "21.00"`, `iva "27.30"`, `total "157.30"`, `fecha` = hoy, `cliente_nombre` y `matricula` de la orden, `taller` con los datos de la configuración; la orden queda `facturada` y llega al tablero `{id, estado: "facturada"}`
- [x] T010 [P] [US1] En `backend/tests/spec007_facturas.rs` (correlativo, FR-004): dos facturas seguidas tienen números consecutivos; dos facturas de órdenes distintas lanzadas a la vez (`tokio::join!`) reciben números distintos y seguidos, sin repeticiones; un intento rechazado (orden sin imputaciones) no gasta número. Que una transacción cancelada no deja hueco ya lo prueba `un_rollback_no_deja_hueco` en `backend/tests/spec000_contador.rs` (analyze U1)
- [x] T011 [P] [US1] En `backend/tests/spec007_facturas.rs`: orden `en_reparacion`, `pendiente_conformidad` o `en_diagnostico` → 409 «La orden está {estado}; solo se puede facturar una orden terminada»; facturar dos veces → la segunda 409 «La orden está facturada; …» y hay una sola factura; orden `terminada` sin imputaciones (insertada por SQL) → 422 «La orden no tiene imputaciones; no se puede facturar» y la orden sigue `terminada`; mecánico → 403; jefe → 201; orden inexistente → 404
- [x] T012 [P] [US1] En `backend/tests/spec007_facturas.rs`: `GET /api/facturas/{id}` como recepción → 200 con la misma factura; mecánico → 403; id inexistente → 404. `factura_de_orden(conexion, orden_id)` devuelve `Some(id)` para la orden facturada y `None` para una sin factura

### Implementation for User Story 1

- [x] T013 [US1] En `backend/src/rutas/facturas.rs` implementar `POST /api/ordenes/{id}/factura` (research D1): `exigir_rol(&[Rol::Recepcion])`; en una transacción, `SELECT o.estado, c.nombre, c.email, v.matricula FROM orden o JOIN vehiculo v … JOIN cliente c … WHERE o.id = $1 FOR UPDATE OF o` (404 «No existe la orden»); si no está `terminada` → 409 «La orden está {estado.texto()}; solo se puede facturar una orden terminada»; leer las imputaciones (`concepto, cantidad, precio_unitario, importe` ordenadas por `creada_en, id`) y, si no hay, 422 «La orden no tiene imputaciones; no se puede facturar»; `numero = contador::siguiente(&mut tx, contador::FACTURA)`; `t = dinero::totales(importes, config.iva_porcentaje)`; `INSERT INTO factura (numero, orden_id, fecha, cliente_nombre, matricula, iva_porcentaje, base, iva, total)` con `fecha = fechas::hoy()` (si choca con el `UNIQUE` de `orden_id` → 409 «Esta orden ya tiene factura»); una `linea_factura` por imputación; `cambiar_estado(tx, id, &[Terminada], Facturada)`; `commit`; publicar el evento después del `commit`; responder 201 con `factura_por_id` (el aviso llega en US2)
- [x] T014 [US1] En `backend/src/rutas/facturas.rs` implementar `GET /api/facturas/{id}` (`exigir_rol(&[Rol::Recepcion])`) con `factura_por_id`, y la función pública `factura_de_orden(conexion, orden_id) -> Resultado<Option<i64>>` para que spec-002 llene `factura_id` de `GET /api/ordenes/{id}` (research D7)
- [x] T015 [US1] Crear `frontend/src/routes/facturas/[id]/+page.svelte`: carga `GET /facturas/{id}` y muestra «Factura nº {numero}», «Fecha: dd/mm/aaaa» (`fecha()`), el taller (nombre, NIF, dirección), «Cliente: {cliente_nombre}» y «Matrícula: {matricula}», `TablaLineas` con las líneas y `Totales`; un botón «Imprimir» que llama a `window.print()`; y en un `<style>` de la página `@media print { :global(header), .no-imprimir { display: none } }` para que la impresión salga solo con la factura (research D3); los errores del servidor se muestran tal cual
- [x] T016 [US1] Crear `frontend/src/routes/ordenes/[id]/Factura.svelte` (sección de la ficha de spec-002): props `orden: {id, estado}`, `facturaId: number|null`, `puedeFacturar` (recepción o jefe) y `onCambio`; si `puedeFacturar` y la orden está «terminada» y sin factura, botón «Facturar» con confirmación «¿Emitir la factura? Ya no se podrá modificar» que llama a `POST /ordenes/{id}/factura` y a `onCambio`; si hay factura, carga `GET /facturas/{facturaId}` y muestra «Factura nº N · dd/mm/aaaa · {total}» y el enlace «Ver e imprimir» a `/facturas/{id}`; en cualquier otro estado, nada (US1/AC5)
- [ ] T017 *(pendiente de spec-002: `Factura.svelte` y `factura_de_orden(conexion, orden_id)` ya existen y están probados)* [US1] Integrar en la ficha de la orden: en `frontend/src/routes/ordenes/[id]/+page.svelte` (spec-002, Persona A) incluir `<Factura>` y en `GET /api/ordenes/{id}` (`backend/src/rutas/ordenes.rs`) llenar `factura_id` con `factura_de_orden` (punto de contacto con la Persona A: solo se añaden esas llamadas)

**Checkpoint**: US1 funciona y pasan T009–T012; la vista `/facturas/{id}` se prueba ya en el navegador con la factura demo nº 1.

---

## Phase 4: User Story 2 - Avisar al cliente para que recoja el coche (Priority: P1)

**Goal**: al emitirse la factura, el cliente recibe «Su vehículo … está listo para recoger» con
el total; si el correo falla, la factura se emite igual y se ve que el aviso no salió (FR-006).

**Independent Test**: facturar la orden demo 8 → en Mailpit llega «Su vehículo 8888KKK está listo
para recoger» con «133,10 €»; con el correo caído, la factura se emite y la orden muestra «No se
pudo avisar al cliente».

### Tests for User Story 2

- [x] T018 [P] [US2] En `backend/tests/spec007_facturas.rs`: con el SMTP de prueba (Mailpit), la respuesta y la base de datos tienen `aviso_enviado = true`; con `app_sin_correo` → 201, la factura está emitida, la orden `facturada` y `aviso_enviado = false` en la respuesta y en la base de datos

### Implementation for User Story 2

- [x] T019 [US2] En `backend/src/rutas/facturas.rs`, **después del `commit`** de T013 (research D2): `estado.correo.enviar(email, &format!("Su vehículo {matricula} está listo para recoger"), cuerpo)` con el cuerpo en español (saludo con el nombre del cliente, que ya puede recoger el vehículo {matricula}, «Total de la factura nº {numero}: {formato::euros(total)}» y el nombre del taller); `UPDATE factura SET aviso_enviado = $2 WHERE id = $1` con el resultado; la respuesta lleva el `aviso_enviado` real. Si el correo falla no se deshace nada (R10)
- [x] T020 [US2] En `frontend/src/routes/ordenes/[id]/Factura.svelte`, si la factura tiene `aviso_enviado = false`, mostrar el aviso «No se pudo avisar al cliente por correo: llámale para decirle que puede recoger el coche»

**Checkpoint**: US1 y US2 funcionan juntas y pasa T018.

---

## Phase 5: User Story 3 - Historial del vehículo por matrícula (Priority: P2)

**Goal**: cualquier empleado escribe una matrícula y ve las reparaciones facturadas de ese
vehículo, de la más reciente a la más antigua (FR-007).

**Independent Test**: en `/historial`, buscar `9999 lll` (datos demo) → aparece la reparación de
la orden 9 con fecha, km, diagnóstico y líneas; buscar una matrícula que no existe → «No hay
historial para esa matrícula».

### Tests for User Story 3

- [x] T021 [P] [US3] En `backend/tests/spec007_facturas.rs`: un vehículo con dos órdenes facturadas (la segunda con fecha posterior) → `GET /api/vehiculos/{matricula}/historial` devuelve `matricula`, `marca`, `modelo` y dos `reparaciones` con la más reciente primero, cada una con `orden_numero`, `fecha`, `km`, `diagnostico`, `factura_numero`, `lineas` y `total`; las órdenes del vehículo sin facturar no aparecen
- [x] T022 [P] [US3] En `backend/tests/spec007_facturas.rs`: la matrícula escrita como `1234 bcd` o `1234-BCD` encuentra `1234BCD`; un vehículo sin facturas → 200 con `reparaciones: []`; una matrícula que no existe → 404 «No hay ningún vehículo con la matrícula …»; `"--"` → 422 «Matrícula no válida»; cualquier rol con sesión (mecánico, almacén) → 200; sin sesión → 401

### Implementation for User Story 3

- [x] T023 [US3] En `backend/src/rutas/facturas.rs` implementar `GET /api/vehiculos/{matricula}/historial` (cualquier rol con sesión; research D5): `normalizar_matricula` (422 «Matrícula no válida»); `SELECT … FROM vehiculo WHERE matricula = $1` (404 «No hay ningún vehículo con la matrícula {m}»); las facturas de sus órdenes con `o.numero, f.fecha, o.km, o.diagnostico, f.numero, f.total` ordenadas por `f.fecha DESC, f.numero DESC`, cada una con sus `linea_factura` por `id`
- [x] T024 [US3] Crear `frontend/src/routes/historial/+page.svelte`: campo «Matrícula» y botón «Buscar» que llama a `GET /vehiculos/{matricula}/historial` (con `encodeURIComponent`); muestra «{matricula} · {marca} {modelo}» y, por cada reparación, «Orden {orden_numero} · Factura nº {factura_numero} · {fecha dd/mm/aaaa} · {numero(km)} km», el diagnóstico, `TablaLineas` y «Total {euros(total)}»; con `reparaciones` vacías o un 404, «No hay historial para esa matrícula»; un 422 se muestra tal cual

**Checkpoint**: todas las historias funcionan; `/historial` se prueba ya en el navegador.

---

## Phase 6: Polish & Cross-Cutting Concerns

- [x] T025 [P] Revisar que todos los mensajes de `backend/src/rutas/facturas.rs`, el correo y las pantallas están en español de España, con importes «1.234,56 €» (`formato.ts` en pantalla, `formato::euros` en el correo) y fechas dd/mm/aaaa
- [x] T026 [P] Revisar que no hay forma de modificar ni borrar una factura (ni endpoint `PUT`/`DELETE` ni botón), ni cobros, reenvío del aviso o envío de la factura por correo, y que el único cambio de estado es `terminada → facturada` (`grep` de `cambiar_estado` en `backend/src/rutas/facturas.rs`) (principio III)
- [x] T027 Ejecutar `cd backend && cargo test` (todas las specs) y `cd frontend && npm run check && npm test && npm run build`
- [ ] T028 *(29/10/2026: facturada la orden demo 8 por la API (Factura nº 2, 133,10 €), correo recibido en Mailpit, historial de 8888KKK y 9999LLL y matrícula inexistente comprobados; falta que una persona revise `/facturas/{id}` e `/historial` en el navegador)* Recorrer en el navegador lo que ya se puede: `/facturas/{id}` de la factura demo nº 1 (ver e imprimir, con vista previa sin menú) y `/historial` (`9999 lll`, una matrícula inexistente) como `ana` y como `luis`; facturar la orden demo 8 por la API, comprobar el correo en Mailpit y su factura en `/facturas/{id}`; anotar SC-001…SC-004 (el botón «Facturar» de la ficha necesita T017)

---

## Phase 7: Logo del taller en la factura (FR-009, añadido el 29/10)

**Goal**: si el taller tiene configurado `TALLER_LOGO`, la factura lo muestra en la cabecera; si
no, sale igual que ahora (research D8).

**Independent Test**: con `TALLER_LOGO` apuntando a una imagen, `/facturas/{id}` muestra el logo
(también en la vista previa de impresión); sin la variable, la factura sale sin logo y sin hueco.

- [x] T029 Añadir a `backend/src/config.rs` el campo `taller_logo: Option<PathBuf>` leído de la variable opcional `TALLER_LOGO` (vacía o ausente → `None`), poner `taller_logo: None` en el `Config` de `backend/tests/comun/mod.rs`, y documentar la variable (comentada) en `.env.example`
- [x] T030 [P] [US1] En `backend/tests/spec007_facturas.rs`: con un `EstadoApp` cuyo `taller_logo` apunta a un archivo `.png` temporal, la factura trae `taller.logo = "/api/taller/logo"` y `GET /api/taller/logo` responde 200 con `Content-Type: image/png` y los mismos bytes; sin `taller_logo`, o con una ruta que no existe, `taller.logo = null` y `GET /api/taller/logo` → 404 «No hay logo configurado»; sin sesión → 401
- [x] T031 [US1] En `backend/src/rutas/facturas.rs`: `taller.logo` (`Option<String>`) en `Factura` = `Some("/api/taller/logo")` solo si `config.taller_logo` está puesto y el archivo existe; y `GET /api/taller/logo` (cualquier sesión) que lee el archivo y lo devuelve con el `Content-Type` según la extensión (`png`, `jpg`/`jpeg`, `svg` → `image/svg+xml`, `webp`), o 404 «No hay logo configurado» (y un aviso en el registro si la ruta está puesta pero no se puede leer)
- [x] T032 [US1] En `frontend/src/lib/tipos.ts` añadir `logo: string | null` a `Factura.taller`, y en `frontend/src/routes/facturas/[id]/+page.svelte` mostrar `<img src={factura.taller.logo} alt="Logo de {nombre del taller}">` (alto máximo ~2 cm, también al imprimir) a la izquierda de los datos del taller solo si `logo` no es `null`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: depende de la base común y de spec-005 (imputaciones y `terminada`, en `main`).
- **Foundational (Phase 2)**: depende de Setup y bloquea todas las historias.
- **US1 → US2**: US2 añade el aviso al mismo endpoint de US1.
- **US3**: solo necesita facturas (las de US1 o las de los datos demo); independiente en el backend.
- **T017**: necesita la ficha y `GET /api/ordenes/{id}` de spec-002 (Persona A).

### Within Each User Story

- Pruebas primero, y deben fallar antes de implementar.
- Backend antes que pantalla.
- Todas las tareas de backend tocan `facturas.rs`, así que van en secuencia.

### Parallel Opportunities

- T002 en paralelo con T001; T005, T006, T007 y T008 en paralelo.
- Las pruebas de cada historia (marcadas [P]) se escriben en paralelo entre sí.
- Las páginas `/facturas/[id]` (T015) e `/historial` (T024) en paralelo con el backend de otra historia.

---

## Parallel Example: User Story 1

```bash
Task: "T009 Pruebas de facturar una orden terminada en backend/tests/spec007_facturas.rs"
Task: "T010 Pruebas del correlativo sin huecos en backend/tests/spec007_facturas.rs"
Task: "T011 Pruebas de estados, duplicados y permisos en backend/tests/spec007_facturas.rs"
Task: "T012 Pruebas de ver factura y factura_de_orden en backend/tests/spec007_facturas.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 y Phase 2.
2. US1 (facturar, ver e imprimir).
3. **Validar** con T009–T012 y la vista `/facturas/{id}` en el navegador.

### Incremental Delivery

1. US1 → factura (el núcleo de HU-07).
2. US2 → aviso al cliente (mismo endpoint).
3. US3 → historial por matrícula.

---

## Notes

- [P] = otro fichero y sin dependencias pendientes.
- Hacer commit al terminar cada tarea o grupo lógico.
- Los cambios en ficheros compartidos (`formato.rs`, `tipos.ts`, `componentes/TablaLineas.svelte`,
  `ordenes/[id]/+page.svelte`, `ordenes.rs`) los revisa también la Persona A.
