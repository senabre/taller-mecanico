# Implementation Plan: spec-004 · Aceptar o rechazar el presupuesto desde un enlace

**Branch**: `004-aceptar-presupuesto-enlace` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md) | **Responsable**: Persona A

**Input**: Feature specification from `/specs/004-aceptar-presupuesto-enlace/spec.md`

**Note**: Lo común está en el **plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md).

## Summary

La página pública `/presupuesto/[token]` muestra el presupuesto y permite aceptarlo o rechazarlo
una sola vez. La respuesta se guarda con `respondido_en` y, según el tipo, cambia el estado de la
orden mediante `estados::cambiar_estado`. Una vez emitido, el enlace se puede copiar desde la
orden.

## Technical Context

Igual que el plan maestro.

- **Tablas**: `presupuesto` (campos `estado`, `respondido_en`).
- **Endpoints**: `GET /api/publico/presupuestos/{token}`,
  `POST /api/publico/presupuestos/{token}/respuesta`
  ([contrato](../000-base-comun/contracts/api.md#spec-004--respuesta-del-cliente)).
- **Pantallas**: `/presupuesto/[token]` (pública, usable desde el móvil) y, en la sección
  Presupuesto de la orden, la decisión, su fecha y hora y el botón «Copiar enlace».

## Constitution Check

| Principio | Comprobación | Estado |
|---|---|---|
| I | Sin cuentas de cliente ni firma digital; token en la URL | ✅ |
| II | «Aceptado el 16/10/2026 a las 10:32» (hora de Madrid) | ✅ |
| III | Sin recordatorios ni cierre sin reparación (Q1-A) | ✅ |
| IV | quickstart §4 (enlace desde Mailpit) | ✅ |
| V | La página pública muestra solo ese presupuesto y el vehículo, sin teléfono ni correo del cliente | ✅ |

## Project Structure

```text
backend/src/rutas/publico_presupuesto.rs
backend/tests/spec004_respuesta.rs     # una sola vez (dos respuestas a la vez, con mutación: debe fallar sin `estado = 'emitido'`), caducado, token inválido, transiciones inicial/ampliación
frontend/src/routes/presupuesto/[token]/+page.svelte
```

**Reglas clave** (en una transacción):

1. El `UPDATE` de la respuesta es el de [research R9](../000-base-comun/research.md), que es la
   única versión (spec-004 Q4; D1 de la base común).
2. Si no se actualiza ninguna fila, el motivo se averigua como dice R9, con el mismo `$hoy`: 404,
   409 «ya respondido» o 409 «caducó el …».
3. Efecto en la orden, en la misma transacción:

   | Respuesta | Presupuesto inicial | Ampliación |
   |---|---|---|
   | Aceptar | `en_diagnostico → en_reparacion` | `pendiente_conformidad → en_reparacion` |
   | Rechazar | `en_diagnostico` (se queda; sin transición, Q2) | `pendiente_conformidad` (se queda) |

- El enlace en la orden (FR-008, Q5): la sección Presupuesto muestra el enlace y «Copiar enlace»
  solo con el presupuesto emitido y vigente. Se oculta **solo en la pantalla**: `GET
  /api/presupuestos/{id}` sigue enviando `enlace`, porque tras la respuesta o la caducidad ya no
  permite cambiar nada (la respuesta da 409). El enlace sigue funcionando para el cliente (US1-3).
- La regla «sin presupuesto aceptado no se imputa» (FR-007) se cumple porque spec-005 solo admite
  imputaciones en `en_reparacion`. Aquí se añade una prueba que lo comprueba.

**Dependencias**: spec-003 (presupuesto emitido y token).
