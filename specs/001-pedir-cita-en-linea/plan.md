# Implementation Plan: spec-001 · Pedir cita en línea

**Branch**: `001-pedir-cita-en-linea` | **Date**: 2026-10-22 (actualizado tras el clarify Q1–Q5) | **Spec**: [spec.md](spec.md) | **Responsable**: Persona A

**Input**: Feature specification from `/specs/001-pedir-cita-en-linea/spec.md`

**Note**: Lo común (pila, arquitectura, autenticación, fechas, errores, estructura) está en el
**plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md). Este plan solo recoge lo
propio de la spec. Decisiones: [research.md](research.md) (D1–D7). El modelo de datos, el contrato
y la guía de validación son los comunes; [data-model.md](data-model.md),
[contracts/api.md](contracts/api.md) y [quickstart.md](quickstart.md) solo explican cómo los usa
esta historia.

## Summary

Una página pública, `/cita`, busca el vehículo por matrícula normalizada (de 2 a 10 letras o
cifras), ofrece los días laborables con plazas en los próximos 30 días y crea la cita. Si la
matrícula es nueva, crea también el cliente y el vehículo, con el teléfono y el correo validados
en el servidor. Si la matrícula es conocida, solo pide día y motivo y nunca modifica al
propietario. Cada matrícula puede tener una sola cita pendiente. Esa regla y el cupo se protegen
en la misma transacción, con un bloqueo por fecha (R6) y otro por matrícula. Recepción y el jefe
ven en `/citas` las citas de un día con nombre y teléfono, nunca con el correo.

## Technical Context

**Language/Version · Dependencies · Storage · Testing · Platform**: igual que el plan maestro
(Rust + Axum + SQLx, SvelteKit, PostgreSQL 16, `#[sqlx::test]`). **No se añade ninguna
dependencia**: las validaciones de teléfono y correo solo comprueban caracteres (D2).

**Project Type**: aplicación web; módulo `citas.rs` dentro del backend común.

**Performance Goals**: cita de un vehículo conocido en menos de 1 minuto (SC-001); de uno nuevo,
en menos de 3 (SC-002); pantallas en menos de 1 s.

**Constraints**: todo en español, fechas dd/mm/aaaa, «hoy» en Europe/Madrid (R12). Configuración
usada: `TALLER_CUPO_DIARIO`.

**Scale/Scope**: 3 tablas existentes (`cliente`, `vehiculo`, `cita`) y ninguna migración nueva;
4 endpoints; 2 pantallas; cambios en 3 ficheros compartidos: `matricula.rs` (la regla de la
matrícula), `tipos.ts` (los tipos del contrato) y `rutas/mod.rs` (una línea para registrar el
módulo).

- **Endpoints** ([contrato común](../000-base-comun/contracts/api.md#spec-001--citas)):
  `GET /api/publico/vehiculos/{matricula}`, `GET /api/publico/dias-disponibles`,
  `POST /api/publico/citas`, `GET /api/citas?fecha=`.
- **Pantallas**: `/cita` (pública, usable desde el móvil) y `/citas` (recepción y jefe).

No queda ningún NEEDS CLARIFICATION: el clarify del 2026-10-22 cerró Q1–Q5, y las decisiones
técnicas están en [research.md](research.md).

## Constitution Check

*GATE: antes de Phase 0 y de nuevo después de Phase 1.* Constitución 1.2.1.

| Principio | Comprobación | Estado |
|---|---|---|
| I. Simplicidad | Sin migraciones ni dependencias nuevas. La regla de la matrícula está en un solo sitio (`normalizar_matricula`, D1). Las validaciones son funciones de caracteres (D2). La cita pendiente reutiliza la transacción y el mecanismo de bloqueo de R6 con una segunda clave (D3). No hay protección de rutas por rol en el frontend: el 403 del servidor basta (D5). | ✅ |
| II. Español y euro | Mensajes de error en español fijados en el contrato. El formulario usa `novalidate` para que el navegador no muestre sus avisos en otro idioma (D2). Fechas dd/mm/aaaa con `formato.ts`. Sin importes. | ✅ |
| III. Cero alcance fantasma | Sin festivos, horas, recordatorios, cancelación, modificación de citas ni edición de clientes (Assumptions). Ningún mensaje promete cambios que la v1 no permite (FR-010, Q5). `cita_pendiente` y `cliente {nombre, telefono}` salen de FR-010 y FR-009. `orden_id` ya estaba en el contrato para spec-002. | ✅ |
| IV. Verificable sin código | Cada escenario de la spec tiene su paso en el [quickstart §1](../000-base-comun/quickstart.md) ([correspondencia](quickstart.md)). La concurrencia (US3-2 y FR-010 con dos peticiones a la vez) solo se puede provocar con pruebas automáticas, como en el plan anterior. Su efecto visible (nunca hay más citas que el cupo, nunca dos pendientes) sí se comprueba en la app. | ✅ |
| V. Datos con respeto | Solo nombre, teléfono y correo, cada uno con su uso descrito en la spec. La API pública devuelve marca, modelo y `cita_pendiente` (sí/no), pero nunca la fecha ni datos personales. La lista no muestra el correo. Desde la página pública no se pueden sobrescribir los datos de contacto (D4). Las pruebas no usan valores demo. | ✅ |

**Re-check post-diseño (Phase 1)**: los cambios en el modelo y el contrato comunes solo concretan
reglas de la spec (formato de matrícula, teléfono y correo, cita pendiente, campos de la lista) y
no añaden tablas ni endpoints. El cambio en `matricula.rs` se marca como compartido. ✅

## Project Structure

### Documentation (this feature)

```text
specs/001-pedir-cita-en-linea/
├── spec.md
├── plan.md                 # este fichero
├── research.md             # decisiones D1–D7
├── data-model.md           # cómo usa esta historia el modelo común
├── contracts/api.md        # qué pantalla usa cada endpoint del contrato común
├── quickstart.md           # escenarios de la spec ↔ pasos del quickstart común §1
├── decisiones-clarify.md   # registro del clarify Q1–Q5
└── checklists/requirements.md
```

### Source Code (repository root)

```text
backend/src/matricula.rs                  # CAMBIO COMPARTIDO (base común; lo usa spec-007): límite de 2 a 10
backend/src/rutas/citas.rs                # 4 endpoints + telefono_valido / correo_valido
backend/tests/spec001_citas.rs            # pruebas de reglas (ver abajo)
frontend/src/routes/cita/+page.svelte
frontend/src/routes/citas/+page.svelte    # spec-002 añade después el botón «Abrir orden»
frontend/src/lib/tipos.ts                 # tipos VehiculoPublico, CitaDelDia (compartido)
```

**Structure Decision**: la del plan maestro. Esta spec añade su módulo de rutas, su fichero de
pruebas y dos pantallas, y toca un fichero compartido: `matricula.rs`.

### Cambio compartido: `backend/src/matricula.rs`

- **Qué**: `normalizar_matricula` devuelve `None` también si la matrícula normalizada tiene menos
  de 2 o más de 10 caracteres (FR-002, Q1). Se amplían sus pruebas unitarias con `"A"`,
  `"ABCDE123456"` (→ `None`) y con `"AB"`, `"ABCDE12345"` (→ válidas).
- **A quién afecta**: a spec-007 (Persona B), en el historial por matrícula. Una búsqueda de 1
  carácter o de más de 10 da 422 «La matrícula no es válida». Ningún vehículo guardado tiene esas
  matrículas (research D1).
- **Cómo se fusiona**: en un commit propio, antes del resto de spec-001, revisado por la Persona B
  (regla del plan maestro para ficheros compartidos). Cierra I11.

### Reglas clave

- **`GET /api/publico/vehiculos/{matricula}`**: `normalizar_matricula` → `None` = 422. Busca el
  vehículo (404 «No hay ningún vehículo con esa matrícula» si no existe) y devuelve `{matricula, marca, modelo, cita_pendiente}`.
  `cita_pendiente` = existe una cita del vehículo con `fecha >= fechas::hoy()`. Nunca devuelve la
  fecha ni datos del cliente.
- **`POST /api/publico/citas`** (D2–D4), en el orden de la tabla de validación del contrato común:
  1. Validar `matricula`, `fecha` (en el rango de FR-006 y de lunes a viernes) y `motivo` → 422
     con el primer error y `detalle.campo`.
  2. Buscar el vehículo, todavía sin bloqueos. Si **no existe**, validar `vehiculo.marca`,
     `vehiculo.modelo`, `cliente.nombre`, `cliente.telefono` y `cliente.email` → 422 (D2). Así,
     todos los 422 llegan antes que cualquier 409.
  3. Abrir la transacción: `pg_advisory_xact_lock(fecha)` (R6) y, **después**,
     `pg_advisory_xact_lock(1, hashtext(matricula))` (D3). Siempre en este orden.
  4. Volver a buscar el vehículo con los bloqueos tomados. Si ahora existe (lo ha dado de alta otra
     persona entre el paso 2 y el 3), se trata como conocido: se ignoran `vehiculo` y `cliente` y
     no se hace ningún `UPDATE` (D4).
  5. Si el vehículo existe y tiene una cita con `fecha >= hoy` → 409 «Este vehículo ya tiene una
     cita pendiente.».
  6. Contar las citas de esa fecha → 409 «Ese día ya está completo» si se ha llegado al cupo.
  7. Si el vehículo sigue sin existir, insertar cliente y vehículo con los datos ya validados en el
     paso 2.
  8. Insertar la cita y responder 201 `{fecha, motivo, matricula}`.
- **`GET /api/citas`**: `Usuario` (401 sin sesión) + `exigir_rol(&[Recepcion])` (el jefe pasa y el
  resto recibe 403). `fecha` por defecto `fechas::hoy()`. Se unen `cita`, `vehiculo` y `cliente`,
  sin seleccionar `email`, y se ordena por `cita.creada_en, cita.id` (D5).
- **Frontend `/cita`**: `novalidate`; si no hay días disponibles, «No quedan fechas libres en
  los próximos 30 días. Llama al taller para pedir cita.»; al salir del campo matrícula llama al `GET`. El 422 se
  muestra bajo la matrícula, el aviso de cita pendiente desactiva Confirmar, el 404 abre los
  campos de vehículo y contacto, encabezados por el texto de pantalla «No conocemos este vehículo.
  Cuéntanos qué coche es y cómo contactarte.» (texto normal, no título), y el 200 los oculta. Al confirmar, un 422 se muestra bajo el campo
  de `detalle.campo`. Con el 409 de cupo, se vuelven a pedir los días y se muestra «Ese día se
  acaba de completar; elige otro».
- **Frontend `/citas`**: un selector de fecha (hoy por defecto) y una tabla con matrícula, marca,
  modelo, motivo, nombre y teléfono; con la lista vacía, «No hay citas para este día»; con un 403,
  el mensaje de error común.

### Pruebas

`backend/src/matricula.rs` (unitarias, cambio compartido): las formas equivalentes de escribir
`1234BCD`; las inválidas `""`, `"12#4"`, `"A"` y `"ABCDE123456"`; y los límites válidos `"AB"` y
`"ABCDE12345"`.

`backend/tests/spec001_citas.rs` (`#[sqlx::test]`, con datos creados en cada prueba y sin valores
demo, principio V):

1. Matrícula nueva con todos los datos → 201; se crean cliente, vehículo y cita.
2. Matrícula `1234-bcd` con el vehículo `1234BCD` existente → se reconoce.
3. Matrícula inválida (`12#4`, `A`, 11 caracteres) en el `GET` y en el `POST` → 422 «La matrícula
   no es válida» y no se crea nada.
4. Teléfono `12345`, correo `ana@correo` y correo vacío → 422 con su mensaje y su
   `detalle.campo`; `+34 612 345 678` y `612-345-678` → 201. Con el día lleno, un teléfono
   incorrecto da el 422, no el 409 (todos los 422 antes que los 409).
5. Motivo vacío, marca vacía o falta `cliente` con una matrícula nueva → 422.
6. Vehículo conocido: el `POST` con otro `cliente` y otro `vehiculo` → 201. El cliente y el
   vehículo no cambian y no se crea ningún cliente nuevo.
7. Vehículo con una cita para hoy o después → `GET` con `cita_pendiente: true` y `POST` con 409
   «Este vehículo ya tiene una cita pendiente.»; con la cita anterior a hoy → `cita_pendiente:
   false` y el `POST` da 201.
8. Cupo: con cupo 2 y 2 citas, el día no sale en `dias-disponibles` y el `POST` da 409. Los
   vehículos se crean por SQL, sin depender del alta de US2.
9. Concurrencia del cupo: dos `POST` a la vez por la última plaza → uno 201 y otro 409. Los
   vehículos se crean por SQL.
10. Concurrencia de la cita pendiente: dos `POST` a la vez con la misma matrícula para **días
    distintos** → uno 201 y otro 409 (comprueba el bloqueo por matrícula de D3).
11. Fecha en sábado, hoy o a 31 días → 422; el último día laborable ≤ hoy + 30 sí se ofrece.
12. `GET /api/citas`: sin sesión → 401; `mecanico` y `almacen` → 403; `recepcion` y `jefe` → 200
    con `cliente.nombre` y `cliente.telefono`, sin ningún campo `email`, en el orden en que se
    pidieron (vehículos creados por SQL). Sin `fecha` → las de hoy.

**Dependencias**: base común. No depende de ninguna otra spec. spec-002 usa `GET /api/citas`
(campo `orden_id`) y spec-007 usa `normalizar_matricula`.

**Cambios en documentos compartidos** (los revisa la Persona B antes de fusionar):
- [data-model.md](../000-base-comun/data-model.md): reglas de `cliente.telefono`,
  `cliente.email` y `vehiculo.matricula`; restricción de una cita pendiente por vehículo; uso de
  `creada_en`.
- [contracts/api.md](../000-base-comun/contracts/api.md): `cita_pendiente`, el 422 por matrícula
  inválida, el 409 por cita pendiente, la tabla de validación, la regla del vehículo conocido y
  los campos y el acceso de `GET /api/citas`.
- [quickstart.md](../000-base-comun/quickstart.md): se reescribe el §1, con un paso 10 que
  cronometra SC-001 y SC-002. En el §2.3 (spec-002) se usa `9999LLL`, porque 1234BCD ya tiene una
  cita pendiente (research D7).
- [plan.md](../000-base-comun/plan.md) (maestro): `matricula.rs` y `rutas/mod.rs` en la lista de
  ficheros compartidos ([hallazgos](hallazgos.md) I5).
- [research.md](../000-base-comun/research.md) R6: la convención de las claves de
  `pg_advisory_xact_lock` ([hallazgos](hallazgos.md) G1).
- `backend/src/matricula.rs` (código compartido, ver arriba).

**Pendiente fuera de esta spec** ([hallazgos.md](hallazgos.md)): I2 e I6, abiertos para el
clarify de spec-002 (Persona A); I4, asignado a spec-007 (Persona B).

## Complexity Tracking

No hay violaciones que justificar.
