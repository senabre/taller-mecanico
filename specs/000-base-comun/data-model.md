# Data Model: Talleres Rueda (compartido por spec-001 … spec-008)

Este es el **único modelo de datos del sistema**. Toda la base de datos se crea en la migración
`backend/migrations/0001_esquema_inicial.sql`, que se escribe en pareja en la base común. Cualquier
cambio posterior se hace con una migración nueva, `NNNN_descripcion.sql`, que **revisan las dos
personas** antes de fusionarla.

Convenciones:
- Nombres en español, `snake_case` y sin tildes.
- Las claves primarias son `id BIGINT GENERATED ALWAYS AS IDENTITY`.
- Los importes son `NUMERIC(12,2)`, las cantidades `NUMERIC(10,2)`, los instantes `TIMESTAMPTZ` y
  las fechas `DATE` (ver research R4 y R12).

## Diagrama de relaciones

```text
cliente 1──* vehiculo 1──* cita 1──0..1 orden *──1 usuario (mecánico)
                                         │
                ┌──────────────┬─────────┼──────────────┬──────────────┐
                *              *         *              0..1           │
           presupuesto    imputacion  pedido         factura           │
                │1            │*  │*     │*  *──1 proveedor  │1        │
                *             │   │      │                   *         │
     linea_presupuesto ──*─1 pieza ─1──*─┘           linea_factura     │
                                                                       │
usuario 1──* sesion        contador (orden, factura)                   │
```

## Tipos enumerados

| Tipo | Valores |
|---|---|
| `rol` | `recepcion`, `mecanico`, `almacen`, `jefe` |
| `estado_orden` | `recibida`, `en_diagnostico`, `en_reparacion`, `en_espera_piezas`, `pendiente_conformidad`, `terminada`, `facturada` |
| `tipo_presupuesto` | `inicial`, `ampliacion` |
| `estado_presupuesto` | `borrador`, `emitido`, `aceptado`, `rechazado` (que esté *caducado* se calcula: `emitido` y `caduca_el < hoy`) |
| `tipo_linea` | `mano_obra`, `pieza` |
| `estado_pedido` | `pendiente`, `recibido` |

Texto visible de cada estado: «Recibida», «En diagnóstico», «En reparación», «En espera de
piezas», «Pendiente de conformidad», «Terminada», «Facturada» (en `frontend/src/lib/formato.ts`).

## Entidades

### usuario *(base común)*
| Campo | Tipo | Reglas |
|---|---|---|
| nombre | TEXT | obligatorio; se muestra en el tablero |
| login | TEXT | único, en minúsculas |
| hash_contrasena | TEXT | argon2id; nunca se devuelve en la API |
| rol | rol | obligatorio |
| activo | BOOLEAN | por defecto `true` |

### sesion *(base común)*
`token_hash` TEXT PK (SHA-256 del token), `usuario_id` FK, `expira_en` TIMESTAMPTZ (12 h,
renovable con la actividad).

### contador *(base común)*
`nombre` TEXT PK (`'orden'`, `'factura'`), `ultimo` BIGINT NOT NULL DEFAULT 0. La migración
inicial crea las dos filas (ver research R5).

### cliente *(spec-001)*
| Campo | Tipo | Reglas |
|---|---|---|
| nombre | TEXT | obligatorio |
| telefono | TEXT | obligatorio |
| email | TEXT | obligatorio, con formato de correo válido |

Principio V: no hay más datos personales. La API pública nunca devuelve ninguno de estos campos.

### vehiculo *(spec-001)*
| Campo | Tipo | Reglas |
|---|---|---|
| matricula | TEXT | **única**, normalizada: en mayúsculas y sin espacios ni guiones (`1234BCD`) |
| marca, modelo | TEXT | obligatorios |
| cliente_id | FK cliente | obligatorio |

### cita *(spec-001)*
| Campo | Tipo | Reglas |
|---|---|---|
| vehiculo_id | FK | obligatorio |
| fecha | DATE | de lunes a viernes, desde mañana hasta dentro de 30 días; nº de citas de la fecha < `TALLER_CUPO_DIARIO` (research R6) |
| motivo | TEXT | obligatorio, no vacío |
| creada_en | TIMESTAMPTZ | |

### orden *(spec-002; el estado lo cambian 003–007)*
| Campo | Tipo | Reglas |
|---|---|---|
| numero | BIGINT | **único**, sale de `contador('orden')` |
| cita_id | FK | **único** (como mucho una orden por cita) |
| vehiculo_id | FK | el mismo de la cita |
| mecanico_id | FK usuario | el usuario debe tener rol `mecanico` |
| km | INTEGER | ≥ 0 y ≥ km de la orden anterior del mismo vehículo |
| danos | TEXT | obligatorio (puede ser «Sin daños visibles») |
| fecha_entrega_prometida | DATE | ≥ fecha de apertura |
| diagnostico | TEXT NULL | lo escribe spec-003 |
| estado | estado_orden | inicial `recibida` |
| abierta_en, terminada_en | TIMESTAMPTZ | |

Valores calculados (con consultas, no se guardan):
- `total_presupuestado` = Σ `base` de los presupuestos `aceptado` de la orden.
- `coste_acumulado` = Σ `importe` de sus imputaciones.
- `retrasada` = estado ≠ `facturada` y `fecha_entrega_prometida` < hoy (Europe/Madrid).

### presupuesto *(spec-003; respuesta en spec-004; ampliación en spec-005)*
| Campo | Tipo | Reglas |
|---|---|---|
| orden_id | FK | |
| tipo | tipo_presupuesto | `inicial` si la orden no tiene ninguno aceptado; `ampliacion` si está `pendiente_conformidad` |
| estado | estado_presupuesto | inicial `borrador` |
| token | TEXT NULL | único; se genera al emitir (research R9) |
| iva_porcentaje | NUMERIC(5,2) | se copia de la configuración al emitir |
| base, iva, total | NUMERIC(12,2) | se fijan al emitir (research R4) |
| emitido_en | TIMESTAMPTZ NULL | |
| caduca_el | DATE NULL | = fecha de emisión (Madrid) + 15 días |
| respondido_en | TIMESTAMPTZ NULL | la hora exacta de la respuesta |

Restricción: como mucho un presupuesto abierto por orden a la vez, es decir, en `borrador` o
`emitido` sin caducar. La base de datos solo garantiza un `borrador` por orden (índice único
parcial `WHERE estado = 'borrador'`). Que no haya un `emitido` vigente lo comprueba spec-003 en la
misma transacción, porque «caducado» depende de hoy y no puede ir en un índice. Un presupuesto
caducado no cuenta y la orden puede tener otro.

### linea_presupuesto *(spec-003)*
`presupuesto_id` FK, `tipo` tipo_linea, `concepto` TEXT, `pieza_id` FK NULL (obligatorio si
`tipo = pieza`), `cantidad` NUMERIC(10,2) > 0, `precio_unitario` NUMERIC(12,2) (de
`TALLER_PRECIO_HORA` o de `pieza.precio_venta`), `importe` NUMERIC(12,2).
- `cantidad`: mano de obra en múltiplos de 0,25 h; pieza en unidades enteras (`CHECK` de la
  migración `0002`, spec-003 D1).
- `concepto`: mano de obra, el que escribe el mecánico (obligatorio); pieza, «referencia
  descripción» puesto por el servidor (spec-003 D3).
- `precio_unitario` e `importe` se fijan al añadir la línea (spec-003 D4).
- Solo se pueden añadir o quitar líneas mientras el presupuesto está en `borrador`; no se editan.

### pieza *(alta en spec-006; la usan 003 y 005)*
| Campo | Tipo | Reglas |
|---|---|---|
| referencia | TEXT | **única** tal cual se escribe, sin espacios al principio ni al final (ej. `PF-123`) |
| descripcion | TEXT | obligatoria |
| precio_venta | NUMERIC(12,2) | ≥ 0 |
| stock | INTEGER | `DEFAULT 0`, `CHECK (stock >= 0)`; al darla de alta es opcional (0 si no se indica) |

### imputacion *(spec-005)*
`orden_id` FK, `tipo` tipo_linea, `concepto` TEXT, `pieza_id` FK NULL, `cantidad` NUMERIC(10,2)
(las horas en múltiplos de 0,25), `precio_unitario`, `importe`, `usuario_id` FK, `creada_en`.
Solo se admite si la orden está `en_reparacion`. Una imputación de pieza descuenta el stock en la
misma transacción: `UPDATE pieza SET stock = stock - $n WHERE id = $1 AND stock >= $n`.
- `cantidad`: horas en múltiplos de 0,25 (`CHECK` de `0001`); piezas en unidades enteras (`CHECK`
  de `0003`, spec-005 D2).
- `concepto`: horas, el que escribe el mecánico (obligatorio); pieza, «referencia descripción»
  puesto por el servidor. Misma validación que `linea_presupuesto` (`lineas.rs`, spec-005 D1).
- Tras cada imputación, si `coste_acumulado > total_presupuestado × 1,10` la orden pasa a
  `pendiente_conformidad` (spec-005 D4). Nunca se modifican ni se borran.

### proveedor *(spec-006)*
`nombre` TEXT único. Vienen **precargados** (los de demo en `seed/demo.sql`); la app no los crea,
edita ni borra (spec-006 FR-010).

### pedido *(spec-006)*
`proveedor_id` FK, `pieza_id` FK, `cantidad` INTEGER > 0, `orden_id` FK, `estado` estado_pedido
(inicial `pendiente`), `pedido_en` TIMESTAMPTZ `DEFAULT now()`, `recibido_en` TIMESTAMPTZ NULL.
Índice único parcial `(orden_id, pieza_id) WHERE estado = 'pendiente'`: como mucho un pedido
pendiente por orden y referencia (spec-006 FR-004a).

### factura *(spec-007)*
| Campo | Tipo | Reglas |
|---|---|---|
| numero | BIGINT | **único**, sale de `contador('factura')` |
| orden_id | FK | **único** (como mucho una factura por orden) |
| fecha | DATE | |
| cliente_nombre, matricula | TEXT | copia del momento de emitirla (factura simplificada, Q2-A) |
| iva_porcentaje, base, iva, total | NUMERIC | se calculan a partir de las imputaciones |
| aviso_enviado | BOOLEAN | si se pudo enviar el correo de «listo para recoger» |

### linea_factura *(spec-007)*
Es una copia de cada imputación: `concepto`, `cantidad`, `precio_unitario`, `importe`. La factura
no cambia aunque después cambie el catálogo.

## Máquina de estados de la orden

Todos los cambios de estado pasan por `estados::cambiar_estado`, que devuelve un `CambioEstado`.
Quien la llama lo publica en el tablero con `CambioEstado::publicar` **después del `commit`**
(research R14). Abrir una orden (`— → recibida`) es un `INSERT` de spec-002, no un cambio de
estado, pero también publica su evento después del `commit`.

| Desde | Hacia | Disparador | Spec |
|---|---|---|---|
| — | recibida | recepción abre la orden | 002 |
| recibida | en_diagnostico | el mecánico guarda el diagnóstico | 003 |
| en_diagnostico | en_reparacion | el cliente acepta el presupuesto inicial | 004 |
| en_diagnostico | en_diagnostico | el cliente rechaza el presupuesto inicial (se puede emitir otro) | 004 |
| en_reparacion | pendiente_conformidad | coste acumulado > 110 % del total presupuestado | 005 |
| pendiente_conformidad | en_reparacion | el cliente acepta la ampliación | 004/005 |
| en_reparacion | en_espera_piezas | almacén registra un pedido para la orden | 006 |
| en_espera_piezas | en_reparacion | se recibe el último pedido pendiente de la orden | 006 |
| en_reparacion | terminada | el mecánico marca la orden como terminada | 005 |
| terminada | facturada | recepción emite la factura | 007 |

Reglas transversales:
- Solo se imputa en `en_reparacion`, así que sin presupuesto aceptado no se trabaja (spec-004
  FR-007).
- No hay ningún cambio de estado manual. En concreto, una orden `en_espera_piezas` solo sale de
  ahí al recibir su último pedido pendiente (spec-006 FR-008a); nadie puede sacarla, tampoco el
  jefe.
- Las órdenes «abiertas» del tablero son todas las que no están en `facturada` (spec-008).
- El historial del vehículo (spec-007) son sus órdenes `facturada` con su factura, de la más
  reciente a la más antigua.
