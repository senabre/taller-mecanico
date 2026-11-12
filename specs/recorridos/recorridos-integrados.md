# Recorridos integrados (enunciado, apartado 5.4)

El enunciado pide al menos dos recorridos que conecten historias de las dos personas: uno satisfactorio y otro con una condición de error, permiso o estado. En cada uno constan las precondiciones, los datos de prueba, los pasos, el resultado esperado y el observado, y la versión del código.

| Recorrido | Tipo | Historias (persona) | Resultado | Evidencias |
|---|---|---|---|---|
| R1 · `3579KLM` de la cita a la factura | Satisfactorio | 001, 002, 004, 008 (A) · 003, 005, 007 (B) | 10/10 | Quickstart común §9; [T039 de spec-008](../008-tablero-ordenes-abiertas/evidencias/T039/informe.md) |
| R2 · Presupuesto rechazado: la orden queda bloqueada | Error, permiso y estado | 002, 004, 008 (A) · 003, 005, 007 (B) | 9/9 | Este documento y la carpeta [`R2/`](R2/) |

---

## R1 · `3579KLM` de la cita a la factura (satisfactorio)

Es el §9 del quickstart común, con su informe en `specs/008-tablero-ordenes-abiertas/evidencias/T039/informe.md`. Lo ejecutó la Persona A con el navegador automático, haciendo todos los papeles; el día B (18/11/2026) se hizo con el reloj adelantado y lo revisaron las dos personas.

---

## R2 · Presupuesto rechazado: la orden queda bloqueada (error, permiso y estado)

**Qué comprueba.** Cuando el cliente rechaza el presupuesto, la orden no avanza: nadie puede imputar, terminar ni facturar, el enlace no admite una segunda respuesta y el tablero sigue mostrando la orden «En diagnóstico». Además, solo el jefe ve el tablero. Al final, un presupuesto nuevo aceptado desbloquea la orden.

- **Fecha**: 12/11/2026. Ejecutado por la Persona A con el navegador automático (Playwright, Chromium en `es-ES`, zona horaria de Madrid). El cliente usa un móvil de 390 px.
- **Versión del código**: rama `entrega`, igual que `spec-008` en `02e9868`.
- **Precondiciones**: base recién creada con las migraciones y `backend/seed/demo.sql`; la app en http://localhost:3000; el correo recogido por un SMTP local.
- **Datos de prueba**: la cita de hoy de `1414NNN` (datos demo); 23.500 km; mecánico Luis; entrega dentro de 3 días; diagnóstico «Aceite degradado»; presupuesto de 1 h «Cambio de aceite» (40,00 € + IVA = 48,40 €). Usuarios `ana`, `luis`, `pepe` y `paco`, contraseña `demo1234`.
- **Tablero**: el de `paco` se abre al principio y no se toca en todo el recorrido.

| Paso | Qué se hace (spec · persona) | Esperado | Observado | | Captura |
|---|---|---|---|---|---|
| R2.1 | ana abre la orden de 1414NNN (002 · A) | Orden Nº 10; su tarjeta sale en «Recibida» | Orden Nº 10; tablero en 109 ms | ✅ | [r01-orden-abierta.png](R2/r01-orden-abierta.png) |
| R2.2 | luis diagnostica y emite un presupuesto de 48,40 € (003 · B) | La tarjeta pasa a «En diagnóstico»; emitir no la mueve; hay enlace para el cliente | tablero en 103 ms; columna tras emitir=En diagnóstico; enlace=true; 48,40 €=true | ✅ | [r02-presupuesto-emitido.png](R2/r02-presupuesto-emitido.png) |
| R2.3 | El cliente rechaza desde el móvil (004 · A) | Confirmación dentro de la página; «Rechazado el 12/11/2026 a las 17:25»; la orden sigue «En diagnóstico» y la tarjeta no se mueve | confirmación=true; página=true; orden=en_diagnostico; tablero=En diagnóstico | ✅ | [r04-rechazado-movil.png](R2/r04-rechazado-movil.png) |
| R2.4 | luis no puede imputar ni terminar (005 · B) | «Inicial · Rechazado el …»; sin botón «Imputar» y con el aviso; por la API, 409 y nada se guarda | ficha=true; aviso=true; botón=0; imputar API=409 «La orden está en diagnóstico; solo se puede imputar en órdenes en reparación»; terminar API=409 «La orden está en diagnóstico; solo se puede terminar una orden en reparación»; imputaciones en la base=0 | ✅ | [r06-sin-imputar.png](R2/r06-sin-imputar.png) |
| R2.5 | ana no puede facturar (007 · B) | Sin botón «Facturar»; por la API, rechazo y ninguna factura | botón=0; API=409 «La orden está en diagnóstico; solo se puede facturar una orden terminada»; facturas=0 | ✅ | [r07-sin-facturar.png](R2/r07-sin-facturar.png) |
| R2.6 | El cliente vuelve a abrir el enlace e intenta aceptar (004 · A) | Se ve su rechazo, sin botones; aceptar por la API da 409 «Este presupuesto ya fue respondido» y la orden no cambia | botones=0; API=409 «Este presupuesto ya fue respondido»; presupuesto=rechazado; orden=en_diagnostico | ✅ | [r08-enlace-ya-respondido.png](R2/r08-enlace-ya-respondido.png) |
| R2.7 | pepe intenta ver el tablero (008 · A) | Sin «Tablero» en el menú; «No tienes permiso para hacer esto», sin tarjetas; la API da 403 | menú=0; mensaje=true; tarjetas=0; API=403 | ✅ | [r09-pepe-sin-tablero.png](R2/r09-pepe-sin-tablero.png) |
| R2.7 | luis intenta ver el tablero (008 · A) | Sin «Tablero» en el menú; «No tienes permiso para hacer esto», sin tarjetas; la API da 403 | menú=0; mensaje=true; tarjetas=0; API=403 | ✅ | [r10-luis-sin-tablero.png](R2/r10-luis-sin-tablero.png) |
| R2.8 | Presupuesto nuevo aceptado: la orden sale del bloqueo | Distinto enlace; al aceptar, la tarjeta pasa a «En reparación»; la ficha conserva las dos respuestas | enlace nuevo=true; tablero en 106 ms; respuestas en la ficha=[Inicial · Rechazado el 12/11/2026 a las 17:25 / Inicial · Aceptado el 12/11/2026 a las 17:25]; imputar ya visible=1 | ✅ | [r11-dos-respuestas.png](R2/r11-dos-respuestas.png) |

### Observaciones

- **R2.4 y R2.5**: los rechazos no dependen solo de la pantalla: las peticiones directas a la API también se rechazan (409, con un mensaje claro) y no se guarda nada.
- **R2.6, primer intento**: el script envió `"decision": "aceptado"` en lugar de `"aceptar"`, y el servidor respondió 422 «Elija aceptar o rechazar» (`primer-intento.log`). Es un error del script, no de la app; además, confirma que la validación (422) va antes que el estado (409), como se decidió en spec-004. Se corrigió el script y se repitió el recorrido entero desde una base limpia: 9/9 (`salida.log`).
- **R2.8**: la ficha conserva las dos respuestas, «Inicial · Rechazado el …» e «Inicial · Aceptado el …» (Q1 de spec-004).

### Revisión

El 12/11/2026 la Persona A y la Persona B revisaron juntas el recorrido R2 y sus capturas. La
Persona B comprobó los pasos de sus specs: R2.2 (diagnóstico y presupuesto, spec-003), R2.4
(imputar y terminar, spec-005) y R2.5 (facturar, spec-007). Sin observaciones.

**Estado**: revisado.
