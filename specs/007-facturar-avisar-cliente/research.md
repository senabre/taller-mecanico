# Research: spec-007 · Facturar la orden y avisar al cliente

Decisiones propias de spec-007. Lo común (dinero R4, correlativos R5, correo R10, fechas R12,
pruebas R13, transiciones R14) está en [000-base-comun/research.md](../000-base-comun/research.md).
Fecha: 2026-10-29.

## D1. Facturar: una transacción con la orden bloqueada

- **Decisión**: `POST /api/ordenes/{id}/factura` (rol `recepcion`; el jefe pasa siempre):
  1. `SELECT … FROM orden WHERE id = $1 FOR UPDATE` (404 si no existe).
  2. Si no está `terminada` → 409 «La orden está {estado}; solo se puede facturar una orden
     terminada» (una orden ya facturada da «La orden está facturada…»).
  3. Sin imputaciones → 422 «La orden no tiene imputaciones; no se puede facturar».
  4. `numero = contador::siguiente(FACTURA)` dentro de la transacción (R5: si algo falla, el
     número vuelve y no hay huecos).
  5. Totales con `dinero::totales(importes, config.iva_porcentaje)` (R4); `fecha = fechas::hoy()`;
     copia del nombre del cliente y de la matrícula del momento (Q2-A).
  6. `INSERT factura` y una `linea_factura` por imputación, en el orden en que se imputaron.
     Si choca con el `UNIQUE (orden_id)` → 409 «Esta orden ya tiene factura» (red de seguridad).
  7. `cambiar_estado(&[Terminada], Facturada)`; `commit`; evento del tablero después del `commit`
     (R14; la orden facturada sale del tablero).
- **Motivo**: FR-001, FR-002, FR-004, FR-005, FR-008 y el caso «se pulsa Facturar dos veces». El
  bloqueo hace que la segunda petición espere y encuentre la orden ya `facturada`.

## D2. Aviso al cliente después de guardar

- **Decisión**: después del `commit`, `correo.enviar(email, "Su vehículo {matricula} está listo
  para recoger", cuerpo)` con el total en formato español; luego
  `UPDATE factura SET aviso_enviado = $resultado`. Si el correo falla, la factura ya está emitida
  y `aviso_enviado` queda `false` (valor por defecto), que la pantalla muestra para que recepción
  llame. No hay reintento desde la app (la spec no lo pide).
- **Formato del total**: `formato::euros(Decimal) -> "157,30 €"` en `backend/src/formato.rs`
  (base común), equivalente al `euros()` de `formato.ts`, con su prueba unitaria. Es la primera vez
  que el backend escribe un importe para una persona (spec-003 lo evitó).
- **Matrícula**: la normalizada que se guarda (`1234BCD`), igual que el quickstart.
- **Motivo**: FR-006, el caso límite del aviso fallido y R10.

## D3. La factura en pantalla y en papel

- **Decisión**: `GET /api/facturas/{id}` (recepción y jefe) devuelve la `Factura` del contrato con
  los datos del taller de la configuración (`TALLER_NOMBRE`, `TALLER_NIF`, `TALLER_DIRECCION`).
  La página `/facturas/[id]` la muestra («Factura nº 12», fecha dd/mm/aaaa, taller, cliente,
  matrícula, líneas y totales) con un botón «Imprimir» que llama a `window.print()`. Una regla
  `@media print` de la propia página oculta el menú y los botones, así que sale solo la factura y
  el navegador permite «Guardar como PDF».
- **Motivo**: aclaración del 29/10 (FR-003). Sin PDF en el servidor ni dependencias nuevas
  (principio I). La regla de impresión vive en la página, sin tocar el layout común.

## D4. Número de factura

- **Decisión**: el `numero` del contador, una sola serie que no se reinicia (aclaración del 29/10),
  mostrado como «Factura nº {numero}». Los datos demo ya dejan el contador en el máximo existente.
- **Motivo**: FR-004.

## D5. Historial por matrícula

- **Decisión**: `GET /api/vehiculos/{matricula}/historial` (cualquier rol con sesión):
  `matricula::normalizar_matricula` (422 «Matrícula no válida» si no se puede); si no hay vehículo
  → 404 «No hay ningún vehículo con la matrícula {m}»; si lo hay → 200 con sus reparaciones
  facturadas `[{orden_numero, fecha, km, diagnostico, factura_numero, lineas, total}]`, de la más
  reciente a la más antigua (`fecha DESC, numero DESC`); sin facturas, `reparaciones: []`. La
  pantalla `/historial` muestra «No hay historial para esa matrícula» en los dos casos.
- **Motivo**: FR-007, US3 y SC-004. Las líneas son las de la factura (lo que se cobró), no las
  imputaciones.

## D6. Líneas de factura en `TablaLineas`

- **Decisión**: `LineaFactura = {concepto, cantidad, precio_unitario, importe}` (como la tabla
  `linea_factura`, sin `tipo`). `TablaLineas` acepta líneas sin `tipo` y entonces muestra la
  cantidad sin «h» ni «uds».
- **Motivo**: data-model (la línea de factura es una copia sin tipo). Añadir `tipo` a
  `linea_factura` obligaría a una migración y a tocar los datos demo para nada que pida la spec.

## D7. Pantallas y punto de contacto con spec-002

- **Decisión**: sección `ordenes/[id]/Factura.svelte` de la ficha de spec-002, solo para
  recepción y jefe: con la orden `terminada`, botón «Facturar» con confirmación; facturada,
  «Factura nº N · dd/mm/aaaa · total», el aviso «No se pudo avisar al cliente por correo: llámale»
  si `aviso_enviado` es `false`, y el enlace «Ver e imprimir» a `/facturas/{id}`. `facturas.rs`
  expone `factura_de_orden(conexion, orden_id) -> Option<i64>` para que spec-002 llene
  `factura_id` de `GET /api/ordenes/{id}`, igual que `pedidos_de_orden`,
  `presupuestos_de_orden` y `resumen_imputaciones`. `/historial` y `/facturas/[id]` son páginas
  propias de esta spec.
- **Motivo**: FR-001 («Facturar» solo en terminadas, US1/AC5) sin editar el módulo de la otra
  persona.

## D8. Logo del taller (opcional)

- **Decisión**: variable opcional `TALLER_LOGO` con la ruta a un archivo de imagen (PNG, JPG,
  SVG o WebP) en el servidor, igual que el resto de datos del taller (R7). Si está puesta y el
  archivo existe, `Factura.taller.logo = "/api/taller/logo"`; si no, `null`. `GET /api/taller/logo`
  (cualquier sesión) devuelve la imagen con su tipo según la extensión, o 404 «No hay logo
  configurado». La factura lo muestra con un `<img>` en la cabecera solo si `logo` no es `null`.
- **Motivo**: FR-009 (petición del 29/10). Sin subir archivos desde la app ni tablas nuevas: el
  logo se cambia como el nombre o el NIF (principio I). Un archivo que falta se trata como «sin
  logo» (caso límite) y queda en el registro del servidor.
- **Alternativas descartadas**: una URL externa (la factura dependería de otro servidor al
  imprimir) y un archivo fijo dentro del repositorio (cada taller tiene el suyo).
