# Implementation Plan: spec-003 · Registrar diagnóstico y preparar presupuesto

**Branch**: `003-diagnostico-presupuesto` | **Date**: 2026-10-22 | **Spec**: [spec.md](spec.md) | **Responsable**: Persona B

**Input**: Feature specification from `/specs/003-diagnostico-presupuesto/spec.md`

**Note**: Lo común está en el **plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md).
Decisiones propias de esta spec: [research.md](research.md). El modelo de datos, el contrato y la
guía de validación son los compartidos (ver Phase 1).

## Summary

El mecánico asignado guarda el diagnóstico de una orden `recibida` (pasa a `en_diagnostico`) y
puede corregirlo hasta que emite un presupuesto. Prepara un presupuesto en borrador con líneas de
mano de obra (múltiplos de 0,25 h a `TALLER_PRECIO_HORA`, concepto escrito) y de piezas (unidades
enteras al `precio_venta`, concepto «referencia descripción»), y puede quitar líneas. Al emitirlo
se fijan totales, IVA, `caduca_el` (+15 días) y el token, queda bloqueado y se envía el correo con
el enlace que usa spec-004. Los mismos endpoints sirven para las **ampliaciones** de spec-005.

## Technical Context

**Language/Version · Dependencies · Storage · Testing · Platform**: igual que el plan maestro
(Rust + Axum + SQLx, SvelteKit, PostgreSQL 16, `#[sqlx::test]`). Sin dependencias nuevas.

**Project Type**: aplicación web; módulo `presupuestos.rs` dentro del backend común.

**Configuración usada**: `TALLER_PRECIO_HORA`, `TALLER_IVA_PORCENTAJE`, `URL_PUBLICA`, SMTP.

**Performance Goals**: un presupuesto de 5 líneas en menos de 5 minutos (SC-001); pantallas en
menos de 1 s.

**Constraints**: todo en español, importes con `formato.ts` y fechas dd/mm/aaaa. Dinero con
`dinero.rs` (R4), «hoy» con `fechas::hoy()` (R12), transiciones con `cambiar_estado` (R14).

**Scale/Scope**: 2 tablas propias (`presupuesto`, `linea_presupuesto`) + `pieza` y `orden`
(lectura), 1 migración pequeña, 6 endpoints, 2 secciones de la ficha de la orden y 2 componentes
compartidos.

- **Endpoints** ([contrato](../000-base-comun/contracts/api.md#spec-003--diagnóstico-y-presupuesto-y-ampliaciones-de-spec-005)):
  `PUT /api/ordenes/{id}/diagnostico`, `POST /api/ordenes/{id}/presupuestos`,
  `GET /api/presupuestos/{id}`, `POST /api/presupuestos/{id}/lineas`,
  `DELETE /api/presupuestos/{id}/lineas/{linea_id}`, `POST /api/presupuestos/{id}/emitir`.
  `GET /api/piezas?q=` ya existe (spec-006).
- **Pantallas**: secciones «Diagnóstico» y «Presupuesto» de `/ordenes/[id]` (la ficha es de
  spec-002), con los componentes compartidos `TablaLineas` y `Totales` (los reutilizan 004, 005 y
  007).

No queda ningún NEEDS CLARIFICATION: las 4 dudas se cerraron en la sesión de clarify del 22/10 y
las decisiones abiertas en [research.md](research.md).

## Constitution Check

*GATE: antes de Phase 0 y de nuevo después de Phase 1.*

| Principio | Comprobación | Estado |
|---|---|---|
| I. Simplicidad | Un precio por hora y un IVA únicos; sin descuentos ni edición de líneas (se quita y se añade). Reglas en la base de datos (`CHECK` de cantidades D1, índice parcial de borrador D5) y `FOR UPDATE`. Sin capas: `presupuestos.rs` hace sus consultas SQLx. | ✅ |
| II. Español y euro | Mensajes en español; importes con `formato.ts` (es-ES, EUR); «Válido hasta dd/mm/aaaa». | ✅ |
| III. Cero alcance fantasma | Quitar líneas y cambiar el diagnóstico están ya en la spec (FR-002a, FR-001a). Sin plantillas, catálogos, descuentos ni edición de líneas. El correo es el de spec-004 FR-001. | ✅ |
| IV. Verificable sin código | Cada SC tiene su paso en [quickstart §3](../000-base-comun/quickstart.md); los importes se comprueban con calculadora. | ✅ |
| V. Datos | Solo se lee el correo del cliente para enviarle su enlace (spec-004 FR-001); el token es aleatorio (R9). | ✅ |

**Re-check post-diseño**: la migración `0002` solo añade dos `CHECK` que concretan FR-002; los
cambios al contrato definen `PresupuestoResumen`, la respuesta del diagnóstico y los errores; no
se añade alcance. ✅

## Project Structure

### Documentation (this feature)

```text
specs/003-diagnostico-presupuesto/
├── spec.md
├── plan.md        # este fichero
└── research.md    # decisiones D1–D8
# data-model, contrato y quickstart: los compartidos de specs/000-base-comun/ (actualizados)
```

### Source Code (repository root)

```text
backend/migrations/0002_spec003_lineas.sql      # CHECK de cantidades (D1)
backend/src/rutas/presupuestos.rs              # diagnóstico, presupuestos, líneas, emitir
backend/tests/spec003_presupuestos.rs          # pruebas de reglas (ver abajo)
frontend/src/lib/componentes/TablaLineas.svelte   # compartido con 004, 005, 007
frontend/src/lib/componentes/Totales.svelte       # compartido con 004, 005, 007
frontend/src/routes/ordenes/[id]/Diagnostico.svelte
frontend/src/routes/ordenes/[id]/Presupuesto.svelte
frontend/src/lib/tipos.ts                       # Presupuesto, PresupuestoResumen (compartido)
```

**Structure Decision**: la del plan maestro. Esta spec añade su módulo de rutas, una migración,
su fichero de pruebas, dos componentes compartidos y dos secciones de la ficha de la orden.

### Reglas clave (cada una en una transacción)

- **Diagnóstico** (FR-001, FR-001a; D2): bloquear la orden; `recibida` → guardar y
  `cambiar_estado(&[Recibida], EnDiagnostico)`, publicar tras el `commit`; `en_diagnostico` sin
  presupuesto emitido → solo guardar; si no, 409 con el motivo. Vacío → 422.
- **Crear presupuesto** (FR-008; D5): bloquear la orden; tipo según el estado; 409 si hay uno
  emitido sin caducar o (índice parcial) un borrador.
- **Añadir línea** (FR-002…FR-005; D1, D3, D4): presupuesto en `borrador` (si no, 409); validar
  cantidad (0,25 h o entero) y concepto → 422; pieza inexistente → 422; precio y concepto los pone
  el servidor; `importe = dinero::importe_linea`. Responde el `Presupuesto` con totales.
- **Quitar línea** (FR-002a): presupuesto en `borrador`; la línea debe ser de ese presupuesto (404).
- **Emitir** (FR-007; D6): bloquear el presupuesto; `borrador` y con líneas; fijar totales,
  `caduca_el = hoy + 15`, token; tras el `commit`, enviar el correo con el enlace.
- **Permisos** (D7): todo lo que escribe exige `exigir_mecanico_de(orden.mecanico_id)`.

### Pruebas (`spec003_presupuestos.rs`)

1. Diagnóstico en orden `recibida` → `en_diagnostico` y evento; cambiarlo sin presupuesto
   emitido → se guarda y no hay evento; con uno emitido → 409; vacío → 422; orden `en_reparacion`
   → 409; otro mecánico → 403.
2. Crear presupuesto en `en_diagnostico` → `inicial`; en `pendiente_conformidad` → `ampliacion`;
   en `recibida` → 409; con un borrador → 409; con uno emitido vigente → 409; con uno caducado → 201.
3. Líneas: 1,5 h × 40,00 = 60,00; PF-123 × 2 = 50,00 con concepto «PF-123 Pastillas freno»;
   totales 110,00 / 23,10 / 133,10; 1,3 h, 2,5 uds, 0, negativa, concepto vacío → 422; pieza
   inexistente → 422; pieza sin stock → se admite.
4. Quitar la de mano de obra → 50,00 / 10,50 / 60,50; línea de otro presupuesto → 404.
5. Emitir → `caduca_el = hoy + 15`, token, totales guardados, correo enviado (o
   `aviso_enviado: false` si falla); sin líneas → 422; añadir o quitar tras emitir → 409; emitir
   dos veces → 409.
6. Redondeo: 3 líneas con importes que obligan a redondear → la base es la suma de los importes
   mostrados y total = base + IVA.
7. `presupuestos_de_orden` devuelve los de esa orden con `caducado` correcto.

**Dependencias**: base común; piezas de spec-006 (ya en `main`); para la pantalla, la ficha de la
orden de spec-002 (Persona A). Spec-004 (A) consume `presupuesto.token`, `estado`, `caduca_el` y
`respondido_en`; A no modifica `presupuestos.rs` y B no modifica `publico_presupuesto.rs`.

**Cambios en documentos compartidos** (los revisa también la Persona A antes de fusionar):
[data-model.md](../000-base-comun/data-model.md) (`linea_presupuesto`: cantidades, concepto,
precio fijado al añadir), [contracts/api.md](../000-base-comun/contracts/api.md)
(`PresupuestoResumen`, respuesta del diagnóstico, `aviso_enviado`, errores) y
[quickstart.md §3](../000-base-comun/quickstart.md) (quitar línea, cambiar diagnóstico, cantidades
no válidas).

## Complexity Tracking

No hay violaciones que justificar.
