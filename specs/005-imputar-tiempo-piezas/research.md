# Research: spec-005 · Imputar tiempo y piezas durante la reparación

Decisiones propias de spec-005. Lo común (dinero R4, fechas R12, pruebas R13, transiciones R14)
está en [000-base-comun/research.md](../000-base-comun/research.md); las reglas de las líneas, en
[003/research.md](../003-diagnostico-presupuesto/research.md). Fecha: 2026-10-29.

## D1. Una sola validación de líneas para presupuesto e imputación

- **Decisión**: se saca de `rutas/presupuestos.rs` a un módulo `backend/src/lineas.rs` la
  validación de una línea nueva: tipo (`mano_obra` | `pieza`), cantidad (> 0, 2 decimales;
  horas en múltiplos de 0,25; piezas enteras), concepto (obligatorio en mano de obra; en pieza,
  «referencia descripción» puesto por el servidor), precio unitario (`config.precio_hora` o
  `pieza.precio_venta`) e importe (`dinero::importe_linea`). La usan `presupuestos.rs` (spec-003) y
  `imputaciones.rs`, con los mismos mensajes de error.
- **Motivo**: FR-001, FR-004 («igual que en el presupuesto») y la aclaración del 29/10. Copiar el
  código dejaría dos validaciones que se pueden separar con el tiempo (principio I: menos código).
- **Alternativas descartadas**: duplicar la validación en `imputaciones.rs`.

## D2. Piezas enteras también en la base de datos

- **Decisión**: migración `0003_spec005_imputaciones.sql` con
  `CHECK (tipo <> 'pieza' OR cantidad = trunc(cantidad))` en `imputacion`. El de las horas
  (múltiplos de 0,25) ya está en `0001`.
- **Motivo**: mismo criterio que `linea_presupuesto` (spec-003 D1) y el stock es entero.

## D3. Imputar: una transacción con la orden bloqueada

- **Decisión**: `POST /api/ordenes/{id}/imputaciones`:
  1. `SELECT estado, mecanico_id FROM orden WHERE id = $1 FOR UPDATE` (404 si no existe);
     `exigir_mecanico_de(mecanico_id)`.
  2. Si no está `en_reparacion` → 409 «La orden está {estado}; solo se puede imputar en órdenes
     en reparación». Si no tiene ningún presupuesto `aceptado` → 409 «La orden no tiene un
     presupuesto aceptado» (por coherencia: con los flujos de 003/004 no debería pasar).
  3. Validar la línea (D1) → 422.
  4. Pieza: `UPDATE pieza SET stock = stock - $n WHERE id = $1 AND stock >= $n`; si no actualiza
     ninguna fila → 422 «Solo hay {n} unidad(es) en stock de {referencia}; el almacén puede
     pedirla al proveedor» (singular con 1: «Solo hay 1 unidad…»; el mecánico no ve Pedidos).
  5. Insertar la imputación con `usuario_id` y `creada_en = now()`.
  6. Regla del 10 % (D4).
- **Motivo**: el bloqueo de la orden evita que dos imputaciones simultáneas se salten el umbral;
  el `UPDATE … WHERE stock >= $n` evita vender stock que no hay, sin `SELECT` previo (mismo
  patrón que spec-006).

## D4. Regla del 10 %, calculada al vuelo

- **Decisión**: tras insertar, en la misma transacción:
  `coste_acumulado = Σ imputacion.importe` y `total_presupuestado = Σ presupuesto.base` de los
  `aceptado` de la orden (inicial + ampliaciones, FR-007). Si
  `coste_acumulado > total_presupuestado × 1,10` (con `Decimal`, sin IVA) →
  `cambiar_estado(&[EnReparacion], PendienteConformidad)` y el evento se publica después del
  `commit` (R14). Exactamente el 110 % no bloquea. La imputación que cruza el umbral se guarda.
- **Motivo**: FR-005, US2 y Assumptions. No se guarda ningún acumulado: siempre es la suma
  (data-model, principio I).

## D5. Respuesta y punto de contacto con spec-002

- **Decisión**: imputar responde 201 y terminar 200 con
  `ResumenImputaciones = {orden_id, estado, coste_acumulado, total_presupuestado,
  imputaciones:[Linea]}` (las imputaciones de la orden, de la más antigua a la más reciente).
  `imputaciones.rs` expone `resumen_imputaciones(conexion, orden_id)`, que spec-002 llama para
  llenar `coste_acumulado`, `total_presupuestado` e `imputaciones` de `GET /api/ordenes/{id}`.
- **Motivo**: el contrato decía «201 `Orden`», pero `Orden` es de spec-002 y aún no existe. Igual
  que en spec-003 (D2, D8) y spec-006 (`pedidos_de_orden`).

## D6. Terminar

- **Decisión**: `POST /api/ordenes/{id}/terminar`, en una transacción con la orden bloqueada:
  `exigir_mecanico_de`; si no está `en_reparacion` → 409 «La orden está {estado}; solo se puede
  terminar una orden en reparación»; sin imputaciones → 422 «Primero hay que imputar el trabajo
  realizado»; si no, `cambiar_estado(&[EnReparacion], Terminada)` (fija `terminada_en`) y evento
  tras el `commit`.
- **Motivo**: FR-008 y la aclaración del 29/10 (sin imputaciones no se podría facturar en
  spec-007). Una orden `pendiente_conformidad` o `en_espera_piezas` da 409 (casos límite).

## D7. Ampliaciones: ya las hace spec-003

- **Decisión**: no hay endpoints nuevos. Con la orden `pendiente_conformidad`,
  `POST /api/ordenes/{id}/presupuestos` (spec-003) crea un presupuesto `ampliacion`, que se
  rellena y emite igual que el inicial. Aceptarlo es spec-004 (Persona A):
  `pendiente_conformidad → en_reparacion`. Lo que aporta spec-005 es que `total_presupuestado`
  sume todos los aceptados (D4).
- **Motivo**: FR-006, FR-007 y el plan maestro. Las pruebas simulan la aceptación con SQL
  (`UPDATE presupuesto SET estado = 'aceptado'` + estado de la orden), sin depender de spec-004.
- **Límite conocido** (aclaración del 29/10): si el cliente no acepta ninguna ampliación, la orden
  se queda `pendiente_conformidad`; no se construye nada para ese caso.

## D8. Pantalla

- **Decisión**: sección `ordenes/[id]/Imputaciones.svelte` de la ficha de spec-002: indicador
  «Acumulado X € de Y € presupuestados» (destacado si pasa del 100 %), `TablaLineas` (sin
  «Quitar»: no se anulan imputaciones), formulario de nueva imputación y botón «Terminar
  reparación» con confirmación. El formulario de línea (mano de obra / pieza con buscador) se
  saca de `Presupuesto.svelte` a un componente compartido `lib/componentes/FormularioLinea.svelte`
  que usan las dos secciones.
- **Motivo**: FR-003 y SC-003 (imputar en menos de 30 s). El mismo formulario en las dos pantallas
  evita duplicar el buscador de piezas y la conversión de «1,5» a «1.5».
