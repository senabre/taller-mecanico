# T032 · «Válido hasta» solo con el presupuesto vigente (hallazgo V1)

**Fecha**: 2026-11-11 · **Responsable**: Persona A · **Origen**: converge-1, F1.

## Cambio

En `frontend/src/routes/ordenes/[id]/Presupuesto.svelte`, una sola condición, `vigente` (emitido y no
caducado, la regla de Q5), decide el enlace y «Válido hasta». Con un presupuesto aceptado o rechazado,
la ficha dice solo «Emitido el dd/mm/aaaa». Con uno caducado sigue «Caducado el dd/mm/aaaa».

## Cómo se comprobó

Chrome sin interfaz, dirigido con Playwright, a 1280 px y a 390 px, con el servidor real y una base de
datos temporal (`taller_verif`), con las migraciones y `backend/seed/demo.sql`. Entra como `paco`. Las
respuestas del cliente van por `POST` al servidor real.

- **Rojo**, antes del cambio: [salida-rojo.txt](salida-rojo.txt), con 16 correctas y 10 fallos. Todos
  los fallos son «Válido hasta» en un presupuesto respondido.
- **Verde**, después del cambio: [salida-verde.txt](salida-verde.txt), con 26 correctas y 0 fallos.

| Caso | Antes | Después |
|---|---|---|
| Emitido y vigente | «Emitido el … · Válido hasta …», con enlace | Igual |
| Rechazado | «Emitido el 11/11/2026 · Válido hasta 26/11/2026» | «Emitido el 11/11/2026» |
| Aceptado | Con «Válido hasta» | Sin «Válido hasta» |
| Ampliación caducada | «Caducado el …» | Igual |
| Orden facturada (demo 9) | Con «Válido hasta» | Sin «Válido hasta» |

Capturas, en escritorio y en móvil: `antes-*-rechazado.png` y `despues-*-rechazado.png`.

## La ficha con la sección Factura de `main`

Tras fusionar `main`, que añade la sección Factura (T031 de spec-002), se abrió la orden demo 9,
facturada. En los dos anchos, las secciones salen en este orden: Datos, Diagnóstico, Presupuesto,
Imputaciones, Pedidos y Factura. La sección Presupuesto tiene su respuesta, no hay desplazamiento
horizontal y no hay errores de JavaScript. Capturas: `antes-*-ficha-facturada.png` y
`despues-*-ficha-facturada.png`.
