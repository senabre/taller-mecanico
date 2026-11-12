# T027 · Verificación del quickstart común §4 en el navegador

- **Fecha**: 05/11/2026, ejecutada por la Persona A con Playwright (Chromium), en español y con la zona horaria de Madrid.
- **Código**: rama `spec-004` en `8d64285`. Base reiniciada con las migraciones y `backend/seed/demo.sql`; correo recogido por un servidor SMTP local (equivale a la bandeja de Mailpit: se comprueba que el correo al cliente lleva el enlace).
- **Anchos**: escritorio 1280 px y móvil 390 px (la página pública, la confirmación, «Copiar enlace», el caducado y el enlace inventado se comprueban en los dos).
- **Resultado**: **20/20 comprobaciones superadas**.

## Preparación

Los pasos previos del quickstart (§2.4 y §2.5: órdenes Nº 10 y Nº 11; §3: diagnóstico y presupuesto de 133,10 € de 1313MMM) y los presupuestos que §4.5 y §4.7 piden emitir se crean por la API con las sesiones de `ana` y `luis`. Esas pantallas son de spec-002, 003 y 005 y tienen su propia verificación. Todo lo que hace el **cliente** y lo que ve el **personal** en §4 se recorre en pantalla.

- **I5(b)**: el §4.5 emite un presupuesto en la orden de 1414NNN sin diagnóstico previo. Antes se escribe el diagnóstico «Aceite degradado» como `luis` (lo indica T027; el hallazgo es de spec-003).
- El **cronometraje de SC-001** no se mide aquí: un script no sirve como medida de una persona. Es T028.

## Resultados

| # | Paso | Esperado | Observado | | Captura |
|---|---|---|---|---|---|
| 1 | Preparación §2.4, §2.5 y §3 (API) | Órdenes Nº 10 y 11 y presupuesto de 1313MMM emitido por 133,10 € | orden 10 (201) y 11 (201); presupuesto 9 emitido, total 133.10, aviso_enviado=true | ✅ | — |
| 2 | §4.1 Correo con el enlace (Mailpit) | Llega al cliente un correo con el enlace del presupuesto | 1793900374.089498.eml para lucia.fernandez@ejemplo.test; enlace http://localhost:3000/presupuesto/UXn96WpdhjyHiVnBM2MqMHyr9iVhcglET9zyFUKFGdA | ✅ | — |
| 3 | §4.1 Página pública (escritorio) | Vehículo, diagnóstico, líneas, base, IVA, total y caducidad; sin datos del cliente; sin desplazamiento lateral | faltan=[]; datos del cliente=[]; sin scroll lateral=true | ✅ | [01-p1-enlace-escritorio.png](01-p1-enlace-escritorio.png) |
| 4 | §4.1 Página pública (movil) | Vehículo, diagnóstico, líneas, base, IVA, total y caducidad; sin datos del cliente; sin desplazamiento lateral | faltan=[]; datos del cliente=[]; sin scroll lateral=true | ✅ | [02-p1-enlace-movil.png](02-p1-enlace-movil.png) |
| 5 | §4.2 Pulsar «Aceptar» | «¿Acepta este presupuesto por 133,10 €? Una vez confirmado, no se podrá cambiar.» con «Sí, aceptar» y «Volver», dentro de la página | pregunta=true; botones=true/true; confirm() del navegador=false | ✅ | [03-p2-confirmacion-movil.png](03-p2-confirmacion-movil.png) |
| 6 | §4.2 «Volver» | La página queda como estaba y no se guarda nada | botones de nuevo=true; estado en la base=emitido | ✅ | [04-p2-volver-movil.png](04-p2-volver-movil.png) |
| 7 | §4.2 «Sí, aceptar» | Se ve «Aceptado el 05/11/2026 a las 18:39» (hora de Madrid) y se guarda en la base | mensaje=true; base=aceptado 1793900379711 | ✅ | [05-p2-aceptado-movil.png](05-p2-aceptado-movil.png) |
| 8 | §4.2 Volver a abrir el enlace | Se ve la decisión con su fecha y hora, sin botones | botones=0; mensaje=true | ✅ | [06-p2-reabierto-movil.png](06-p2-reabierto-movil.png) |
| 9 | §4.3 Orden de 1313MMM (luis) | «En reparación», «Inicial · Aceptado el 05/11/2026 a las 18:39» y sin enlace | en reparación=true; línea=true; enlace visible=false; estado en la base=en_reparacion | ✅ | [07-p3-orden-1313MMM.png](07-p3-orden-1313MMM.png) |
| 10 | §4.4 Orden demo 5, «Copiar enlace» (ana, escritorio) | Enlace visible en un campo de solo lectura; «Copiar enlace» lo copia y pasa a «Copiado» | enlace=http://localhost:3000/presupuesto/demo-presupuesto-emitido; solo lectura=true; «Copiado»=true; portapapeles igual=true; sin scroll lateral=true | ✅ | [08-p4-copiar-enlace-escritorio.png](08-p4-copiar-enlace-escritorio.png) |
| 11 | §4.4 Orden demo 5, «Copiar enlace» (ana, movil) | Enlace visible en un campo de solo lectura; «Copiar enlace» lo copia y pasa a «Copiado» | enlace=http://localhost:3000/presupuesto/demo-presupuesto-emitido; solo lectura=true; «Copiado»=true; portapapeles igual=true; sin scroll lateral=true | ✅ | [09-p4-copiar-enlace-movil.png](09-p4-copiar-enlace-movil.png) |
| 12 | §4.4 Orden demo 5 como marta, su mecánica | No aparece la opción de imputar (y el servidor también lo impide) | botón «Imputar»=0; aviso=true; POST directo=409 «La orden está en diagnóstico; solo se puede imputar en órdenes en reparación» | ✅ | [10-p4-sin-imputar-marta.png](10-p4-sin-imputar-marta.png) |
| 13 | §4.5 Rechazar el presupuesto de 1414NNN | Confirmación «¿Rechaza este presupuesto por 48,40 €? Una vez confirmado, no se podrá cambiar.»; «Rechazado el 05/11/2026 a las 18:39» en la página; la orden sigue «En diagnóstico» con «Inicial · Rechazado el 05/11/2026 a las 18:39» | correo=true; confirmación=true; página=true; ficha en diagnóstico=true; ficha línea=true; estado en la base=en_diagnostico | ✅ | [12-p5-rechazado.png](12-p5-rechazado.png) |
| 14 | §4.6 Orden demo 7 (luis) | «Pendiente de conformidad»; la ampliación sale «Caducado el 04/11/2026», sin enlace ni «Copiar enlace» | pendiente=true; caducado=true; enlace=0; copiar=0 | ✅ | [14-p6-orden7-caducada.png](14-p6-orden7-caducada.png) |
| 15 | §4.7 Rechazar la ampliación en la primera pestaña | La orden sigue «Pendiente de conformidad» con «Inicial · Aceptado el 18/10/2026 a las 14:30» y «Ampliación · Rechazado el 05/11/2026 a las 18:39» | correo=true; título ampliación=true; pendiente=true; inicial=true; ampliación=true | ✅ | [16-p7-orden7-dos-respuestas.png](16-p7-orden7-dos-respuestas.png) |
| 16 | §4.7 Aceptar en la segunda pestaña | «Este presupuesto ya fue respondido» y la orden no cambia | aviso=true; orden/ampliación en la base=pendiente_conformidad/rechazado | ✅ | [17-p7-pestana2-ya-respondido.png](17-p7-pestana2-ya-respondido.png) |
| 17 | §4.8 Presupuesto caducado (escritorio) | «Este presupuesto caducó el 04/11/2026. Contacte con el taller», sin botones | mensaje=true; botones=0; sin scroll lateral=true | ✅ | [18-p8-caducado-escritorio.png](18-p8-caducado-escritorio.png) |
| 18 | §4.9 Enlace inventado (escritorio) | Solo «Este enlace no es válido» | texto=«Este enlace no es válido» | ✅ | [19-p9-enlace-invalido-escritorio.png](19-p9-enlace-invalido-escritorio.png) |
| 19 | §4.8 Presupuesto caducado (movil) | «Este presupuesto caducó el 04/11/2026. Contacte con el taller», sin botones | mensaje=true; botones=0; sin scroll lateral=true | ✅ | [20-p8-caducado-movil.png](20-p8-caducado-movil.png) |
| 20 | §4.9 Enlace inventado (movil) | Solo «Este enlace no es válido» | texto=«Este enlace no es válido» | ✅ | [21-p9-enlace-invalido-movil.png](21-p9-enlace-invalido-movil.png) |

## Observaciones

- La confirmación sale dentro de la página, sin diálogos del navegador (D5): el script registra cualquier `confirm()` y no apareció ninguno.
- La fecha y la hora de cada respuesta se contrastan con `respondido_en` de la base, convertida a la hora de Madrid.
- §4.4: además de que `marta` no ve «Imputar», un `POST` directo a sus imputaciones da 409, así que la regla no depende solo de la pantalla (FR-007, SC-004).
- §4.7: la segunda pestaña recibe «Este presupuesto ya fue respondido» y la base sigue con la orden «pendiente_conformidad» y la ampliación «rechazado»: una respuesta no sobrescribe otra.
- Captura 16: tras el rechazo de la ampliación, la ficha ofrece «Nuevo presupuesto», como pide US2-2.

## Cómo repetirlo

`node t027.mjs <carpeta de evidencias> <carpeta de correos>`, con la app en http://localhost:3000, la base recién cargada y un SMTP local en el puerto 1025. El script está en esta carpeta.
