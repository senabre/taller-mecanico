# Verificación en el navegador · spec-001 (T032 y T034)

**Fecha**: 2026-10-29 · **Commit verificado**: `2cdb35a` (rama `spec-001`) · **Responsable**: Persona A · **Revisor**: Persona B (pendiente)

## Cómo se hizo

Recorrido del [quickstart común §1](../../../000-base-comun/quickstart.md) (pasos 1–10), de los pasos
añadidos a T032 (a)–(e) y de la parte de T034 que no depende de spec-002, ejecutado con un
**navegador automatizado** (Chromium sin interfaz, Playwright 1.56) que sigue los mismos pasos que una
persona: escribe en los campos, elige en los selectores, pulsa los botones y lee la pantalla.

- Entorno: la aplicación compilada del commit indicado (`npm run build` + backend Rust), PostgreSQL 16
  con las migraciones 0001–0003 y la base de datos **recién cargada con `backend/seed/demo.sql`**,
  `.env.example` con cupo diario 2.
- Fecha del sistema: jueves 29/10/2026 (Europe/Madrid). Días ofrecidos al empezar: 30/10, 02/11,
  03/11, 04/11, 05/11, 06/11…
- Escritorio a 1280×900; `/cita` también a 390 px de ancho (móvil).
- Guion completo y reproducible: [t032.mjs](t032.mjs) (`node t032.mjs <carpeta>`; con `cupo0` como
  segundo argumento hace el paso (c)).

## Resultado

**31/31 comprobaciones superadas** con el cupo normal. El paso **(c) no se superó** en esta
primera pasada (ver [hallazgo H1](#hallazgo-h1--con-cupo-0-se-ofrecen-días-que-el-servidor-rechaza));
tras corregirlo en T041 se repitió y **se supera** (ver [Repetición tras T041](#repetición-tras-t041-2026-11-02)).

| # | Paso | Esperado | Observado | ✓ | Captura |
|---|------|----------|-----------|---|---------|
| 1 | §1.1 Matrícula «12#4» | «La matrícula no es válida» y no deja confirmar | error=true, confirmar activo=false | ✅ | [01-p1-matricula-124.png](01-p1-matricula-124.png) |
| 2 | §1.1 Matrícula «A» | «La matrícula no es válida» y no deja confirmar | error=true, confirmar activo=false | ✅ | [02-p1-matricula-A.png](02-p1-matricula-A.png) |
| 3 | §1.1 Matrícula «ABCDE123456» | «La matrícula no es válida» y no deja confirmar | error=true, confirmar activo=false | ✅ | [03-p1-matricula-ABCDE123456.png](03-p1-matricula-ABCDE123456.png) |
| 4 | §1.1 Matrícula «AB» | Se acepta y pide los datos del vehículo nuevo | sin error=true, pide datos=true | ✅ | [04-p1-matricula-AB.png](04-p1-matricula-AB.png) |
| 5 | §1.2 «1234-bcd» (US2-1) | Pide marca, modelo, nombre, teléfono y correo | campos=1,1,1,1,1 | ✅ | [05-p2-vehiculo-nuevo.png](05-p2-vehiculo-nuevo.png) |
| 6 | §1.3 Teléfono «12345» (US2-3) | «El teléfono no es válido» y no se confirma | error=true | ✅ | [06-p3-telefono-no-valido.png](06-p3-telefono-no-valido.png) |
| 7 | §1.3 Correo «ana@correo» (US2-3) | «El correo no es válido» y no se confirma | error=true | ✅ | [07-p3-correo-no-valido.png](07-p3-correo-no-valido.png) |
| 8 | §1.3 Alta y confirmación (US2-2, US1-2) | Confirmación con 30/10/2026 y el motivo | confirmada con fecha dd/mm/aaaa y motivo | ✅ | [08-p3-cita-confirmada.png](08-p3-cita-confirmada.png) |
| 9 | §1.4 «1234BCD» con cita pendiente (US1-1, US1-4) | Marca y modelo, aviso sin fecha, sin datos del cliente, no deja confirmar | marca/modelo=true, aviso=true, confirmar activo=false | ✅ | [09-p4-cita-pendiente.png](09-p4-cita-pendiente.png) |
| 10 | §1.5 «+34 612 345 678» (US2-4) | Se confirma | confirmada | ✅ | [10-p5-telefono-internacional.png](10-p5-telefono-internacional.png) |
| 11 | §1.5 / (a) Día lleno (US3-1, SC-003) | 30/10/2026 ya no aparece y 02/11/2026 sí | primer día ofrecido=02/11/2026 | ✅ | [11-p5-a-dia-lleno-no-aparece.png](11-p5-a-dia-lleno-no-aparece.png) |
| 12 | §1.6 «1111BBB» (US1-1, US1-7) | «Seat Ibiza», solo día y motivo, sin avisos | Seat Ibiza=true, campos de contacto=0, aviso=false | ✅ | [12-p6-vehiculo-conocido.png](12-p6-vehiculo-conocido.png) |
| 13 | §1.6 Cita de 1111BBB (US1-2, US1-6) | Se confirma | confirmada para 02/11/2026 | ✅ | [13-p6-confirmada.png](13-p6-confirmada.png) |
| 14 | /cita en móvil (390 px) | Usable, sin desplazamiento horizontal | ancho del documento=390px | ✅ | [14-movil-cita.png](14-movil-cita.png) |
| 15 | §1.7 «Citas» abre con hoy (US1-8) | Título «Citas de hoy» y selector con la fecha de hoy | título=Citas de hoy, selector=2026-10-29 | ✅ | [15-p7-ana-hoy.png](15-p7-ana-hoy.png) |
| 16 | §1.7 Lista del día (US1-3, US1-8, FR-009) | 1234BCD y después 5678CDF, con nombre y teléfono, sin correo | filas=1234BCD, 5678CDF; columnas=Matrícula | Marca | Modelo | Motivo | Nombre | Teléfono | ✅ | [16-p7-ana-dia-1.png](16-p7-ana-dia-1.png) |
| 17 | §1.7 1111BBB con sus datos sin cambiar (US1-7, FR-003) | «María López» y 600111222 | 1111BBB · Seat · Ibiza · Cambio de aceite · María López · 600111222 | ✅ | [17-p7-ana-dia-2.png](17-p7-ana-dia-2.png) |
| 18 | §1.8 Día sin citas (US1-9) | «No hay citas para este día» | mensaje visible | ✅ | [18-p8-dia-sin-citas.png](18-p8-dia-sin-citas.png) |
| 19 | §1.9 luis (US1-10) | Sin «Citas» en el menú; en /citas solo «No tienes permiso para hacer esto» | menú con Citas=false, mensaje=true, tabla=0 | ✅ | [19-p9-luis.png](19-p9-luis.png) |
| 20 | §1.9 pepe (US1-10) | Sin «Citas» en el menú; en /citas solo «No tienes permiso para hacer esto» | menú con Citas=false, mensaje=true, tabla=0 | ✅ | [20-p9-pepe.png](20-p9-pepe.png) |
| 21 | §1.9 Sin sesión (US1-10) | Lleva a /entrar | url=/entrar | ✅ | [21-p9-sin-sesion.png](21-p9-sin-sesion.png) |
| 22 | §1.9 paco (jefe) | Ve la misma lista que ana | filas=1234BCD, 5678CDF | ✅ | [22-p9-paco.png](22-p9-paco.png) |
| 23 | §1.10 SC-001 vehículo conocido | Recorrido con solo matrícula, día y motivo (3 datos) | confirmada; 3 datos escritos; 1.3 s de máquina | ✅ | [23-p10-sc001-conocido.png](23-p10-sc001-conocido.png) |
| 24 | §1.10 SC-002 vehículo nuevo | Recorrido con 8 datos | confirmada; 8 datos escritos; 1.4 s de máquina | ✅ | [24-p10-sc002-nuevo.png](24-p10-sc002-nuevo.png) |
| 25 | (b) US3-2 última plaza en dos pestañas | A se confirma; B ve «Ese día se acaba de completar; elige otro», selector vacío, día retirado y Confirmar desactivado | A=true, aviso B=true, selector=Elige un día, día sigue=false, confirmar activo=false | ✅ | [25-b-pestana-A-confirmada.png](25-b-pestana-A-confirmada.png), [26-b-pestana-B-dia-completado.png](26-b-pestana-B-dia-completado.png) |
| 26 | (b) US3-2 B elige otro día | Se confirma | confirmada para 04/11/2026 | ✅ | [27-b-pestana-B-otro-dia.png](27-b-pestana-B-otro-dia.png) |
| 27 | (d) T035 misma matrícula nueva en dos pestañas (FR-010) | En A: marca y modelo de B, aviso, campos ocultos y Confirmar desactivado | Skoda Octavia=true, aviso=true, campos nuevos=0, confirmar activo=false | ✅ | [28-d-t035-pestana-A.png](28-d-t035-pestana-A.png) |
| 28 | (d) T035 en /citas | Una sola cita de 7777XYZ, con el nombre de B | 7777XYZ · Skoda · Octavia · Prueba T035 · Pestaña B · 611000222 | ✅ | [29-d-t035-lista.png](29-d-t035-lista.png) |
| 29 | (e) T037 respuesta tardía en /citas (FR-009) | Con la respuesta de 02/11/2026 retrasada 1,5 s, la tabla sigue siendo la de 30/10/2026 | título=Citas del 30/10/2026; filas=1234BCD, 5678CDF | ✅ | [30-e-t037-respuesta-tardia.png](30-e-t037-respuesta-tardia.png) |
| 30 | (e) T037 cambios rápidos y «Hoy» | La tabla corresponde al día del título | título=Citas de hoy; sin citas=true | ✅ | [31-e-t037-cambios-rapidos-y-hoy.png](31-e-t037-cambios-rapidos-y-hoy.png) |
| 31 | T034 «9999LLL» (§2.3, parte de spec-001) | «Opel Corsa», sin aviso, se confirma y aparece en /citas | Opel Corsa=true, aviso=false, confirmada=true, en /citas=true | ✅ | [32-t034-9999LLL.png](32-t034-9999LLL.png), [33-t034-9999LLL-confirmada.png](33-t034-9999LLL-confirmada.png), [34-t034-9999LLL-en-citas.png](34-t034-9999LLL-en-citas.png) |
| 32 | (c) «No quedan fechas libres» con `TALLER_CUPO_DIARIO=0` | En lugar del selector, «No quedan fechas libres en los próximos 30 días. Llama al taller para pedir cita.» y sin poder confirmar | El selector **ofrece** todos los días sin citas (09/11–27/11). Al confirmar, el servidor responde 409 «Ese día ya está completo» y la pantalla muestra «Ese día se acaba de completar; elige otro»; el día sigue en la lista | ❌ | [c1-cupo0-ofrece-dias.png](c1-cupo0-ofrece-dias.png), [c2-cupo0-al-confirmar.png](c2-cupo0-al-confirmar.png) |

### Sobre SC-001 y SC-002 (paso 10)

Los tiempos medidos (≈1,3 s) son **de máquina** y no sirven para validar SC-001 (< 1 min) ni
SC-002 (< 3 min), que hablan de una persona. Lo que sí queda comprobado es el recorrido: un vehículo
conocido solo necesita **3 datos** (matrícula, día y motivo) y uno nuevo **8**.

**Cronometraje humano (2026-11-04)**, hecho a mano por la responsable, en Chrome en español y con los
datos demo del día:

| Criterio | Recorrido | Límite | Tiempo | ✓ |
|----------|-----------|--------|--------|---|
| SC-001 | Cita de `2222CCC` (vehículo conocido) | < 1 min | **47 s** | ✅ |
| SC-002 | Cita de `9012DFG` (vehículo nuevo, 8 datos) | < 3 min | **1 min 27 s** | ✅ |

### Sobre T037 (paso e)

Con el navegador automático sí se pudo provocar el caso de F3: se retrasó 1,5 s la respuesta de
`GET /api/citas?fecha=02/11` y, mientras tanto, se eligió el 30/10. Al llegar la respuesta tardía, la
tabla siguió mostrando las citas del 30/10 (1234BCD y 5678CDF): la respuesta antigua se descartó.

## Hallazgo H1 · Con cupo 0 se ofrecen días que el servidor rechaza

- **Tipo / gravedad**: comportamiento contradictorio (FR-007) · baja: solo ocurre con
  `TALLER_CUPO_DIARIO=0`; con el cupo de la demo (2) no aparece.
- **Evidencia**: con cupo 0, `GET /api/publico/dias-disponibles` devolvió 09/11–27/11 y
  `POST /api/publico/citas` con `5555GGG` para el 09/11 respondió `409 {"error":"Ese día ya está completo"}`.
- **Causa**: `dias_disponibles` (`backend/src/rutas/citas.rs`) solo marca como completos los días que
  aparecen en `SELECT fecha, count(*) … GROUP BY fecha`, es decir, los que ya tienen alguna cita.
  Un día sin citas nunca se considera lleno, aunque `0 >= cupo`. El `POST` sí cuenta 0 ≥ 0.
- **Consecuencia**: el formulario ofrece días que no se pueden reservar y, tras el 409, los vuelve a
  ofrecer. Además, el procedimiento (c) de T030/T032 suponía que con cupo 0 no se ofrece ningún día.
- **Propuesta**: que `dias_disponibles` trate como completo cualquier día con `citas >= cupo`, contando
  0 para los días sin citas (por ejemplo, si `cupo == 0` no ofrecer ninguno), con una prueba nueva
  «con cupo 0 no se ofrece ningún día» que falle antes del cambio. Después, repetir el paso (c).

## Repetición tras T041 (2026-11-02)

**Commit verificado**: `00e6436` (incluye T041). Mismo entorno y mismo guion, con la base de datos
recién cargada con `demo.sql`. Fecha del sistema: lunes 02/11/2026.

| Paso | Esperado | Antes (`2cdb35a`) | Después (`00e6436`) | Captura |
|------|----------|-------------------|---------------------|---------|
| (c) con `TALLER_CUPO_DIARIO=0` | `dias-disponibles` vacío; «No quedan fechas libres en los próximos 30 días. Llama al taller para pedir cita.» en lugar del selector; Confirmar desactivado; sin que el mensaje aparezca mientras cargan los días | Se ofrecían 09/11–27/11 y el `POST` respondía 409 ❌ | `GET /api/publico/dias-disponibles` → `[]`. Mensaje exacto, sin selector, Confirmar desactivado. Con la respuesta de días retrasada 1,5 s, durante la carga se ve el selector vacío y **no** el mensaje ✅ | [c3-tras-T041-cargando-dias.png](c3-tras-T041-cargando-dias.png), [c4-tras-T041-no-quedan-fechas.png](c4-tras-T041-no-quedan-fechas.png) |
| Regresión: pasos 1–10, (a), (b), (d), (e), móvil y T034 con cupo 2 | Las mismas 31 comprobaciones | 31/31 ✅ | **31/31** ✅ (días ofrecidos desde el 03/11) | — |

H1 queda verificado en el navegador: el paso (c) de T032 y el mensaje de T030 se han visto en pantalla.

## Qué queda pendiente

1. ~~Corregir H1 (nueva tarea correctora) y repetir el paso (c).~~ Hecho: T041 y repetición del 2026-11-02.
2. ~~Que una persona cronometre el paso 10 (SC-001 y SC-002).~~ Hecho el 2026-11-04: ver «Sobre SC-001 y SC-002 (paso 10)».
3. ~~T034, segunda parte: abrir la orden de `9999LLL` con 100.000 km cuando exista spec-002.~~
   Trasladada a spec-002 (2026-11-03): con su Q2 la orden solo se abre el día de la cita, así que
   la regla del km la comprueba el quickstart común §2.6 con la cita demo de hoy de `8888KKK`.

## Aviso para volver a ejecutar el guion

Los datos demo de spec-002 (parte 5 de `backend/seed/demo.sql`, propuesta en su research D8)
añaden citas de **hoy** (`1313MMM`, `1414NNN`, `8888KKK`) y una de ayer (`1515PPP`). Las
comprobaciones registradas aquí se hicieron con los datos demo anteriores y siguen siendo
válidas. Pero si se vuelve a ejecutar [t032.mjs](t032.mjs) con los datos nuevos, hay que ajustar
la comprobación de **«hoy sin citas»** de la fila 30 («Citas de hoy; sin citas=true»), porque la
lista de hoy ya no estará vacía. Conviene revisar también la fila 15, que abre «Citas» con la fecha
de hoy.
4. ~~Revisión de la Persona B (T031).~~ Hecho el 2026-11-02: la Persona B revisó y dio por correctos los cambios compartidos de spec-001 (T031 marcada).
