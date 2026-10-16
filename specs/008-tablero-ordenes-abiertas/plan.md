# Implementation Plan: spec-008 · Tablero de órdenes abiertas

**Branch**: `008-tablero-ordenes-abiertas` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md) | **Responsable**: Persona A

**Input**: Feature specification from `/specs/008-tablero-ordenes-abiertas/spec.md`

**Note**: Lo común está en el **plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md).

## Summary

`/tablero`, solo para el jefe, muestra 6 columnas con las órdenes no facturadas, la carga de
órdenes por mecánico y las órdenes retrasadas destacadas. Se actualiza solo escuchando
`GET /api/eventos` (SSE, research R11). Cada spec que cambia un estado publica el evento con
`CambioEstado::publicar` después del `commit` (research R14).

## Technical Context

Igual que el plan maestro.

- **Tablas**: `orden`, `usuario`, `vehiculo` (solo lectura).
- **Endpoints**: `GET /api/tablero` y `GET /api/eventos` (base común)
  ([contrato](../000-base-comun/contracts/api.md#spec-008--tablero)).
- **Pantallas**: `/tablero`, con columnas que tienen scroll propio, tarjetas `OrdenResumen`,
  insignia «Retrasada» en el color de error de Skeleton con borde destacado, una barra de carga
  por mecánico y el aviso «Sin conexión, reintentando…».

## Constitution Check

| Principio | Comprobación | Estado |
|---|---|---|
| I | Una consulta para las columnas y otra para la carga; SSE en vez de WebSockets; sin arrastrar tarjetas | ✅ |
| II | Textos y fechas en español | ✅ |
| III | Solo consulta: no se cambian estados desde el tablero | ✅ |
| IV | quickstart §8 (dos navegadores) | ✅ |
| V | Las tarjetas no muestran datos del cliente, solo la matrícula | ✅ |

## Project Structure

```text
backend/src/rutas/tablero.rs
backend/src/eventos.rs                  # (base común) broadcast + SSE
backend/tests/spec008_tablero.rs        # 6 columnas aunque vacías, sin facturadas, retrasada (ayer sí / hoy no), carga con 0
frontend/src/routes/tablero/+page.svelte
```

**Reglas clave**:
- `retrasada` = `fecha_entrega_prometida < fechas::hoy()` (Europe/Madrid). Se calcula en el
  backend para que el criterio sea el mismo en todas las pantallas.
- Carga por mecánico: `LEFT JOIN` de los usuarios mecánicos activos con las órdenes no facturadas,
  de modo que aparecen los que tienen 0.
- Frontend: con `EventSource('/api/eventos')`, en cada evento `orden` se vuelve a pedir
  `/api/tablero`. Con `onerror` se muestra el aviso de desconexión; la reconexión es automática.

**Dependencias**: base común (`eventos`, `cambiar_estado`). Se puede hacer desde el principio con
las órdenes demo en varios estados.
