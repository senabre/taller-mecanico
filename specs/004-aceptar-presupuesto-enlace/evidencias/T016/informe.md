# Comprobación de la ficha de la orden · spec-004 (T016)

**Fecha**: 2026-11-05 · **Responsable**: Persona A · **No sustituye a T027**, el recorrido completo del
quickstart.

## Cómo se hizo

Chrome sin interfaz, dirigido con Playwright, a 1280 px y a 390 px, con el servidor real y una base de
datos temporal (`taller_verif`), con las migraciones y `backend/seed/demo.sql`. Entra como `paco`, el
jefe del taller. Las respuestas del cliente van por `POST` al servidor real, y los casos que el seed no
trae (un segundo presupuesto y un borrador) se crean en la base temporal.

La comprobación se escribió **antes** que el código:

- **Rojo**, [salida-rojo.txt](salida-rojo.txt): 20 correctas y 18 fallos, sin T016. Las de «sin enlace»
  salen correctas porque aún no hay enlace: solo dicen algo en verde.
- **Verde**, [salida-verde.txt](salida-verde.txt): 44 correctas y 0 fallos, con el enlace ya presente en
  el caso vigente. Repetida tres veces, con el mismo resultado.

## Resultados

Cada caso se comprobó en los dos anchos. Las capturas tienen delante `escritorio-` o `movil-`.

| Caso | Resultado | Captura |
|---|---|---|
| Emitido y vigente | Enlace en un campo de solo lectura y «Copiar enlace» | `ficha-emitido.png` |
| Copiar | El botón pasa a «Copiado» y vuelve; el portapapeles tiene el enlace | · |
| Aceptado | «Inicial · Aceptado el dd/mm/aaaa a las HH:mm», en hora de Madrid; sin enlace | `ficha-aceptado.png` |
| Rechazado | «Inicial · Rechazado el …»; sin enlace | · |
| Rechazado y otro emitido | La respuesta del primero y el enlace del nuevo | · |
| Dos respuestas | Se ven las dos; sin enlace | `ficha-dos-respuestas.png` |
| Ampliación caducada | «Caducado el …», la respuesta del inicial y sin enlace | `ficha-caducado.png` |
| Ampliación rechazada | «Ampliación · Rechazado el …»; sin enlace | · |
| Borrador | Sin enlace | · |

Sin desplazamiento horizontal en el caso vigente y en el de dos respuestas, y sin errores de
JavaScript. La captura de móvil del caso vigente se revisó a ojo: el campo recorta el enlace y
«Copiar enlace» baja a la línea siguiente, sin desbordar.

## Un fallo de la comprobación, no de la aplicación

Una ejecución dio un fallo en móvil, «el botón pasa a “Copiado”». La comprobación miraba el botón justo
después de pulsarlo, sin esperar a que terminara la copia, que es asíncrona. Ese mismo fallo traía el
portapapeles correcto. Se cambió para que espere hasta 1,5 s, y por eso se repitió tres veces.

## Sin comprobar

- **Sin HTTPS**: `navigator.clipboard` no existe fuera de HTTPS y de `localhost`. En ese caso el botón no
  hace nada y el enlace se copia a mano del campo (research D6). Este caso no se probó.
- **Respuesta con la ficha abierta**: la ficha no se actualiza sola, así que sigue enseñando el enlace
  hasta recargar la página.
