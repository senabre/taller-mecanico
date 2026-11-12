# T033 · «Facturar» con confirmación en la página e historial sin validación del navegador (C1)

- **Fecha**: 12/11/2026. Persona B, con Claude Code y un navegador automático (Playwright 1.48,
  Chromium en `es-ES`, zona horaria de Madrid).
- **Código**: rama `spec-007` sobre `076f626`; «antes» con ese código y «después» con T033, la
  interfaz recién compilada en los dos casos (comprobando antes que la página de entrada arranca).
- **Datos**: base aparte `taller_c1`, recién migrada y con `backend/seed/demo.sql` antes de cada
  ejecución. Como `ana` (recepción): la orden demo 8 está terminada y la 9, facturada (`9999LLL`).
- **Cómo repetirlo**: con los scripts de [spec-003 T036](../../../003-diagnostico-presupuesto/evidencias/T036/scripts/):
  `ejecutar.sh t-007.mjs antes|despues`.

## Qué se comprueba

| Paso | Esperado |
|---|---|
| (a) «Facturar» en la orden 8 | La pregunta en la página con «Sí, facturar» y «Volver», sin diálogo del navegador; «Volver» no factura y «Sí, facturar» emite «Factura nº 2» |
| (a) A 390 px | La confirmación cabe sin desplazamiento horizontal |
| (b) `/historial` sin matrícula | «Escribe la matrícula», sin mensaje del navegador |
| (c) `/historial` con «9999 lll» | La reparación facturada de la orden 9 (camino normal) |

## Resultado

| | Correctas | Fallos | Salida |
|---|---|---|---|
| Antes | 1 | 3 | [salida-007-antes.txt](salida-007-antes.txt) |
| Después | 4 | 0 | [salida-007-despues.txt](salida-007-despues.txt) |

- **Antes**: «Facturar» abría un `confirm()` del navegador, y buscar sin matrícula sacaba «Completa
  este campo» del navegador. El historial con una matrícula ya funcionaba.
- **Después**: capturas [confirmación](despues-007-a1-confirmacion.png),
  [a 390 px](despues-007-a2-movil.png), [facturada](despues-007-a3-facturada.png),
  [sin matrícula](despues-007-b-vacia.png) e [historial](despues-007-c-historial.png).
