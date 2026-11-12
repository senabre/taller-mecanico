# Comprobación de la página pública · spec-004 (T015, T022 y T026)

**Fecha**: 2026-11-05 · **Responsable**: Persona A · **No sustituye a T027**, el recorrido completo del
quickstart.

## Cómo se hizo

Chrome sin interfaz, dirigido con Playwright, a 1280 px y a 390 px, con la aplicación compilada y el
servidor real, sobre una base de datos temporal (`taller_verif`) con las migraciones y
`backend/seed/demo.sql`. La salida completa está en [salida.txt](salida.txt): **106 comprobaciones
correctas y 0 fallos**, 53 por cada ancho.

Esta comprobación sustituye a la parcial del 04/11, cuyas partes simuladas no contaban. Ya no hay
ninguna respuesta del servidor inventada. Solo se fuerzan tres cosas, que no se pueden provocar de otra
forma:

- **El paso del tiempo**: se cambia `caduca_el` en la base temporal, para que un presupuesto caduque con
  la página abierta.
- **La vuelta a empezar**: entre un caso y otro, el presupuesto vuelve a `emitido` y la orden a su
  estado, porque cada presupuesto solo se responde una vez.
- **La caída de la conexión**: el navegador se deja sin red al responder, y se corta la petición al
  cargar.

Además de leer la pantalla, cada respuesta se contrasta con la base de datos, también la hora, que se
compara con `respondido_en` en hora de Madrid. La primera línea del presupuesto lleva un concepto largo,
para ver la tabla a 390 px.

## Resultados

Cada caso se comprobó en los dos anchos. Las capturas tienen delante `escritorio-` o `movil-`.

| Caso | Resultado | Captura |
|---|---|---|
| Vigente | Taller, vehículo, total en €, «Válido hasta dd/mm/aaaa», y solo «Aceptar» y «Rechazar» | `vigente.png` |
| Confirmar la aceptación | «¿Acepta este presupuesto por 133,10 €? Una vez confirmado, no se podrá cambiar.», con «Sí, aceptar» y «Volver» | `confirmar-aceptar.png` |
| Confirmar el rechazo | «¿Rechaza este presupuesto por 133,10 €? …», con «Sí, rechazar» y «Volver» | `confirmar-rechazar.png` |
| «Volver» | La página queda como estaba, y en la base sigue `emitido` | · |
| Aceptar | «Aceptado el … a las …», sin botones; en la base, `aceptado`, y la orden pasa a `en_reparacion` | `aceptado.png` |
| Rechazar | «Rechazado el … a las …», sin botones; en la base, `rechazado`, y la orden no cambia. Sigue igual al volver a abrir el enlace | `rechazado.png` |
| 409 «ya fue respondido» | Con dos pestañas abiertas de verdad: la segunda enseña el mensaje y la decisión que ganó, sin botones, y la base no cambia | `409-ya-respondido.png` |
| 409 de caducidad | Caduca con la página abierta: el aviso con la fecha se ve **una sola vez**, sin botones, y no se guarda nada | `409-caducado.png` |
| Caducado al abrir | «Este presupuesto caducó el dd/mm/aaaa. Contacte con el taller», sin botones; es la ampliación del seed y lo dice | `caducado.png` |
| Ampliación vigente | Aceptarla devuelve la orden de `pendiente_conformidad` a `en_reparacion` | · |
| Sin red al responder | «Ha ocurrido un error inesperado», y no se guarda nada | · |
| Sin conexión al cargar | «No se pudo cargar el presupuesto» | · |
| Enlace inválido | Solo «Este enlace no es válido» | `enlace-invalido.png` |

En los casos en los que se abre la página y en los dos anchos: sin desplazamiento horizontal, sin el
nombre, el teléfono ni el correo del cliente, y sin el menú del personal. En todo el recorrido, sin
`confirm()` y sin errores de JavaScript.

Las capturas de móvil del estado vigente, la confirmación y los dos 409 se revisaron a ojo: se leen
bien, y el concepto largo se parte en varias líneas sin desbordar la tabla.

## Lo que queda

T027 (el recorrido completo del quickstart en el navegador) y T028 (SC-001, cronometrado por una
persona). Esta comprobación la dirigió un programa: no mide cuánto tarda una persona en responder.
