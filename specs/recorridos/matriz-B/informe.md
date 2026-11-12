# Matriz de trazabilidad · comprobaciones añadidas de la Persona B

- **Fecha**: 12/11/2026. Persona B, con un navegador automático (Playwright 1.48, Chromium en `es-ES`,
  zona horaria de Madrid, 1440 × 900).
- **Código**: rama `entrega`, commit `0c44f78`, interfaz recién compilada (`npm run build`).
- **Datos**: base aparte `taller_matriz` en el mismo Postgres, recién migrada y con
  `backend/seed/demo.sql` cargado ese mismo día. Se usó una base aparte para no borrar la de desarrollo.
- **Motivo**: al construir la matriz, seis criterios de la Persona B no tenían evidencia: cinco de
  tiempo, que las notas de verificación no recogían, y el logo de la factura en pantalla.
- **Resultado**: 7/7 (datos completos en `resultados.json`).

## Sobre los tiempos

Los criterios de tiempo hablan de una persona («un mecánico prepara…», «la recepcionista emite…»).
Aquí **no se cronometró a una persona**: se midió el tiempo de máquina, desde que se abre la pantalla
hasta que se ve el resultado, con el navegador automático escribiendo y pulsando. Ese tiempo demuestra
que la aplicación no es el cuello de botella. Para valorar el tiempo de una persona, cada fila indica
cuántos campos y clics hacen falta. Por eso, en la matriz, estos criterios quedan como **«Verificado
con desviación»**.

| Criterio | Recorrido | Campos y clics de una persona | Tiempo de máquina | Límite | | Captura |
|---|---|---|---|---|---|---|
| 003 SC-001 · Presupuesto de 5 líneas | `luis`, Orden Nº 4 («en diagnóstico»): «Nuevo presupuesto» y 4 líneas de mano de obra (0,5 + 1 + 0,25 + 0,5 h) + PF-123 × 2 → base 140,00, IVA 29,40, total 169,40 € | 1 clic para crearlo; por línea, 2 campos y 1 clic (la pieza: buscar, elegir y unidades) | 1,02 s | 5 min | ✅ | `c03-003-presupuesto-5-lineas.png` |
| 005 SC-003 · Una imputación | `luis`, Orden Nº 3 («en reparación»): «Revisión de frenos» 0,25 h → «Acumulado 50,00 € de 110,00 €» | 2 campos y 1 clic | 0,24 s | 30 s | ✅ | `c02-005-imputacion.png` |
| 006 SC-001 · Un pedido | `pepe`, `/pedidos`: Recambios Sur, PF-123, 1 ud, orden 2 → «Pedido registrado: 1 × PF-123 para la orden 2» | 4 campos y 2 clics | 0,56 s | 1 min | ✅ | `c01-006-pedido.png` |
| 007 SC-001 · Una factura | `ana`, Orden Nº 8 («terminada»): «Facturar» y confirmar → «Factura nº 2 · 12/11/2026 · 133,10 €» | 0 campos y 2 clics | 0,24 s | 30 s | ✅ | `c04-007-factura.png` |
| 007 SC-004 · El historial | `ana`, `/historial`: `9999 lll` y «Buscar» → «25/10/2026 · Orden 9 · Factura nº 1 · 120.400 km» | 1 campo y 1 clic | 0,22 s | 10 s | ✅ | `c05-007-historial.png` |

## Logo de la factura (007 US1-4)

| Caso | Cómo se prepara | Observado | | Capturas |
|---|---|---|---|---|
| Sin logo | `TALLER_LOGO` del `.env` apunta a un archivo que no existe (el caso límite de la spec) | Factura nº 1 (`9999LLL`): ninguna imagen, en pantalla ni al imprimir; la cabecera empieza por el nombre del taller, sin hueco | ✅ | `b01-factura-sin-logo.png`, `b02-impresion-sin-logo.png` |
| Con logo | `TALLER_LOGO` apunta a una imagen SVG de prueba que existe; servidor reiniciado | Factura nº 2 (`8888KKK`): una imagen `src="/api/taller/logo"`, `alt="Logo de Talleres Rueda"`, cargada, a la izquierda de los datos del taller; sigue visible en la vista de impresión | ✅ | `b03-factura-con-logo.png`, `b04-impresion-con-logo.png` |

Script: `t-matriz-b.mjs` (fases `sinlogo` y `principal`).
