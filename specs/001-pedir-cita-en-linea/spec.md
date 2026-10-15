# Feature Specification: spec-001 · Pedir cita en línea (HU-01)

**Feature Branch**: `001-pedir-cita-en-linea`

**Created**: 2026-10-15

**Status**: Draft

**Input**: HU-01 · Como cliente del taller, quiero pedir cita indicando la matrícula y el motivo de
la visita, para no tener que llamar ni desplazarme al taller para reservar. Prioridad Alta · 3 pts.

**Responsable**: Persona A

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
   día, **Then** la cita aparece con la matrícula y el motivo.

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
- El cliente deja el motivo vacío: no se permite confirmar y se indica que el motivo es
  obligatorio.
- No hay ningún día disponible en el periodo ofrecido: se muestra un mensaje indicando que no
  hay fechas libres y que llame al taller.
- Al escribir una matrícula conocida solo se muestran los datos del vehículo (marca y modelo),
  nunca los datos personales del propietario.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El sistema MUST ofrecer una página pública, sin registro previo, para pedir cita.
- **FR-002**: El sistema MUST identificar al vehículo por su matrícula, ignorando espacios,
  guiones y mayúsculas/minúsculas.
- **FR-003**: Si la matrícula existe, el sistema MUST mostrar marca y modelo del vehículo sin
  pedir que se escriban de nuevo, y MUST NOT mostrar datos personales del propietario.
- **FR-004**: Si la matrícula no existe, el sistema MUST pedir marca, modelo, nombre, teléfono y
  correo electrónico, y MUST dar de alta el vehículo y su propietario al confirmar la cita.
- **FR-005**: El sistema MUST pedir un motivo de la visita (texto libre, obligatorio).
- **FR-006**: El sistema MUST ofrecer solo días laborables (lunes a viernes) a partir del día
  siguiente y durante los próximos 30 días naturales.
- **FR-007**: El sistema MUST NOT ofrecer un día cuyo número de citas haya alcanzado el cupo
  diario, y MUST impedir que se supere el cupo aunque dos clientes confirmen a la vez.
- **FR-008**: Al confirmar, el sistema MUST mostrar al cliente la fecha de la cita y el motivo
  registrado.
- **FR-009**: El personal de recepción MUST poder ver la lista de citas de un día.

### Key Entities

- **Vehículo**: identificado por su matrícula; tiene marca y modelo y un propietario.
- **Cliente (propietario)**: nombre, teléfono y correo electrónico; puede tener varios vehículos.
- **Cita**: un vehículo, una fecha (día, sin hora) y un motivo.
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
- El correo electrónico es el canal para los avisos posteriores (enlace de presupuesto en
  spec-004 y aviso de recogida en spec-007); por eso se pide aquí. No se piden más datos
  personales (principio V).
- La recepción también puede usar este mismo formulario para registrar una cita por teléfono
  o en el mostrador.
