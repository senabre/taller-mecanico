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
backend/tests/spec008_tablero.rs        # 6 columnas aunque vacías, sin facturadas, orden de las tarjetas, retrasada (ayer sí / hoy no), carga con 0 e inactivo con órdenes, 403 para recepción, mecánico y almacén
frontend/src/routes/tablero/+page.svelte
frontend/src/lib/medianoche.ts          # ms hasta la próxima 00:00 Europe/Madrid, con Intl (clarify Q3)
frontend/src/lib/medianoche.test.ts     # vi.useFakeTimers: el tablero se vuelve a pedir a las 00:00 de Madrid aunque el navegador esté en otra zona
```

**Reglas clave**:
- `retrasada` = orden no facturada con `fecha_entrega_prometida < fechas::hoy()` (Europe/Madrid),
  como dice el [data-model común](../000-base-comun/data-model.md), que es la fuente única. Se
  calcula en el backend para que el criterio sea el mismo en todas las pantallas (corregido por el
  hallazgo I1 de spec-002, 2026-11-03).
- Las tarjetas usan el tipo `OrdenResumen` y su consulta de `backend/src/rutas/ordenes.rs`
  (spec-002, `pub`), que ya calcula `retrasada`; `tablero.rs` no escribe una consulta propia de
  las órdenes, solo las agrupa por estado (hallazgo I2 de spec-002). Como `resumenes()` también
  devuelve las facturadas, `tablero.rs` las descarta al agrupar.
- Orden de las tarjetas (clarify Q4): dentro de cada columna, `fecha_entrega_prometida` ascendente
  y después `numero` ascendente. Se ordena en `tablero.rs`, sin tocar la consulta de `ordenes.rs`.
- Carga por mecánico (clarify Q1): `LEFT JOIN` de los usuarios mecánicos con las órdenes no
  facturadas, quedándose con los activos (aunque tengan 0) y con los inactivos que tengan alguna
  orden abierta. Cada fila lleva `activo`, para que la pantalla marque «(de baja)». Así la suma
  de la carga coincide con el número de tarjetas (SC-003).
- Acceso (clarify Q2): `exigir_rol(&[Rol::Jefe])` en `GET /api/tablero` → 403 para los demás
  roles; `/tablero` muestra entonces `+error.svelte`. `GET /api/eventos` sigue abierto a cualquier
  usuario con sesión: solo envía `{id, estado}`.
- Frontend (clarify Q3): se vuelve a pedir `/api/tablero` entero (1) con cada evento `orden` de
  `EventSource('/api/eventos')`; (2) en `onopen` tras un `onerror`, porque lo enviado durante la
  desconexión se ha perdido; y (3) a las 00:00 Europe/Madrid, con un `setTimeout` hasta la
  siguiente medianoche calculada con `Intl.DateTimeFormat` (`timeZone: 'Europe/Madrid'`), no con
  la zona del navegador, y que se vuelve a programar después. Con `onerror` se muestra el aviso
  de desconexión; la reconexión la hace `EventSource` solo.
- Eventos que mueven el tablero (comprobado en la rama spec-008, 2026-11-12): los 9 movimientos
  publican `CambioEstado::publicar` después del `commit`. Son abrir (`ordenes.rs`, spec-002);
  recibida→en diagnóstico (`presupuestos.rs`, 003); aceptar el inicial y la ampliación
  (`publico_presupuesto.rs`, 004); pasar del 110 % y terminar (`imputaciones.rs`, 005); pedir
  pieza y recibirla (`almacen.rs`, 006); y facturar (`facturas.rs`, 007).

**Dependencias**: base común (`eventos`, `cambiar_estado`); spec-002 (`OrdenResumen` y el evento
`{id, recibida}` al abrir una orden); y los eventos de las specs 003 a 007 para que el tablero se
mueva en todos los cambios de estado.
