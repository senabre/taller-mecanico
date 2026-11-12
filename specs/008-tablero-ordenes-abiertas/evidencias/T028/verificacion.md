# T028 · Verificación del quickstart común §8 en el navegador

- **Fecha**: 12/11/2026. Ejecutada por la Persona A con Playwright (Chromium, `es-ES`, zona horaria de Madrid).
- **Código**: rama `spec-008` en `c01fb8e`.
- **Datos**: base recién creada con las migraciones y `backend/seed/demo.sql`, sin recorrer §1–§7 antes; por eso la carga de partida es «Luis 5 · Marta 3», como prevé el paso 3. (El §8 después del §7 se recorre en la regresión, T030.)
- **Dos navegadores**: el tablero de `paco` no se toca durante todo el recorrido; los cambios se hacen desde otro contexto (`luis` o `ana`).
- **Anchos**: pantalla grande de 1920 × 1080 y móvil de 390 px.
- **Resultado**: **13/14 comprobaciones superadas**. La que falla es la parte *opcional* del paso 9, con las herramientas de desarrollo: ver abajo.

## Resultados

| # | Paso | Esperado | Observado | | Captura |
|---|---|---|---|---|---|
| 1 | §8.1 ana y el tablero | Sin «Tablero» en el menú; /tablero dice «No tienes permiso para hacer esto», sin columnas; la API da 403 | menú=0; mensaje=true; columnas=0; API=403 | ✅ | [01-p1-ana-sin-permiso.png](01-p1-ana-sin-permiso.png) |
| 2 | §8.1 luis y el tablero | Sin «Tablero» en el menú; /tablero dice «No tienes permiso para hacer esto», sin columnas; la API da 403 | menú=0; mensaje=true; columnas=0; API=403 | ✅ | [02-p1-luis-sin-permiso.png](02-p1-luis-sin-permiso.png) |
| 3 | §8.1 pepe y el tablero | Sin «Tablero» en el menú; /tablero dice «No tienes permiso para hacer esto», sin columnas; la API da 403 | menú=0; mensaje=true; columnas=0; API=403 | ✅ | [03-p1-pepe-sin-permiso.png](03-p1-pepe-sin-permiso.png) |
| 4 | §8.2 Columnas (paco) | «Tablero» en el menú; 6 columnas en orden; sin 9999LLL ni 1313MMM; todas las abiertas, cada una en su columna | menú=1; columnas=[Recibida, En diagnóstico, En reparación, En espera de piezas, Pendiente de conformidad, Terminada]; tarjetas=8 de 8 abiertas; 9999LLL=false; 1313MMM=false; fuera de su columna=[]; vacías=0 | ✅ | [04-p2-tablero-1920.png](04-p2-tablero-1920.png) |
| 5 | §8.3 Carga por mecánico | «Luis 5 · Marta 3» (datos recién cargados); coincide con las tarjetas; Pedro no sale | carga=«Luis 5 · Marta 3»; tarjetas por mecánico={"Luis":5,"Marta":3} | ✅ | — |
| 6 | §8.4 Retrasadas (lo que hay que encontrar en SC-001) | Solo 8888KKK, en «Terminada», con borde destacado y «Retrasada»; 3333DDD (entrega hoy) sin «Retrasada» | retrasadas=[8888KKK (Terminada, borde 2px)]; 3333DDD retrasada=false | ✅ | — |
| 7 | §8.5 Orden por fecha de entrega | «En reparación»: 3333DDD (hoy) encima de 2222CCC (pasado mañana) | orden=[3333DDD,2222CCC] | ✅ | — |
| 8 | §8.6 luis termina 2222CCC | En menos de 5 s pasa a «Terminada»; la carga de Luis no cambia | confirm()=«¿Marcar la reparación como terminada? Ya no se podrán imputar más horas ni piezas.»; tiempo=109 ms; columna=Terminada; carga=«Luis 5 · Marta 3» | ✅ | [05-p6-2222CCC-terminada.png](05-p6-2222CCC-terminada.png) |
| 9 | §8.7 ana abre una orden de 8888KKK para Marta | En menos de 5 s su tarjeta sale en «Recibida» y la carga pasa a «Marta 4» | tiempo=108 ms; carga=«Luis 5 · Marta 4» | ✅ | [06-p7-8888KKK-recibida.png](06-p7-8888KKK-recibida.png) |
| 10 | §8.8 ana factura 2222CCC | En menos de 5 s la tarjeta desaparece y la carga pasa a «Luis 4» | confirm()=«¿Emitir la factura? Ya no se podrá modificar.»; tiempo=111 ms; carga=«Luis 4 · Marta 4» | ✅ | [07-p8-2222CCC-facturada.png](07-p8-2222CCC-facturada.png) |
| 11 | §8.9 Parar y volver a arrancar el servidor | «Sin conexión, reintentando…» y, al volver, el aviso desaparece solo, sin recargar | aviso tras 114 ms; vuelta tras 2925 ms de arrancar | ✅ | [08-p9-sin-conexion.png](08-p9-sin-conexion.png) |
| 12 | §8.9 Red cortada en el navegador de paco (opcional) | Con la red cortada, luis termina 3333DDD; al volver la red, el aviso se va y 3333DDD ya está en «Terminada», sin recargar | aviso=false; terminar=200; columna sin red=En reparación; al volver: null ms | ❌ | [10-p9-tras-red-cortada.png](10-p9-tras-red-cortada.png) |
| 13 | §8.10 Móvil a 390 px | Una columna entera y el borde de la siguiente; las columnas se desplazan dentro del tablero; la página no se mueve de lado; la carga cabe | {"pagina":390,"tira":true,"primera":true,"siguiente":true,"cargaAncho":true} | ✅ | [11-p10-movil-390.png](11-p10-movil-390.png) |
| 14 | §8.10 Pantalla grande (1920 px) | Las 6 columnas una al lado de otra, ocupando todo el ancho, sin desplazamiento lateral | {"fila":true,"dentro":true,"ancho":1888,"pagina":1920} | ✅ | [13-p10-pantalla-grande.png](13-p10-pantalla-grande.png) |

## Observaciones

### 1. El paso 9 opcional (red cortada con las herramientas de desarrollo) no sirve tal como está escrito

Con la red del navegador de `paco` puesta en «Sin conexión» (emulación de Chromium, lo mismo que hace el menú de las herramientas de desarrollo), la conexión de eventos **ya abierta no se corta**: el tablero no muestra «Sin conexión, reintentando…», el aviso de terminar `3333DDD` no le llega mientras tanto y, al volver la red, sigue sin enterarse durante 20 s. Es un comportamiento conocido de Chromium: la emulación sin conexión afecta a las peticiones nuevas, no a un `EventSource` que ya está abierto.

- **No es un fallo del tablero**: con un corte real (el servidor parado, primera parte del paso 9) el aviso sale en 0,1 s y el tablero se recupera solo a los 3 s de volver a arrancar el servidor, ya al día.
- **Sí es un fallo del quickstart**: la parte opcional pide comprobar algo que esa herramienta no provoca. Propuesta para el converge: quitarla y dejar la primera parte (parar y arrancar el servidor), que sí prueba la reconexión, seguida de «terminar `3333DDD` con el tablero conectado».
- Queda un caso no cubierto por el diseño: si la red se cae «en silencio» (sin que la conexión se cierre), el tablero no lo nota hasta que el sistema operativo da la conexión por perdida, y se pone al día con el siguiente evento. Se anota como hallazgo de spec-008 para decidir en el converge.

### 2. Dos confirmaciones con `confirm()` del navegador (para la Persona B)

«Terminar reparación» (spec-005) y «Facturar» (spec-007) piden confirmación con el diálogo del navegador:

- «¿Marcar la reparación como terminada? Ya no se podrán imputar más horas ni piezas.»
- «¿Emitir la factura? Ya no se podrá modificar.»

El texto es español, pero los botones del diálogo («Aceptar»/«Cancelar», o «OK»/«Cancel») salen en el idioma del navegador, y los diálogos bloquean las pruebas automáticas. En spec-004 se descartó `confirm()` por el principio II (D5). No bloquea el tablero; se anota para la Persona B.

### 3. Tiempos

Las tres actualizaciones del tablero (terminar, abrir, facturar) llegaron en unos 110 ms. Son tiempos de máquina: SC-002 lo cronometra una persona en T029.

## Cómo repetirlo

`node t028.mjs <carpeta>` con la base recién cargada y la app en http://localhost:3000. El script para el servidor y lo vuelve a arrancar él mismo en el paso 9.
