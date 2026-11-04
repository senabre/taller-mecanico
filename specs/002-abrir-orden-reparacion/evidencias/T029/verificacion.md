# Verificación en el navegador · spec-002, US1 (parte de T029)

**Fecha**: 2026-11-04 · **Código verificado**: rama `spec-002` en `72375e5` + el frontend de T023–T026
todavía sin commit (copiado del equipo de la autora) · **Responsable**: Persona A · **Revisor**: Persona B (pendiente)

## Cómo se hizo

Recorrido del [quickstart común §2](../../../000-base-comun/quickstart.md), pasos 1–5 y 7, y de la
ficha compartida `ordenes/[id]` (T026), con un **navegador automatizado** (Chromium sin interfaz,
Playwright 1.56) que sigue los pasos de una persona: entra con cada usuario, pulsa los botones,
rellena los campos y lee la pantalla.

- Base de datos recién creada con las migraciones 0001–0003 y `backend/seed/demo.sql` **del mismo
  día** (04/11/2026, Europe/Madrid), con la parte 5 de T001: citas de hoy para 1313MMM, 1414NNN y
  8888KKK; cita de ayer sin orden para 1515PPP; `pedro` de baja. Contador de órdenes en 9.
- La cita de 1234BCD del §1.3 se creó antes con `POST /api/publico/citas` para el 05/11/2026.
- **No se hizo el paso 6** (8888KKK con 30.000 km): la regla del kilometraje es T028 (US2), todavía
  sin implementar.
- Guion reproducible: [t029.mjs](t029.mjs) (`node t029.mjs <carpeta> <día de la cita de 1234BCD>`).

## Resultado: 23/23

| # | Paso | Esperado | Observado | ✓ | Captura |
|---|------|----------|-----------|---|---------|
| 1 | §2.1 Citas de hoy | 1313MMM, 1414NNN y 8888KKK con «Abrir orden» | con botón=true,true,true | ✅ | [01-p1-citas-de-hoy.png](01-p1-citas-de-hoy.png) |
| 2 | §2.1 Cita de otro día (1234BCD) | Sin «Abrir orden» | sin botón | ✅ | [02-p1-cita-otro-dia.png](02-p1-cita-otro-dia.png) |
| 3 | §2.1 Ayer (Q2, Q5) | 1515PPP sin botón; 1111BBB con enlace «Orden Nº 1» | 1515PPP botón=false; 1111BBB=Orden Nº 1 | ✅ | [03-p1-ayer.png](03-p1-ayer.png) |
| 4 | §2.1 Abrir por URL una cita de ayer (US1-5) | «Solo se puede abrir la orden el día de la cita» | mensaje visible | ✅ | [04-p1-ayer-por-url.png](04-p1-ayer-por-url.png) |
| 5 | §2.2 Formulario de 1313MMM (US1-1, US1-7) | Matrícula, marca, modelo y motivo; mecánicos Luis y Marta, sin Pedro | 1313MMM=true; mecánicos=Luis, Marta | ✅ | [05-p2-formulario.png](05-p2-formulario.png) |
| 6 | §2.3 Error de danos (US1-8, FR-003, FR-006) | «Describe los daños visibles o escribe “Sin daños visibles”» bajo su campo y no se guarda | mensaje=«Describe los daños visibles o escribe “Sin daños visibles”»; página=/ordenes/nueva | ✅ | [06-p3-danos-vacios.png](06-p3-danos-vacios.png) |
| 7 | §2.3 Error de km (US1-8, FR-003, FR-006) | «El kilometraje debe ser un número entero igual o mayor que 0» bajo su campo y no se guarda | mensaje=«El kilometraje debe ser un número entero igual o mayor que 0»; página=/ordenes/nueva | ✅ | [07-p3-km-negativo.png](07-p3-km-negativo.png) |
| 8 | §2.3 Error de entrega (US1-8, FR-003, FR-006) | «La fecha de entrega prometida no puede ser anterior a hoy» bajo su campo y no se guarda | mensaje=«La fecha de entrega prometida no puede ser anterior a hoy»; página=/ordenes/nueva | ✅ | [08-p3-entrega-ayer.png](08-p3-entrega-ayer.png) |
| 9 | §2.4 Ficha de la orden (US1-2, US1-3, SC-002) | Orden Nº 10, «Recibida», 85.000 km, cita, matrícula y mecánico | Orden Nº 10; Recibida=true; 85.000 km=true; url=/ordenes/10 | ✅ | [09-p4-ficha-orden-10.png](09-p4-ficha-orden-10.png) |
| 10 | §2.4 SC-001 (tiempo de máquina) | Recorrido desde la lista hasta la ficha | 5.1 s de máquina con 3 errores intermedios; el cronometraje humano queda pendiente | ✅ | [09-p4-ficha-orden-10.png](09-p4-ficha-orden-10.png) |
| 11 | §2.4 Enlace en Citas (US1-4, Q5) | 1313MMM muestra «Orden Nº 10» y lleva a la ficha | Orden Nº 10; tras pulsar=/ordenes/10 | ✅ | [10-p4-citas-orden-10.png](10-p4-citas-orden-10.png) |
| 12 | §2.5 Primera pestaña (SC-002, US1-9) | Orden Nº 11 (consecutiva) | Orden Nº 11 | ✅ | [11-p5-pestana-1-orden-11.png](11-p5-pestana-1-orden-11.png) |
| 13 | §2.5 Segunda pestaña (US1-4, FR-008) | «Esta cita ya tiene una orden (Nº 11)» con enlace a la orden | mensaje=true; enlace=/ordenes/11 | ✅ | [12-p5-pestana-2-ya-tiene-orden.png](12-p5-pestana-2-ya-tiene-orden.png) |
| 14 | §2.5 «Órdenes» (FR-008) | Una sola orden de 1414NNN; las recién abiertas (11 y 10) primero, por fecha de apertura | órdenes de 1414NNN=1; números=11,10,1,4,5,2,3,6,8,7,9 | ✅ | [13-p5-ordenes.png](13-p5-ordenes.png) |
| 15 | Lista: orden 8 retrasada | «Retrasada» en la orden 8 | 8 · 8888KKK · Terminada · Marta · 03/11/2026 Retrasada | ✅ | [13-p5-ordenes.png](13-p5-ordenes.png) |
| 16 | §2.7 «Mis órdenes» de luis | Título «Mis órdenes», con la 10 y la 11 y sin órdenes de Marta | título=Mis órdenes; números=11,10,1,4,2,3,7,9; mecánicos=Luis | ✅ | [14-p7-mis-ordenes-luis.png](14-p7-mis-ordenes-luis.png) |
| 17 | Ficha como luis (T026) | Datos, Diagnóstico, Presupuesto, Imputaciones y Pedidos; diagnóstico editable | secciones=Datos, Diagnóstico, Presupuesto, Imputaciones, Pedidos; textarea de diagnóstico=1 | ✅ | [15-ficha-luis.png](15-ficha-luis.png) |
| 18 | Ficha: guardar diagnóstico | El estado pasa a «En diagnóstico» sin recargar | En diagnóstico=true | ✅ | [16-ficha-luis-diagnostico.png](16-ficha-luis-diagnostico.png) |
| 19 | Ficha como ana | Ve las secciones y no puede editar | secciones=5; textarea de diagnóstico=0 | ✅ | [17-ficha-ana.png](17-ficha-ana.png) |
| 20 | Ficha inexistente | «No existe la orden» | No existe la orden | ✅ | [18-ficha-999.png](18-ficha-999.png) |
| 21 | Ficha de la orden demo 2 | Muestra presupuesto e imputaciones | sí | ✅ | [19-ficha-orden-2.png](19-ficha-orden-2.png) |
| 22 | Ficha de la orden demo 6 | Muestra un pedido pendiente | sí | ✅ | [20-ficha-orden-6.png](20-ficha-orden-6.png) |
| 23 | Ficha de la orden demo 8 | Muestra «Retrasada» | sí | ✅ | [21-ficha-orden-8.png](21-ficha-orden-8.png) |

### Notas

- **SC-001**: el tiempo medido es de máquina y no sirve; queda pendiente cronometrarlo una persona
  (menos de 2 minutos desde la lista de citas hasta la ficha).
- **Lista de órdenes**: se ordena por fecha de apertura, como dice el contrato. En los datos demo
  de la base común los números no siguen el orden de las fechas (la orden 1 es de ayer y la 9 del
  15/10), así que la lista no sale ordenada por número. No afecta a ningún criterio; con datos
  reales, número y fecha avanzan juntos. Se anota como observación de los datos demo.
- En la primera pasada fallaron dos comprobaciones por errores del guion (suponía la lista ordenada
  por número y buscaba «€» tras un espacio normal en vez del no separable). Se corrigió el guion y se
  repitió todo sobre una base de datos limpia.

## Pendiente

1. Paso 6 del §2, cuando esté T028 (US2).
2. Cronometraje humano de SC-001.
3. Revisión de la Persona B (T031).
