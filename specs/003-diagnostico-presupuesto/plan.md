# Implementation Plan: spec-003 · Registrar diagnóstico y preparar presupuesto

**Branch**: `003-diagnostico-presupuesto` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md) | **Responsable**: Persona B

**Input**: Feature specification from `/specs/003-diagnostico-presupuesto/spec.md`

**Note**: Lo común está en el **plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md).

## Summary

El mecánico asignado guarda el diagnóstico (`recibida → en_diagnostico`) y prepara un presupuesto
en borrador con líneas de mano de obra (a `TALLER_PRECIO_HORA`) y de piezas (al `precio_venta` de
la referencia). Al emitirlo se fijan totales, IVA, `caduca_el` (+15 días) y el token, y se envía
el correo con el enlace que usa spec-004. Los mismos endpoints sirven para las **ampliaciones** de
spec-005.

## Technical Context

Igual que el plan maestro. Configuración usada: `TALLER_PRECIO_HORA`, `TALLER_IVA_PORCENTAJE`,
`URL_PUBLICA`.

- **Tablas**: `presupuesto`, `linea_presupuesto`, `pieza` (lectura).
- **Endpoints**: `PUT /api/ordenes/{id}/diagnostico`, `POST /api/ordenes/{id}/presupuestos`,
  `GET /api/presupuestos/{id}`, `POST|DELETE …/lineas`, `POST …/emitir`, `GET /api/piezas?q=`
  ([contrato](../000-base-comun/contracts/api.md#spec-003--diagnóstico-y-presupuesto-y-ampliaciones-de-spec-005)).
- **Pantallas**: sección «Diagnóstico y presupuesto» de `/ordenes/[id]`, con los componentes
  compartidos `TablaLineas` y `Totales`.

## Constitution Check

| Principio | Comprobación | Estado |
|---|---|---|
| I | Un precio por hora único; sin descuentos; IVA único | ✅ |
| II | Importes con `formato.ts` (es-ES, EUR) | ✅ |
| III | Sin plantillas de presupuesto ni catálogos | ✅ |
| IV | quickstart §3 (los totales se comprueban con calculadora) | ✅ |
| V | — | ✅ |

## Project Structure

```text
backend/src/rutas/presupuestos.rs        # incluye emitir() → correo::enviar_enlace_presupuesto
backend/src/dinero.rs                    # (base común) importe_linea, totales(base, iva%)
backend/tests/spec003_presupuestos.rs    # importes/IVA/redondeo, caducidad +15, bloqueo tras emitir, solo piezas existentes, solo mecánico asignado
frontend/src/routes/ordenes/[id]/Presupuesto.svelte
frontend/src/lib/componentes/TablaLineas.svelte, Totales.svelte   # compartidos con 004, 005, 007
```

**Reglas clave**:
- Al crear un presupuesto, el tipo se deduce del estado de la orden: `en_diagnostico` → `inicial`;
  `pendiente_conformidad` → `ampliacion`. En cualquier otro estado, 409.
- Como mucho un presupuesto abierto por orden (FR-008). La base de datos solo impide dos
  `borrador` (índice parcial). Al crear un presupuesto, en la misma transacción y con la orden
  bloqueada (`SELECT … FOR UPDATE`), devolver 409 si ya hay uno `emitido` con
  `caduca_el >= fechas::hoy()`. Un caducado no bloquea.
- Solo el mecánico asignado (o el jefe): `usuario.exigir_mecanico_de(orden.mecanico_id)` de
  `auth.rs`.
- `emitir`: se recalcula todo con `dinero.rs`, `caduca_el = fechas::hoy() + 15` y se genera el
  token (research R9). Se envía el correo **después** de confirmar (research R10).
- Para probar sin spec-006: piezas de `seed/demo.sql`.

**Dependencias**: spec-002 (orden), las piezas de spec-006 o los datos demo, y el correo de la base
común.
