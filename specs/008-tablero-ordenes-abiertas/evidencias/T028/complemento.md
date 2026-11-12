# T028 · Complemento: lo que piden T011, T012 y T025

- **Fecha**: 12/11/2026, misma rama (`c01fb8e`) y datos demo recién cargados.
- **Cómo se preparan los estados**: la columna vacía y la columna llena se preparan cambiando el estado de órdenes demo con SQL (y se restauran al terminar); el fallo de carga se provoca cortando por la red solo la petición a `/api/tablero`. Ninguno se puede preparar con los datos demo tal cual.
- **Resultado**: **6/6**.

| Comprobación | Esperado | Observado | | Captura |
|---|---|---|---|---|
| T011 Ancho del menú y del contenido (1920 px) | Menú y contenido a 1152 px en las demás pantallas y a todo el ancho en /tablero, también navegando sin recargar | {"/citas":{"menu":1152,"main":1152},"/ordenes":{"menu":1152,"main":1152},"/almacen":{"menu":1152,"main":1152},"/pedidos":{"menu":1152,"main":1152},"/historial":{"menu":1152,"main":1152},"/tablero":{"menu":1920,"main":1920}}; navegando: /citas={"menu":1152,"main":1152}, /tablero={"menu":1920,"main":1920} | ✅ | [b01-t011-citas-1920.png](b01-t011-citas-1920.png), [b07-t011-tablero-1920.png](b07-t011-tablero-1920.png) |
| T025 Con un 403 no se abre /api/eventos (ana) | Pide /api/tablero (403) y no abre /api/eventos | peticiones=[/api/yo,/api/tablero]; mensaje=true | ✅ | [b02-t025-ana-403.png](b02-t025-ana-403.png) |
| T012 Falla la primera carga (red interceptada) | «No se pudo cargar el tablero» | mensaje=true | ✅ | [b03-t012-no-se-pudo-cargar.png](b03-t012-no-se-pudo-cargar.png) |
| T012 Columna vacía (preparada por SQL) | La columna sigue y dice «Sin órdenes» | existe=1; texto=«Pendiente de conformidad /  / Sin órdenes» | ✅ | [b04-t012-columna-vacia.png](b04-t012-columna-vacia.png) |
| T012 Columna llena (preparada por SQL, 1920 px) | Las 8 tarjetas en «En reparación»; la columna se desplaza por dentro; la página no se mueve de lado | {"tarjetas":8,"desborda":true,"desplaza":true,"pagina":true} | ✅ | [b05-t012-columna-llena-1920.png](b05-t012-columna-llena-1920.png) |
| T012 Columna llena (preparada por SQL, 390 px) | Las 8 tarjetas en «En reparación»; la columna se desplaza por dentro; la página no se mueve de lado | {"tarjetas":8,"desborda":true,"desplaza":true,"pagina":true} | ✅ | [b06-t012-columna-llena-390.png](b06-t012-columna-llena-390.png) |

Script: `t028b.mjs`.

**Repetición de T011 (mismo día)**: la primera versión solo medía el contenido (`main`) y su captura de «Citas» se tomó después de volver al tablero. Se repitió midiendo también el menú (`header nav`) y capturando `/citas` antes de volver; esa es la fila de T011 de arriba (`resultados-t011.json`). Las demás filas no cambian (`resultados-b.json`).
