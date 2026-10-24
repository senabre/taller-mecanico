# Research: spec-003 · Registrar diagnóstico y preparar presupuesto

Decisiones propias de spec-003. Todo lo común (pila, dinero R4, enlace R9, correo R10, fechas R12,
pruebas R13, transiciones R14) está en [000-base-comun/research.md](../000-base-comun/research.md).
Fecha: 2026-10-22.

## D1. Cantidades: 0,25 h y unidades enteras, garantizadas por la base de datos

- **Decisión**: en `linea_presupuesto`, la mano de obra va en múltiplos de 0,25 h y la pieza en
  unidades enteras (aclaración del 22/10). Se añaden dos `CHECK` en una migración nueva
  `0002_spec003_lineas.sql`, igual que el que ya tiene `imputacion`:
  `CHECK (tipo <> 'mano_obra' OR cantidad * 4 = trunc(cantidad * 4))` y
  `CHECK (tipo <> 'pieza' OR cantidad = trunc(cantidad))`. El código valida antes y responde 422 con
  el motivo en español («Las horas van en múltiplos de 0,25 (p. ej. 1,5)», «Las unidades de pieza
  deben ser un número entero»); el `CHECK` es la red de seguridad.
- **Motivo**: FR-002 y los casos límite. Mismo criterio que spec-005 (imputaciones) para que
  presupuesto e imputación cuadren.
- **Alternativas descartadas**: validar solo en el código, porque `0001` ya lo hace en la base de
  datos para `imputacion` y sería incoherente; tocar `0001`, porque ya está fusionada en `main`.

## D2. El diagnóstico se puede cambiar hasta que se emite un presupuesto

- **Decisión**: `PUT /api/ordenes/{id}/diagnostico`, en una transacción con la orden bloqueada
  (`SELECT … FOR UPDATE`):
  - Orden `recibida` → guarda el diagnóstico y `cambiar_estado(&[Recibida], EnDiagnostico)`; el
    evento se publica después del `commit` (R14).
  - Orden `en_diagnostico` sin ningún presupuesto emitido (`estado <> 'borrador'`) → solo
    `UPDATE orden SET diagnostico`. **No** cambia el estado ni publica evento.
  - `en_diagnostico` con algún presupuesto ya emitido → 409 «Ya se ha emitido un presupuesto para
    esta orden; el diagnóstico no se puede cambiar».
  - Cualquier otro estado → 409 «La orden está {estado}; el diagnóstico solo se registra en
    órdenes recibidas o en diagnóstico».
  - Diagnóstico vacío o solo espacios → 422.
- **Motivo**: aclaración del 22/10 (FR-001a). «Algún presupuesto emitido» incluye los rechazados y
  caducados: lo que se envió al cliente no debe cambiar de diagnóstico. La transición
  `en_diagnostico → en_diagnostico` de `estados.rs` es de spec-004 (rechazo) y aquí no se usa.
- **Respuesta**: 200 `{id, numero, estado, diagnostico}`. El `Orden` completo es de spec-002, que
  aún no existe; la pantalla recarga la orden después de guardar.

## D3. Concepto de la línea

- **Decisión**: mano de obra → `concepto` obligatorio, sin espacios al principio ni al final.
  Pieza → el servidor ignora cualquier `concepto` recibido y guarda
  `format!("{referencia} {descripcion}")` de la pieza en ese momento.
- **Motivo**: aclaración del 22/10 (FR-003). Coincide con los datos demo («PF-123 Pastillas freno»).

## D4. El precio de cada línea se fija al añadirla

- **Decisión**: al añadir una línea se guardan `precio_unitario` (mano de obra:
  `config.precio_hora`; pieza: `pieza.precio_venta`) e `importe = dinero::importe_linea(cantidad,
  precio)`. Los totales de un borrador se calculan al leerlo con `dinero::totales(importes,
  config.iva_porcentaje)`. Al emitir se calculan una última vez y se guardan `base`, `iva`, `total`
  e `iva_porcentaje` (R4).
- **Motivo**: el mecánico ve en cada línea el precio que se guarda; las piezas no se pueden editar
  (spec-006) y el precio por hora solo cambia en la configuración, así que la diferencia es
  teórica. Recalcular precios al emitir sería lógica extra sin caso de uso (principio I).

## D5. Un solo presupuesto abierto por orden

- **Decisión**: `POST /api/ordenes/{id}/presupuestos` bloquea la orden `FOR UPDATE` y:
  - deduce el tipo: `en_diagnostico` → `inicial`; `pendiente_conformidad` → `ampliacion`
    (spec-005); otro estado → 409 «La orden está {estado}; no se puede preparar un presupuesto»;
  - 409 «Ya hay un presupuesto emitido sin caducar para esta orden» si existe uno `emitido` con
    `caduca_el >= fechas::hoy()`;
  - inserta el borrador; si choca con `presupuesto_borrador_unico` → 409 «Ya hay un presupuesto en
    borrador para esta orden».
- **Motivo**: FR-008. El índice parcial cubre los borradores; «sin caducar» depende de hoy y se
  comprueba con la orden bloqueada (data-model).

## D6. Bloqueo tras emitir y emisión

- **Decisión**:
  - Añadir o quitar líneas exige `estado = 'borrador'` (409 «El presupuesto ya está emitido; no se
    puede modificar»). Quitar una línea de otro presupuesto → 404.
  - `POST /api/presupuestos/{id}/emitir`, en una transacción con el presupuesto bloqueado: 409 si
    no está en borrador; 422 «El presupuesto no tiene líneas» si no tiene; si no, fija totales,
    `emitido_en = now()`, `caduca_el = fechas::hoy() + 15` y `token = auth::nuevo_token()` (32
    bytes base64url, R9) y pasa a `emitido`.
  - Después del `commit`, envía al correo del cliente el enlace `{URL_PUBLICA}/presupuesto/{token}`
    (spec-004 FR-001, R10), sin importes. Si el correo falla no se deshace nada: la respuesta lleva
    `aviso_enviado: false` y la pantalla muestra el aviso. Ver y copiar el enlace es spec-004
    FR-008 (Persona A).
  - Emitir bloquea también la orden (`FOR UPDATE OF p, o`), igual que cambiar el diagnóstico, para
    que los dos no se crucen (FR-001a).
- **Motivo**: FR-007, US3 y la frontera 003 → 004 del plan maestro (B emite, A responde).
- **Caducidad**: «Válido hasta» = `caduca_el`; un presupuesto `emitido` con `caduca_el < hoy` se
  muestra como caducado (`caducado: true`). «Caducado» no es un estado guardado.

## D7. Quién puede hacer qué

- **Decisión**: diagnóstico, crear presupuesto, añadir y quitar líneas y emitir →
  `usuario.exigir_mecanico_de(orden.mecanico_id)` (el jefe pasa siempre). `GET
  /api/presupuestos/{id}` → cualquier rol con sesión (contrato).
- **Motivo**: caso límite «una orden no asignada al mecánico».

## D8. Punto de contacto con spec-002 (Persona A)

- **Decisión**: `presupuestos.rs` expone `presupuestos_de_orden(conexion, orden_id,
  iva_porcentaje) -> Vec<PresupuestoResumen>` (con `estado.config.iva_porcentaje`), con `PresupuestoResumen = {id, tipo, estado, caducado, total,
  emitido_en, caduca_el}` (en borrador, `total` es el calculado al momento). Spec-002 la llama para
  llenar `presupuestos` de `GET /api/ordenes/{id}`, igual que `pedidos_de_orden` en spec-006.
  Las secciones de pantalla son componentes (`Diagnostico.svelte`, `Presupuesto.svelte`) que la
  ficha `ordenes/[id]/+page.svelte` de spec-002 incluye.
- **Motivo**: el contrato nombra `PresupuestoResumen` pero no lo define; la ficha de la orden es de
  spec-002 y aún no existe. Así ninguna de las dos personas edita el módulo de la otra.
- **Consecuencia**: igual que en spec-006, el recorrido en pantalla de quickstart §3 necesita la
  ficha de spec-002. El backend se prueba por completo con `#[sqlx::test]`.
