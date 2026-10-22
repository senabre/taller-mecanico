# Contrato de API: spec-001 · Pedir cita en línea

El contrato es **único** y está en
[000-base-comun/contracts/api.md § spec-001 · Citas](../../000-base-comun/contracts/api.md#spec-001--citas),
con su tabla de validación. Las convenciones (errores, fechas, cookie de sesión) son las del
mismo documento. Aquí solo se explica qué pantalla usa cada endpoint y qué requisito cumple cada
respuesta. Tras el clarify, se han hecho estos cambios en el contrato común (los revisa la
Persona B):

- `GET /api/publico/vehiculos/{matricula}`: añade `cita_pendiente` y el 422 por matrícula
  inválida (Q1, Q3).
- `POST /api/publico/citas`: añade el 409 por cita pendiente, la tabla de validación de los 422
  con `detalle.campo`, y la regla de que con una matrícula conocida se ignoran `vehiculo` y
  `cliente` (Q2, Q3, Q5).
- `GET /api/citas`: añade `cliente: {nombre, telefono}` (sin correo), el orden, la `fecha`
  opcional y los 401/403 (Q4).

## Uso desde las pantallas

### `/cita` (pública)

| Momento | Llamada | Qué muestra el formulario | Requisito |
|---|---|---|---|
| Al abrir | `GET /api/publico/dias-disponibles` | los días en el selector; si la lista está vacía, que no hay fechas libres y que llame al taller | FR-006, FR-007 |
| Al salir del campo matrícula | `GET /api/publico/vehiculos/{matricula}` | **200** sin cita pendiente: marca y modelo, y solo pide día y motivo · **200** con `cita_pendiente: true`: marca, modelo y «Este vehículo ya tiene una cita pendiente.», sin poder confirmar · **404**: pide marca, modelo, nombre, teléfono y correo · **422**: «La matrícula no es válida», sin poder confirmar | FR-002, FR-003, FR-004, FR-010 |
| Al confirmar | `POST /api/publico/citas` (`vehiculo` y `cliente` solo si fue 404) | **201**: fecha en dd/mm/aaaa y motivo · **409** cita pendiente: el aviso de FR-010 · **409** día completo: vuelve a pedir los días y muestra «Ese día se acaba de completar; elige otro» · **422**: el mensaje bajo el campo de `detalle.campo` | FR-004a, FR-007, FR-008, FR-010 |

El formulario usa `novalidate` (research D2): todos los mensajes salen del servidor.

### `/citas` (recepción y jefe)

| Momento | Llamada | Qué muestra | Requisito |
|---|---|---|---|
| Al abrir | `GET /api/citas` (sin `fecha`) | las citas de hoy | FR-009 |
| Al elegir otro día | `GET /api/citas?fecha=…` | matrícula, marca, modelo, motivo, nombre y teléfono, en el orden en que se pidieron; con lista vacía, «No hay citas para este día» | FR-009 |
| Otro rol | la llamada da **403** | el mensaje común «No tienes permiso para hacer esto» y ninguna lista | FR-009 |

`orden_id` no se muestra como dato. Lo usa spec-002 para el botón «Abrir orden».
