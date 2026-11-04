# Implementation Plan: spec-004 · Aceptar o rechazar el presupuesto desde un enlace

**Branch**: `004-aceptar-presupuesto-enlace` | **Date**: 2026-11-04 (actualizado tras el clarify Q1–Q5) | **Spec**: [spec.md](spec.md) | **Responsable**: Persona A

**Input**: Feature specification from `/specs/004-aceptar-presupuesto-enlace/spec.md`

**Note**: Lo común (pila, arquitectura, fechas, errores, correo, eventos, estructura) está en el
**plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md). Este plan solo recoge lo propio
de la spec. Decisiones: [research.md](research.md) (D1–D8). El modelo de datos, el contrato y la guía
de validación son los comunes; [data-model.md](data-model.md), [contracts/api.md](contracts/api.md) y
[quickstart.md](quickstart.md) solo explican cómo los usa esta historia.

## Summary

La página pública `/presupuesto/[token]` muestra el presupuesto y deja aceptarlo o rechazarlo una
sola vez, con un paso de confirmación. La respuesta se guarda con una sola sentencia (research R9)
que comprueba a la vez que sigue emitido y sin caducar; en la misma transacción, aceptar cambia el
estado de la orden con `estados::cambiar_estado`. El mismo mecanismo sirve para las ampliaciones
(FR-009). En la ficha de la orden se ven todas las respuestas con su fecha y hora, y, mientras el
presupuesto está vigente, el enlace con «Copiar enlace».

## Technical Context

**Language/Version · Dependencies · Storage · Testing · Platform**: igual que el plan maestro
(Rust + Axum + SQLx, SvelteKit, PostgreSQL 16, `#[sqlx::test]`). **No se añade ninguna dependencia ni
migración**: la tabla `presupuesto` ya tiene `estado`, `token`, `caduca_el` y `respondido_en`.

**Project Type**: aplicación web; módulo `publico_presupuesto.rs` dentro del backend común.

**Performance Goals**: responder en menos de 1 minuto desde que se abre el enlace (SC-001);
pantallas en menos de 1 s.

**Constraints**: todo en español; importes en € con `euros()`; fechas dd/mm/aaaa y horas de Madrid
(R12); la página pública, usable a 390 px.

**Scale/Scope**: 1 tabla existente (`presupuesto`) y `orden` por `cambiar_estado`; 2 endpoints
públicos; 1 pantalla nueva y un cambio en la sección Presupuesto de la ficha.

- **Endpoints** ([contrato común](../000-base-comun/contracts/api.md#spec-004--respuesta-del-cliente)):
  `GET /api/publico/presupuestos/{token}`, `POST /api/publico/presupuestos/{token}/respuesta`.
- **Pantallas**: `/presupuesto/[token]` (pública) y la sección Presupuesto de `/ordenes/[id]`.

No queda ningún NEEDS CLARIFICATION: el clarify del 2026-11-04 cerró Q1–Q5, y los dos puntos que dejó
para el plan están resueltos en [research.md](research.md): D4 (texto del 404) y D5 (confirmación).

## Constitution Check

*GATE: antes de Phase 0 y de nuevo después de Phase 1.* Constitución 1.2.1.

| Principio | Comprobación | Estado |
|---|---|---|
| I. Simplicidad | Sin cuentas de cliente ni firma digital: un token en la URL. Sin dependencias ni migraciones. Una sola sentencia para «solo cuenta la primera respuesta» (R9). Un solo código para inicial y ampliación (D3). Reutiliza `TablaLineas`, `Totales` y `cambiar_estado`. | ✅ |
| II. Español y euro | Textos fijados en español de España, con trato de usted. La confirmación va dentro de la página porque `confirm()` usa el idioma del navegador (D5). Importes con `euros()`; «Aceptado el 16/10/2026 a las 10:32» en hora de Madrid. | ✅ |
| III. Cero alcance fantasma | Sin recordatorios ni cierre sin reparación (Q1-A). Un presupuesto caducado no admite ninguna respuesta (Q3). «Copiado» es solo la confirmación del botón que pide FR-008. La ampliación caducada de los datos demo solo sirve para verificar US3 (D7). | ✅ |
| IV. Verificable sin código | Cada escenario tiene su paso en el [quickstart común §4](../000-base-comun/quickstart.md) ([correspondencia](quickstart.md)), con el enlace desde Mailpit. El caducado se comprueba con un dato demo, sin tocar la base de datos (D7). SC-001 lo cronometra una persona. Dos respuestas a la vez solo se provocan con pruebas automáticas; su efecto visible (la segunda pestaña ve «ya fue respondido») sí se comprueba en la app. | ✅ |
| V. Datos con respeto | La página pública muestra solo ese presupuesto y el vehículo, **sin nombre, teléfono ni correo del cliente**: la consulta no une con `cliente` (D1). El 404 no da pistas de si existe el presupuesto (D4). Token aleatorio de 32 bytes (R9); el token demo es ficticio y está documentado. Las pruebas no usan valores demo. | ✅ |

**Re-check post-diseño (Phase 1)**: los cambios en el contrato, el data-model y el quickstart comunes
solo concretan reglas de la spec (textos, transición quitada, `respondido_en`) y no añaden tablas. Los
ficheros compartidos están en la tabla de abajo, para la revisión de la Persona B. ✅

## Project Structure

### Documentation (this feature)

```text
specs/004-aceptar-presupuesto-enlace/
├── spec.md
├── plan.md                 # este fichero
├── research.md             # decisiones D1–D8
├── data-model.md           # cómo usa esta historia el modelo común
├── contracts/api.md        # qué pantalla usa cada endpoint del contrato común
├── quickstart.md           # escenarios de la spec ↔ pasos del quickstart común §4
├── decisiones-clarify.md   # registro del clarify Q1–Q5
└── checklists/requirements.md
```

### Source Code (repository root)

```text
backend/src/rutas/publico_presupuesto.rs           # GET y POST públicos
backend/src/rutas/mod.rs                           # COMPARTIDO: pub mod y .merge(...)
backend/src/rutas/presupuestos.rs                  # DE B (spec-003): respondido_en en PresupuestoResumen y caducado() pub(crate)
backend/tests/spec004_respuesta.rs                 # pruebas de reglas (ver abajo)
backend/seed/demo.sql                              # COMPARTIDO: parte 6 y cita de 7777JJJ (D7)
frontend/src/lib/tipos.ts                          # COMPARTIDO: PresupuestoPublico
frontend/src/routes/presupuesto/[token]/+page.svelte
frontend/src/routes/ordenes/[id]/Presupuesto.svelte  # DE B (spec-003): respuestas y «Copiar enlace»
```

**Structure Decision**: la del plan maestro. Esta spec añade su módulo de rutas, su fichero de pruebas
y una pantalla, y toca los ficheros compartidos de la tabla.

### Ficheros compartidos (los revisa la Persona B en una tarea final, como T031 en spec-002)

| Fichero | Cambio | Por qué | Estado |
|---|---|---|---|
| `backend/src/rutas/presupuestos.rs` (spec-003) | `respondido_en` en `PresupuestoResumen`; `caducado()` pasa a `pub(crate)`, para que haya una sola definición (principio I) | Q1, I2 | Pendiente, con su prueba antes; commit propio (T014), que B revisa pronto |
| `frontend/src/routes/ordenes/[id]/Presupuesto.svelte` (spec-003) | Respuestas con fecha y hora, enlace y «Copiar enlace» (D6) | Q1, Q5, FR-008 | Pendiente |
| `backend/src/rutas/mod.rs` | `pub mod publico_presupuesto;` y `.merge(publico_presupuesto::rutas())` | Registrar el módulo | Pendiente |
| `frontend/src/lib/tipos.ts` | `respondido_en` en `PresupuestoResumen` (hecho) y `PresupuestoPublico` (pendiente) | Q1, tipos del contrato | Parcial |
| `backend/seed/demo.sql` | Parte 6, al final: ampliación caducada en la orden 7, con su propio `INSERT` de línea por token (no le alcanza el `CROSS JOIN` de la parte 4). Parte 3: cita de `7777JJJ` de hace 8 a hace 18 días (D7) | US3 verificable en la app, con fechas coherentes | Pendiente |
| `backend/src/estados.rs` y `backend/tests/spec000_estados.rs` | Sin la autotransición: 8 transiciones | Q2 | Hecho en el clarify |
| `specs/000-base-comun/contracts/api.md` | `PresupuestoResumen`, textos del 404, del 409 y del 422 de spec-004; la fila de `GET /api/piezas` vuelve a su tabla | Q1, Q3, D4, G2 | Hecho |
| `specs/000-base-comun/data-model.md` | Sin la fila de la autotransición; regla sobre el rechazo | Q2 | Hecho en el clarify |
| `specs/000-base-comun/quickstart.md` | §0 (token demo caducado) y §4 (confirmación, segunda pestaña, enlace, cronometraje de SC-001, enlace oculto en un caducado, ampliación rechazada, caducado y enlace inválido) | Principio IV | Hecho |
| `specs/000-base-comun/research.md` | R9: única versión del `UPDATE`; Q1 de la tabla inicial dice «sigue» | Q4, Q2 | Hecho |
| `specs/003-diagnostico-presupuesto/research.md` | Nota de la autotransición | Q2 | Hecho en el clarify |

`presupuestos.rs` y `Presupuesto.svelte` son de la Persona B, y el plan maestro dice que A no modifica
el primero y pide acordar antes de tocar. Por eso la Persona A lo habla con B antes de implementar y
**le avisa antes de tocar esos ficheros**. T014 va en un commit propio para que B lo revise pronto, sin
esperar a la tarea final. Ese commit no existe hasta tener hecho el `POST` (T013): `respondido_en` se
añade después, para ver antes su prueba en rojo. Se acepta así (N2): sigue siendo mucho antes de la
tarea final.

### Reglas clave

- **`GET /api/publico/presupuestos/{token}`** (sin sesión): D1. Token inexistente → 404 «Este enlace
  no es válido» (D4). Sirve también después de responder o de caducar (US1-3, Q5).
- **`POST /api/publico/presupuestos/{token}/respuesta`** (sin sesión), en este orden (D2):
  1. `decision` es `aceptar` o `rechazar` → si no, 422 «Elija aceptar o rechazar».
  2. Transacción: el `UPDATE` de [research R9](../000-base-comun/research.md), que es la única versión
     (spec-004 Q4; D1 de la base común).
  3. Si no se actualiza ninguna fila, el motivo se averigua como dice R9, con el mismo `$hoy`: 404,
     409 «Este presupuesto ya fue respondido» o 409 «Este presupuesto caducó el dd/mm/aaaa. Contacte
     con el taller».
  4. Efecto en la orden, en la misma transacción (D3):

     | Respuesta | Presupuesto inicial | Ampliación |
     |---|---|---|
     | Aceptar | `en_diagnostico → en_reparacion` | `pendiente_conformidad → en_reparacion` |
     | Rechazar | `en_diagnostico` (se queda; sin transición, Q2) | `pendiente_conformidad` (se queda) |

  5. `commit`; después, si se aceptó, `CambioEstado::publicar`. Responde 200 con lo mismo que el `GET`.
- **`GET /api/ordenes/{id}`** (spec-002): `presupuestos` trae `respondido_en` (Q1).
- **El enlace en la orden** (FR-008, Q5): la sección Presupuesto muestra el enlace y «Copiar enlace»
  solo con el presupuesto emitido y vigente. Se oculta **solo en la pantalla**: `GET
  /api/presupuestos/{id}` sigue enviando `enlace`, porque tras la respuesta o la caducidad ya no
  permite cambiar nada (la respuesta da 409). El enlace sigue funcionando para el cliente (US1-3).
- **Frontend `/presupuesto/[token]`**: vehículo, diagnóstico, líneas, totales y caducidad. Según el
  estado: «Aceptar» y «Rechazar» con su confirmación (D5); la decisión con su fecha y hora, sin
  botones; o el aviso de caducidad, sin botones. Con un 409, muestra el mensaje y vuelve a pedir el
  presupuesto. Con un 404, solo «Este enlace no es válido». Sin datos del cliente (principio V) y
  usable a 390 px (D8).
- La regla «sin presupuesto aceptado no se imputa» (FR-007) se cumple con las dos barreras de `imputar`
  (spec-005): primero el estado, que solo admite `en_reparacion`, y después que la orden tenga un
  presupuesto aceptado. Aquí se añade una prueba que comprueba la primera.

### Pruebas

`backend/tests/spec004_respuesta.rs` (`#[sqlx::test]`, con datos creados en cada prueba y sin valores
demo, principio V):

1. Ver: un presupuesto emitido y vigente → 200 con vehículo, diagnóstico, líneas, totales, caducidad
   y `decision: null`, sin ningún dato del cliente (US1-1, FR-002).
2. Aceptar el inicial → 200 con `decision` y `respondido_en`; la orden pasa a `en_reparacion` y se
   publica el evento después del `commit` (US1-2, FR-004, FR-006).
3. Volver a abrir tras responder: el `GET` da la decisión y su fecha; un segundo `POST` → 409 «Este
   presupuesto ya fue respondido» y `respondido_en` no cambia (US1-3, FR-003).
4. Concurrencia: dos respuestas a la vez (aceptar y aceptar; aceptar y rechazar) → un 200 y un 409
   «Este presupuesto ya fue respondido» (texto exacto), la orden cambia una sola vez y un rechazo no
   sobrescribe una aceptación. Repetir 20 veces. **Se comprueba, en las dos variantes, que falla
   quitando `estado = 'emitido'` del `UPDATE`** (US1-4, FR-003, Q4).
5. Rechazar el inicial → «rechazado» con `respondido_en`; la orden sigue `en_diagnostico` y no se
   publica ningún evento (US2-1, Q2).
6. Ampliación: aceptar → `en_reparacion`; rechazar → sigue `pendiente_conformidad` (US2-2, FR-009).
7. Caducidad, en el presupuesto inicial y en una ampliación: `caduca_el` ayer → `caducado: true`, y
   aceptar y rechazar dan 409 con el aviso, sin guardar nada; `caduca_el` hoy → se puede responder;
   uno ya respondido con `caduca_el` pasado sigue con su decisión y da 409 «ya fue respondido»
   (US3-1, US3-2, FR-005, SC-003, Q3).
8. Token inexistente → 404 «Este enlace no es válido», igual en el `GET` y en el `POST` (FR-002, D4).
9. `decision` que no sea `aceptar` ni `rechazar`, o ausente en `{}` → 422 «Elija aceptar o rechazar».
10. Ficha: con un inicial rechazado y otro aceptado, `GET /api/ordenes/{id}` trae los dos con su
    `respondido_en` (US2-3, FR-004, SC-002, Q1).
11. Sin presupuesto aceptado no se imputa: con uno emitido, rechazado o caducado, `POST
    /api/ordenes/{id}/imputaciones` → 409 con el texto exacto del estado: «La orden está en diagnóstico;
    solo se puede imputar en órdenes en reparación». Se comprueba por mutación quitando esa condición
    (US4-1, FR-007, SC-004).
12. Enlace: tras responder, `GET /api/presupuestos/{id}` sigue trayendo `enlace` (Q5).

**Dependencias**: base común; spec-003 (presupuesto emitido, token y correo) y spec-005 (imputaciones),
ya en `main`; spec-002 (ficha de la orden), que **no** está en `main`: `spec-004` la lleva dentro, y el
orden de fusión es 001 → 002 → 004. El correo con el enlace (FR-001) ya lo envía spec-003
al emitir: aquí no se cambia.

**Cambios en documentos compartidos** (los revisa la Persona B antes de fusionar, en la tarea final):
los de la tabla de arriba. Los cambios en código compartido se hacen al implementar, con la misma
revisión.

## Complexity Tracking

Sin violaciones de la constitución que justificar.
