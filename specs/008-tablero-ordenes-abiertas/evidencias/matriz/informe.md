# Matriz de trazabilidad · comprobaciones añadidas de HU-08 (US2-2 y US4-5)

- **Fecha**: 12/11/2026. Persona A, con el navegador automático (Playwright, Chromium en `es-ES`, zona horaria de Madrid).
- **Código**: rama `entrega` (igual que `spec-008` en `02e9868`), base recién cargada con `demo.sql`.
- **Motivo**: al construir la matriz, estos dos criterios solo tenían cobertura parcial en la aplicación.
- **Resultado**: 2/2.

| Criterio | Cómo se prepara | Observado | | Capturas |
|---|---|---|---|---|
| HU-08 US2-2 · Un mecánico de baja con órdenes abiertas sale en la carga marcado «(de baja)» | Con SQL, Pedro (de baja) pasa a ser el mecánico de la orden demo 1; se restaura al terminar. Los datos demo no tienen ningún caso así | Carga «Luis 4 · Marta 3 · Pedro (de baja) 1»; la tarjeta de 1111BBB dice «Pedro»; la carga suma 8, igual que las tarjetas | ✅ | `c01-us2-2-de-baja.png` |
| HU-08 US4-5 · Lo que cambia mientras el tablero está sin conexión aparece al volver | Se para el servidor (corte real); con el servidor parado, la orden de 3333DDD pasa a «terminada» directamente en la base, sin que nadie avise al tablero; se vuelve a arrancar el servidor | Con el servidor parado: «Sin conexión, reintentando…» y 3333DDD sigue en «En reparación». Al volver a arrancar: 3333DDD en «Terminada» a los 1,8 s, sin recargar, y el aviso desaparece | ✅ | `c02-us4-5-sin-conexion.png`, `c03-us4-5-al-volver.png` |

Script: `t040.mjs`.
