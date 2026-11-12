# T029 · Regresión del quickstart común entero (spec-004)

- **Fecha**: 09/11/2026. Ejecutada por la Persona A con Playwright (Chromium, `es-ES`, zona horaria de Madrid).
- **Código**: rama `spec-004` en `8d64285` (incluye spec-001, 002, 003, 005, 006 y 007 de `main`).
- **Base**: recién creada con las migraciones y `backend/seed/demo.sql` (parte 6 y cita de 7777JJJ a 18 días, de T001). Correo recogido por un SMTP local en el puerto 1025, en lugar de Mailpit.
- **Orden**: §1 → §2 → §0 → §3 → §4 → D7 → §5 → §6 → §7, en un solo recorrido sobre la misma base, así que la numeración (orden Nº 10 y Nº 11, factura nº 2) es la del quickstart.

## Resumen

| Sección | Comprobaciones | Evidencias |
|---|---|---|
| §0 Base común | 5/5 | `s0/` |
| §1 spec-001 · Pedir cita | 31/31 | `s1/` |
| §2 spec-002 · Abrir orden | 25/25 | `s2/` |
| §3 spec-003 · Diagnóstico y presupuesto | 7/7 | `s3/` |
| §4 spec-004 · Respuesta del cliente | 20/20 | `s4/` |
| D7 · Orden demo 7 (research D7) | 3/3 | `d7/` |
| §5 spec-005 · Imputar | 6/6 | `s5/` |
| §6 spec-006 · Almacén y pedidos | 9/9 | `s6/` |
| §7 spec-007 · Factura e historial | 3/5 | `s7/` |
| §7 · repetición de §7.3 y §7.4 | 2/2 | `s7/` |
| §8 spec-008 · Tablero | **Pendiente**: spec-008 aún no existe | — |
| §9 Integración | **Pendiente**: necesita §8 | — |

**Resultado**: §0 a §7 sin ningún fallo de la aplicación. Los dos fallos que quedan en la tabla (§7.3 y §7.4) son del **script**, no de la app: se explican abajo y se repitieron sobre el mismo estado, ya en verde. Antes hubo otros dos fallos de script (§1 y §3.7), corregidos y con el recorrido entero repetido desde una base limpia.

## Lo que research D7 da por comprobado

- **Lo presupuestado de la orden 7 sigue en 110,00 €**: solo cuenta el inicial aceptado; ni la ampliación caducada de los datos demo ni la rechazada en §4.7 suman.
- **Se puede emitir otra ampliación** en la orden 7: se creó, se le añadió una línea y se emitió (201, 201, 200).
- **La tarjeta no cambia**: la entrega prometida de la orden 7 no depende de la fecha de la cita, que es lo único que movió T001, y hoy no está retrasada. El tablero es de spec-008 (§8): queda pendiente comprobarlo en pantalla.

## Fallos del script (no de la aplicación)

1. **§1, comprobación (e) de T037** (primera ejecución): el script de spec-001 esperaba «No hay citas para este día» en el día de hoy. Desde spec-002, los datos demo tienen tres citas hoy (1313MMM, 1414NNN y 8888KKK), y la tabla las muestra bien. Se corrigió la expectativa y el recorrido completo se repitió desde una base limpia: 31/31.
2. **§3.7** (primera ejecución): el script buscaba el aviso con `role=alert` y la pantalla lo muestra como texto junto al botón. El aviso es el correcto; se corrigió el selector y se repitió desde una base limpia: 7/7.
3. **§7.3**: el script tomaba por «menú» la cabecera propia de la factura. La captura `s7/s7-03-factura-impresion.png` ya mostraba la factura sin menú. Se corrigió (el menú es el que tiene «Salir») y se repitió sobre el mismo estado: el menú se ve en pantalla y no al imprimir.
4. **§7.4**: el asunto llega codificado y partido en dos líneas (cabecera MIME plegada), y el script no lo desplegaba. Repetido con el asunto decodificado: «Su vehículo 1313MMM está listo para recoger», con el total 157,30 €.

## Fuera del alcance de esta regresión

- **§2 en móvil**: tras §2 ya no queda ninguna cita de hoy sin orden, así que el formulario a 390 px no se puede abrir. Está comprobado en la verificación de spec-002 (T029 de 002).
- **§1.10 y §2.4, cronometrajes**: los tiempos del script son de máquina, no valen como SC; los humanos están en T032 de 001, T035 de 002 y T028 de 004.
- **§7.3 con `TALLER_LOGO`**: solo se comprobó sin logo (no sale ninguna imagen).
- **El selector de fecha** de `/citas` sale en formato de EE. UU. en el Chromium sin ventana: es conocido (D9 de spec-002) y en Chrome en español sale en dd/mm/aaaa.

## Observación para la Persona B (no bloquea)

- §3.7: cuando el diagnóstico no se puede cambiar, el aviso es correcto y la base no cambia, pero el cuadro de texto sigue mostrando lo escrito («Otro diagnóstico») hasta recargar (captura `s3/s3-06-diagnostico-bloqueado.png`).

## Detalle

### §0 Base común

| Paso | Observado | |
|---|---|---|
| §0.1 Menú de luis | ve=[Mis órdenes,Historial]; no debería ver=[] | ✅ |
| §0.2 Contraseña errónea | mensaje visible | ✅ |
| §0.3 Menú de paco | ve=[Citas,Órdenes,Historial,Almacén,Pedidos,Tablero] | ✅ |
| §0.4 Menú de ana | ve=[Citas,Órdenes,Historial]; Tablero=false | ✅ |
| §0.4 Menú de pepe | ve=[Almacén,Pedidos,Historial]; Tablero=false | ✅ |
### §1 spec-001 · Pedir cita

| Paso | Observado | |
|---|---|---|
| §1.1 Matrícula «12#4» | error=true, confirmar activo=false | ✅ |
| §1.1 Matrícula «A» | error=true, confirmar activo=false | ✅ |
| §1.1 Matrícula «ABCDE123456» | error=true, confirmar activo=false | ✅ |
| §1.1 Matrícula «AB» | sin error=true, pide datos=true | ✅ |
| §1.2 «1234-bcd» (US2-1) | campos=1,1,1,1,1 | ✅ |
| §1.3 Teléfono «12345» (US2-3) | error=true | ✅ |
| §1.3 Correo «ana@correo» (US2-3) | error=true | ✅ |
| §1.3 Alta y confirmación (US2-2, US1-2) | confirmada con fecha dd/mm/aaaa y motivo | ✅ |
| §1.4 «1234BCD» con cita pendiente (US1-1, US1-4) | marca/modelo=true, aviso=true, confirmar activo=false | ✅ |
| §1.5 «+34 612 345 678» (US2-4) | confirmada | ✅ |
| §1.5 / (a) Día lleno (US3-1, SC-003) | primer día ofrecido=11/11/2026 | ✅ |
| §1.6 «1111BBB» (US1-1, US1-7) | Seat Ibiza=true, campos de contacto=0, aviso=false | ✅ |
| §1.6 Cita de 1111BBB (US1-2, US1-6) | confirmada para 11/11/2026 | ✅ |
| /cita en móvil (390 px) | ancho del documento=390px | ✅ |
| §1.7 «Citas» abre con hoy (US1-8) | título=Citas de hoy, selector=2026-11-09 | ✅ |
| §1.7 Lista del día (US1-3, US1-8, FR-009) | filas=1234BCD, 5678CDF; columnas=Matrícula / Marca / Modelo / Motivo / Nombre / Teléfono / Orden | ✅ |
| §1.7 1111BBB con sus datos sin cambiar (US1-7, FR-003) | 1111BBB · Seat · Ibiza · Cambio de aceite · María López · 600111222 ·  | ✅ |
| §1.8 Día sin citas (US1-9) | mensaje visible | ✅ |
| §1.9 luis (US1-10) | menú con Citas=false, mensaje=true, tabla=0 | ✅ |
| §1.9 pepe (US1-10) | menú con Citas=false, mensaje=true, tabla=0 | ✅ |
| §1.9 Sin sesión (US1-10) | url=/entrar | ✅ |
| §1.9 paco (jefe) | filas=1234BCD, 5678CDF | ✅ |
| §1.10 SC-001 vehículo conocido | confirmada; 3 datos escritos; 1.1 s de máquina | ✅ |
| §1.10 SC-002 vehículo nuevo | confirmada; 8 datos escritos; 1.2 s de máquina | ✅ |
| (b) US3-2 última plaza en dos pestañas | A=true, aviso B=true, selector=Elige un día, día sigue=false, confirmar activo=false | ✅ |
| (b) US3-2 B elige otro día | confirmada para 13/11/2026 | ✅ |
| (d) T035 misma matrícula nueva en dos pestañas (FR-010) | Skoda Octavia=true, aviso=true, campos nuevos=0, confirmar activo=false | ✅ |
| (d) T035 en /citas | 7777XYZ · Skoda · Octavia · Prueba T035 · Pestaña B · 611000222 ·  | ✅ |
| (e) T037 respuesta tardía en /citas (FR-009) | título=Citas del 10/11/2026; filas=1234BCD, 5678CDF | ✅ |
| (e) T037 cambios rápidos y «Hoy» | título=Citas de hoy; citas de hoy=true; 1234BCD=false | ✅ |
| T034 «9999LLL» (§2.3, parte de spec-001) | Opel Corsa=true, aviso=false, confirmada=true, en /citas=true | ✅ |
### §2 spec-002 · Abrir orden

| Paso | Observado | |
|---|---|---|
| §2.6 8888KKK con 30.000 km (US2, FR-004) | mensaje=«El kilometraje no puede ser menor que el de la visita anterior (39.800 km)»; página=/ordenes/nueva | ✅ |
| §2.6 Corregido a 40.000 km (US2, SC-002) | Orden Nº 12; 40.000 km=true | ✅ |
| §2.1 Citas de hoy | con botón=true,true,true | ✅ |
| §2.1 Cita de otro día (1234BCD) | sin botón | ✅ |
| §2.1 Ayer (Q2, Q5) | 1515PPP botón=false; 1111BBB=Orden Nº 1 | ✅ |
| §2.1 Abrir por URL una cita de ayer (US1-5) | mensaje visible | ✅ |
| §2.2 Formulario de 1313MMM (US1-1, US1-7) | 1313MMM=true; mecánicos=Luis, Marta | ✅ |
| §2.3 Error de danos (US1-8, FR-003, FR-006) | mensaje=«Describe los daños visibles o escribe “Sin daños visibles”»; página=/ordenes/nueva | ✅ |
| §2.3 Error de km (US1-8, FR-003, FR-006) | mensaje=«El kilometraje debe ser un número entero igual o mayor que 0»; página=/ordenes/nueva | ✅ |
| §2.3 Error de entrega (US1-8, FR-003, FR-006) | mensaje=«La fecha de entrega prometida no puede ser anterior a hoy»; página=/ordenes/nueva | ✅ |
| §2.4 Ficha de la orden (US1-2, US1-3, SC-002) | Orden Nº 10; Recibida=true; 85.000 km=true; url=/ordenes/10 | ✅ |
| §2.4 SC-001 (tiempo de máquina) | 4.9 s de máquina con 3 errores intermedios; el cronometraje humano queda pendiente | ✅ |
| §2.4 Enlace en Citas (US1-4, Q5) | Orden Nº 10; tras pulsar=/ordenes/10 | ✅ |
| §2.5 Primera pestaña (SC-002, US1-9) | Orden Nº 11 | ✅ |
| §2.5 Segunda pestaña (US1-4, FR-008) | mensaje=true; enlace=/ordenes/11 | ✅ |
| §2.5 «Órdenes» (FR-008) | órdenes de 1414NNN=1; números=11,10,1,4,5,2,3,6,8,7,9 | ✅ |
| Lista: orden 8 retrasada | 8 · 8888KKK · Terminada · Marta · 08/11/2026 Retrasada | ✅ |
| §2.7 «Mis órdenes» de luis | título=Mis órdenes; números=11,10,1,4,2,3,7,9; mecánicos=Luis | ✅ |
| Ficha como luis (T026) | secciones=Datos, Diagnóstico, Presupuesto, Imputaciones, Pedidos; textarea de diagnóstico=1 | ✅ |
| Ficha: guardar diagnóstico | En diagnóstico=true | ✅ |
| Ficha como ana | secciones=5; textarea de diagnóstico=0 | ✅ |
| Ficha inexistente | No existe la orden | ✅ |
| Ficha de la orden demo 2 | sí | ✅ |
| Ficha de la orden demo 6 | sí | ✅ |
| Ficha de la orden demo 8 | sí | ✅ |
### §3 spec-003 · Diagnóstico y presupuesto

| Paso | Observado | |
|---|---|---|
| §3.1 Diagnóstico y cambio | en diagnóstico=true/true; guardado=true | ✅ |
| §3.2 Líneas del presupuesto | faltan=[] | ✅ |
| §3.3 Totales | faltan=[] | ✅ |
| §3.4 1,3 h y 2,5 unidades | 1,3 h=«Las horas van en múltiplos de 0,25 (p. ej. 1,5)»; 2,5 uds: navegador=«Please enter a valid value. The two nearest valid values are 2 and 3.», API=422 «Las unidades de pieza deben ser un número entero» | ✅ |
| §3.5 Añadir y quitar «Prueba» | añadida=true; quitada=true; totales=true | ✅ |
| §3.6 Emitir | válido=true; formulario=0; botones quitar=0; correo=true | ✅ |
| §3.7 Cambiar el diagnóstico tras emitir | pantalla=«Ya se ha emitido un presupuesto para esta orden; el diagnóstico no se puede cambiar»; en la base=«Pastillas y discos delanteros desgastados» | ✅ |
### §4 spec-004 · Respuesta del cliente

| Paso | Observado | |
|---|---|---|
| Preparación (reutiliza §2 y §3) | orden 10 y 11; presupuesto 9 emitido, total 133.10 | ✅ |
| §4.1 Correo con el enlace (Mailpit) | 1794246783.786067.eml para lucia.fernandez@ejemplo.test; enlace http://localhost:3000/presupuesto/qR_uW2BNGPR4A2ckFyp512dDIgQflE6qDMgmC2JL2mc | ✅ |
| §4.1 Página pública (escritorio) | faltan=[]; datos del cliente=[]; sin scroll lateral=true | ✅ |
| §4.1 Página pública (movil) | faltan=[]; datos del cliente=[]; sin scroll lateral=true | ✅ |
| §4.2 Pulsar «Aceptar» | pregunta=true; botones=true/true; confirm() del navegador=false | ✅ |
| §4.2 «Volver» | botones de nuevo=true; estado en la base=emitido | ✅ |
| §4.2 «Sí, aceptar» | mensaje=true; base=aceptado 1794246797562 | ✅ |
| §4.2 Volver a abrir el enlace | botones=0; mensaje=true | ✅ |
| §4.3 Orden de 1313MMM (luis) | en reparación=true; línea=true; enlace visible=false; estado en la base=en_reparacion | ✅ |
| §4.4 Orden demo 5, «Copiar enlace» (ana, escritorio) | enlace=http://localhost:3000/presupuesto/demo-presupuesto-emitido; solo lectura=true; «Copiado»=true; portapapeles igual=true; sin scroll lateral=true | ✅ |
| §4.4 Orden demo 5, «Copiar enlace» (ana, movil) | enlace=http://localhost:3000/presupuesto/demo-presupuesto-emitido; solo lectura=true; «Copiado»=true; portapapeles igual=true; sin scroll lateral=true | ✅ |
| §4.4 Orden demo 5 como marta, su mecánica | botón «Imputar»=0; aviso=true; POST directo=409 «La orden está en diagnóstico; solo se puede imputar en órdenes en reparación» | ✅ |
| §4.5 Rechazar el presupuesto de 1414NNN | correo=true; confirmación=true; página=true; ficha en diagnóstico=true; ficha línea=true; estado en la base=en_diagnostico | ✅ |
| §4.6 Orden demo 7 (luis) | pendiente=true; caducado=true; enlace=0; copiar=0 | ✅ |
| §4.7 Rechazar la ampliación en la primera pestaña | correo=true; título ampliación=true; pendiente=true; inicial=true; ampliación=true | ✅ |
| §4.7 Aceptar en la segunda pestaña | aviso=true; orden/ampliación en la base=pendiente_conformidad/rechazado | ✅ |
| §4.8 Presupuesto caducado (escritorio) | mensaje=true; botones=0; sin scroll lateral=true | ✅ |
| §4.9 Enlace inventado (escritorio) | texto=«Este enlace no es válido» | ✅ |
| §4.8 Presupuesto caducado (movil) | mensaje=true; botones=0; sin scroll lateral=true | ✅ |
| §4.9 Enlace inventado (movil) | texto=«Este enlace no es válido» | ✅ |
### D7 · Orden demo 7 (research D7)

| Paso | Observado | |
|---|---|---|
| D7 Lo presupuestado de la orden 7 | total_presupuestado=110.00; estado=pendiente_conformidad; presupuestos=inicial:aceptado, ampliacion:emitido, ampliacion:rechazado | ✅ |
| D7 Emitir otra ampliación en la orden 7 | crear=201 (ampliacion); línea=201; emitir=200 (emitido) | ✅ |
| D7 Tarjeta de la orden 7 | entrega prometida=10/11/2026; retrasada hoy=false | ✅ |
### §5 spec-005 · Imputar

| Paso | Observado | |
|---|---|---|
| §5.1 Terminar sin imputar | botón en pantalla=0; API=422 «Primero hay que imputar el trabajo realizado» | ✅ |
| §5.2 Imputar PF-123 × 2 y luego × 3 | concepto=true; stock=2; error=«Solo hay 2 unidades en stock de PF-123; el almacén puede pedirla al proveedor» | ✅ |
| §5.3 Imputar 1 h | Acumulado 90,00 € de 110,00 € presupuestados | ✅ |
| §5.4 Imputar 1 h más | Acumulado 130,00 € de 110,00 € presupuestados; pendiente=true; imputar=0; terminar=0 | ✅ |
| §5.5 Ampliación de 1 h aceptada desde el correo | correo=true; aceptada=true; en reparación=true; Acumulado 130,00 € de 150,00 € presupuestados | ✅ |
| §5.6 Terminar | terminada=true; imputar=0 | ✅ |
### §6 spec-006 · Almacén y pedidos

| Paso | Observado | |
|---|---|---|
| §6.1 Alta de FA-9 sin stock y repetida | fila=«FA-9 Filtro aceite 12,00 € 0»; repetida=true | ✅ |
| §6.2 Proveedores | opciones=[Elige un proveedor,Autopartes Norte,Recambios Sur] | ✅ |
| §6.3 Pedido FA-9 para la orden 3 | fila=«3 Recambios Sur FA-9 · Filtro aceite 1 09/11/2026 Marcar recibido»; orden=en_espera_piezas | ✅ |
| §6.4 FA-9 otra vez | Ya hay un pedido pendiente de FA-9 para la orden 3 | ✅ |
| §6.5 PF-123 para la misma orden | pendientes de 3=2; orden=en_espera_piezas | ✅ |
| §6.6 Pedido para una orden en diagnóstico | La orden 4 está en diagnóstico; solo se puede pedir para órdenes en reparación o en espera de piezas | ✅ |
| §6.7 Orden 3 como luis | FA-9=true; PF-123=true; imputar=0; terminar=0 | ✅ |
| §6.8 Recibir FA-9 | stock=1; orden=en_espera_piezas; botón=0 | ✅ |
| §6.9 Recibir PF-123 | orden=en_reparacion; ficha=true | ✅ |
### §7 spec-007 · Factura e historial

| Paso | Observado | |
|---|---|---|
| §7.1 Orden en reparación (Nº 2) como ana | Facturar=0 | ✅ |
| §7.2 Facturar la orden de 1313MMM | Factura nº 2 · 09/11/2026 · 157,30 € · Ver e imprimir; anterior=1 | ✅ |
| §7.2 y §7.3 Ver e imprimir | faltan=[]; menú al imprimir=true; imágenes=0 | ❌ |
| §7.4 Correo al cliente | asunto=false; total=true | ❌ |
| §7.5 Historial | faltan en 1313=[]; 9999LLL=true; inexistente=true | ✅ |
### §7 · repetición de §7.3 y §7.4 tras corregir el script

| Paso | Observado | |
|---|---|---|
| §7.3 Al imprimir, sin menú (repetido) | menú en pantalla=true; al imprimir=false; imágenes=0 | ✅ |
| §7.4 Correo al cliente (repetido) | asunto=«Su vehículo 1313MMM está listo para recoger»; total=true | ✅ |

## Cómo repetirlo

Scripts en `scripts/`: `reset.sh` (base limpia y servidor), `smtp.py` (correo), y después `t032.mjs` (§1), `t029.mjs` y `t029b.mjs` con `SIN_MOVIL=1` (§2), `t029r.mjs` con la sección `0` y `3`, `t027.mjs` con `REUSAR=1` (§4), `t029d7.mjs`, y `t029r.mjs` con `5`, `6` y `7`. Las salidas de consola están en `salida/`.
