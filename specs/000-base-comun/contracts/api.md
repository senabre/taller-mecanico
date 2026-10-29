# Contrato de la API HTTP · Talleres Rueda

Este es el contrato **único y compartido**. Si una spec necesita un endpoint nuevo o un cambio, se
añade aquí y lo revisan las dos personas antes de implementarlo.

## Convenciones

- Base: `/api`. JSON en UTF-8. Los campos van en `snake_case` y en español.
- **Autenticación**: cookie `sesion` (research R8). `🔓` = público; si no, se indican los roles
  permitidos (`jefe` siempre puede).
- **Importes**: string decimal con 2 decimales y punto (`"133.10"`), para que JS no lo pase a
  float. El frontend le da formato `133,10 €`.
- **Fechas**: `"2026-10-20"` (DATE). **Instantes**: ISO 8601 con zona (`"2026-10-16T10:32:00+02:00"`).
- **Matrícula**: el servidor la normaliza siempre (research, data-model `vehiculo`).
- **Errores**: `{"error": "Mensaje en español para el usuario", "detalle": {…opcional}}`.
  | Código | Cuándo |
  |---|---|
  | 400 | JSON mal formado |
  | 401 | sin sesión |
  | 403 | rol no permitido o la orden no es del mecánico |
  | 404 | no existe |
  | 409 | conflicto de estado (p. ej., facturar una orden no terminada, cupo completo, respuesta ya dada) |
  | 422 | validación (km menor, cantidad ≤ 0, stock insuficiente…) |

### Objetos comunes

```jsonc
// OrdenResumen (tablero, listas)
{ "id": 7, "numero": 42, "matricula": "1234BCD", "estado": "en_reparacion",
  "mecanico": { "id": 3, "nombre": "Luis" }, "fecha_entrega_prometida": "2026-10-20",
  "retrasada": false }

// Linea (presupuesto, imputación, factura)
{ "id": 1, "tipo": "mano_obra" | "pieza", "concepto": "Cambio de pastillas",
  "pieza_id": null, "cantidad": "1.50", "precio_unitario": "40.00", "importe": "60.00" }

// Totales
{ "base": "110.00", "iva_porcentaje": "21.00", "iva": "23.10", "total": "133.10" }
```

---

## Base común (spec-000)

| Método y ruta | Acceso | Entrada | Salida |
|---|---|---|---|
| `POST /api/sesion` | 🔓 | `{login, contrasena}` | 204 + cookie · 401 «Usuario o contraseña incorrectos» |
| `DELETE /api/sesion` | todos | — | 204 |
| `GET /api/yo` | todos | — | `{id, nombre, rol}` |
| `GET /api/mecanicos` | todos | — | `[{id, nombre}]` (activos con rol mecánico) |
| `GET /api/eventos` | todos | — | `text/event-stream`; eventos `orden` con `{id, estado}` (research R11) |

## spec-001 · Citas

| Método y ruta | Acceso | Entrada | Salida |
|---|---|---|---|
| `GET /api/publico/vehiculos/{matricula}` | 🔓 | — | `{matricula, marca, modelo, cita_pendiente}` · 404 «No hay ningún vehículo con esa matrícula» · 422 «La matrícula no es válida». **Nunca** devuelve datos del cliente ni la fecha de la cita pendiente. |
| `GET /api/publico/dias-disponibles` | 🔓 | — | `["2026-10-16", …]` (de lunes a viernes, desde mañana hasta dentro de 30 días, con plazas) |
| `POST /api/publico/citas` | 🔓 | `{matricula, fecha, motivo, vehiculo?: {marca, modelo}, cliente?: {nombre, telefono, email}}`. Si la matrícula es nueva, `vehiculo` y `cliente` son obligatorios. Si ya existe, se **ignoran**: nunca se modifican el vehículo ni su propietario | 201 `{fecha, motivo, matricula}` · 409 «Este vehículo ya tiene una cita pendiente.» · 409 «Ese día ya está completo» · 422 con el primer campo incorrecto (tabla de abajo) |
| `GET /api/citas?fecha=YYYY-MM-DD` | recepcion | `fecha` opcional (si falta, hoy en Europe/Madrid) | `[{id, matricula, marca, modelo, motivo, cliente: {nombre, telefono}, orden_id|null}]`, en el orden en que se pidieron. **Nunca** devuelve el correo · 401 sin sesión · 403 otros roles |

Validación de spec-001 (422). Esta tabla es la **fuente única** de las reglas de los campos; los
demás documentos remiten a ella. El servidor comprueba en este orden y responde el primer error
como `{"error": "<mensaje>", "detalle": {"campo": "<campo>"}}`. Los campos de `vehiculo` y
`cliente` solo se comprueban si la matrícula es nueva. **Todos los 422 llegan antes que cualquier
409**: primero las filas de esta tabla, después la cita pendiente (409) y por último el cupo
(409).

| Campo | Regla | Mensaje |
|---|---|---|
| `matricula` | tras quitar espacios y guiones y pasar a mayúsculas, de 2 a 10 letras (A-Z) o cifras | «La matrícula no es válida» |
| `fecha` | uno de los días que ofrece `dias-disponibles` (sin contar el cupo, que da 409) | «Elige uno de los días disponibles» |
| `motivo` | no vacío | «El motivo es obligatorio» |
| `vehiculo.marca`, `vehiculo.modelo` | no vacíos | «La marca es obligatoria», «El modelo es obligatorio» |
| `cliente.nombre` | no vacío | «El nombre es obligatorio» |
| `cliente.telefono` | tras quitar espacios y guiones, «+» inicial opcional y de 9 a 15 cifras | «El teléfono no es válido» |
| `cliente.email` | una sola «@», texto delante y, detrás, un dominio con un punto que tiene texto a ambos lados | «El correo no es válido» |

## spec-002 · Orden de reparación

| Método y ruta | Acceso | Entrada | Salida |
|---|---|---|---|
| `POST /api/ordenes` | recepcion | `{cita_id, km, danos, mecanico_id, fecha_entrega_prometida}` | 201 `Orden` · 422 `{"error":"El kilometraje no puede ser menor que el de la visita anterior (85.000 km)","detalle":{"km_anterior":85000}}` · 409 si la cita ya tiene orden |
| `GET /api/ordenes?mias=true` | recepcion, mecanico | — | `[OrdenResumen]` (con `mias` solo las del mecánico que ha entrado) |
| `GET /api/ordenes/{id}` | todos | — | `Orden` = OrdenResumen + `{cita:{fecha,motivo}, vehiculo:{marca,modelo}, km, danos, diagnostico, total_presupuestado, coste_acumulado, presupuestos:[PresupuestoResumen], imputaciones:[Linea], pedidos:[Pedido], factura_id}` |

## spec-003 · Diagnóstico y presupuesto (y ampliaciones de spec-005)

| Método y ruta | Acceso | Entrada | Salida |
|---|---|---|---|
| `PUT /api/ordenes/{id}/diagnostico` | mecanico asignado | `{diagnostico}` | 200 `{id, numero, estado, diagnostico}` (`recibida → en_diagnostico`; en `en_diagnostico` solo lo cambia) · 409 «Ya se ha emitido un presupuesto para esta orden; el diagnóstico no se puede cambiar» · 409 si la orden no está recibida ni en diagnóstico · 422 si está vacío |
| `POST /api/ordenes/{id}/presupuestos` | mecanico asignado | — | 201 `Presupuesto` en borrador. El tipo se deduce del estado: `en_diagnostico` → inicial; `pendiente_conformidad` → ampliación; otro estado → 409. 409 si ya hay uno en borrador o uno emitido sin caducar |
| `GET /api/presupuestos/{id}` | todos | — | `Presupuesto` = `{id, orden_id, tipo, estado, caducado, lineas:[Linea], …Totales, emitido_en, caduca_el, respondido_en, enlace}` (en borrador los totales se calculan al momento; `enlace` es null hasta emitir) |
| `POST /api/presupuestos/{id}/lineas` | mecanico asignado | `{tipo:"mano_obra", concepto, cantidad}` o `{tipo:"pieza", pieza_id, cantidad}` (`cantidad` como string decimal: horas en múltiplos de 0,25, piezas enteras; el concepto de una pieza lo pone el servidor) | 201 `Presupuesto` · 409 «El presupuesto ya está emitido; no se puede modificar» · 422 (cantidad no válida, concepto vacío, pieza inexistente) |
| `DELETE /api/presupuestos/{id}/lineas/{linea_id}` | mecanico asignado | — | 200 `Presupuesto` · 409 si no está en borrador · 404 si la línea no es de ese presupuesto |
| `POST /api/presupuestos/{id}/emitir` | mecanico asignado | — | 200 `Presupuesto` + `aviso_enviado` (fija totales, `caduca_el` = hoy + 15 y token, y después envía el correo con el enlace; si falla, `aviso_enviado: false`) · 409 si ya está emitido · 422 «El presupuesto no tiene líneas» |

`PresupuestoResumen` (campo `presupuestos` de `GET /api/ordenes/{id}`; lo da
`presupuestos::presupuestos_de_orden`): `{id, tipo, estado, caducado, total, emitido_en, caduca_el}`.
| `GET /api/piezas?q=texto` | todos | — | `[{id, referencia, descripcion, precio_venta, stock}]` (búsqueda por referencia o descripción) |

## spec-004 · Respuesta del cliente

| Método y ruta | Acceso | Entrada | Salida |
|---|---|---|---|
| `GET /api/publico/presupuestos/{token}` | 🔓 | — | `{taller:{nombre}, matricula, marca, modelo, diagnostico, tipo, lineas, …Totales, caduca_el, caducado, decision: null|"aceptado"|"rechazado", respondido_en}` · 404 si el token no existe |
| `POST /api/publico/presupuestos/{token}/respuesta` | 🔓 | `{decision: "aceptar"|"rechazar"}` | 200 (igual que el GET) · 409 «Este presupuesto ya fue respondido» · 409 «Este presupuesto caducó el dd/mm/aaaa» |

Efectos (en transacción): ver la máquina de estados en [data-model.md](../data-model.md).

## spec-005 · Imputaciones

| Método y ruta | Acceso | Entrada | Salida |
|---|---|---|---|
| `POST /api/ordenes/{id}/imputaciones` | mecanico asignado | `{tipo:"mano_obra", concepto, cantidad}` (múltiplo de 0,25) o `{tipo:"pieza", pieza_id, cantidad}` (entera; el concepto lo pone el servidor). Mismas reglas que las líneas de presupuesto | 201 `ResumenImputaciones` (si se pasa del 10 %, `estado: "pendiente_conformidad"`) · 409 «La orden está {estado}; solo se puede imputar en órdenes en reparación» · 409 si no tiene presupuesto aceptado · 422 «Solo hay 2 unidades en stock de PF-123; el almacén puede pedirla al proveedor» («1 unidad» en singular) · 422 (cantidad o concepto no válidos, pieza inexistente) |
| `POST /api/ordenes/{id}/terminar` | mecanico asignado | — | 200 `ResumenImputaciones` (`en_reparacion → terminada`) · 409 si no está en reparación · 422 «Primero hay que imputar el trabajo realizado» |

`ResumenImputaciones` = `{orden_id, estado, coste_acumulado, total_presupuestado,
imputaciones:[Linea]}` (de la más antigua a la más reciente). Lo da
`imputaciones::resumen_imputaciones`, que spec-002 usa para llenar esos campos de `Orden`.

Las ampliaciones usan los endpoints de spec-003 (`POST /api/ordenes/{id}/presupuestos` con la
orden en `pendiente_conformidad`).

## spec-006 · Almacén y pedidos

| Método y ruta | Acceso | Entrada | Salida |
|---|---|---|---|
| `POST /api/piezas` | almacen | `{referencia, descripcion, precio_venta, stock?}` (`stock` opcional, 0 por defecto) | 201 `{id, referencia, descripcion, precio_venta, stock}` · 409 «Ya existe la referencia PF-123» · 422 (referencia o descripción vacías, precio < 0, stock < 0 o no entero) |
| `GET /api/proveedores` | almacen | — | `[{id, nombre}]` (precargados; no hay alta, edición ni baja) |
| `GET /api/pedidos?estado=pendiente` | almacen | — | `[Pedido]` = `{id, proveedor:{id,nombre}, pieza:{id,referencia,descripcion}, cantidad, orden:{id,numero}, estado, pedido_en, recibido_en}` (sin `estado`, todos) |
| `POST /api/pedidos` | almacen | `{proveedor_id, pieza_id, cantidad, orden_numero}` | 201 `Pedido` (si la orden estaba `en_reparacion`, pasa a `en_espera_piezas`) · 409 «La orden 50 está en diagnóstico; solo se puede pedir para órdenes en reparación o en espera de piezas» · 409 «Ya hay un pedido pendiente de PF-123 para la orden 42» · 422 (cantidad no entera o ≤ 0, proveedor o referencia inexistentes, «No existe la orden 99») |
| `POST /api/pedidos/{id}/recibir` | almacen | — | 200 `Pedido` (suma el stock; si es el último pendiente de la orden, esta vuelve a `en_reparacion`) · 409 «Este pedido ya está recibido» · 404 |

No hay ningún endpoint para cambiar a mano el estado de una orden: una orden en espera de piezas
solo sale de ahí con `recibir` (spec-006 FR-008a).

## spec-007 · Factura e historial

| Método y ruta | Acceso | Entrada | Salida |
|---|---|---|---|
| `POST /api/ordenes/{id}/factura` | recepcion | — | 201 `Factura` (la orden pasa a `facturada` y se envía el aviso) · 409 si no está terminada o ya tiene factura · 422 si no tiene imputaciones |
| `GET /api/facturas/{id}` | recepcion | — | `Factura` = `{id, numero, fecha, taller:{nombre,nif,direccion}, cliente_nombre, matricula, lineas, …Totales, aviso_enviado}` |
| `GET /api/vehiculos/{matricula}/historial` | todos | — | `{matricula, marca, modelo, reparaciones:[{orden_numero, fecha, km, diagnostico, factura_numero, lineas, total}]}` (de la más reciente a la más antigua) |

## spec-008 · Tablero

| Método y ruta | Acceso | Entrada | Salida |
|---|---|---|---|
| `GET /api/tablero` | jefe | — | `{columnas: [{estado, ordenes:[OrdenResumen]}] /* los 6 estados abiertos, en orden, aunque estén vacíos */, carga_mecanicos: [{id, nombre, ordenes_abiertas}] /* incluye los que tienen 0 */}` |

Se actualiza con `GET /api/eventos` (base común).

---

## Rutas de pantalla (frontend)

| Ruta | Acceso | Spec |
|---|---|---|
| `/cita` | 🔓 | 001 |
| `/presupuesto/[token]` | 🔓 | 004 |
| `/entrar` | 🔓 | 000 |
| `/citas` | recepcion | 001/002 |
| `/ordenes`, `/ordenes/[id]` | recepcion, mecanico | 002, 003, 005, 007 |
| `/almacen`, `/pedidos` | almacen | 006 |
| `/historial` | todos | 007 |
| `/tablero` | jefe | 008 |
