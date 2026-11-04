# Data Model: spec-004 · Aceptar o rechazar el presupuesto desde un enlace

El modelo de datos es **único** y está en
[000-base-comun/data-model.md](../000-base-comun/data-model.md): la tabla `presupuesto` ya existe en
la migración `0001_esquema_inicial.sql`, con `estado`, `token`, `caduca_el` y `respondido_en`. Esta
spec **no crea tablas ni migraciones**. Aquí solo se explica cómo la usa la historia y qué reglas
comprueba.

## Qué hace spec-004 con cada tabla

| Tabla | Lee | Escribe | Nunca |
|---|---|---|---|
| `presupuesto` | por `token`: tipo, estado, totales, `caduca_el`, `respondido_en`, `orden_id` | `estado` (`aceptado` o `rechazado`) y `respondido_en`, una sola vez | cambiar una respuesta ya guardada; tocar líneas o totales |
| `linea_presupuesto` | las líneas para la página pública | — | — |
| `orden` | el diagnóstico para la página pública | el estado, solo con `estados::cambiar_estado` y solo al aceptar | cambiar el estado al rechazar (Q2) |
| `vehiculo` | matrícula, marca y modelo | — | — |
| `cliente` | — | — | leerla: la página pública no muestra ningún dato del cliente (principio V) |

## Reglas que comprueba esta spec

Las reglas están en el documento común y en [research R9](../000-base-comun/research.md), que tiene
la única versión del `UPDATE`. La tabla dice dónde se comprueba cada una.

| Regla | Requisito | Dónde se comprueba |
|---|---|---|
| Una sola respuesta por presupuesto | FR-003 (Q4) | el `UPDATE` de R9 (`estado = 'emitido'`), también con dos a la vez (research D2) |
| Un presupuesto caducado no admite respuesta | FR-005 (Q3) | el `UPDATE` de R9 (`caduca_el >= $hoy`); el mismo criterio que `caducado()` de spec-003 |
| `respondido_en` es la hora exacta de la respuesta | FR-004 | `now()` en el `UPDATE`; se muestra en hora de Madrid (R12) |
| Aceptar cambia la orden en la misma transacción | FR-006, FR-009 | `POST` (research D3) |
| Rechazar no cambia el estado de la orden | Q2 | `POST`: sin `cambiar_estado`; máquina de estados del data-model común |
| Sin presupuesto aceptado no se imputa | FR-007 | spec-005: solo imputa en `en_reparacion` |

## Estados

| Estado del presupuesto | En la página pública | En la orden |
|---|---|---|
| `emitido`, vigente | «Aceptar» y «Rechazar» | enlace y «Copiar enlace» |
| `emitido`, caducado (`caduca_el < hoy`, calculado) | aviso de caducidad, sin botones | «Caducado el …», sin enlace |
| `aceptado` o `rechazado` | la decisión con su fecha y hora, sin botones | la respuesta con su fecha y hora, sin enlace |

`borrador` no tiene token, así que no se llega a él desde un enlace.

## Valores calculados

`decision` y `caducado` del contrato no se guardan: salen de `estado` y de `caduca_el`.
