# Implementation Plan: spec-005 · Imputar tiempo y piezas durante la reparación

**Branch**: `005-imputar-tiempo-piezas` | **Date**: 2026-10-29 | **Spec**: [spec.md](spec.md) | **Responsable**: Persona B

**Input**: Feature specification from `/specs/005-imputar-tiempo-piezas/spec.md`

**Note**: Lo común está en el **plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md).
Decisiones propias de esta spec: [research.md](research.md). El modelo de datos, el contrato y la
guía de validación son los compartidos (ver Phase 1).

## Summary

El mecánico asignado imputa horas (múltiplos de 0,25, con concepto) y piezas (enteras, concepto
«referencia descripción») en órdenes `en_reparacion`. Cada pieza descuenta el stock en la misma
transacción y no se puede imputar más de lo que hay. Tras cada imputación se compara el coste
acumulado con la suma de los presupuestos aceptados; si supera el 110 %, la orden pasa a
`pendiente_conformidad`. Las ampliaciones reutilizan los endpoints de spec-003 y la respuesta de
spec-004. El mecánico termina la orden si tiene al menos una imputación.

## Technical Context

**Language/Version · Dependencies · Storage · Testing · Platform**: igual que el plan maestro
(Rust + Axum + SQLx, SvelteKit, PostgreSQL 16, `#[sqlx::test]`). Sin dependencias nuevas.

**Project Type**: aplicación web; módulo `imputaciones.rs` dentro del backend común.

**Configuración usada**: `TALLER_PRECIO_HORA`.

**Performance Goals**: una imputación en menos de 30 s (SC-003); pantallas en menos de 1 s.

**Constraints**: todo en español, importes con `formato.ts`, horas con coma (1,25 h). Dinero con
`dinero.rs` y `Decimal` (R4); transiciones con `cambiar_estado` y evento tras el `commit` (R14).

**Scale/Scope**: 1 tabla propia (`imputacion`) + `pieza` (stock) y `presupuesto` (lectura de los
aceptados), 1 migración pequeña, 1 módulo compartido de validación de líneas, 2 endpoints, 1
sección de la ficha de la orden y 1 componente compartido.

- **Endpoints** ([contrato](../000-base-comun/contracts/api.md#spec-005--imputaciones)):
  `POST /api/ordenes/{id}/imputaciones`, `POST /api/ordenes/{id}/terminar`. Las ampliaciones usan
  los de spec-003 (ya en `main`).
- **Pantallas**: sección «Imputaciones» de `/ordenes/[id]` (la ficha es de spec-002) con el
  indicador «Acumulado X € de Y € presupuestados», la tabla de imputaciones, el formulario y el
  botón «Terminar reparación».

No queda ningún NEEDS CLARIFICATION: las 3 dudas se cerraron en la sesión de clarify del 29/10 y
las decisiones abiertas en [research.md](research.md).

## Constitution Check

*GATE: antes de Phase 0 y de nuevo después de Phase 1.*

| Principio | Comprobación | Estado |
|---|---|---|
| I. Simplicidad | Acumulado y presupuestado calculados al vuelo, sin guardarlos. Stock y umbral protegidos por la base de datos (`UPDATE … WHERE stock >= n`, `CHECK`) y `FOR UPDATE`. Una sola validación de líneas (D1) y un solo formulario (D8) para presupuesto e imputación. | ✅ |
| II. Español y euro | Mensajes en español; importes con `formato.ts`; horas «1,25 h». | ✅ |
| III. Cero alcance fantasma | Sin anular ni corregir imputaciones, sin control horario, sin salida para una ampliación rechazada (límite conocido escrito en la spec). No hay endpoints nuevos para ampliaciones. | ✅ |
| IV. Verificable sin código | Cada SC tiene su paso en [quickstart §5](../000-base-comun/quickstart.md); el acumulado se comprueba con calculadora y el stock en `/almacen`. | ✅ |
| V. Datos | Solo se guarda qué mecánico imputa y cuándo (entidad «Imputación» de la spec). | ✅ |

**Re-check post-diseño**: la migración `0003` solo añade un `CHECK` que concreta FR-002; mover la
validación de líneas a `lineas.rs` y el formulario a un componente no cambia el comportamiento de
spec-003 (sus 20 pruebas siguen valiendo); el contrato cambia la respuesta (`ResumenImputaciones`
en vez de `Orden`) y añade errores. Sin alcance nuevo. ✅

## Project Structure

### Documentation (this feature)

```text
specs/005-imputar-tiempo-piezas/
├── spec.md
├── plan.md        # este fichero
└── research.md    # decisiones D1–D8
# data-model, contrato y quickstart: los compartidos de specs/000-base-comun/ (actualizados)
```

### Source Code (repository root)

```text
backend/migrations/0003_spec005_imputaciones.sql   # CHECK de piezas enteras (D2)
backend/src/lineas.rs                              # validación de líneas, compartida con 003 (D1)
backend/src/rutas/presupuestos.rs                  # pasa a usar lineas.rs (sin cambiar su comportamiento)
backend/src/rutas/imputaciones.rs                  # imputar, terminar, resumen_imputaciones
backend/tests/spec005_imputaciones.rs              # pruebas de reglas (ver abajo)
frontend/src/lib/componentes/FormularioLinea.svelte # sacado de Presupuesto.svelte (D8)
frontend/src/routes/ordenes/[id]/Presupuesto.svelte # pasa a usar FormularioLinea
frontend/src/routes/ordenes/[id]/Imputaciones.svelte
frontend/src/lib/tipos.ts                          # ResumenImputaciones (compartido)
```

**Structure Decision**: la del plan maestro. Esta spec añade su módulo de rutas, una migración,
su fichero de pruebas y una sección de la ficha; y saca a módulos compartidos la validación y el
formulario de línea que ya existían en spec-003.

### Reglas clave (cada una en una transacción)

- **Imputar** (FR-001, FR-002, FR-004, FR-005; D3, D4): bloquear la orden; mecánico asignado;
  `en_reparacion` y con presupuesto aceptado (409); validar la línea (422); pieza → descontar stock
  con `WHERE stock >= n` (422 «Solo hay N unidades…; el almacén puede pedirla al proveedor»); insertar; si acumulado > presupuestado ×
  1,10 → `pendiente_conformidad`; evento tras el `commit`. Responde 201 `ResumenImputaciones`.
- **Terminar** (FR-008; D6): bloquear la orden; mecánico asignado; `en_reparacion` (409); al menos
  una imputación (422); `cambiar_estado(&[EnReparacion], Terminada)`; evento tras el `commit`.
- **Ampliación** (FR-006, FR-007; D7): endpoints de spec-003; aceptación en spec-004.
  `total_presupuestado` = Σ `base` de los `aceptado`.
- **Permisos**: `exigir_mecanico_de(orden.mecanico_id)` (el jefe pasa siempre).

### Pruebas (`spec005_imputaciones.rs`)

1. Imputar 2 uds de una pieza con stock 10 → stock 8; 1 h a 40,00 € + 2 × 25,00 € con 110,00 €
   presupuestados → acumulado 90,00 de 110,00; concepto de pieza «referencia descripción».
2. Stock 1 e imputar 2 → 422 «Solo hay 1 unidad…» (singular), stock 0 → «Solo hay 0 unidades…», el stock y las imputaciones no cambian.
3. Validación compartida con 003: 1,3 h, 2,5 uds, 0, negativa, concepto vacío, pieza inexistente
   → 422; `CHECK` de piezas enteras en la base de datos.
4. 10 %: con 100,00 presupuestados, llegar a 110,00 → sigue `en_reparacion`; una imputación que lo
   lleva a 115,00 → se guarda, la orden pasa a `pendiente_conformidad`, llega el evento y la
   siguiente imputación → 409.
5. Estados: `en_espera_piezas`, `pendiente_conformidad`, `terminada`, `en_diagnostico` → 409;
   `en_reparacion` sin presupuesto aceptado → 409; otro mecánico → 403.
6. Ampliación: con la orden bloqueada, crear y emitir una ampliación de 50,00 (endpoints de 003);
   simular la aceptación por SQL → `total_presupuestado` 150,00 y se puede volver a imputar hasta
   165,00 sin bloqueo.
7. Terminar: con imputaciones → `terminada`, `terminada_en` puesto, evento, y ya no admite
   imputaciones (409); sin imputaciones → 422; `pendiente_conformidad` → 409.
8. `resumen_imputaciones` devuelve las de esa orden con sus totales.
9. Las 20 pruebas de spec-003 siguen pasando tras sacar la validación a `lineas.rs`.

**Dependencias**: base común; piezas y stock de spec-006 y ampliaciones de spec-003 (ya en
`main`); aceptación de presupuestos de spec-004 y ficha de la orden de spec-002 (Persona A). Las
pruebas simulan lo de spec-004 con SQL; la pantalla espera a la ficha de spec-002.

**Cambios en documentos compartidos** (los revisa también la Persona A):
[data-model.md](../000-base-comun/data-model.md) (`imputacion`: concepto, piezas enteras, regla
del 10 %), [contracts/api.md](../000-base-comun/contracts/api.md) (`ResumenImputaciones`, errores
de imputar y terminar) y [quickstart.md §5](../000-base-comun/quickstart.md) (stock insuficiente,
terminar sin imputaciones, datos demo).

## Complexity Tracking

No hay violaciones que justificar.
