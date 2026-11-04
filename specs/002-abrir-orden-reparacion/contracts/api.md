# Contrato de API: spec-002 · Recibir el vehículo y abrir la orden de reparación

El contrato es **único** y está en
[000-base-comun/contracts/api.md § spec-002 · Orden de reparación](../../000-base-comun/contracts/api.md#spec-002--orden-de-reparación),
con su tabla de validación. Las convenciones (errores, fechas, cookie de sesión) son las del
mismo documento. Aquí solo se explica qué pantalla usa cada endpoint y qué requisito cumple cada
respuesta. Tras el clarify, se han hecho estos cambios en el contrato común (los revisa la
Persona B):

- `POST /api/ordenes`: tabla de validación con el orden de las comprobaciones y los textos de
  km, daños, mecánico, fecha de entrega y km anterior (todos los 422 antes que cualquier 409);
  404 «No existe la cita»; 409 «Solo se puede abrir la orden el día de la cita» (Q2); 409 «Esta
  cita ya tiene una orden (Nº 42)» con `detalle: {orden_id, numero}` (Q5); el número no se consume
  si algo falla.
- `GET /api/ordenes/{id}`: qué parte del objeto `Orden` da cada spec y 404 «No existe la orden».
- `GET /api/citas` (sección spec-001): añade `orden_numero` (Q5).

## Uso desde las pantallas

### `/citas` (recepción y jefe; pantalla de spec-001)

| Momento | Llamada | Qué muestra | Requisito |
|---|---|---|---|
| Al elegir un día | `GET /api/citas?fecha=…` | En la columna «Orden»: «Orden Nº N» como enlace a `/ordenes/{orden_id}` si la cita tiene orden; «Abrir orden» si no la tiene **y** el día es hoy; nada si es otro día | FR-001, FR-008 (Q2, Q5) |

### `/ordenes/nueva?cita=…` (recepción y jefe)

| Momento | Llamada | Qué muestra | Requisito |
|---|---|---|---|
| Al abrir | `GET /api/citas` sin `fecha` (el servidor da las de hoy en Europe/Madrid; solo se abren citas de hoy) y `GET /api/mecanicos` | Matrícula, marca, modelo y motivo de la cita; en el selector, solo mecánicos activos. Si la cita no está en la lista de hoy: «Solo se puede abrir la orden el día de la cita», sin formulario | FR-001, FR-005 (Q2, Q3) |
| Al guardar | `POST /api/ordenes` | **201**: va a la ficha de la orden · **422**: el mensaje bajo el campo de `detalle.campo` (`km`, `danos`, `mecanico_id`, `fecha_entrega_prometida`) · **409** cita con orden: el mensaje y un enlace a la orden de `detalle.orden_id` · **409** otro día: el mensaje como error general | FR-002–FR-008 |

El formulario usa `novalidate`: todos los mensajes de error de los campos salen del servidor. Se
consideran **etiquetas**, y no se enumeran aquí, los nombres de los campos (Kilómetros, Daños
visibles, Mecánico, Fecha de entrega prometida), los textos de los botones («Guardar»,
«Guardando…»), la opción vacía del selector («Elige un mecánico») y el rótulo «Motivo:». Aparte de
las etiquetas, los únicos textos propios del formulario son:

- «Abrir orden» (título de la página);
- «Solo se puede abrir la orden el día de la cita» (la cita no está en la lista de hoy; es el mismo
  texto del 409);
- «Ver la orden» (enlace junto al 409 de la cita que ya tiene orden);
- «No se pudo cargar la cita» y «Ha ocurrido un error inesperado» (solo si falla la conexión o el
  servidor responde sin el mensaje de error del contrato).

### `/ordenes/[id]` (todos los roles)

| Momento | Llamada | Qué muestra | Requisito |
|---|---|---|---|
| Al abrir y tras cada cambio de una sección | `GET /api/ordenes/{id}` | Sección «Datos» (spec-002): número, estado, matrícula, marca, modelo, fecha y motivo de la cita, km, daños, mecánico, entrega prometida y «Retrasada» si lo está. Debajo, las secciones de spec-003, 005 y 006 con los campos que les pasa la ficha ([research](../research.md) D6) · **404**: «No existe la orden» | US1-3, SC-004 |

Si falla la conexión, la ficha muestra «No se pudo cargar la orden».

### `/ordenes` (recepción y jefe: «Órdenes»; mecánicos: «Mis órdenes»)

| Momento | Llamada | Qué muestra | Requisito |
|---|---|---|---|
| Al abrir | `GET /api/ordenes` (recepción, jefe) o `GET /api/ordenes?mias=true` (mecánico) | Número (enlace a la ficha), matrícula, estado, mecánico y entrega prometida, de la más reciente a la más antigua | Menú común (escenario 1 de la US1 de la base común y quickstart §0.4) y quickstart §3 (hallazgo E1) |

Con la lista vacía, «No hay órdenes»; si falla la conexión, «No se pudo cargar la lista de órdenes».

`GET /api/ordenes` y el evento `{id, estado: "recibida"}` de `GET /api/eventos` los usa también el
tablero de spec-008.
