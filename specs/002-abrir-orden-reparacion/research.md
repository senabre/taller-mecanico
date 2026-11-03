# Research: spec-002 · Recibir el vehículo y abrir la orden de reparación

Decisiones propias de spec-002 después del clarify (sesión 2026-11-02, Q1–Q5; registro en
[decisiones-clarify.md](decisiones-clarify.md)). Todo lo común (pila, autenticación, fechas en
Europe/Madrid, errores, contador sin huecos, eventos del tablero) está en
[000-base-comun/research.md](../000-base-comun/research.md) y no se repite aquí. Fecha:
2026-11-03.

## D1. La orden solo se abre el día de la cita, y lo decide el servidor

- **Decisión**: `POST /api/ordenes` compara la fecha de la cita con `fechas::hoy()` (Europe/Madrid,
  R12). Si no coinciden, responde 409 «Solo se puede abrir la orden el día de la cita» (Q2,
  FR-001). La pantalla `/citas` solo muestra «Abrir orden» en la lista de hoy, pero eso es una
  ayuda: la regla vive en el servidor, así que también se cumple con una pestaña abierta desde
  ayer o con una petición hecha a mano.
- **Por qué 409 y no 422**: los datos de la petición pueden ser correctos; lo que no encaja es el
  momento (la cita es de otro día). Es un conflicto con el estado de la cita, igual que «la cita ya
  tiene orden».
- **Alternativas descartadas**: comprobarlo solo en el frontend, porque una pestaña abierta antes
  de medianoche dejaría abrir la orden de una cita de ayer (FR-001 dice MUST rechazarlo).

## D2. Una cita, una orden, y el número solo se pide al final

- **Decisión**: el `POST` sigue este orden, el mismo criterio que spec-001 (todos los 422 antes
  que cualquier 409):
  1. **Sin transacción**: la cita existe (404 «No existe la cita»); los campos de la tabla de
     validación del contrato común, en su orden (422 con `detalle.campo`), incluido el kilometraje
     anterior (D3).
  2. La cita es de hoy (409, D1).
  3. **Transacción**: `SELECT … FROM cita WHERE id = $1 FOR UPDATE`, que bloquea la fila de esa
     cita hasta el `commit`. Con la cita bloqueada se mira si ya tiene orden: si la tiene, 409
     «Esta cita ya tiene una orden (Nº 42)», con `detalle: {orden_id, numero}`.
  4. Solo entonces `contador::siguiente(tx, contador::ORDEN)` (R5), el `INSERT` de la orden y el
     `commit`.
- **Motivo**: con dos intentos a la vez para la misma cita, el segundo espera al bloqueo de la
  fila, ve la orden del primero y responde 409 sin haber tocado el contador: una orden, un 409 y
  ningún hueco (FR-002, FR-008, Q5). Cualquier 422 o 409 sale antes del contador, así que tampoco
  consume número. Las órdenes de citas distintas no se esperan por la cita, y el bloqueo de fila de
  `contador('orden')` de R5 las numera de una en una.
- **Orden de los bloqueos**: siempre la fila de la cita y después la de `contador('orden')`.
  Ningún otro proceso los toma al revés (spec-007 usa `contador('factura')`, otra fila), así que
  no hay interbloqueos.
- **Por qué un bloqueo de fila y no `pg_advisory_xact_lock`**: el objeto que hay que proteger es
  una fila concreta que ya existe (la cita). `FOR UPDATE` lo dice directamente y no necesita una
  clave nueva en el registro de R6, que es solo para bloqueos consultivos.
- **Alternativas descartadas**: confiar solo en el `UNIQUE (cita_id)` de `orden` y convertir el
  error 23505 en 409, porque el segundo intento llegaría a pedir número (aunque el `rollback` lo
  devolviera) y el error de la base de datos no trae el número de la orden existente, que el
  mensaje necesita; `SERIALIZABLE` con reintentos, por la misma razón que en R6.

## D3. Kilometraje: la visita anterior es la orden más reciente del vehículo para otra cita

- **Decisión**: `SELECT km FROM orden WHERE vehiculo_id = $1 AND cita_id <> $2 ORDER BY abierta_en
  DESC, id DESC LIMIT 1`. Si existe y el km nuevo es menor → 422 «El kilometraje no puede ser menor
  que el de la visita anterior (85.000 km)», con `detalle: {campo: "km", km_anterior: 85000}`. El
  número del mensaje se escribe con `formato::numero` («85.000»). Igual al anterior → se acepta
  (US2-2). Sin órdenes anteriores → cualquier km ≥ 0 (US2-3, Assumptions de la spec).
- **Por qué `cita_id <> $2`**: si otra pestaña ya abrió la orden de esta cita, esa orden no es una
  «visita anterior», es la misma visita. Sin la condición, el segundo intento con menos km daría
  el 422 del kilometraje en lugar del 409 de Q5.
- **Por qué fuera de la transacción**: un vehículo tiene como mucho una cita pendiente (spec-001
  FR-010) y cada cita una orden, así que no puede aparecer otra orden del mismo vehículo entre la
  lectura y el `INSERT`. Leerlo antes deja todos los 422 antes de cualquier 409 (D2).

## D4. Validación de los campos (Q3, Q4 y fechas)

- **Decisión**: la **fuente única** de las reglas y los textos es la tabla de validación de
  spec-002 en el [contrato común](../000-base-comun/contracts/api.md#spec-002--orden-de-reparación).
  Este documento solo explica las decisiones:
  - **Mecánico (Q3)**: se comprueba en el servidor con `SELECT 1 FROM usuario WHERE id = $1 AND rol
    = 'mecanico' AND activo`. Es la misma condición que ya usa `GET /api/mecanicos`, así que la
    lista del formulario y la comprobación no pueden discrepar.
  - **Daños (Q4)**: obligatorios tras `trim`; se guardan tal como se escribieron, sin los espacios
    del principio y del final. Un campo vacío nunca se convierte en «Sin daños visibles».
  - **Kilometraje**: entero de 0 a 2.147.483.647 (`INTEGER` de la tabla). Vacío, negativo, con
    decimales o con letras → el mismo mensaje.
  - **Fecha de entrega prometida**: ≥ `fechas::hoy()`; hoy se acepta (el caso límite solo prohíbe
    «anterior a hoy»). Sin límite superior, porque la spec no lo pide.
- **Carrera aceptada**: si un mecánico se da de baja entre la comprobación y el `INSERT`
  (milisegundos), la orden se abre con él. No se bloquea la fila del usuario porque el escenario 7
  de la US1 (baja mientras el formulario estaba abierto) ya lo cubre la comprobación, y una baja
  no deshace órdenes ya asignadas.

## D5. Abrir una orden publica su evento para el tablero

- **Decisión**: abrir una orden es un `INSERT`, no pasa por `estados::cambiar_estado`, pero
  después del `commit` publica `CambioEstado { id, estado: Estado::Recibida }` con
  `publicar(&estado_app.eventos)` (R11; hallazgo I4 de la base común). Si la transacción falla,
  no se publica nada.
- **Motivo**: el tablero de spec-008 se actualiza solo con eventos; sin este, una orden nueva no
  aparecería hasta recargar.

## D6. `GET /api/ordenes/{id}`: un esqueleto de spec-002 y una línea por sección de B

- **Decisión**: el manejador vive en `backend/src/rutas/ordenes.rs` (spec-002, Persona A) y
  construye el objeto `Orden` del contrato así:
  - **spec-002**: una consulta propia con los campos de `OrdenResumen` (`id`, `numero`,
    `matricula`, `estado`, `mecanico`, `fecha_entrega_prometida`, `retrasada`), `cita: {fecha,
    motivo}`, `vehiculo: {marca, modelo}`, `km`, `danos`, `diagnostico` (columna de la tabla; la
    escribe spec-003) y `factura_id` (`SELECT id FROM factura WHERE orden_id = …`, tabla de la
    migración común; spec-007 no necesita tocarlo).
  - **spec-003**: `presupuestos` = `presupuestos::presupuestos_de_orden(&mut conn, id,
    config.iva_porcentaje)`.
  - **spec-005**: `total_presupuestado`, `coste_acumulado` e `imputaciones` salen de
    `imputaciones::resumen_imputaciones(&mut conn, id)`.
  - **spec-006**: `pedidos` = `almacen::pedidos_de_orden(&mut conn, id)`.
  
  Cada sección de B es **una llamada** a una función de su propio módulo, que ya existe; B nunca
  edita la consulta de spec-002 y spec-002 nunca edita las funciones de B. Si una spec de B
  necesita más datos en la ficha, cambia su función, no `ordenes.rs`.
- **Coordinación con B (revisa la Persona B)**: `ResumenImputaciones` tiene hoy sus campos
  privados. Para copiar tres de ellos al objeto `Orden` sin duplicar claves, `imputaciones.rs`
  tiene que hacer públicos `coste_acumulado`, `total_presupuestado` e `imputaciones` (un cambio de
  visibilidad, sin cambiar lo que devuelve). Se descarta `#[serde(flatten)]` del resumen entero
  porque repetiría `estado` y añadiría `orden_id` al objeto `Orden`.
- **Prueba de spec-006**: al existir el endpoint, se quita el `#[ignore]` de
  `la_ficha_de_la_orden_trae_sus_pedidos` (spec-006, Persona B) y debe pasar sin cambios.
- **Acceso**: todos los roles con sesión (contrato). 404 «No existe la orden».
- **Frontend**: `ordenes/[id]/+page.svelte` (contenedor compartido, plan maestro) carga la orden
  una vez y pasa a cada sección lo que ya espera: `Datos` (spec-002), `Diagnostico` (`orden`),
  `Presupuesto` (`orden`, `presupuestos`), `Imputaciones` (`resumen` construido con `{orden_id:
  orden.id, estado, coste_acumulado, total_presupuestado, imputaciones}`) y `Pedidos` (`pedidos`).
  `onCambio` y `onguardado` vuelven a pedir la orden. Una sección nueva de B es una línea en el
  contenedor.

## D7. La lista de citas enseña la orden de cada cita (Q5)

- **Decisión**: `GET /api/citas` (spec-001, Persona A) añade `orden_numero` (número o `null`)
  junto a `orden_id`, para que `/citas` pueda mostrar «Orden Nº 42» sin otra petición. En `/citas`:
  con `orden_id` → enlace «Orden Nº 42» a `/ordenes/{id}`; sin orden y con el día de hoy elegido
  → botón «Abrir orden» a `/ordenes/nueva?cita={id}`; otro día → nada.
- **Formulario** `/ordenes/nueva?cita=…`: muestra la cita (matrícula, marca, modelo y motivo, de
  `GET /api/citas?fecha=<hoy>`), y los campos Kilómetros, Daños visibles, Mecánico (de `GET
  /api/mecanicos`) y Fecha de entrega prometida. Usa `novalidate`, como `/cita`: los mensajes
  salen del servidor y se muestran bajo el campo de `detalle.campo`. Con el 409 de Q5 muestra el
  mensaje y el enlace a la orden existente (`detalle.orden_id`). Con un 201 lleva a la ficha.
- **Alternativas descartadas**: un endpoint nuevo `GET /api/citas/{id}`, porque la lista de hoy
  ya trae todo lo que el formulario enseña (principio I).

## D8. Citas de hoy para el quickstart (datos demo, fichero compartido)

- **Problema**: con Q2, una orden solo se abre el día de su cita, pero después del quickstart §1
  todas las citas son de días futuros (las pide la página pública, desde mañana). El §2 ya no
  puede abrir la orden de 1234BCD ni de 5678CDF, y §3, §4, §5 y §7 trabajan sobre «la orden de
  1234BCD».
- **Decisión (propuesta; revisa la Persona B)**: añadir una **parte 5** a `backend/seed/demo.sql`,
  sin tocar las partes 1–4 ni las matrículas del §1 (1111BBB, 2222CCC, 9999LLL, 1234BCD,
  5678CDF):
  | Dato | Para qué |
  |---|---|
  | Vehículo `1313MMM` (Seat León) de Lucía Fernández, sin órdenes, cita **hoy** «Revisión general» | §2.1: primera orden (Nº 10), vehículo sin visitas anteriores (US2-3); después §3–§7 |
  | Vehículo `1414NNN` (Renault Mégane) de Andrés Gil, sin órdenes, cita **hoy** «Cambio de aceite» | §2.2: Nº 11 consecutivo y dos pestañas (Q5); después «la otra orden» de §4 |
  | Cita **hoy** de `8888KKK` (orden 8, 39.800 km) «Revisión de frenos» | §2.3: rechazo del km menor (FR-004) sin crear ninguna orden |
  | Vehículo `1515PPP` (Ford Fiesta) de Jorge Martín, cita **ayer** sin orden | §2: una cita de otro día no ofrece «Abrir orden» (Q2) |
  | Usuario `pedro`, mecánico **de baja** (`activo = false`, misma contraseña ficticia) | §2.1: la lista de mecánicos no lo ofrece (Q3) |
  
  - Las fechas usan `(now() AT TIME ZONE 'Europe/Madrid')::date`, no `current_date`, para que «hoy»
    sea el de R12 aunque el servidor de PostgreSQL esté en UTC (entre las 00:00 y las 02:00 de
    Madrid, `current_date` aún sería ayer).
  - No se añade ningún dato personal nuevo: los vehículos son de clientes demo que ya existen
    (principio V); `pedro` es un usuario ficticio más, como los otros cinco.
  - El contador no cambia: la parte 5 no crea órdenes, así que la primera orden real sigue siendo
    la Nº 10.
  - Hay que cargar los datos demo el mismo día en que se recorre el quickstart, como ya pasa con el
    cupo del §1.
- **Efectos en el quickstart común (revisa la Persona B)**: §2 se reescribe con esos datos; en
  §3, §4, §5 y §7, «la orden de 1234BCD» pasa a ser «la orden de 1313MMM» y «la otra orden», la de
  1414NNN; en §0 se añade `pedro` (de baja, no puede entrar). §9 recorre el flujo con un vehículo
  nuevo: con Q2 la cita y la orden quedan en días distintos, así que se recorre en dos días
  (pedir la cita un día y seguir el día de la cita).
- **Efecto en las evidencias de spec-001**: el guion `evidencias/T032/t032.mjs` espera «sin citas»
  en la lista de hoy (fila 30); con la parte 5 hoy tiene citas. Las evidencias ya registradas no
  cambian, pero si se vuelve a ejecutar el guion sobre los datos demo nuevos hay que ajustar esa
  comprobación.
- **Alternativas descartadas**: usar `9999LLL` para el §2.3, porque el §1 y T034 ya lo usan y su
  cita la pide la página pública (desde mañana); crear órdenes demo nuevas para el km, porque
  cambiaría la numeración y las cuentas del tablero del §8.
