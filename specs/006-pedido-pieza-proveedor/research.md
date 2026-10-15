# Research: spec-006 · Pedir al proveedor una pieza sin stock

Decisiones propias de spec-006. Todo lo común (pila, dinero, fechas, autenticación, transiciones)
está en [000-base-comun/research.md](../000-base-comun/research.md). Fecha: 2026-10-15.

## D1. Proveedores precargados, sin alta en la API

- **Decisión**: se quita `POST /api/proveedores` del contrato. Solo queda `GET /api/proveedores`,
  que alimenta el desplegable del pedido. Los proveedores de demo («Recambios Sur», «Autopartes
  Norte») van en `backend/seed/demo.sql`. Los reales los inserta el equipo con `psql` al desplegar,
  igual que los usuarios se crean fuera de la app.
- **Motivo**: FR-010 dice que esta spec no crea, edita ni borra proveedores (principio III).
- **Alternativas descartadas**: mantener el alta de proveedores del plan anterior, porque es
  alcance fantasma; y una migración con datos de proveedores, porque mezclaría el esquema con datos
  propios de cada taller.

## D2. Un solo pedido pendiente por orden y referencia

- **Decisión**: índice único parcial
  `CREATE UNIQUE INDEX pedido_pendiente_unico ON pedido (orden_id, pieza_id) WHERE estado = 'pendiente'`.
  Si el `INSERT` choca con él, se responde 409 «Ya hay un pedido pendiente de PF-123 para la orden
  42». Cuando el pedido se recibe, deja de contar y se puede volver a pedir la misma referencia.
- **Motivo**: lo garantiza la base de datos incluso con dos peticiones a la vez, sin más código
  (FR-004a).
- **Alternativas descartadas**: comprobarlo antes con un `SELECT`, porque dos peticiones
  simultáneas podrían pasar las dos.

## D3. Bloqueo de la orden en crear y recibir

- **Decisión**: crear un pedido y recibirlo empiezan con
  `SELECT estado FROM orden WHERE id = $1 FOR UPDATE`, dentro de la transacción.
- **Motivo**: sin el bloqueo, si se recibe el último pedido mientras se registra otro para la misma
  orden, la orden podría quedar «en reparación» con un pedido pendiente. Eso incumpliría SC-002.
  Con ~10 usuarios el bloqueo no se nota.
- **Alternativas descartadas**: aislamiento `SERIALIZABLE` con reintentos, porque es más complejo
  para el mismo resultado.

## D4. La orden solo sale de la espera al recibir su último pedido

- **Decisión**: no existe ningún endpoint para cambiar el estado de una orden a mano. En la tabla
  de transiciones de `estados.rs` (data-model), la única salida de `en_espera_piezas` es
  `→ en_reparacion` y solo la ejecuta `POST /api/pedidos/{id}/recibir`. «Terminar» (spec-005) e
  imputar exigen `en_reparacion` y devuelven 409 en espera. El jefe tampoco tiene forma de saltárselo.
- **Motivo**: FR-008a. Al no haber endpoint, no hay nada que permitir ni que comprobar por rol.

## D5. La orden se indica por su número

- **Decisión**: `POST /api/pedidos` recibe `orden_numero`, el número visible (p. ej. 42), en vez
  del `id` interno. Si no existe → 422 «No existe la orden 99». Si su estado no lo permite → 409
  «La orden 50 está en diagnóstico; solo se puede pedir para órdenes en reparación o en espera de
  piezas».
- **Motivo**: el almacén no puede abrir `/ordenes` (es de recepción y mecánico) y conoce la orden
  por su número, como en los ejemplos de la spec. Así no hace falta dar al almacén acceso a un
  endpoint de la Persona A y el mensaje del motivo (FR-004) se ve tal cual.
- **Alternativas descartadas**: un desplegable con las órdenes «en reparación» y «en espera», que
  obligaría a abrir `GET /api/ordenes` al rol almacén (punto de contacto con A).

## D6. Validaciones de la referencia

- **Decisión**: se quitan los espacios del principio y del final de `referencia` y `descripcion`,
  y no pueden quedar vacías. La referencia es única tal cual se escribe. `precio_venta` ≥ 0 con 2
  decimales. `stock` es opcional, entero y ≥ 0, y vale 0 si no se indica. Los errores son 422 con el
  motivo en español. Una referencia repetida da 409 «Ya existe la referencia PF-123».
- **Motivo**: FR-001, FR-002 y los casos límite de la spec. No se normalizan mayúsculas porque la
  spec no lo pide (principio III).

## D7. Fechas del pedido

- **Decisión**: `pedido_en` y `recibido_en` son `TIMESTAMPTZ` con valor `now()` del servidor. En
  pantalla se muestran como dd/mm/aaaa (R12). No se pueden escribir a mano.
- **Motivo**: la spec habla de «la fecha del día» y de «guardar la fecha de recepción».

## D8. Sin aviso al mecánico

- **Decisión**: al volver a `en_reparacion` se publica el evento normal del tablero
  (`CambioEstado::publicar` después del `commit`, R11 y R14) y no se hace nada más: ni correo ni
  notificación.
- **Motivo**: es la aclaración de la spec (Assumptions). El mecánico ve el estado en su lista.
