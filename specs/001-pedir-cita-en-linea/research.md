# Research: spec-001 · Pedir cita en línea

Decisiones propias de spec-001 después del clarify (sesión 2026-10-22, Q1–Q5; registro en
[decisiones-clarify.md](decisiones-clarify.md)). Todo lo común (pila, autenticación, fechas en
Europe/Madrid, errores, cupo) está en [000-base-comun/research.md](../000-base-comun/research.md)
y no se repite aquí. Fecha: 2026-10-22.

## D1. Matrícula válida: de 2 a 10 letras o cifras, en `normalizar_matricula` (cambio compartido)

- **Decisión**: `normalizar_matricula` (`backend/src/matricula.rs`) devuelve `None` también si la
  matrícula normalizada tiene menos de 2 o más de 10 caracteres. Las rutas de spec-001 convierten
  ese `None` en 422 «La matrícula no es válida» antes de consultar la base de datos (FR-002, Q1).
  No se añade ninguna migración: el `CHECK (matricula ~ '^[A-Z0-9]+$')` de `vehiculo` se queda
  como está, porque todas las altas pasan por `normalizar_matricula`.
- **Cambio compartido**: `matricula.rs` es de la base común y lo usa también spec-007 (Persona B)
  en `GET /api/vehiculos/{matricula}/historial`. Con el cambio, buscar en el historial una
  matrícula de 1 carácter o de más de 10 da 422 en vez de «sin reparaciones». Ningún vehículo
  guardado puede tener esas matrículas, así que el resultado visible para B no cambia. El cambio
  y sus pruebas los revisa la Persona B antes de fusionarlo.
- **Motivo**: un solo sitio para la regla, que ya aplican 001 y 007. Cierra el hallazgo I11.
- **Alternativas descartadas**: comprobar la longitud solo en `citas.rs`, porque 001 y 007
  tendrían reglas distintas para la misma matrícula; añadir una migración con el límite en el
  `CHECK`, porque no protege nada que no proteja ya el servidor (principio I).

## D2. Validación de teléfono, correo y campos obligatorios en el servidor

- **Decisión**: `POST /api/publico/citas` valida todo en el servidor, aunque el formulario no lo
  haya hecho. Las funciones `telefono_valido` y `correo_valido` están en `citas.rs` y solo
  comprueban caracteres; no se añade el crate `regex` ni ninguna otra dependencia.
  - Teléfono: tras quitar espacios y guiones, un «+» inicial opcional y de 9 a 15 cifras.
  - Correo: una sola «@», texto delante y, detrás, un dominio con un punto que tiene texto a
    ambos lados.
  - Matrícula, fecha, motivo, marca, modelo y nombre: no vacíos tras quitar espacios.
  Se valida en el orden fijo de la tabla de validación del contrato común, que es la fuente
  única de estas reglas, y se devuelve el **primer** error con el campo afectado:
  `{"error": "El teléfono no es válido", "detalle": {"campo": "cliente.telefono"}}`. Los valores se
  guardan tal como se escribieron, quitando solo los espacios al principio y al final.
- **Cuándo**: la matrícula, la fecha y el motivo se validan primero. Después se busca el
  vehículo **sin bloqueos** y, si es nuevo, se validan sus datos. Solo entonces se toman los
  bloqueos de D3. Así, todos los 422 llegan antes que cualquier 409: una matrícula nueva con un
  teléfono incorrecto para un día lleno recibe el 422 del teléfono, no el 409 del cupo
  ([hallazgos](hallazgos.md) I3).
- **Motivo**: FR-004a exige que no se pueda confirmar con datos incorrectos, y una petición puede
  llegar sin pasar por el formulario. Con un solo error por respuesta, el formulario sigue
  mostrando el mensaje en el campo afectado (US2-3), sin estructuras de error nuevas.
- **Frontend**: el formulario usa `novalidate`, para que el navegador no muestre sus propios
  avisos (`type="email"`, `required`) en otro idioma ni con otro texto (principio II). Los
  mensajes salen siempre del servidor.
- **Alternativas descartadas**: devolver todos los errores a la vez, porque cambia el formato de
  error común sin que la spec lo pida; repetir las reglas en TypeScript, porque serían dos copias
  de la misma regla.

## D3. Una sola cita pendiente por matrícula: segundo bloqueo en la transacción del cupo

- **Decisión**: en la transacción de `POST /api/publico/citas`, después del bloqueo por fecha de
  R6 (`pg_advisory_xact_lock` con la fecha), se toma un segundo bloqueo por matrícula:
  `pg_advisory_xact_lock(1, hashtext($matricula))`. La forma de dos claves de 32 bits no se
  solapa con la de una clave de 64 bits que usa R6. Con los dos bloqueos tomados, se comprueba
  `EXISTS (cita del vehículo con fecha >= hoy)` (hoy en Europe/Madrid, R12) y, si existe, se
  responde 409 «Este vehículo ya tiene una cita pendiente.». Los bloqueos se toman siempre en el
  mismo orden, primero la fecha y después la matrícula, así que no puede haber interbloqueos.
- **Por qué no basta el bloqueo del cupo**: el de R6 es por fecha. Dos peticiones de la misma
  matrícula para días **distintos** no se esperarían entre sí y las dos pasarían la
  comprobación. El bloqueo por matrícula las serializa. Además, evita que dos altas simultáneas
  de la misma matrícula nueva choquen en el `UNIQUE` de `vehiculo`.
- **Segunda búsqueda**: con los bloqueos tomados, se vuelve a buscar el vehículo, porque la
  búsqueda de D2 se hizo sin bloqueos. Si otra persona lo ha dado de alta entretanto, se aplica
  D4 (vehículo conocido). El bloqueo por matrícula garantiza que esta segunda búsqueda no cambia
  hasta el `commit`.
- **Orden de las comprobaciones**: primero la cita pendiente y después el cupo. Si los dos
  fallan, el cliente ve el aviso de cita pendiente, porque elegir otro día no lo arreglaría.
- **GET**: `GET /api/publico/vehiculos/{matricula}` devuelve `cita_pendiente: true|false` para que
  el formulario muestre el aviso nada más escribir la matrícula (US1-4). Nunca devuelve la fecha
  (principio V, Q3).
- **Alternativas descartadas**: un índice único parcial, porque «pendiente» depende de hoy y no
  puede ir en un índice (el mismo caso que el presupuesto caducado en data-model); un único
  bloqueo global para todas las citas, porque es igual de sencillo pero cambia R6, que es una
  decisión del plan maestro.

## D4. Vehículo conocido: el POST nunca toca el vehículo ni el propietario

- **Decisión**: si la matrícula ya existe, el `POST` ignora por completo `vehiculo` y `cliente`
  aunque lleguen en la petición. Solo inserta la cita y no hace ningún `UPDATE` de `cliente` ni
  de `vehiculo`. Si la matrícula es nueva, `vehiculo` y `cliente` son obligatorios (si faltan, se
  tratan como campos vacíos → 422).
- **Motivo**: FR-003 y Q5. Si un cambio de datos desde la página pública desviara el correo, otra
  persona podría aceptar o rechazar el presupuesto ajeno (spec-004). Ignorar los datos también
  resuelve el caso en el que otra persona da de alta la misma matrícula mientras el cliente
  rellena el formulario: la segunda búsqueda de D3 lo encuentra y la cita queda con el
  propietario registrado.
- **Alternativas descartadas**: responder 422 si llegan datos de contacto con una matrícula
  conocida, porque ese caso de carrera daría un error que el cliente no puede entender.

## D5. Lista de citas: recepción y jefe, en el orden en que se pidieron

- **Decisión**: `GET /api/citas` exige sesión (401) y llama a `exigir_rol(&[Recepcion])`, que deja
  pasar también al jefe (FR-002 de la base común); el resto recibe 403. `fecha` es opcional: si
  falta, se usa `fechas::hoy()`, así el frontend no calcula «hoy» por su cuenta. Se ordena por
  `creada_en, id`. Cada cita devuelve `cliente: {nombre, telefono}` y **nunca** el correo. Se
  mantiene `orden_id`, que spec-002 necesita para el botón «Abrir orden» y no es un dato personal.
- **Frontend**: el menú ya muestra «Citas» solo a recepción y jefe (layout de la base común). Si
  otro rol escribe `/citas` a mano, la página recibe 403 y muestra el mensaje común «No tienes
  permiso para hacer esto» sin ninguna lista; sin sesión, el layout lleva a `/entrar`.
- **Alternativas descartadas**: añadir una protección de rutas por rol en el frontend, porque el
  servidor ya impide ver los datos (principio I).

## D6. Cupo y días disponibles

Sin cambios respecto al plan anterior: R6 para el cupo, días de lunes a viernes desde mañana y
durante 30 días naturales (FR-006), y 409 «Ese día ya está completo» si se llena mientras el
cliente decide. Una fecha fuera de ese rango → 422 «Elige uno de los días disponibles». Si no
queda ningún día, el formulario muestra «No quedan fechas libres en los próximos 30 días. Llama
al taller para pedir cita.».

## D7. Efectos en otros documentos y specs

- **Quickstart común §1**: se reescribe con los pasos de Q1–Q5 (ver [quickstart.md](quickstart.md)).
- **Quickstart común §2.3 (spec-002, Persona A)**: pedía una segunda cita para 1234BCD, que tras
  §1 ya tiene una pendiente y FR-010 lo impide. Pasa a usar el vehículo demo `9999LLL` (última
  orden con 120.400 km, cita ya pasada).
- **Observación para spec-002 (no se resuelve aquí)**: su Assumptions dice que, si un cliente
  llega sin cita, recepción le registra primero la cita y después abre la orden, pero FR-006 de
  spec-001 solo ofrece días desde mañana. Ese caso ya no encajaba antes del clarify y lo tiene
  que decidir el clarify de spec-002.
