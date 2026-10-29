# Implementation Plan: spec-007 · Facturar la orden y avisar al cliente

**Branch**: `007-facturar-avisar-cliente` | **Date**: 2026-10-29 | **Spec**: [spec.md](spec.md) | **Responsable**: Persona B

**Input**: Feature specification from `/specs/007-facturar-avisar-cliente/spec.md`

**Note**: Lo común está en el **plan maestro** [000-base-comun/plan.md](../000-base-comun/plan.md).
Decisiones propias de esta spec: [research.md](research.md). El modelo de datos, el contrato y la
guía de validación son los compartidos (ver Phase 1).

## Summary

Recepción factura una orden `terminada`: la factura simplificada (Q2-A) copia las imputaciones a
`linea_factura`, toma el siguiente número de una serie única (`contador('factura')`), calcula base,
IVA y total con `dinero.rs` y pasa la orden a `facturada`. Después de guardar se envía el correo
«Su vehículo … está listo para recoger» con el total y se anota si salió. La factura se ve en la
app y se imprime con la impresión del navegador. El historial por matrícula lista las reparaciones
facturadas de un vehículo.

## Technical Context

**Language/Version · Dependencies · Storage · Testing · Platform**: igual que el plan maestro
(Rust + Axum + SQLx, SvelteKit, PostgreSQL 16, `#[sqlx::test]`). Sin dependencias nuevas ni
migraciones: `factura`, `linea_factura` y `contador('factura')` ya existen en `0001`.

**Project Type**: aplicación web; módulo `facturas.rs` dentro del backend común.

**Configuración usada**: `TALLER_IVA_PORCENTAJE`, `TALLER_NOMBRE`, `TALLER_NIF`,
`TALLER_DIRECCION`, `TALLER_LOGO` (opcional, FR-009), SMTP.

**Performance Goals**: facturar en menos de 30 s (SC-001); historial en menos de 10 s (SC-004);
pantallas en menos de 1 s.

**Constraints**: todo en español, importes en € con formato español (`formato.ts` y, en el correo,
`formato::euros`), fechas dd/mm/aaaa. Correlativo con `contador.rs` (R5), dinero con `dinero.rs`
(R4), transición con `cambiar_estado` y evento tras el `commit` (R14), correo tras el `commit` (R10).

**Scale/Scope**: 2 tablas (`factura`, `linea_factura`) + `imputacion`, `orden`, `vehiculo` y
`cliente` (lectura), 3 endpoints, 1 sección de la ficha de la orden y 2 páginas.

- **Endpoints** ([contrato](../000-base-comun/contracts/api.md#spec-007--factura-e-historial)):
  `POST /api/ordenes/{id}/factura`, `GET /api/facturas/{id}`,
  `GET /api/vehiculos/{matricula}/historial` y `GET /api/taller/logo` (D8).
- **Pantallas**: sección «Factura» de `/ordenes/[id]` (la ficha es de spec-002), la página
  imprimible `/facturas/[id]` y `/historial` (la entrada del menú ya existe en la base común).

No queda ningún NEEDS CLARIFICATION: las 2 dudas se cerraron en la sesión de clarify del 29/10 y
las decisiones abiertas en [research.md](research.md). El logo (FR-009) se añadió a la spec el 29/10
a petición de la Persona B, antes de construirlo (principio III).

## Constitution Check

*GATE: antes de Phase 0 y de nuevo después de Phase 1.*

| Principio | Comprobación | Estado |
|---|---|---|
| I. Simplicidad | Impresión del navegador, sin PDF en el servidor; logo como una variable más de configuración, sin subir archivos (D8); serie de facturas única; sin migraciones; `linea_factura` sin `tipo` (D6); reglas con `FOR UPDATE` y `UNIQUE`. | ✅ |
| II. Español y euro | Factura, correo y pantallas en español; importes «157,30 €» (también en el correo, D2) y fechas dd/mm/aaaa. | ✅ |
| III. Cero alcance fantasma | Sin cobros, entrega, VeriFactu, factura completa, reenvío del aviso ni envío de la factura por correo. Imprimir está en la spec (FR-003, aclaración del 29/10). | ✅ |
| IV. Verificable sin código | Cada SC tiene su paso en [quickstart §7](../000-base-comun/quickstart.md); los totales se comprueban con calculadora y el correo en Mailpit. | ✅ |
| V. Datos | Factura simplificada: solo nombre del cliente y matrícula (Q2-A); el correo se usa solo para el aviso de FR-006. | ✅ |

**Re-check post-diseño**: el contrato concreta errores, roles, `LineaFactura` y el 404 del
historial; `formato.rs` gana `euros()` y `TablaLineas` acepta líneas sin `tipo`. Sin alcance
nuevo. ✅

## Project Structure

### Documentation (this feature)

```text
specs/007-facturar-avisar-cliente/
├── spec.md
├── plan.md        # este fichero
└── research.md    # decisiones D1–D7
# data-model, contrato y quickstart: los compartidos de specs/000-base-comun/ (actualizados)
```

### Source Code (repository root)

```text
backend/src/formato.rs                         # + euros(Decimal) (base común, D2)
backend/src/rutas/facturas.rs                  # facturar, ver, historial, factura_de_orden
backend/tests/spec007_facturas.rs              # pruebas de reglas (ver abajo)
frontend/src/lib/componentes/TablaLineas.svelte # acepta líneas sin tipo (D6)
frontend/src/lib/tipos.ts                      # Factura, LineaFactura, Historial (compartido)
frontend/src/routes/ordenes/[id]/Factura.svelte
frontend/src/routes/facturas/[id]/+page.svelte  # vista imprimible
frontend/src/routes/historial/+page.svelte
```

**Structure Decision**: la del plan maestro. Esta spec añade su módulo de rutas, su fichero de
pruebas, una sección de la ficha y dos páginas, y amplía `formato.rs` y `TablaLineas`.

### Reglas clave

- **Facturar** (FR-001…FR-006, FR-008; D1, D2), en una transacción: bloquear la orden;
  `terminada` (409); con imputaciones (422); número del contador; totales; insertar factura y
  líneas (`UNIQUE` → 409); `terminada → facturada`; `commit`; evento; correo; `aviso_enviado`.
- **Ver factura** (FR-003; D3): recepción y jefe; datos del taller de la configuración.
- **Historial** (FR-007; D5): normalizar la matrícula (422), vehículo inexistente (404), facturas
  de la más reciente a la más antigua.
- **Permisos**: facturar y ver factura → `exigir_rol(&[Recepcion])`; historial → cualquier sesión.

### Pruebas (`spec007_facturas.rs`)

1. Facturar una orden `terminada` con 2 h a 40,00 y PF-123 × 2 → 201, líneas copiadas en orden,
   base 130,00, IVA 27,30, total 157,30, fecha de hoy, `cliente_nombre` y `matricula`, orden
   `facturada`, evento del tablero.
2. Correlativo: dos facturas seguidas → números consecutivos; si una falla (sin imputaciones),
   el siguiente número no deja hueco.
3. Estados: `en_reparacion`, `pendiente_conformidad`… → 409; facturar dos veces → 409 y una sola
   factura; sin imputaciones → 422; mecánico → 403; jefe → 201.
4. Aviso: con SMTP válido `aviso_enviado = true`; con un puerto cerrado → 201, factura emitida y
   `aviso_enviado = false`.
5. `GET /api/facturas/{id}`: datos del taller, líneas y totales; mecánico → 403; 404.
6. Historial: dos facturas del mismo vehículo → la más reciente primero, con km, diagnóstico,
   líneas y total; `1234 bcd` se normaliza; vehículo sin facturas → `[]`; matrícula inexistente →
   404; no válida → 422; cualquier rol → 200.
7. `formato::euros`: «157,30 €», «1.234,56 €», «0,00 €».
8. `factura_de_orden` devuelve el id de la factura de la orden o nada.

**Dependencias**: spec-005 (imputaciones y estado `terminada`, ya en `main`); para la sección de
la ficha, spec-002 (Persona A). Datos demo: la orden 8 `terminada` para facturar y la 9
`facturada` para el historial.

**Cambios en documentos compartidos** (los revisa también la Persona A):
[data-model.md](../000-base-comun/data-model.md) (`factura`: serie única, `aviso_enviado` tras el
`commit`), [contracts/api.md](../000-base-comun/contracts/api.md) (errores, roles, `LineaFactura`,
404/422 del historial) y [quickstart.md §7](../000-base-comun/quickstart.md) (imprimir, número,
aviso).

## Complexity Tracking

No hay violaciones que justificar.
