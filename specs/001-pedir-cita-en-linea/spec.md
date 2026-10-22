# Feature Specification: spec-001 · Pedir cita en línea (HU-01)

**Feature Branch**: `001-pedir-cita-en-linea`

**Created**: 2026-10-15

**Status**: Draft

**Input**: HU-01 · Como cliente del taller, quiero pedir cita indicando la matrícula y el motivo de
la visita, para no tener que llamar ni desplazarme al taller para reservar. Prioridad Alta · 3 pts.

**Responsable**: Persona A

## Clarifications

### Session 2026-10-22

- Q: ¿Qué matrículas acepta el formulario de cita como válidas, una vez normalizadas? → A: Solo
  letras y cifras, de 2 a 10 caracteres; si no cumple, se avisa «La matrícula no es válida», no se
  busca el vehículo y no se deja confirmar (cierra el hallazgo I11 de la base común).
- Q: ¿Qué teléfono y qué correo electrónico acepta el formulario cuando el vehículo es nuevo? →
  A: Ambos obligatorios. Teléfono de 9 a 15 cifras con «+» inicial opcional (se ignoran espacios
  y guiones); correo con una sola «@», texto delante y dominio con punto detrás. Cada campo
  incorrecto tiene su propio mensaje y no se deja confirmar. El teléfono sirve para que recepción
  llame al cliente si hay algún problema con la cita.
- Q: ¿Puede una misma matrícula tener más de una cita pendiente (fecha de hoy o posterior)? →
  A: No. Una sola cita pendiente por matrícula; si ya la tiene, se avisa sin mostrar la fecha y no
  se deja confirmar. Cuando su fecha ha pasado, se puede pedir otra.
- Q: ¿Quién puede ver la lista de citas de un día y qué datos muestra cada cita? → A: Solo
  recepción y jefe de taller, con sesión iniciada. Por defecto muestra hoy y se puede elegir otro
  día. Cada cita muestra matrícula, marca, modelo, motivo, nombre y teléfono, en el orden en que
  se pidieron. Sin citas: «No hay citas para este día».
- Q: Con una matrícula conocida cuyos datos de contacto han cambiado, ¿qué permite hacer el
  formulario? → A: No se piden ni se pueden cambiar: solo se piden día y motivo, la cita queda
  asociada al propietario registrado y no se muestra ningún aviso. Editar clientes queda fuera de
  esta spec.
- Q: ¿El aviso de FR-010 debe invitar a llamar al taller para cambiar la cita pendiente? → A: No.
  El aviso queda en «Este vehículo ya tiene una cita pendiente.», porque cancelar o modificar una
  cita queda fuera de esta spec y ningún mensaje debe prometer un cambio que la v1 no permite.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Cliente con vehículo ya conocido pide cita (Priority: P1)

Un cliente entra en la página pública de citas, escribe la matrícula de su coche, que ya ha
estado antes en el taller, y ve la marca y el modelo sin tener que escribirlos. Elige un día
disponible, escribe el motivo de la visita y confirma. La pantalla le muestra la fecha de la
cita y el motivo registrado.

**Why this priority**: Es la entrada natural de todo el proceso del taller y la mayoría de los
clientes son habituales.

**Independent Test**: Con un vehículo ya registrado, pedir una cita desde la página pública y
comprobar que la confirmación muestra la fecha y el motivo.

**Acceptance Scenarios**:

1. **Given** un vehículo con matrícula 1234 BCD ya registrado, **When** el cliente escribe
   «1234BCD» en el formulario de cita, **Then** se muestran su marca y modelo sin escribirlos.
2. **Given** el cliente ha elegido un día disponible y escrito el motivo, **When** confirma,
   **Then** ve un mensaje con la fecha de la cita (dd/mm/aaaa) y el motivo registrado.
3. **Given** la cita confirmada, **When** el personal de recepción consulta las citas de ese
   día, **Then** la cita aparece con matrícula, marca, modelo, motivo, nombre y teléfono del
   propietario, y sin el correo.
4. **Given** el vehículo 1234 BCD ya tiene una cita con fecha de hoy o posterior, **When** el
   cliente escribe «1234BCD», **Then** ve exactamente el aviso «Este vehículo ya tiene una cita
   pendiente.», sin la fecha de esa cita ni ninguna invitación a llamar al taller o a cambiarla, y
   no puede confirmar otra.
5. **Given** el formulario de cita abierto, **When** el cliente escribe «12#4», «A» o
   «ABCDE123456» (11 caracteres), **Then** ve «La matrícula no es válida», no se muestran marca ni
   modelo ni se piden datos de vehículo nuevo, y no puede confirmar; en cambio, «AB» y
   «ABCDE12345» (10 caracteres) se aceptan como matrículas.
6. **Given** el vehículo 1234 BCD tuvo una cita con fecha anterior a hoy y no tiene ninguna otra,
   **When** el cliente escribe «1234BCD», elige un día disponible, escribe el motivo y confirma,
   **Then** ve la confirmación con la fecha y el motivo de la nueva cita.
7. **Given** el vehículo 1234 BCD ya registrado con un propietario, **When** el cliente escribe
   «1234BCD», **Then** el formulario solo pide día y motivo, sin campos de nombre, teléfono o
   correo ni ningún aviso para cambiarlos; y, al confirmar, la lista de recepción muestra la cita
   con el nombre y el teléfono del propietario registrado, sin cambios.
8. **Given** un usuario con rol recepción o jefe de taller con sesión iniciada, **When** abre la
   lista de citas, **Then** ve las citas de hoy y puede elegir otro día; si dos citas de ese día
   se pidieron primero para 1234 BCD y después para 5678 CDF, aparecen en ese orden.
9. **Given** un usuario con rol recepción con sesión iniciada, **When** elige en la lista un día
   sin citas, **Then** ve «No hay citas para este día».
10. **Given** un usuario con rol mecánico o almacén con sesión iniciada, o alguien sin sesión
    iniciada, **When** busca la lista de citas en el menú o intenta abrir su página, **Then** no
    ve la opción en el menú y no puede abrir la lista.

---

### User Story 2 - Cliente con vehículo nuevo pide cita (Priority: P1)

Un cliente cuyo coche nunca ha estado en el taller escribe la matrícula. El sistema no la
conoce, así que le pide los datos mínimos del vehículo y del contacto. Al confirmar la cita,
el vehículo queda dado de alta junto con ella.

**Why this priority**: Sin esto ningún cliente nuevo puede entrar en el sistema.

**Independent Test**: Pedir cita con una matrícula inexistente, rellenar los datos solicitados
y comprobar que, al pedir otra cita con la misma matrícula, el vehículo ya se reconoce.

**Acceptance Scenarios**:

1. **Given** una matrícula que no existe, **When** el cliente la escribe, **Then** el formulario
   le pide marca, modelo, nombre, teléfono y correo electrónico.
2. **Given** el cliente ha rellenado esos datos, elegido día y motivo, **When** confirma,
   **Then** se crean el vehículo y la cita y ve la confirmación con fecha y motivo.
3. **Given** una matrícula que no existe y el resto de datos, día y motivo rellenados, **When**
   el cliente escribe el teléfono «12345» o el correo «ana@correo» (sin punto en el dominio), o
   deja vacío alguno de los dos, e intenta confirmar, **Then** ve «El teléfono no es válido» o
   «El correo no es válido» en el campo afectado y no puede confirmar.
4. **Given** una matrícula que no existe y el resto de datos, día y motivo rellenados, **When**
   el cliente escribe el teléfono «+34 612 345 678» o «612-345-678» y el correo «ana@correo.es» y
   confirma, **Then** se crean el vehículo y la cita y ve la confirmación con fecha y motivo.

---

### User Story 3 - Los días completos no se ofrecen (Priority: P1)

El taller solo recibe un número máximo de coches al día (cupo diario). Cuando un día alcanza
ese cupo, deja de ofrecerse como disponible.

**Why this priority**: Es la regla que evita saturar el taller; sin ella la cita en línea
genera problemas en lugar de resolverlos.

**Independent Test**: Con un cupo de 2, pedir dos citas para el mismo día y comprobar que en
el tercer intento ese día ya no aparece como disponible.

**Acceptance Scenarios**:

1. **Given** un cupo diario de 2 y un día con 2 citas, **When** un cliente abre el selector de
   fecha, **Then** ese día no aparece como disponible.
2. **Given** un día con 1 cita y cupo 2, **When** dos clientes intentan confirmar a la vez la
   última plaza, **Then** solo uno lo consigue y el otro ve un aviso de que el día se ha
   completado y puede elegir otro.

### Edge Cases

- La matrícula se escribe con espacios, guiones o minúsculas: se considera la misma
  matrícula («1234-bcd» = «1234BCD»).
- La matrícula, ya normalizada, tiene caracteres que no son letras ni cifras («12#4»), menos de
  2 caracteres («A») o más de 10: se muestra «La matrícula no es válida», no se muestran datos
  del vehículo ni se piden datos nuevos, y no se puede confirmar.
- Vehículo nuevo con teléfono «12345» o correo «ana@correo» (sin punto en el dominio): se muestra
  el mensaje del campo afectado y no se puede confirmar; «+34 612 345 678» y «612-345-678» son
  teléfonos válidos.
- El cliente deja el motivo vacío: no se permite confirmar y se indica que el motivo es
  obligatorio.
- No hay ningún día disponible en el periodo ofrecido: se muestra «No quedan fechas libres en los
  próximos 30 días. Llama al taller para pedir cita.» y no se puede confirmar.
- Un usuario con rol mecánico o almacén, o alguien sin sesión iniciada, intenta abrir la lista
  de citas: no la ve en el menú y no puede abrirla.
- Recepción elige un día sin citas: se muestra «No hay citas para este día».
- Al escribir una matrícula conocida solo se muestran los datos del vehículo (marca y modelo),
  nunca los datos personales del propietario.
- Un vehículo conocido llega con otro teléfono, otro correo u otro dueño: el formulario no ofrece
  campos de contacto. La cita se guarda con el propietario registrado, cuyos datos no cambian.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El sistema MUST ofrecer una página pública, sin registro previo, para pedir cita.
- **FR-002**: El sistema MUST identificar al vehículo por su matrícula, ignorando espacios,
  guiones y mayúsculas/minúsculas. Una vez normalizada, la matrícula es válida solo si tiene
  entre 2 y 10 caracteres y todos son letras (A-Z) o cifras; si no, el sistema MUST mostrar
  «La matrícula no es válida», MUST NOT buscar el vehículo y MUST NOT permitir confirmar.
- **FR-003**: Si la matrícula existe, el sistema MUST mostrar marca y modelo del vehículo sin
  pedir que se escriban de nuevo, y MUST NOT mostrar datos personales del propietario. En ese caso
  el formulario solo pide día y motivo: MUST NOT pedir ni permitir cambiar nombre, teléfono o
  correo, y la cita queda asociada al propietario ya registrado.
- **FR-004**: Si la matrícula no existe, el sistema MUST pedir marca, modelo, nombre, teléfono y
  correo electrónico (todos obligatorios), y MUST dar de alta el vehículo y su propietario al
  confirmar la cita.
- **FR-004a**: El teléfono es válido si, tras quitar espacios y guiones, tiene de 9 a 15 cifras,
  con un «+» inicial opcional; si no, el sistema MUST mostrar «El teléfono no es válido». El
  correo es válido si contiene una sola «@», con texto delante y, detrás, un dominio que incluye
  un punto con texto a ambos lados (p. ej. ana@correo.es); si no, el sistema MUST mostrar «El
  correo no es válido». Con cualquiera de los dos campos incorrecto o vacío, el sistema MUST NOT
  permitir confirmar.
- **FR-005**: El sistema MUST pedir un motivo de la visita (texto libre, obligatorio).
- **FR-006**: El sistema MUST ofrecer solo días laborables (lunes a viernes) a partir del día
  siguiente y durante los próximos 30 días naturales.
- **FR-007**: El sistema MUST NOT ofrecer un día cuyo número de citas haya alcanzado el cupo
  diario, y MUST impedir que se supere el cupo aunque dos clientes confirmen a la vez.
- **FR-008**: Al confirmar, el sistema MUST mostrar al cliente la fecha de la cita y el motivo
  registrado.
- **FR-009**: Los usuarios con rol recepción o jefe de taller, con sesión iniciada, MUST poder
  ver la lista de citas de un día. La lista abre con el día de hoy y permite elegir otro día. Cada
  cita muestra matrícula, marca, modelo, motivo, nombre y teléfono del propietario (nunca el
  correo), en el orden en que se pidieron. Si el día no tiene citas, MUST mostrar «No hay citas
  para este día». Los demás roles MUST NOT ver esta opción en el menú ni poder abrir la lista, y
  sin sesión iniciada tampoco se puede abrir.
- **FR-010**: Una matrícula MUST NOT tener más de una cita pendiente (con fecha de hoy o
  posterior), tampoco si se confirman dos a la vez. Si ya la tiene, el sistema MUST mostrar «Este
  vehículo ya tiene una cita pendiente.», MUST NOT mostrar
  la fecha de esa cita y MUST NOT permitir confirmar. Cuando la fecha de la cita ya ha pasado, la
  matrícula puede pedir una nueva.

### Key Entities

- **Vehículo**: identificado por su matrícula; tiene marca y modelo y un propietario.
- **Cliente (propietario)**: nombre, teléfono y correo electrónico; puede tener varios vehículos.
- **Cita**: un vehículo, una fecha (día, sin hora) y un motivo. Cada vehículo tiene como mucho
  una cita pendiente (fecha de hoy o posterior).
- **Cupo diario**: número máximo de citas que el taller acepta por día.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Un cliente con vehículo ya registrado completa una cita en menos de 1 minuto
  escribiendo solo la matrícula, el día y el motivo.
- **SC-002**: Un cliente nuevo completa una cita en menos de 3 minutos.
- **SC-003**: Con cupo diario N, nunca hay más de N citas en un mismo día (comprobable pidiendo
  N+1 citas para el mismo día: la última no es posible).
- **SC-004**: Tras confirmar, el 100 % de las citas aparecen en la lista de recepción del día
  correspondiente.

## Assumptions

- Las citas son por día, sin hora concreta, porque el cupo del taller es diario.
- El cupo diario es un único valor fijado en la configuración del sistema; su edición desde
  pantalla queda fuera de esta spec.
- Los festivos no se gestionan en la v1; solo se excluyen sábados y domingos.
- Cancelar o modificar una cita queda fuera de esta spec.
- En la v1 ningún rol (tampoco recepción ni el jefe de taller) puede editar los datos de un
  cliente ya registrado ni cambiar el propietario de un vehículo, porque ninguna HU lo pide
  (principio III). Queda como trabajo futuro. Por eso ninguna pantalla muestra un mensaje que
  prometa cambiarlos.
- El correo electrónico es el canal para los avisos posteriores (enlace de presupuesto en
  spec-004 y aviso de recogida en spec-007); por eso se pide aquí. El teléfono sirve para que
  recepción llame al cliente si hay algún problema con la cita. No se piden más datos personales
  (principio V).
- La recepción también puede usar este mismo formulario para registrar una cita por teléfono
  o en el mostrador.
