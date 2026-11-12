# T036 · Mensajes y confirmación de la aplicación, no del navegador (C1 de analyze-1)

- **Fecha**: 12/11/2026. Persona B, con Claude Code y un navegador automático (Playwright 1.48,
  Chromium en `es-ES`, zona horaria de Madrid).
- **Código**: rama `spec-003` sobre `fc16c4f`; «antes» con ese código y «después» con el cambio de
  T036, la interfaz recién compilada (`npm run build`) en los dos casos.
- **Datos**: base aparte `taller_c1` en el mismo Postgres, recién migrada y con
  `backend/seed/demo.sql` cargado antes de cada ejecución. Orden demo 4 (`luis`, en diagnóstico y sin
  presupuesto) y, a 390 px, la orden demo 1.
- **Cómo repetirlo**: `scripts/ejecutar.sh t-c1-003.mjs antes|despues`, con `playwright` instalado
  junto al script.

## Qué se comprueba

| Paso | Esperado |
|---|---|
| (a) Pieza PF-123 con 2,5 unidades | El mensaje de la aplicación «Las unidades de pieza deben ser un número entero» y ningún mensaje del navegador |
| (b) Mano de obra sin concepto | «Escribe el concepto de la mano de obra» y ningún mensaje del navegador |
| (c) «Emitir presupuesto» | La pregunta «Una vez emitido no se podrá modificar. ¿Emitir el presupuesto?» en la página, con «Sí, emitir» y «Volver», sin diálogo del navegador; «Volver» no emite y «Sí, emitir» emite («Válido hasta») |
| (c) A 390 px | La confirmación cabe sin desplazamiento horizontal |

## Resultado

| | Correctas | Fallos | Salida |
|---|---|---|---|
| Antes | 0 | 3 | [salida-antes.txt](salida-antes.txt) |
| Después | 4 | 0 | [salida-despues.txt](salida-despues.txt) |

- **Antes**: el navegador bloqueaba el envío con sus propios mensajes («Introduce un valor válido. Los
  dos valores válidos más aproximados son 2 y 3.», «Completa este campo») y «Emitir» abría un
  `confirm()`. Con Chromium en español salen en español, pero los pone el navegador: en la regresión
  T030 de spec-008, con otro Chromium, salieron en inglés.
- **Después**: los mensajes son los del servidor y la confirmación va en la página, como en spec-004.
  Capturas: [unidades](despues-a-unidades.png), [concepto](despues-b-concepto.png),
  [confirmación](despues-c1-confirmacion.png), [emitido](despues-c2-emitido.png) y
  [confirmación a 390 px](despues-movil-confirmacion.png).
