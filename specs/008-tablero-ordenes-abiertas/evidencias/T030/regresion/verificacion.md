# T030 · Regresión del quickstart común entero (spec-008), con el arreglo de V3

- **Fecha**: 12/11/2026. Ejecutada por la Persona A con Playwright (Chromium, `es-ES`, zona horaria de Madrid).
- **Código**: rama `spec-008` en `02e9868` (con el arreglo de V3: `GET /api/tablero` sin transacción).
- **Base**: recién creada con las migraciones y `backend/seed/demo.sql`. Correo recogido por un SMTP local en el puerto 1025, en lugar de Mailpit.
- **Orden**: §1 → §2 → §0 → §3 → §4 → D7 → §5 → §6 → §7 → §8, en un solo recorrido sobre la misma base. El §8 se recorre después del §7, así que la carga de partida es «Luis 6 · Marta 3», como dice el paso 3.
- El primer intento (antes del arreglo) se paró en el §1 por V3: ver `hallazgo-conexiones.md`.

## Resumen

| Sección | Comprobaciones | Evidencias |
|---|---|---|
| §0 Base común | 5/5 | `s0/` |
| §1 spec-001 · Pedir cita | 31/31 | `s1/` |
| §2 spec-002 · Abrir orden | 24/24 | `s2/` |
| §3 spec-003 · Diagnóstico y presupuesto | 7/7 | `s3/` |
| §4 spec-004 · Respuesta del cliente | 20/20 | `s4/` |
| D7 · Orden demo 7 | 3/3 | `d7/` |
| §5 spec-005 · Imputar | 6/6 | `s5/` |
| §6 spec-006 · Almacén y pedidos | 9/9 | `s6/` |
| §7 spec-007 · Factura e historial | 5/5 | `s7/` |
| §8 spec-008 · Tablero | 13/14 | `s8/` |
| §9 Integración | **Pendiente**: se hace con la Persona B en dos días reales (día A y día B) | — |

**Resultado**: §0–§8 sin ningún fallo de la aplicación.

- La única comprobación que falla (§8, «Red cortada en el navegador de paco») es la parte opcional del paso 9 que **se quitó del quickstart en el converge-1** (V1): el script todavía la incluye para dejar constancia de que la emulación sin conexión de Chromium no corta un `EventSource` abierto. No es parte del quickstart actual.
- §2: con `SOLO_RECHAZO`, el §2.6 solo comprueba el rechazo de 30.000 km para 8888KKK y no guarda la orden, porque el §8.7 abre después una orden de 8888KKK. En móvil no se comprueba (ya no queda ninguna cita de hoy sin orden; está en T029 de spec-002).
- D7: la tarjeta de la orden 7 sale en «Pendiente de conformidad» en el tablero (§8.2), lo que T029 de spec-004 dejó pendiente.

## V3 tras el arreglo

`estres.sh` (200 peticiones al tablero cortadas a los 0,5–4,5 ms, 3 tandas) sobre la base del final de la regresión: **0 errores, ninguna conexión en transacción y 30 de 30 inicios de sesión correctos** (`estres-tras-arreglo.txt`). Con el código anterior daba 18 errores (`../T030/estres-codigo-actual.txt` del primer intento).

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
| §1.5 / (a) Día lleno (US3-1, SC-003) | primer día ofrecido=16/11/2026 | ✅ |
| §1.6 «1111BBB» (US1-1, US1-7) | Seat Ibiza=true, campos de contacto=0, aviso=false | ✅ |
| §1.6 Cita de 1111BBB (US1-2, US1-6) | confirmada para 16/11/2026 | ✅ |
| /cita en móvil (390 px) | ancho del documento=390px | ✅ |
| §1.7 «Citas» abre con hoy (US1-8) | título=Citas de hoy, selector=2026-11-12 | ✅ |
| §1.7 Lista del día (US1-3, US1-8, FR-009) | filas=1234BCD, 5678CDF; columnas=Matrícula / Marca / Modelo / Motivo / Nombre / Teléfono / Orden | ✅ |
| §1.7 1111BBB con sus datos sin cambiar (US1-7, FR-003) | 1111BBB · Seat · Ibiza · Cambio de aceite · María López · 600111222 ·  | ✅ |
| §1.8 Día sin citas (US1-9) | mensaje visible | ✅ |
| §1.9 luis (US1-10) | menú con Citas=false, mensaje=true, tabla=0 | ✅ |
| §1.9 pepe (US1-10) | menú con Citas=false, mensaje=true, tabla=0 | ✅ |
| §1.9 Sin sesión (US1-10) | url=/entrar | ✅ |
| §1.9 paco (jefe) | filas=1234BCD, 5678CDF | ✅ |
| §1.10 SC-001 vehículo conocido | confirmada; 3 datos escritos; 1.2 s de máquina | ✅ |
| §1.10 SC-002 vehículo nuevo | confirmada; 8 datos escritos; 1.2 s de máquina | ✅ |
| (b) US3-2 última plaza en dos pestañas | A=true, aviso B=true, selector=Elige un día, día sigue=false, confirmar activo=false | ✅ |
| (b) US3-2 B elige otro día | confirmada para 18/11/2026 | ✅ |
| (d) T035 misma matrícula nueva en dos pestañas (FR-010) | Skoda Octavia=true, aviso=true, campos nuevos=0, confirmar activo=false | ✅ |
| (d) T035 en /citas | 7777XYZ · Skoda · Octavia · Prueba T035 · Pestaña B · 611000222 ·  | ✅ |
| (e) T037 respuesta tardía en /citas (FR-009) | título=Citas del 13/11/2026; filas=1234BCD, 5678CDF | ✅ |
| (e) T037 cambios rápidos y «Hoy» | título=Citas de hoy; citas de hoy=true; 1234BCD=false | ✅ |
| T034 «9999LLL» (§2.3, parte de spec-001) | Opel Corsa=true, aviso=false, confirmada=true, en /citas=true | ✅ |
### §2 spec-002 · Abrir orden

| Paso | Observado | |
|---|---|---|
| §2.6 8888KKK con 30.000 km (US2, FR-004) | mensaje=«El kilometraje no puede ser menor que el de la visita anterior (39.800 km)»; página=/ordenes/nueva | ✅ |
| §2.1 Citas de hoy | con botón=true,true,true | ✅ |
| §2.1 Cita de otro día (1234BCD) | sin botón | ✅ |
| §2.1 Ayer (Q2, Q5) | 1515PPP botón=false; 1111BBB=Orden Nº 1 | ✅ |
| §2.1 Abrir por URL una cita de ayer (US1-5) | mensaje visible | ✅ |
| §2.2 Formulario de 1313MMM (US1-1, US1-7) | 1313MMM=true; mecánicos=Luis, Marta | ✅ |
| §2.3 Error de danos (US1-8, FR-003, FR-006) | mensaje=«Describe los daños visibles o escribe “Sin daños visibles”»; página=/ordenes/nueva | ✅ |
| §2.3 Error de km (US1-8, FR-003, FR-006) | mensaje=«El kilometraje debe ser un número entero igual o mayor que 0»; página=/ordenes/nueva | ✅ |
| §2.3 Error de entrega (US1-8, FR-003, FR-006) | mensaje=«La fecha de entrega prometida no puede ser anterior a hoy»; página=/ordenes/nueva | ✅ |
| §2.4 Ficha de la orden (US1-2, US1-3, SC-002) | Orden Nº 10; Recibida=true; 85.000 km=true; url=/ordenes/10 | ✅ |
| §2.4 SC-001 (tiempo de máquina) | 5.0 s de máquina con 3 errores intermedios; el cronometraje humano queda pendiente | ✅ |
| §2.4 Enlace en Citas (US1-4, Q5) | Orden Nº 10; tras pulsar=/ordenes/10 | ✅ |
| §2.5 Primera pestaña (SC-002, US1-9) | Orden Nº 11 | ✅ |
| §2.5 Segunda pestaña (US1-4, FR-008) | mensaje=true; enlace=/ordenes/11 | ✅ |
| §2.5 «Órdenes» (FR-008) | órdenes de 1414NNN=1; números=11,10,1,4,5,2,3,6,8,7,9 | ✅ |
| Lista: orden 8 retrasada | 8 · 8888KKK · Terminada · Marta · 11/11/2026 Retrasada | ✅ |
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
| §4.1 Correo con el enlace (Mailpit) | 1794496998.840766.eml para lucia.fernandez@ejemplo.test; enlace http://localhost:3000/presupuesto/-vP0vi7UOrN7SwFb2lPXqzffC4hT8fgBBmkJiMGUWaw | ✅ |
| §4.1 Página pública (escritorio) | faltan=[]; datos del cliente=[]; sin scroll lateral=true | ✅ |
| §4.1 Página pública (movil) | faltan=[]; datos del cliente=[]; sin scroll lateral=true | ✅ |
| §4.2 Pulsar «Aceptar» | pregunta=true; botones=true/true; confirm() del navegador=false | ✅ |
| §4.2 «Volver» | botones de nuevo=true; estado en la base=emitido | ✅ |
| §4.2 «Sí, aceptar» | mensaje=true; base=aceptado 1794497013190 | ✅ |
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
### D7 · Orden demo 7

| Paso | Observado | |
|---|---|---|
| D7 Lo presupuestado de la orden 7 | total_presupuestado=110.00; estado=pendiente_conformidad; presupuestos=inicial:aceptado, ampliacion:emitido, ampliacion:rechazado | ✅ |
| D7 Emitir otra ampliación en la orden 7 | crear=201 (ampliacion); línea=201; emitir=200 (emitido) | ✅ |
| D7 Tarjeta de la orden 7 | entrega prometida=13/11/2026; retrasada hoy=false | ✅ |
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
| §6.3 Pedido FA-9 para la orden 3 | fila=«3 Recambios Sur FA-9 · Filtro aceite 1 12/11/2026 Marcar recibido»; orden=en_espera_piezas | ✅ |
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
| §7.2 Facturar la orden de 1313MMM | Factura nº 2 · 12/11/2026 · 157,30 € · Ver e imprimir; anterior=1 | ✅ |
| §7.2 y §7.3 Ver e imprimir | faltan=[]; menú al imprimir=false; imágenes=0 | ✅ |
| §7.4 Correo al cliente | asunto=true; total=true | ✅ |
| §7.5 Historial | faltan en 1313=[]; 9999LLL=true; inexistente=true | ✅ |
### §8 spec-008 · Tablero

| Paso | Observado | |
|---|---|---|
| §8.1 ana y el tablero | menú=0; mensaje=true; columnas=0; API=403 | ✅ |
| §8.1 luis y el tablero | menú=0; mensaje=true; columnas=0; API=403 | ✅ |
| §8.1 pepe y el tablero | menú=0; mensaje=true; columnas=0; API=403 | ✅ |
| §8.2 Columnas (paco) | menú=1; columnas=[Recibida, En diagnóstico, En reparación, En espera de piezas, Pendiente de conformidad, Terminada]; tarjetas=9 de 9 abiertas; 9999LLL=false; 1313MMM=false; fuera de su columna=[]; vacías=0 | ✅ |
| §8.3 Carga por mecánico | carga=«Luis 6 · Marta 3»; tarjetas por mecánico={"Luis":6,"Marta":3} | ✅ |
| §8.4 Retrasadas (lo que hay que encontrar en SC-001) | retrasadas=[8888KKK (Terminada, borde 2px)]; 3333DDD retrasada=false | ✅ |
| §8.5 Orden por fecha de entrega | orden=[3333DDD,2222CCC] | ✅ |
| §8.6 luis termina 2222CCC | confirm()=«¿Marcar la reparación como terminada? Ya no se podrán imputar más horas ni piezas.»; tiempo=106 ms; columna=Terminada; carga=«Luis 6 · Marta 3» | ✅ |
| §8.7 ana abre una orden de 8888KKK para Marta | tiempo=106 ms; carga=«Luis 6 · Marta 4» | ✅ |
| §8.8 ana factura 2222CCC | confirm()=«¿Emitir la factura? Ya no se podrá modificar.»; tiempo=106 ms; carga=«Luis 5 · Marta 4» | ✅ |
| §8.9 Parar y volver a arrancar el servidor | aviso tras 106 ms; vuelta tras 2888 ms de arrancar | ✅ |
| §8.9 Red cortada en el navegador de paco (opcional) | aviso=false; terminar=200; columna sin red=En reparación; al volver: null ms | ❌ |
| §8.10 Móvil a 390 px | {"pagina":390,"tira":true,"primera":true,"siguiente":true,"cargaAncho":true} | ✅ |
| §8.10 Pantalla grande (1920 px) | {"fila":true,"dentro":true,"ancho":1888,"pagina":1920} | ✅ |

## Cómo repetirlo

Scripts en `scripts/`: `reset.sh` (base limpia y servidor), `smtp.py`, y después `t032.mjs` (§1), `t029.mjs` y `t029b.mjs` con `SIN_MOVIL=1 SOLO_RECHAZO=1` (§2), `t029r.mjs` con `0` y `3`, `t027.mjs` con `REUSAR=1` (§4), `t029d7.mjs`, `t029r.mjs` con `5`, `6` y `7`, y `t028.mjs` con `CARGA_LUIS=6` (§8). Las salidas de consola están en `salida/`.
