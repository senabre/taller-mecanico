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
  `/presupuesto/demo-presupuesto-emitido`.
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
6. Emitir → «Válido hasta» = hoy + 15 días; ya no se pueden añadir ni quitar líneas, y en
   Mailpit llega el correo con el enlace.
7. Intentar cambiar el diagnóstico → «Ya se ha emitido un presupuesto para esta orden; el
   diagnóstico no se puede cambiar».

## §4 spec-004 · Respuesta del cliente
1. En Mailpit, abrir el correo del presupuesto y pulsar el enlace → se ve el presupuesto completo.
2. Aceptar y confirmar → se ve la fecha y la hora. Volver a abrir el enlace → ya no hay botones.
3. Como `luis`, la orden de 1313MMM está «En reparación» con «Aceptado el … a las …».
4. Comprobar en otra orden con presupuesto sin responder que **no** aparece la opción de imputar.
5. (Rechazo) Emitir un presupuesto en la orden de 1414NNN y rechazarlo desde el enlace → la orden vuelve a
   «En diagnóstico».

## §5 spec-005 · Imputar
1. Orden de 1313MMM, intentar terminarla sin imputar nada → «Primero hay que imputar el trabajo
   realizado».
2. Imputar PF-123 × 2 → concepto «PF-123 Pastillas freno»; en `/almacen` el stock de PF-123 pasa
   de 4 a **2**. Intentar imputar PF-123 × 3 → «Solo hay 2 unidades en stock de PF-123…».
3. Imputar «Cambio de pastillas» 1 h → «Acumulado 90,00 € de 110,00 €».
4. Imputar 1 h más → acumulado 130,00 € (> 121,00 €) → la orden pasa a «Pendiente de
   conformidad» y ya no deja imputar ni terminar.
5. Crear una ampliación de 1 h (40,00 €), emitirla y aceptarla desde Mailpit → la orden vuelve a
   «En reparación» con 150,00 € presupuestados.
6. Marcar como terminada → «Terminada» y ya no deja imputar.

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
2. En la orden de 1313MMM («Terminada») pulsar Facturar → «Factura nº N» con su número correlativo, las líneas
   imputadas, base, IVA y total que cuadran con la calculadora. Sin escribir nada.
3. Pulsar «Ver e imprimir» y después «Imprimir» → la vista previa de impresión muestra solo la factura (sin menú),
   y se puede guardar como PDF. Sin `TALLER_LOGO` en el `.env` sale sin logo; con `TALLER_LOGO` apuntando a una
   imagen y la app reiniciada, el logo sale en la cabecera.
4. En Mailpit llega «Su vehículo 1313MMM está listo para recoger», con el total.
5. En `/historial`, buscar `1313 mmm` → aparece la reparación con fecha, km, diagnóstico y líneas. Buscar `9999LLL`
   (datos demo) → aparece su reparación facturada; buscar una matrícula que no existe → «No hay historial para esa
   matrícula».

## §8 spec-008 · Tablero
1. Como `paco`, abrir `/tablero` → 6 columnas (vacías incluidas) y la carga «Luis N · Marta N».
2. La orden facturada no aparece.
3. En otro navegador, como `luis`, terminar una orden → en menos de 5 s la tarjeta cambia de
   columna en el tablero de `paco`, sin recargar.
4. Con una orden cuya entrega prometida ya pasó (datos demo) → la tarjeta aparece destacada como
   «Retrasada».

## §9 Integración (los dos juntos)
Recorrer §1 → §8 seguidos con un vehículo nuevo, desde la cita hasta la factura, mirando el
tablero en otra ventana durante todo el recorrido. Como la orden solo se abre el día de la cita
(spec-002 Q2) y la página pública solo ofrece días desde mañana, el recorrido ocupa dos días:
pedir la cita un día y seguir el día de la cita.
