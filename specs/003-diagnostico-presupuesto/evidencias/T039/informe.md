# T038 y T039 · La sección «Diagnóstico» (converge-1, F1 y F2)

- **Fecha**: 12/11/2026. Persona B, con Claude Code y un navegador automático (Playwright 1.48,
  Chromium en `es-ES`, zona horaria de Madrid).
- **Código**: rama `spec-003`, con T036 ya hecho; «antes» sin T038 y T039 y «después» con ellos, la
  interfaz recién compilada en los dos casos.
- **Datos**: base aparte `taller_c1`, recién migrada y con `backend/seed/demo.sql` antes de cada
  ejecución. Órdenes demo 1 (recibida, `luis`), 4 (en diagnóstico sin presupuesto, `luis`) y 5
  (presupuesto emitido, de `marta`; se abre como `paco`, que puede editar todas).
- **Cómo repetirlo**: `../T036/scripts/ejecutar.sh t-diag-003.mjs antes|despues`.

## Qué se comprueba

| Paso | Esperado |
|---|---|
| (a) T038 · Guardar el diagnóstico vacío (orden 1) | El mensaje de la aplicación «Escribe el diagnóstico», sin mensaje del navegador |
| (b) T039 · Orden con un presupuesto emitido (orden 5) | El diagnóstico guardado y el aviso «Ya se ha emitido un presupuesto para esta orden; el diagnóstico no se puede cambiar.», sin cuadro ni «Guardar diagnóstico»; a 390 px, sin desplazamiento horizontal |
| (c) Sin presupuesto emitido (orden 4) | Se sigue pudiendo cambiar y se guarda sin error |

## Resultado

| | Correctas | Fallos | Salida |
|---|---|---|---|
| Antes | 1 | 2 | [salida-diag-antes.txt](salida-diag-antes.txt) |
| Después | 4 | 0 | [salida-diag-despues.txt](salida-diag-despues.txt) |

- **Antes**: (a) el navegador bloqueaba el envío con «Completa este campo»; (b) la orden 5 ofrecía el
  cuadro y «Guardar diagnóstico», aunque guardar daba 409 y el cuadro se quedaba con el texto
  rechazado (V2 de spec-004). (c) ya pasaba.
- **Después**: capturas [vacío](despues-diag-a-vacio.png),
  [presupuesto emitido](despues-diag-b-emitido.png) y [a 390 px](despues-diag-b-movil.png).
