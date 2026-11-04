# Implementation Plan: spec-002 · Recibir el vehículo y abrir la orden de reparación

**Branch**: `002-abrir-orden-reparacion` | **Date**: 2026-11-03 (actualizado tras el clarify Q1–Q5) | **Spec**: [spec.md](spec.md) | **Responsable**: Persona A

**Input**: Feature specification from `/specs/002-abrir-orden-reparacion/spec.md`

**Note**: Lo común (pila, arquitectura, autenticación, fechas, errores, contador, eventos,
estructura) está en el **plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md). Este
plan solo recoge lo propio de la spec. Decisiones: [research.md](research.md) (D1–D9). El modelo
de datos, el contrato y la guía de validación son los comunes; [data-model.md](data-model.md),
[contracts/api.md](contracts/api.md) y [quickstart.md](quickstart.md) solo explican cómo los usa
esta historia.

## Summary

El día de la cita, recepción abre la orden desde la lista `/citas`. El servidor comprueba que la
cita es de hoy (Europe/Madrid), valida kilómetros, daños, mecánico y fecha de entrega, y compara
los km con la visita anterior del vehículo. Después, en una transacción con la fila de la cita
bloqueada, comprueba que la cita no tiene orden, pide el siguiente número de `contador('orden')`
y crea la orden en estado `recibida`. Tras el `commit` publica el evento del tablero. La lista de
citas muestra «Orden Nº N» en las citas atendidas. Esta spec crea también la ficha
`/ordenes/[id]`, con su sección «Datos», en la que se encajan las secciones de spec-003, 005 y
006 ya hechas por la Persona B.

## Technical Context

**Language/Version · Dependencies · Storage · Testing · Platform**: igual que el plan maestro
(Rust + Axum + SQLx, SvelteKit, PostgreSQL 16, `#[sqlx::test]`). **No se añade ninguna
dependencia ni migración**: las tablas `orden` y `contador` ya existen en
`0001_esquema_inicial.sql`.

**Project Type**: aplicación web; módulo `ordenes.rs` dentro del backend común.

**Performance Goals**: abrir una orden en menos de 2 minutos desde la lista de citas (SC-001);
pantallas en menos de 1 s.

**Constraints**: todo en español; km con separador de miles («85.000 km») y fechas dd/mm/aaaa;
«hoy» siempre con `fechas::hoy()` (Europe/Madrid, R12).

**Scale/Scope**: 2 tablas existentes (`orden`, `contador`) más lecturas de `cita`, `vehiculo`,
`usuario` y `factura`; 3 endpoints nuevos (`POST /api/ordenes`, `GET /api/ordenes`, `GET
/api/ordenes/{id}`) y un campo nuevo en `GET /api/citas` (spec-001); 3 pantallas (`/ordenes`,
`/ordenes/nueva`, `/ordenes/[id]`) y un cambio en `/citas`.

- **Endpoints** ([contrato común](../000-base-comun/contracts/api.md#spec-002--orden-de-reparación)):
  `POST /api/ordenes`, `GET /api/ordenes?mias=true`, `GET /api/ordenes/{id}`; usa también `GET
  /api/mecanicos` (base común) y `GET /api/citas` (spec-001).
- **Pantallas**: `/citas` (columna «Orden»), `/ordenes/nueva?cita=…` (formulario), `/ordenes`
  (lista; «Mis órdenes» para los mecánicos) y `/ordenes/[id]` (ficha con secciones).

No queda ningún NEEDS CLARIFICATION: el clarify del 2026-11-02 cerró Q1–Q5 (y los hallazgos I2 e
I6 de spec-001), y las decisiones técnicas están en [research.md](research.md).

## Constitution Check

*GATE: antes de Phase 0 y de nuevo después de Phase 1.* Constitución 1.2.1.

| Principio | Comprobación | Estado |
|---|---|---|
| I. Simplicidad | Sin migraciones ni dependencias nuevas. «Una cita, una orden» con un bloqueo de fila sobre la cita (`FOR UPDATE`, D2), sin claves consultivas nuevas. El número, con el `contador` de R5. La ficha reutiliza las funciones que ya tiene cada módulo de B (D6), y el formulario, la lista de hoy de spec-001 (D7) en vez de un endpoint nuevo. | ✅ |
| II. Español y euro | Mensajes de error en español fijados en el contrato común. Km con separador de miles con `formato::numero` («85.000 km») y fechas dd/mm/aaaa con `formato.ts`. El formulario usa `novalidate` para que el navegador no muestre avisos propios. Sin importes nuevos. | ✅ |
| III. Cero alcance fantasma | Sin atención sin cita (Q1), sin fotos de daños, sin editar ni anular órdenes. El enlace «Orden Nº N» de `/citas` lo pide FR-008 (Q5). `orden_numero` en `GET /api/citas` existe solo para ese enlace. Los datos demo nuevos (D8) solo sirven para verificar Q1–Q5 en la app. `/ordenes` y «Mis órdenes» no salen de un FR de spec-002: los exige el menú común (escenario 1 de la US1 de la base común, «Mis órdenes»; quickstart §0.4, «Órdenes»; el jefe las ve por FR-002 común) y los usa el quickstart §3 («Como `luis`, abrir la orden…»). Se implementan aquí porque spec-002 es la que crea las órdenes (hallazgo E1, justificado). | ✅ |
| IV. Verificable sin código | Cada escenario y caso límite de la spec tiene su paso en el [quickstart común §2](../000-base-comun/quickstart.md) ([correspondencia](quickstart.md)), con datos demo que dan citas de hoy (D8), incluido el cronometraje de SC-001. **SC-002** se reescribió en el clarify (Q6, hallazgo C1 de [analyze-1](analyze-1.md)) para comprobarse en la app: el §2 abre seguidas las órdenes 10 y 11 con intentos rechazados entre medias (pasos 3–5) y la lista de citas muestra los números consecutivos. La apertura simultánea (FR-002 y Q5) solo se provoca con pruebas automáticas; su efecto visible (el mensaje de Q5 en dos pestañas) sí se comprueba en la app. | ✅ |
| V. Datos con respeto | No se piden datos personales nuevos. La ficha y la lista de órdenes muestran matrícula, marca y modelo, nunca datos del cliente. Los datos demo nuevos reutilizan clientes ficticios que ya existen; `pedro` es un usuario ficticio con la contraseña demo documentada. Las pruebas no usan valores demo. | ✅ |

**Re-check post-diseño (Phase 1)**: los cambios en el contrato, el data-model y el quickstart
comunes solo concretan reglas de la spec (textos de validación, mecánico activo, día de la cita,
`orden_numero`) y no añaden tablas. Los ficheros compartidos que se tocan están marcados para la
revisión de la Persona B (ver abajo). ✅

## Project Structure

### Documentation (this feature)

```text
specs/002-abrir-orden-reparacion/
├── spec.md
├── plan.md                 # este fichero
├── research.md             # decisiones D1–D9
├── data-model.md           # cómo usa esta historia el modelo común
├── contracts/api.md        # qué pantalla usa cada endpoint del contrato común
├── quickstart.md           # escenarios de la spec ↔ pasos del quickstart común §2
├── decisiones-clarify.md   # registro del clarify Q1–Q5
└── checklists/requirements.md
```

### Source Code (repository root)

```text
backend/src/rutas/ordenes.rs              # POST, GET lista y GET detalle (Persona A)
backend/src/rutas/mod.rs                  # COMPARTIDO: una línea, .merge(ordenes::rutas())
backend/src/rutas/citas.rs                # spec-001 (Persona A): orden_numero en GET /api/citas
backend/src/rutas/imputaciones.rs         # DE B: campos de ResumenImputaciones públicos (D6)
backend/tests/spec002_ordenes.rs          # pruebas de reglas (ver abajo)
backend/tests/spec006_pedidos.rs          # DE B: quitar #[ignore] de la_ficha_de_la_orden_trae_sus_pedidos
backend/seed/demo.sql                     # COMPARTIDO: parte 5, citas de hoy (D8)
frontend/src/lib/tipos.ts                 # COMPARTIDO: Orden, PeticionOrden, CitaDelDia.orden_numero
frontend/src/routes/citas/+page.svelte    # spec-001 (Persona A): columna «Orden»
frontend/src/routes/ordenes/+page.svelte
frontend/src/routes/ordenes/nueva/+page.svelte
frontend/src/routes/ordenes/[id]/+page.svelte     # COMPARTIDO: contenedor de secciones
frontend/src/routes/ordenes/[id]/Datos.svelte
```

**Structure Decision**: la del plan maestro. Esta spec añade su módulo de rutas, su fichero de
pruebas y tres pantallas, y toca los ficheros compartidos marcados arriba.

### Ficheros compartidos (revisa la Persona B antes de fusionar)

| Fichero | Cambio | Por qué |
|---|---|---|
| `backend/seed/demo.sql` | Parte 5: citas de hoy de `1313MMM`, `1414NNN` y `8888KKK`, cita de ayer sin orden de `1515PPP`, mecánico de baja `pedro` (D8) | Con Q2, el quickstart §2 necesita citas de hoy |
| `backend/src/rutas/mod.rs` | `.merge(ordenes::rutas())` | Registrar el módulo |
| `backend/src/rutas/imputaciones.rs` (de B) | Tres campos de `ResumenImputaciones` pasan a `pub` (D6) | La ficha copia esos campos a `Orden` |
| `backend/tests/spec006_pedidos.rs` (de B) | Quitar el `#[ignore]` | El endpoint ya existe |
| `frontend/src/lib/tipos.ts` | `Orden`, `PeticionOrden`, `orden_numero` en `CitaDelDia` | Tipos del contrato |
| `frontend/src/routes/ordenes/[id]/+page.svelte` | Contenedor: carga `Orden` y pasa sus campos a cada sección (D6) | Plan maestro: `ordenes/[id]` es compartido |
| `specs/000-base-comun/contracts/api.md` | Sección spec-002 con la tabla de validación, 404/409 y `GET /api/ordenes/{id}`; `orden_numero` en `GET /api/citas` | Fuente única de reglas y textos |
| `specs/000-base-comun/data-model.md` | `orden`: mecánico activo, día de la cita, «visita anterior» | Q2, Q3, D3 |
| `specs/000-base-comun/plan.md` (maestro) | `backend/seed/demo.sql` en la lista de ficheros compartidos | Hallazgo G1 |
| `specs/000-base-comun/quickstart.md` | §0 (`pedro`), §2 reescrito, «1234BCD» → «1313MMM» en §3, §4, §5 y §7, §9 en dos días | D8 |

### Reglas clave

- **`POST /api/ordenes`** (recepción; el jefe pasa), en el orden de la tabla de validación del
  contrato común (D2):
  1. La cita existe → si no, 404 «No existe la cita».
  2. Sin transacción, los campos en el orden de la tabla: `km` (entero ≥ 0), `danos` (no vacío
     tras `trim`), `mecanico_id` (usuario activo con rol `mecanico`), `fecha_entrega_prometida`
     (≥ `fechas::hoy()`) y, por último, el km frente a la visita anterior (D3) → 422 con el
     primer error y `detalle.campo`.
  3. `cita.fecha == fechas::hoy()` → si no, 409 «Solo se puede abrir la orden el día de la cita»
     (D1).
  4. Transacción: `SELECT … FROM cita WHERE id = $1 FOR UPDATE`. Si ya hay una orden con ese
     `cita_id` → 409 «Esta cita ya tiene una orden (Nº 42)», con `detalle: {orden_id, numero}`.
  5. `contador::siguiente(&mut tx, contador::ORDEN)`, `INSERT` en `orden` con `vehiculo_id` de la
     cita, `estado = 'recibida'` y `danos` con `trim`; `commit`.
  6. Después del `commit`: `CambioEstado { id, estado: Estado::Recibida }.publicar(&eventos)`
     (D5). Responde 201 con el objeto `Orden` (el mismo que el `GET`).
- **`GET /api/ordenes/{id}`** (todos): consulta propia + `presupuestos_de_orden` (003) +
  `resumen_imputaciones` (005) + `pedidos_de_orden` (006); `factura_id` por subconsulta. 404 «No
  existe la orden» (D6).
- **`GET /api/ordenes?mias=true`** (recepción y mecánicos; el jefe pasa). No sale de un FR de
  spec-002: lo exigen el menú común («Órdenes» y «Mis órdenes») y el quickstart §3, y se hace aquí
  porque spec-002 crea las órdenes (hallazgo E1). Devuelve `[OrdenResumen]`, de la
  más reciente a la más antigua; con `mias=true`, solo las del mecánico que ha entrado. `retrasada`
  como en el data-model común (estado ≠ `facturada` y entrega < hoy).
- **`GET /api/citas`** (spec-001): añade `orden_numero` con un `LEFT JOIN` que ya existe (D7).
- **Frontend `/citas`**: columna «Orden»: «Orden Nº N» (enlace) si la cita tiene orden; «Abrir
  orden» solo si el día elegido es hoy; nada en otro caso.
- **Frontend `/ordenes/nueva?cita=…`**: `novalidate`; cabecera con matrícula, marca, modelo y
  motivo de la cita de hoy; campos Kilómetros, Daños visibles, Mecánico (lista de `GET
  /api/mecanicos`) y Fecha de entrega prometida; 422 bajo el campo de `detalle.campo`; 409 de Q5
  con enlace a la orden existente; 409 de D1 como error general; 201 → `/ordenes/{id}`. Si la cita
  no está en la lista de hoy, muestra «Solo se puede abrir la orden el día de la cita» sin
  formulario.
- **Frontend `/ordenes/[id]`**: sección «Datos» con número, estado, matrícula, marca, modelo,
  fecha y motivo de la cita, km («85.000 km»), daños, mecánico y entrega prometida (dd/mm/aaaa,
  «Retrasada» si lo está). Debajo, las secciones de B (D6).
- **Frontend `/ordenes`**: tabla con número (enlace a la ficha), matrícula, estado, mecánico y
  entrega prometida.

### Pruebas

`backend/tests/spec002_ordenes.rs` (`#[sqlx::test]`, con datos creados en cada prueba y sin
valores demo, principio V; las citas de hoy, con `fechas::hoy()`):

1. Abrir la orden de una cita de hoy → 201, estado `recibida`, número = último + 1, km, daños,
   mecánico, entrega y `vehiculo_id` de la cita; un suscriptor de `eventos` recibe `{id,
   recibida}` (US1-1, FR-005, FR-006, FR-007, D5).
2. Correlativo: con el contador en 41 → 42; tras un 422 de cada campo y tras los dos 409, la
   siguiente orden sigue siendo la 42, sin huecos (US1-2, US1-9, FR-002, SC-002).
3. Concurrencia del número: 10 `POST` a la vez para 10 citas distintas → números consecutivos, sin
   huecos ni repetidos (FR-002).
4. Concurrencia de la cita: dos `POST` a la vez para la misma cita → un 201 y un 409 «Esta cita ya
   tiene una orden (Nº N)» con `detalle.orden_id` y `detalle.numero`; 1 orden y el contador
   avanzado en 1. Repetir 20 veces. Se comprueba que falla quitando el `FOR UPDATE` (FR-008, Q5,
   US1-4).
5. Día de la cita: citas de mañana y de ayer → 409 «Solo se puede abrir la orden el día de la
   cita» y no se consume número (FR-001, Q2, US1-5).
6. Kilometraje: visitas anteriores con 70.000 y después 85.000 → 80.000 da 422 «El kilometraje no
   puede ser menor que el de la visita anterior (85.000 km)» con `detalle.km_anterior = 85000`;
   85.000 → 201; vehículo sin órdenes con 0 → 201; con una segunda pestaña y menos km para la
   misma cita → 409 de Q5, no el 422 (FR-004, US2-1, US2-2, US2-3, SC-003, D3).
7. Campos: km vacío, `-1`, `1.5` y `"abc"` → 422 del km; daños `""` y `"   "` → 422 de Q4; daños
   `"  Rayón  "` → se guarda «Rayón»; entrega ayer → 422 de la fecha; entrega hoy → 201; orden de
   las comprobaciones según la tabla (FR-003, Q4, US1-8).
8. Mecánico: de baja, con rol recepción, jefe e id inexistente → 422 «Elige un mecánico de la
   lista» (FR-005, Q3, US1-7).
9. Acceso y existencia: sin sesión → 401; mecánico y almacén → 403; recepción y jefe → 201; cita
   inexistente → 404 «No existe la cita».
10. Detalle: `GET /api/ordenes/{id}` con todos los roles → los campos de spec-002 y las secciones
    vacías (`presupuestos`, `imputaciones`, `pedidos`, `factura_id: null`, importes «0.00»); 404
    «No existe la orden» (US1-3, SC-004). Además pasa `la_ficha_de_la_orden_trae_sus_pedidos` de
    spec-006, ya sin `#[ignore]`.
11. Lista de citas: tras abrir la orden, `GET /api/citas` trae `orden_id` y `orden_numero` (Q5,
    US1-4).
12. `GET /api/ordenes?mias=true`: como `luis`, solo las suyas; como recepción, todas, de la más
    reciente a la más antigua; `retrasada` con entrega ayer sí y hoy no.

Las pruebas de spec-001 tienen que seguir pasando con `orden_numero` en `GET /api/citas`.

**Dependencias**: base común; spec-001 (citas y `GET /api/citas`); las funciones de la ficha de
spec-003, 005 y 006 (ya en `main`). spec-008 usa el evento de D5, `OrdenResumen` y `GET
/api/ordenes`.

**Cambios en documentos compartidos** (los revisa la Persona B antes de fusionar; se hacen en este
plan): el contrato, el data-model y el quickstart comunes de la tabla de arriba. Los cambios en
código compartido (`demo.sql`, `mod.rs`, `imputaciones.rs`, `spec006_pedidos.rs`, `tipos.ts`,
`ordenes/[id]`) se hacen al implementar, con la misma revisión.

## Complexity Tracking

Sin violaciones de la constitución que justificar.
