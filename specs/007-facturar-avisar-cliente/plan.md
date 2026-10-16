# Implementation Plan: spec-007 · Facturar la orden y avisar al cliente

**Branch**: `007-facturar-avisar-cliente` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md) | **Responsable**: Persona B

**Input**: Feature specification from `/specs/007-facturar-avisar-cliente/spec.md`

**Note**: Lo común está en el **plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md).

## Summary

Recepción factura una orden `terminada`. La factura simplificada (Q2-A) se genera copiando las
imputaciones a `linea_factura`, con un número de `contador('factura')` y los totales de
`dinero.rs`. La orden pasa a `facturada` y se envía el correo «listo para recoger». El historial
por matrícula lista las órdenes facturadas.

## Technical Context

Igual que el plan maestro. Configuración usada: `TALLER_IVA_PORCENTAJE`, `TALLER_NOMBRE`,
`TALLER_NIF`, `TALLER_DIRECCION`.

- **Tablas**: `factura`, `linea_factura`, `contador`, `imputacion` (lectura).
- **Endpoints**: `POST /api/ordenes/{id}/factura`, `GET /api/facturas/{id}`,
  `GET /api/vehiculos/{matricula}/historial`
  ([contrato](../000-base-comun/contracts/api.md#spec-007--factura-e-historial)).
- **Pantallas**: sección «Factura» de `/ordenes/[id]` (el botón solo aparece si la orden está
  `terminada`), la vista de factura imprimible con la impresión del navegador y `/historial`.

## Constitution Check

| Principio | Comprobación | Estado |
|---|---|---|
| I | Factura imprimible con la impresión del navegador, sin generar PDF en el servidor | ✅ |
| II | Factura en español, € y fechas dd/mm/aaaa | ✅ |
| III | Sin cobros, VeriFactu ni factura completa | ✅ |
| IV | quickstart §7 | ✅ |
| V | Solo el nombre del cliente y la matrícula (Q2-A); el NIF del taller sale de la configuración | ✅ |

## Project Structure

```text
backend/src/rutas/facturas.rs          # facturar, ver, historial (usa matricula::normalizar_matricula, base común)
backend/tests/spec007_facturas.rs      # solo terminada, sin imputaciones, una por orden, correlativo, totales = imputaciones, aviso fallido no deshace
frontend/src/routes/ordenes/[id]/Factura.svelte
frontend/src/routes/facturas/[id]/+page.svelte      # vista imprimible
frontend/src/routes/historial/+page.svelte
```

**Reglas clave**:
- **Facturar**, en una transacción:
  1. `SELECT … FOR UPDATE` de la orden. Si no está `terminada`, 409.
  2. Si no hay imputaciones, 422.
  3. `contador::siguiente("factura")`.
  4. Insertar la factura y copiar las líneas.
  5. `cambiar_estado(terminada → facturada)`.

  Después del commit, `correo::aviso_recogida`; el resultado se guarda en `aviso_enviado`.
- Historial: órdenes `facturada` del vehículo con su factura, ordenadas por `fecha DESC`.

**Dependencias**: spec-005 (imputaciones y estado `terminada`). Para empezar antes: una orden
`terminada` con imputaciones en los datos demo.
