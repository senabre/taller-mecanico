# Quickstart: validar Talleres Rueda usando la aplicación

Esta guía la puede seguir **una persona no técnica** con la app arrancada (principio IV). Cada
sección comprueba una spec. Si un paso no da el resultado esperado, la spec no está terminada.

## Arrancar (lo hace una persona del equipo)

Requisitos: Docker, Rust estable y Node 22.

```bash
cp .env.example .env                       # ajustar valores; ningún secreto en git
docker compose up -d                       # PostgreSQL + Mailpit (correo de pruebas)
cd backend && cargo sqlx migrate run       # crea todas las tablas
psql "$DATABASE_URL" -f seed/demo.sql      # usuarios y piezas de ejemplo
# …o, sin psql instalado, con el psql del contenedor (en lugar de la línea anterior):
# docker compose exec -T postgres sh -c 'psql -v ON_ERROR_STOP=1 -U "$POSTGRES_USER" -d "$POSTGRES_DB"' < seed/demo.sql
cd ../frontend && npm ci && npm run build  # compila la SPA
cd ../backend && cargo run                 # app en http://localhost:3000
```

- Pruebas automáticas de reglas de negocio: `cd backend && cargo test`.
- Correos enviados: **http://localhost:8025** (Mailpit).
- Configuración demo (`.env.example`): cupo diario **2**, precio hora **40,00 €**, IVA **21 %**.
- Usuarios demo (contraseña `demo1234`, ficticia y solo para demostración; constitución, principio V): `ana` (recepción), `luis` y `marta` (mecánicos), `pepe`
  (almacén), `paco` (jefe). `pedro` es un mecánico **de baja**: no puede entrar ni se le pueden
  asignar órdenes (spec-002 Q3).
- Citas demo de **hoy** (spec-002 Q2: la orden solo se abre el día de la cita): `1313MMM`,
  `1414NNN` y `8888KKK`; y una de **ayer** sin orden, `1515PPP`. Por eso los datos demo se cargan
  el mismo día en que se recorre el quickstart.
- Pieza demo: `PF-123 Pastillas freno`, 25,00 €, stock 4. (`BJ-7 Bujía`, stock 0, solo está para el
  pedido demo pendiente de la orden en espera de piezas.)
- Presupuesto demo emitido (token ficticio, solo para demostración; constitución, principio V):
  `/presupuesto/demo-presupuesto-emitido`. Ampliación demo caducada de la orden 7, también con token
  ficticio: `/presupuesto/demo-presupuesto-caducado`.
- Proveedores demo (precargados): «Recambios Sur» y «Autopartes Norte». En producción, los
  proveedores reales los inserta el equipo con `psql` (la app no los gestiona).

## §0 Base común
1. Ir a `/entrar` y entrar como `luis` → ve «Mis órdenes» e «Historial» y no ve Citas, Almacén,
   Pedidos ni Tablero.
2. Entrar con contraseña errónea → «Usuario o contraseña incorrectos».
3. Entrar como `paco` → ve todo el menú, incluido el Tablero.
4. Entrar como `ana` → ve Citas, Órdenes e Historial; como `pepe` → ve Almacén, Pedidos e
   Historial. Ninguno de los dos ve el Tablero.

## §1 spec-001 · Pedir cita
1. Sin entrar, abrir `/cita` y escribir `12#4` → «La matrícula no es válida» y no deja confirmar.
   Lo mismo con `A` y con `ABCDE123456`; en cambio, `AB` sí se acepta (pide los datos del
   vehículo nuevo).
2. Escribir `1234-bcd` → pide marca, modelo, nombre, teléfono y correo.
3. Rellenar con teléfono `12345`, elegir el primer día disponible, motivo «Ruido al frenar» y
   Confirmar → «El teléfono no es válido». Cambiarlo por `612-345-678` y poner el correo
   `ana@correo` → «El correo no es válido». Cambiarlo por `ana@correo.es` y Confirmar → se ven la
   fecha (dd/mm/aaaa) y el motivo.
4. Volver a `/cita` y escribir `1234BCD` → aparecen la marca y el modelo, sin datos del cliente,
   y el aviso «Este vehículo ya tiene una cita pendiente.», sin fecha. No deja confirmar.
5. Pedir cita con la matrícula nueva `5678CDF` para el **mismo día** del paso 3 (teléfono
   `+34 612 345 678`) → se confirma. Al volver a `/cita`, ese día **ya no aparece** (cupo 2).
6. Escribir `1111BBB` (vehículo demo cuya última cita ya pasó) → aparece «Seat Ibiza» y solo pide
   día y motivo, sin campos de contacto ni avisos. Elegir otro día, motivo «Cambio de aceite» y
   Confirmar → se confirma.
7. Entrar como `ana` → «Citas» abre con las citas de hoy. Elegir el día del paso 3 → aparecen
   1234BCD y después 5678CDF, con marca, modelo, motivo, nombre y teléfono, y sin correo. En el
   día del paso 6, 1111BBB aparece con «María López» y 600111222: sus datos no han cambiado.
8. Elegir un día sin citas → «No hay citas para este día».
9. Entrar como `luis` → no ve «Citas» en el menú; si escribe `/citas`, no ve ninguna lista. Como
   `paco` → ve la lista igual que `ana`.
10. Cronometrar dos citas nuevas, desde abrir `/cita` hasta ver la confirmación, eligiendo días
    que aún tengan plazas:
    - con el vehículo demo `2222CCC` (conocido y sin cita pendiente): **menos de 1 minuto**
      (spec-001 SC-001);
    - con la matrícula nueva `9012DFG`, escribiendo bien todos los datos: **menos de 3 minutos**
      (SC-002).

La concurrencia (dos clientes a la vez por la última plaza o con la misma matrícula) la comprueban
las pruebas automáticas (`cargo test`).

## §2 spec-002 · Abrir orden
1. Como `ana`, «Citas» abre con hoy → `1313MMM`, `1414NNN` y `8888KKK` tienen «Abrir orden».
   Elegir el día de la cita de 1234BCD (§1.3) → no tiene «Abrir orden»; elegir ayer → `1515PPP`
   tampoco (la orden solo se abre el día de la cita). Un cliente que llega hoy sin cita no está en
   la lista y la página pública solo le ofrece días desde mañana (§1).
2. «Abrir orden» en `1313MMM` → se ven su matrícula, marca, modelo y motivo. El selector de
   mecánicos ofrece a Luis y a Marta, y no a Pedro (de baja) ni a otros usuarios.
3. Guardar con los daños vacíos → «Describe los daños visibles o escribe “Sin daños visibles”»;
   con «-5» km → «El kilometraje debe ser un número entero igual o mayor que 0»; con la entrega de
   ayer → «La fecha de entrega prometida no puede ser anterior a hoy».
4. Guardar con 85.000 km, «Rayón puerta trasera izquierda», mecánico Luis y entrega dentro de 3
   días → la ficha muestra la orden **Nº 10** en estado «Recibida», con la cita, la matrícula y el
   mecánico (los errores del paso 3 no han gastado ningún número). Cronometrar desde la lista de
   citas hasta ver la ficha: **menos de 2 minutos** (spec-002 SC-001). En «Citas», `1313MMM` muestra
   «Orden Nº 10», que lleva a la ficha.
5. Abrir el formulario de `1414NNN` en **dos pestañas**. En la primera, guardar con 23.500 km,
   «Sin daños visibles», mecánico Luis y entrega dentro de 2 días → orden **Nº 11** (consecutiva).
   En la segunda, guardar → «Esta cita ya tiene una orden (Nº 11)», con un enlace a esa orden; en
   «Órdenes» solo hay una orden de `1414NNN`.
6. «Abrir orden» en `8888KKK` con 30.000 km → se rechaza: «El kilometraje no puede ser menor que el
   de la visita anterior (39.800 km)».
7. Como `luis`, «Mis órdenes» → aparecen la 10 y la 11.

## §3 spec-003 · Diagnóstico y presupuesto
1. Como `luis`, abrir la orden de 1313MMM, diagnóstico «Pastillas delanteras desgastadas» → «En
   diagnóstico». Cambiarlo a «Pastillas y discos delanteros desgastados» → se guarda y sigue «En
   diagnóstico».
2. Nuevo presupuesto: mano de obra «Cambio de pastillas» 1,5 h → 60,00 €; pieza PF-123 × 2 →
   «PF-123 Pastillas freno», 50,00 €.
3. Comprobar: base **110,00 €**, IVA 21 % **23,10 €**, total **133,10 €**.
4. Intentar añadir 1,3 h o 2,5 unidades de una pieza → se rechaza explicando el motivo.
5. Añadir una línea «Prueba» de 1 h y quitarla → los totales vuelven a 110,00 / 23,10 / 133,10.
6. Emitir y confirmar con «Sí, emitir» → «Válido hasta» = hoy + 15 días; ya no se pueden añadir ni quitar líneas, y en
   Mailpit llega el correo con el enlace.
7. El diagnóstico ya no se puede cambiar: se ve sin «Guardar diagnóstico» y con el aviso «Ya se ha
   emitido un presupuesto para esta orden; el diagnóstico no se puede cambiar.»

## §4 spec-004 · Respuesta del cliente
1. En Mailpit, abrir el correo del presupuesto y pulsar el enlace → se ven el vehículo, el
   diagnóstico, las líneas, la base, el IVA, el total y la caducidad, sin el nombre, el teléfono ni
   el correo del cliente. Con el ancho de un móvil (390 px) se lee sin desplazar la página de lado.
2. Pulsar «Aceptar» → «¿Acepta este presupuesto por 133,10 €? Una vez confirmado, no se podrá
   cambiar.»; «Volver» deja la página como estaba. Aceptar otra vez y pulsar «Sí, aceptar» → se ve
   la fecha y la hora. **Cronometrar** desde abrir el enlace hasta ver ese mensaje: **menos de 1
   minuto** (spec-004 SC-001). Volver a abrir el enlace → ya no hay botones.
3. Como `luis`, la orden de 1313MMM está «En reparación» con «Aceptado el … a las …», y ya no se ve
   el enlace.
4. Como `ana`, en la orden demo 5 (presupuesto sin responder) se ven el enlace y «Copiar enlace», que
   lo copia. Como `marta`, su mecánica, comprobar que en esa orden **no** aparece la opción de imputar.
5. (Rechazo) Emitir un presupuesto en la orden de 1414NNN y rechazarlo desde el enlace → la orden sigue
   «En diagnóstico», con «Rechazado el … a las …».
6. (Enlace de un caducado) Como `luis`, abrir la orden demo 7 («Pendiente de conformidad») → su
   ampliación sale «Caducado el …», y **no** aparecen el enlace ni «Copiar enlace».
7. (Ampliación rechazada) Como `luis`, en la orden demo 7 («Pendiente de conformidad») crear una
   ampliación de 1 h y emitirla. Abrir el enlace de Mailpit en **dos pestañas**. Rechazarla en la
   primera → la orden sigue «Pendiente de conformidad» y muestra las dos respuestas: «Inicial ·
   Aceptado el …» y «Ampliación · Rechazado el …». En la segunda, pulsar «Aceptar» y confirmar →
   «Este presupuesto ya fue respondido», y la orden no cambia.
8. (Caducado) Abrir `/presupuesto/demo-presupuesto-caducado` → «Este presupuesto caducó el dd/mm/aaaa.
   Contacte con el taller», sin botones.
9. (Enlace modificado) Abrir `/presupuesto/enlace-inventado` → solo «Este enlace no es válido».

## §5 spec-005 · Imputar
1. Orden de 1313MMM, sin imputar nada: no aparece «Terminar reparación» y se ve «Primero hay que
   imputar el trabajo realizado.»
2. Imputar PF-123 × 2 → concepto «PF-123 Pastillas freno»; en `/almacen` el stock de PF-123 pasa
   de 4 a **2**. Intentar imputar PF-123 × 3 → «Solo hay 2 unidades en stock de PF-123…».
3. Imputar «Cambio de pastillas» 1 h → «Acumulado 90,00 € de 110,00 €».
4. Imputar 1 h más → acumulado 130,00 € (> 121,00 €) → la orden pasa a «Pendiente de
   conformidad» y ya no deja imputar ni terminar.
5. Crear una ampliación de 1 h (40,00 €), emitirla y aceptarla desde Mailpit → la orden vuelve a
   «En reparación» con 150,00 € presupuestados.
6. Marcar como terminada y confirmar con «Sí, terminar» → «Terminada» y ya no deja imputar.

## §6 spec-006 · Almacén y pedidos
1. Como `pepe`, en `/almacen` dar de alta `FA-9 Filtro aceite`, 12,00 €, **sin** indicar stock →
   aparece con stock 0. Intentar darla de alta otra vez → «Ya existe la referencia FA-9».
2. En `/pedidos`, el desplegable de proveedores ya trae «Recambios Sur» (precargado; no hay
   opción de crear proveedores).
3. Con una orden «En reparación» (nº **R**), registrar el pedido Recambios Sur, FA-9, 1 ud, orden R
   → el pedido sale «Pendiente» con la fecha de hoy y la orden R pasa a «En espera de piezas».
4. Registrar otra vez FA-9 para la orden R → «Ya hay un pedido pendiente de FA-9 para la orden R».
5. Registrar PF-123, 1 ud, para la orden R → se admite y la orden sigue en espera.
6. Registrar un pedido para una orden «En diagnóstico» → se rechaza explicando el motivo.
7. Como `luis`, abrir la orden R → ve los dos pedidos pendientes y **no** puede imputar ni terminar.
8. Como `pepe`, marcar recibido el pedido de FA-9 → stock de FA-9 = 1 y la orden R **sigue** en
   espera. El botón «Marcar recibido» ya no aparece en ese pedido.
9. Marcar recibido el de PF-123 → la orden R vuelve sola a «En reparación» (también en el tablero
   de `paco`), sin que nadie la cambie.

## §7 spec-007 · Factura e historial
1. Como `ana`, en una orden «En reparación» → **no** aparece «Facturar».
2. En la orden de 1313MMM («Terminada») pulsar Facturar y confirmar con «Sí, facturar» → «Factura nº N» con su número correlativo, las líneas
   imputadas, base, IVA y total que cuadran con la calculadora. Sin escribir nada.
3. Pulsar «Ver e imprimir» y después «Imprimir» → la vista previa de impresión muestra solo la factura (sin menú),
   y se puede guardar como PDF. Sin `TALLER_LOGO` en el `.env` sale sin logo; con `TALLER_LOGO` apuntando a una
   imagen y la app reiniciada, el logo sale en la cabecera.
4. En Mailpit llega «Su vehículo 1313MMM está listo para recoger», con el total.
5. En `/historial`, buscar `1313 mmm` → aparece la reparación con fecha, km, diagnóstico y líneas. Buscar `9999LLL`
   (datos demo) → aparece su reparación facturada; buscar una matrícula que no existe → «No hay historial para esa
   matrícula».

## §8 spec-008 · Tablero
Se recorre después del §7, con los datos demo cargados ese mismo día. Hacen falta **dos
navegadores** (o una ventana normal y otra privada, porque la sesión va en una cookie) y una persona
con un reloj para los pasos 4 y 6 (SC-001 y SC-002). La correspondencia con la spec está en
[spec-008 quickstart](../008-tablero-ordenes-abiertas/quickstart.md).

1. Como `ana`, `luis` y `pepe`: ninguno ve «Tablero» en el menú. Escribir `/tablero` a mano → «No
   tienes permiso para hacer esto», sin columnas ni tarjetas.
2. Como `paco`, abrir `/tablero` → 6 columnas en este orden: Recibida, En diagnóstico, En
   reparación, En espera de piezas, Pendiente de conformidad y Terminada. `9999LLL` y `1313MMM`
   (facturadas) **no** aparecen. Con los datos demo ninguna columna queda vacía; la columna vacía
   con «Sin órdenes» la comprueban las pruebas automáticas.
3. Arriba, la carga «Luis 6 · Marta 3» (con los datos recién cargados, sin el §1–§7: «Luis 5 ·
   Marta 3»). Contar las tarjetas de cada uno → coinciden. Pedro (de baja, sin órdenes) no sale.
4. **Cronometrar** (SC-001): una persona que no haya visto el tablero dice qué órdenes van con
   retraso, desde que se ve la página → en **menos de 10 s** nombra `8888KKK` (Terminada, entrega
   ayer, con borde destacado y «Retrasada»), y **no** `3333DDD` (entrega hoy, sin «Retrasada»).
5. En «En reparación», `3333DDD` (entrega hoy) está **encima** de `2222CCC` (entrega pasado mañana):
   las tarjetas van por fecha de entrega, no por fecha de apertura.
6. **Cronometrar** (SC-002), sin tocar el navegador de `paco`. En el otro, como `luis`, abrir la orden
   de `2222CCC` y pulsar «Terminar reparación» → en **menos de 5 s** la tarjeta pasa a «Terminada»,
   y la carga de Luis no cambia.
7. En el otro navegador, como `ana`, «Abrir orden» en `8888KKK` (cita de hoy) con 40.000 km,
   «Sin daños visibles», mecánica Marta y entrega mañana → en menos de 5 s aparece su tarjeta en
   «Recibida» y la carga pasa a «Marta 4».
8. Como `ana`, en la orden de `2222CCC` («Terminada»), pulsar «Facturar» → en menos de 5 s su
   tarjeta desaparece y la carga pasa a «Luis 5».
9. Parar el servidor (Ctrl+C en la terminal de `cargo run`) → en el navegador de `paco` aparece
   «Sin conexión, reintentando…». Volver a arrancarlo (`cargo run`) → en unos segundos el aviso
   desaparece solo, sin recargar, y el tablero sigue al día. Después, como `luis`, terminar la
   orden de `3333DDD` → con el tablero ya conectado, la tarjeta pasa sola a «Terminada». (Que lo que
   cambie mientras el tablero está sin conexión aparezca al volver lo comprueban las pruebas
   automáticas; cortar la red desde las herramientas de desarrollo no sirve, porque no corta una
   conexión de eventos ya abierta: V1 de spec-008.)
10. En un **móvil** conectado a la misma red, abrir `http://<IP del ordenador>:3000/tablero` como
    `paco` (o, con las herramientas de desarrollo, a **390 px** de ancho): se ve una columna entera y
    el borde de la siguiente; las columnas se desplazan con el dedo en horizontal **dentro del tablero**, la
    página no se mueve de lado, y la carga salta de línea si no cabe. A 1280 px o más, las 6
    columnas caben una al lado de otra y ocupan todo el ancho.
11. El paso de medianoche no se comprueba a mano: lo cubre `npm test` en `frontend/`
    (`medianoche.test.ts`).

## §9 Integración (los dos juntos)
Un vehículo nuevo, `3579KLM`, recorre todo el flujo, desde la cita hasta la factura, con el tablero
de `paco` abierto en otra ventana. Se hace después del §8 y ocupa **dos sesiones**: el día A, el del
recorrido §0–§8, y el día B, el de la cita. La orden solo se abre el día de la cita (spec-002 Q2), y
la página pública solo ofrece días laborables desde mañana con plaza: después del §1, el primer día
libre **puede no ser el siguiente** (el §1 llena alguno, y no hay citas en fin de semana).

**Día A**, al terminar el §8:
1. Sin entrar, en `/cita`, escribir `3579KLM` → pide los datos del vehículo y del cliente. Rellenar
   marca «Seat», modelo «Arona», un nombre, un teléfono válido y un correo, motivo «Revisión de frenos», y elegir
   el **primer día que ofrezca la página** → se confirma. Anotar ese día: es el día B. El tablero no
   cambia (las citas no salen en él).

**Día B** (el de la cita). **No se recargan los datos demo**: el `demo.sql` se carga sobre una base
vacía y borraría la cita del día A. Por eso los datos demo están desfasados, y **es normal que
alguna orden demo salga «Retrasada»** en el tablero y en la lista de órdenes: por ejemplo
`3333DDD`, cuya entrega era el día A, y cualquiera cuya entrega ya haya pasado. Lo que se comprueba
es la orden nueva.
2. Como `paco`, abrir `/tablero` y dejarlo abierto. La carga coincide con las tarjetas de cada
   mecánico.
3. Como `ana`, en «Citas» de hoy aparece `3579KLM` con «Abrir orden». Abrirla con 15.000 km, «Sin
   daños visibles», mecánico Luis y entrega dentro de 3 días → orden **Nº 13** (la siguiente a la
   Nº 12 del §8.7). En menos de 5 s su tarjeta aparece en «Recibida» y Luis suma 1.
4. Como `luis`, en esa orden, diagnóstico «Pastillas delanteras desgastadas» → la tarjeta pasa a
   «En diagnóstico». Crear el presupuesto con una línea de mano de obra «Cambio de pastillas», 1 h,
   y emitirlo → la tarjeta **no** se mueve (emitir no cambia el estado). En Mailpit llega el correo
   con el enlace.
5. Abrir el enlace del correo, aceptar y confirmar → «Aceptado el … a las …», y la tarjeta pasa a
   «En reparación».
6. Como `luis`, imputar 1 h de «Cambio de pastillas» (lo presupuestado, sin pasar del 110 %) y pulsar
   «Terminar reparación» → la tarjeta pasa a «Terminada».
7. Como `ana`, pulsar «Facturar» → «Factura nº N» con su total, y en Mailpit «Su vehículo 3579KLM
   está listo para recoger». La tarjeta desaparece del tablero y Luis resta 1.
8. En `/historial`, buscar `3579KLM` → aparece la reparación con su diagnóstico y sus líneas.

En todo el día B, cada cambio de la orden nueva se ve en el tablero de `paco` en menos de 5 s, sin
recargar.
