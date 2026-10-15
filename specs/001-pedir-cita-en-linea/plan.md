# Implementation Plan: spec-001 · Pedir cita en línea

**Branch**: `001-pedir-cita-en-linea` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md) | **Responsable**: Persona A

**Input**: Feature specification from `/specs/001-pedir-cita-en-linea/spec.md`

**Note**: Lo común (pila, arquitectura, autenticación, dinero, fechas, estructura) está en el
**plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md). Este plan solo recoge lo
propio de la spec.

## Summary

Una página pública, `/cita`, busca el vehículo por matrícula normalizada, ofrece los días
laborables con plazas en los próximos 30 días y crea la cita, y también el cliente y el vehículo
si son nuevos. El cupo se protege con un bloqueo por fecha (research R6). Recepción ve las citas
del día en `/citas`.

## Technical Context

Igual que el plan maestro. Configuración usada: `TALLER_CUPO_DIARIO`.

- **Tablas**: `cliente`, `vehiculo`, `cita` ([data-model](../000-base-comun/data-model.md)).
- **Endpoints**: `GET /api/publico/vehiculos/{matricula}`, `GET /api/publico/dias-disponibles`,
  `POST /api/publico/citas`, `GET /api/citas?fecha=` ([contrato](../000-base-comun/contracts/api.md#spec-001--citas)).
- **Pantallas**: `/cita` (pública, usable desde el móvil) y `/citas` (recepción).

## Constitution Check

| Principio | Comprobación | Estado |
|---|---|---|
| I | Citas por día y sin hora; sin cancelación ni modificación | ✅ |
| II | Fechas dd/mm/aaaa; días de lunes a viernes en Europe/Madrid | ✅ |
| III | Sin festivos, recordatorios ni gestión de citas | ✅ |
| IV | quickstart §1 | ✅ |
| V | Solo nombre, teléfono y correo; la consulta pública de matrícula devuelve solo marca y modelo | ✅ |

## Project Structure

```text
backend/src/rutas/citas.rs        # 4 endpoints + normalizar_matricula() (la usan 007 y otras)
backend/tests/spec001_citas.rs    # normalización, alta nueva, vehículo existente, cupo, concurrencia
frontend/src/routes/cita/+page.svelte
frontend/src/routes/citas/+page.svelte
```

**Reglas clave a implementar**:
- `normalizar_matricula`: en mayúsculas y sin espacios ni guiones. Va en `citas.rs` como `pub fn`.
- `POST /api/publico/citas`, todo en una transacción:
  1. Validar que la fecha está dentro del rango y es laborable.
  2. `pg_advisory_xact_lock(fecha)`.
  3. Contar las citas de esa fecha y devolver 409 si se ha llegado al cupo.
  4. Si la matrícula es nueva, crear cliente y vehículo.
  5. Crear la cita.
- Si al confirmar hay 409, el frontend vuelve a pedir los días disponibles y muestra «Ese día se
  acaba de completar; elige otro».

**Dependencias**: base común. No depende de ninguna otra spec.
