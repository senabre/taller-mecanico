# T033 · Mensajes de /almacen y /pedidos de la aplicación, no del navegador (C1 de analyze-1)

- **Fecha**: 12/11/2026. Persona B, con Claude Code y un navegador automático (Playwright 1.48,
  Chromium en `es-ES`, zona horaria de Madrid).
- **Código**: rama `spec-006` sobre `7bf2648`; «antes» con ese código y «después» con T033, la
  interfaz recién compilada en los dos casos.
- **Datos**: base aparte `taller_c1`, recién migrada y con `backend/seed/demo.sql` antes de cada
  ejecución. Como `pepe` (almacén); la orden demo 3 está en reparación.
- **Cómo repetirlo**: con los scripts de [spec-003 T036](../../../003-diagnostico-presupuesto/evidencias/T036/scripts/):
  `ejecutar.sh t-006.mjs antes|despues`.

## Qué se comprueba

| Paso | Esperado |
|---|---|
| (a1) `/almacen`, referencia vacía | «Indica la referencia de la pieza», sin mensaje del navegador |
| (a2) `/almacen`, stock -1 | «El stock debe ser un número entero de 0 o más», sin mensaje del navegador |
| (a3) `/almacen`, FA-9 sin stock | Aparece con stock 0 (camino normal) |
| (b1) `/pedidos`, cantidad 0 | «La cantidad debe ser un número entero mayor que 0», sin mensaje del navegador |
| (b2) `/pedidos`, sin nº de orden | «Indica el número de la orden», sin mensaje del navegador |
| (b3) `/pedidos`, FA-9 × 1 para la orden 3 | «Pedido registrado: 1 × FA-9 para la orden 3» (camino normal) |
| (c) `/pedidos` a 390 px | Sin desplazamiento horizontal |

## Resultado

| | Correctas | Fallos | Salida |
|---|---|---|---|
| Antes | 3 | 4 | [salida-006-antes.txt](salida-006-antes.txt) |
| Después | 7 | 0 | [salida-006-despues.txt](salida-006-despues.txt) |

- **Antes**: los cuatro casos de error los paraba el navegador con sus mensajes («Completa este
  campo», «El valor debe ser superior o igual a 0», «El valor debe ser superior o igual a 1») y no
  llegaban al servidor. Los caminos normales ya funcionaban.
- **Después**: capturas [referencia vacía](despues-006-a1-referencia.png),
  [stock -1](despues-006-a2-stock.png), [cantidad 0](despues-006-b1-cantidad.png),
  [sin orden](despues-006-b2-orden.png), [registrado](despues-006-b3-registrado.png) y
  [a 390 px](despues-006-c-movil.png).
- **Nota sobre la compilación**: algunas compilaciones hechas justo después de `npm run check` o de
  borrar las cachés dejaron la interfaz sin arrancar («Cannot read properties of undefined (reading
  'data')» en la página de entrada), con este cambio y con el de spec-005 (T035). Al volver a compilar
  el mismo código, sin tocarlo, desapareció: cada cambio por separado y los dos juntos arrancaron
  bien. Antes de cada ejecución válida se comprobó que la página de entrada arranca.
