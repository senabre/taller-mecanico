# Feature Specification: spec-002 · Recibir el vehículo y abrir la orden de reparación (HU-02)

**Feature Branch**: `002-abrir-orden-reparacion`

**Created**: 2026-10-15

**Status**: Draft

**Input**: HU-02 · Como recepcionista, quiero abrir la orden de reparación al recibir el coche
anotando kilometraje y daños visibles, para evitar discusiones posteriores sobre el estado en que
se entregó. Prioridad Alta · 3 pts.

**Responsable**: Persona A

## Clarifications

### Session 2026-11-02

- Q: Si un cliente llega al taller sin cita, ¿qué puede hacer la recepción ese mismo día, dado que
  spec-001 solo ofrece días desde mañana (FR-006) y no admite una segunda cita pendiente (FR-010)?
  → A: En la v1 no se atiende sin cita el mismo día: el cliente pide cita (desde mañana) y la orden
  se abre el día de esa cita. Toda orden nace de una cita; atender sin cita queda como trabajo
  futuro (cierra el hallazgo I2 de spec-001).
- Q: ¿Se puede abrir la orden de una cita cuya fecha no es hoy (futura, o pasada y sin orden)? →
  A: No. Solo el mismo día de la cita (hoy en Europe/Madrid). En otro día no se ofrece «Abrir
  orden» y, si se intenta, se rechaza con «Solo se puede abrir la orden el día de la cita» (cierra
  el hallazgo I6 de spec-001).
- Q: ¿A quién se puede asignar como mecánico al abrir la orden? → A: Solo a usuarios con rol
  mecánico y activos. La lista solo ofrece esos; con cualquier otro valor (de baja, de otro rol o
  inexistente) la orden no se guarda y aparece «Elige un mecánico de la lista».
- Q: ¿Es obligatorio rellenar los daños visibles, y cómo se registra un coche sin ninguno? → A:
  Obligatorio. Si no hay daños, se escribe «Sin daños visibles». Vacío o solo espacios: no se
  guarda y aparece «Describe los daños visibles o escribe “Sin daños visibles”».
- Q: En la lista de citas del día, ¿qué se ve en una cita que ya tiene orden, y qué ocurre si se
  intenta abrirle una segunda? → A: En lugar de «Abrir orden», la cita muestra «Orden Nº 42» (su
  número) como enlace a la orden. Un segundo intento (p. ej. desde otra pestaña) no se guarda, no
  consume número y muestra «Esta cita ya tiene una orden (Nº 42)».

### Session 2026-11-03

- Q: ¿Cómo debe redactarse SC-002 para que una persona pueda comprobarlo en la aplicación con los
  datos demo, sin tener que abrir diez órdenes? → A: Tras abrir seguidas varias órdenes, con
  intentos rechazados entre medias (datos incorrectos, una segunda pestaña o un km menor), sus
  números son consecutivos al último existente, sin huecos ni repetidos; se ve en la lista de
  citas («Orden Nº N»). La apertura simultánea queda en FR-002 y en las pruebas automáticas
  (cierra el hallazgo C1 de analyze-1).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Abrir la orden al recibir el coche (Priority: P1)

El día de la cita, la recepcionista busca la cita en la lista del día, pulsa «Abrir orden»,
anota los kilómetros del cuentakilómetros, describe los golpes o arañazos visibles, elige el
mecánico asignado y la fecha de entrega prometida al cliente, y guarda. La orden recibe
automáticamente su número y queda en estado «recibida».

**Why this priority**: La orden es la pieza central de todo el flujo; sin ella no hay
diagnóstico, presupuesto ni factura.

**Independent Test**: Con una cita del día, abrir la orden y comprobar que tiene número, los
km, los daños, el mecánico y la fecha prometida, y que está en estado «recibida».

**Acceptance Scenarios**:

1. **Given** una cita para hoy, **When** la recepcionista abre la orden con 85.000 km, daños
   «rayón puerta trasera izquierda», mecánico «Luis» y entrega prometida dentro de 3 días,
   **Then** la orden se guarda en estado «recibida» con todos esos datos.
2. **Given** la última orden abierta tiene el número 41, **When** se abre una nueva orden,
   **Then** recibe el número 42.
3. **Given** una orden abierta, **When** se consulta, **Then** muestra la cita, el vehículo
   (matrícula) y el mecánico asignado a los que está vinculada.
4. **Given** una cita de hoy que ya tiene la orden Nº 42, **When** la recepcionista consulta las
   citas de hoy, **Then** esa cita muestra «Orden Nº 42» como enlace a la orden, en lugar de
   «Abrir orden»; y si en otra pestaña que aún mostraba «Abrir orden» se intenta abrir otra, no se
   guarda, no se consume ningún número y aparece «Esta cita ya tiene una orden (Nº 42)».
5. **Given** una cita para mañana o una cita de ayer sin orden, **When** la recepcionista consulta
   las citas de ese día, **Then** no ve «Abrir orden» en esa cita; y si se intenta abrir la orden
   de todas formas, no se guarda y aparece «Solo se puede abrir la orden el día de la cita».
6. **Given** un cliente llega hoy sin cita, **When** la recepcionista consulta las citas de hoy y
   después pide una cita para su vehículo en el formulario público, **Then** ese vehículo no
   aparece en la lista de hoy, así que no hay ninguna orden que abrir, y el formulario solo ofrece
   días a partir de mañana.
7. **Given** los mecánicos activos Luis y Marta, y el mecánico Pedro dado de baja, **When** la
   recepcionista abre la orden de una cita de hoy, **Then** la lista de mecánicos ofrece a Luis y
   a Marta, sin Pedro ni usuarios de otros roles (recepción, almacén o jefe de taller); y si Marta
   se da de baja mientras el formulario la mostraba y se guarda con ella, la orden no se guarda, no
   se consume ningún número y aparece «Elige un mecánico de la lista».
8. **Given** una cita de hoy, **When** la recepcionista deja vacía la descripción de los daños (o
   solo con espacios) y guarda, **Then** la orden no se guarda, no se consume ningún número y
   aparece «Describe los daños visibles o escribe “Sin daños visibles”»; y si escribe «Sin daños
   visibles», la orden se guarda con ese texto.
9. **Given** la última orden abierta tiene el número 41 y hay dos citas de hoy sin orden, **When**
   la recepcionista intenta abrir la primera con un dato incorrecto, lo corrige y la guarda, y
   después abre la segunda en dos pestañas y guarda en las dos, **Then** las dos órdenes reciben
   los números 42 y 43, los intentos rechazados no han gastado ningún número, y la lista de citas
   muestra «Orden Nº 42» y «Orden Nº 43».

---

### User Story 2 - Control del kilometraje (Priority: P1)

Si el kilometraje anotado es menor que el registrado en la visita anterior del mismo vehículo,
el sistema lo rechaza y muestra el valor previo.

**Why this priority**: Es la medida de control que pide el taller para evitar errores y
discusiones.

**Independent Test**: Para un vehículo con una visita anterior de 85.000 km, intentar abrir
una orden con 80.000 km y comprobar el rechazo con el mensaje que muestra 85.000 km.

**Acceptance Scenarios**:

1. **Given** una visita anterior con 85.000 km, **When** se anotan 80.000 km, **Then** la orden
   no se guarda y aparece «El kilometraje no puede ser menor que el de la visita anterior
   (85.000 km)».
2. **Given** una visita anterior con 85.000 km, **When** se anotan 85.000 km, **Then** la orden
   se guarda.
3. **Given** un vehículo sin visitas anteriores, **When** se anota cualquier kilometraje mayor
   o igual a 0, **Then** la orden se guarda.

### Edge Cases

- Dos recepcionistas abren órdenes a la vez: cada una recibe un número distinto y
  consecutivo, sin repetir ni saltar ninguno.
- Se intenta guardar una orden con datos inválidos (km vacío o negativo): no se guarda y no se
  consume ningún número.
- Se elige un mecánico que entretanto se ha dado de baja (o llega un valor que no es un mecánico
  activo): no se guarda y aparece «Elige un mecánico de la lista»; no se consume ningún número.
- Se intenta abrir una segunda orden para la misma cita (p. ej. desde otra pestaña): no se guarda,
  no se consume ningún número y aparece «Esta cita ya tiene una orden (Nº 42)», con el número de la
  orden existente.
- El coche no tiene daños visibles: se escribe «Sin daños visibles» y se guarda. Si el campo se
  deja vacío o solo con espacios, no se guarda y aparece «Describe los daños visibles o escribe
  “Sin daños visibles”»; un campo vacío nunca equivale a «sin daños».
- La fecha de entrega prometida es anterior a hoy: no se permite.
- La cita es de otro día (futura, o pasada y sin orden porque el cliente no vino): no se puede
  abrir su orden. Si el cliente llega otro día, pide una cita nueva (una cita pasada ya no cuenta
  como pendiente en spec-001 FR-010).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: La recepción MUST poder abrir una orden de reparación a partir de una cita, solo
  el mismo día de la cita (hoy en Europe/Madrid). Con una cita de otro día, anterior o posterior,
  el sistema MUST NOT ofrecer abrir la orden y MUST rechazarlo con «Solo se puede abrir la orden el
  día de la cita».
- **FR-002**: El sistema MUST asignar a cada orden un número correlativo automático, sin huecos
  ni repeticiones, también cuando se abren órdenes a la vez.
- **FR-003**: La orden MUST registrar los kilómetros del cuentakilómetros (número entero, mayor
  o igual a 0) y una descripción en texto de los golpes o arañazos visibles. La descripción es
  obligatoria: si no hay daños se escribe «Sin daños visibles»; vacía o solo con espacios, el
  sistema MUST NOT guardar la orden y MUST mostrar «Describe los daños visibles o escribe “Sin
  daños visibles”».
- **FR-004**: El sistema MUST rechazar un kilometraje menor que el de la visita anterior del
  mismo vehículo y MUST mostrar el valor previo en el mensaje.
- **FR-005**: La orden MUST quedar vinculada a la cita, al vehículo y a un mecánico asignado. El
  mecánico MUST ser un usuario con rol mecánico y activo; el formulario solo ofrece esos y, con
  cualquier otro valor, el sistema MUST NOT guardar la orden y MUST mostrar «Elige un mecánico de
  la lista».
- **FR-006**: La orden MUST registrar la fecha de entrega prometida al cliente (necesaria para
  el tablero, spec-008).
- **FR-007**: Una orden recién abierta MUST quedar en estado «recibida».
- **FR-008**: Una cita MUST tener como máximo una orden. En la lista de citas, una cita con orden
  MUST mostrar «Orden Nº <número>» como enlace a esa orden en lugar de «Abrir orden». Un intento de
  abrir una segunda orden MUST NOT guardarse ni consumir número, y MUST mostrar «Esta cita ya tiene
  una orden (Nº <número>)».

### Key Entities

- **Orden de reparación**: número correlativo, fecha de apertura, kilómetros, daños visibles,
  fecha de entrega prometida, estado; vinculada a una cita, un vehículo y un mecánico.
- **Mecánico**: miembro del personal del taller al que se asignan órdenes: un usuario con rol
  mecánico y activo (los dados de baja no se pueden asignar).
- **Estados de la orden** (comunes a todas las specs): recibida, en diagnóstico, en reparación,
  en espera de piezas, pendiente de conformidad, terminada y facturada.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: La recepcionista abre una orden en menos de 2 minutos desde la lista de citas.
- **SC-002**: Tras abrir seguidas varias órdenes, con intentos rechazados entre medias (datos
  incorrectos, una segunda pestaña o un km menor), sus números son consecutivos al último
  existente, sin huecos ni repetidos, y así se ven en la lista de citas («Orden Nº N»). La
  apertura simultánea de órdenes la garantiza FR-002 y la comprueban las pruebas automáticas.
- **SC-003**: El 100 % de los intentos de anotar un kilometraje menor que el anterior se
  rechazan mostrando el valor previo.
- **SC-004**: Toda orden muestra a qué cita, vehículo y mecánico pertenece.

## Assumptions

- Toda orden nace de una cita. Atender a un cliente que llega sin cita el mismo día queda fuera
  de la v1 (principio III): ese cliente pide cita desde la página pública (spec-001, días desde
  mañana) y su orden se abre el día de esa cita. Queda como trabajo futuro.
- Los daños se describen solo con texto; las fotos quedan fuera de la v1.
- La lista de mecánicos ya existe en el sistema (personal del taller dado de alta en la
  configuración inicial).
- La fecha de entrega prometida es un requisito derivado de la narrativa («fecha de entrega que
  se le prometió al cliente»); se pide aquí porque es el momento en que se recibe el coche.
- Por «visita anterior» se entiende la orden más reciente del mismo vehículo.
