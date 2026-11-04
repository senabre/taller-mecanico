# Contrato de API: spec-004 · Aceptar o rechazar el presupuesto desde un enlace

El contrato es **único** y está en
[000-base-comun/contracts/api.md § spec-004 · Respuesta del cliente](../../000-base-comun/contracts/api.md#spec-004--respuesta-del-cliente).
Las convenciones (errores, fechas, importes) son las del mismo documento. Aquí solo se explica qué
pantalla usa cada endpoint y qué requisito cumple cada respuesta. Tras el clarify y el plan, se han
hecho estos cambios en el contrato común (los revisa la Persona B):

- `GET` y `POST /api/publico/presupuestos/{token}…`: 404 «Este enlace no es válido» (research D4).
- `POST /api/publico/presupuestos/{token}/respuesta`: 409 de caducidad con el texto completo y para
  las dos decisiones (Q3); 422 «Elige aceptar o rechazar».
- `PresupuestoResumen`: añade `respondido_en` (Q1).

## Uso desde las pantallas

### `/presupuesto/[token]` (pública)

| Momento | Llamada | Qué muestra | Requisito |
|---|---|---|---|
| Al abrir | `GET /api/publico/presupuestos/{token}` | **200**: taller, matrícula, marca, modelo, diagnóstico, líneas, base, IVA, total y caducidad. Según el caso: vigente, «Aceptar» y «Rechazar»; con `decision`, «Aceptado el 16/10/2026 a las 10:32» o «Rechazado el …», sin botones; con `caducado`, «Este presupuesto caducó el dd/mm/aaaa. Contacte con el taller», sin botones · **404**: solo «Este enlace no es válido» | FR-002, FR-005, US1-1, US1-3, US3-1 |
| Al pulsar «Aceptar» o «Rechazar» | ninguna | La confirmación de [research D5](../research.md), con el total en € | FR-003 |
| Al confirmar | `POST /api/publico/presupuestos/{token}/respuesta` con `{decision}` | **200**: la decisión con su fecha y hora, sin botones · **409** ya respondido o caducado: el mensaje, y vuelve a pedir el presupuesto, que queda sin botones | FR-003, FR-004, FR-005, US1-4, US3-2 |

Además de las etiquetas de los datos y los botones («Aceptar», «Rechazar», «Sí, aceptar», «Sí,
rechazar», «Volver»), los únicos textos propios de la página son las dos preguntas de confirmación,
«Aceptado el …», «Rechazado el …» y «No se pudo cargar el presupuesto» o «Ha ocurrido un error
inesperado» (solo si falla la conexión). Los demás mensajes salen del servidor. La página nunca
muestra el nombre, el teléfono ni el correo del cliente (principio V).

### `/ordenes/[id]`, sección Presupuesto (spec-003; personal con sesión)

| Momento | Llamada | Qué muestra | Requisito |
|---|---|---|---|
| Al abrir | `GET /api/ordenes/{id}` (`presupuestos`, con `respondido_en`) | Una línea por cada respuesta de la orden: «Inicial · Aceptado el … a las …», «Ampliación · Rechazado el …» | FR-004, SC-002, US1-2, US2-3 |
| Al abrir | `GET /api/presupuestos/{id}` (el más reciente; ya lo pide la sección) | Con el presupuesto emitido y vigente: `enlace` en un campo de solo lectura y «Copiar enlace», que pasa a «Copiado» un momento. Con otro estado, nada: se oculta solo en la pantalla | FR-008, US1-5 |

`GET /api/presupuestos/{id}` sigue enviando `enlace` después de la respuesta o de la caducidad: ya no
permite cambiar nada (Q5).
