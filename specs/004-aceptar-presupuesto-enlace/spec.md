# Feature Specification: spec-004 · Aceptar o rechazar el presupuesto desde un enlace (HU-04)

**Feature Branch**: `004-aceptar-presupuesto-enlace`

**Created**: 2026-10-15

**Status**: Draft

**Input**: HU-04 · Como cliente del taller, quiero recibir un enlace desde el que aceptar o rechazar
el presupuesto, para no tener que pasar por el taller a firmarlo. Prioridad Alta · 5 pts.

**Responsable**: Persona A

## User Scenarios & Testing *(mandatory)*

### User Story 1 - El cliente acepta el presupuesto (Priority: P1)

Al emitirse un presupuesto (spec-003), el cliente recibe un correo con un enlace. Al abrirlo ve
el presupuesto completo: vehículo, diagnóstico, líneas, base imponible, IVA, total y fecha de
caducidad. Pulsa «Aceptar», confirma y ve un mensaje con la fecha y hora de su respuesta. La
orden pasa a «en reparación».

**Why this priority**: Es la regla de oro del taller: no se toca un coche sin presupuesto
aceptado.

**Independent Test**: Emitir un presupuesto, abrir el enlace recibido por correo, aceptar y
comprobar en la orden el estado «en reparación» y la fecha y hora de aceptación.

**Acceptance Scenarios**:

1. **Given** un presupuesto emitido y vigente, **When** el cliente abre el enlace, **Then** ve el
   presupuesto completo con sus líneas, base, IVA, total y fecha de caducidad.
2. **Given** el cliente pulsa «Aceptar» y confirma, **When** el personal consulta la orden,
   **Then** ve «Aceptado el 16/10/2026 a las 10:32» y la orden está «en reparación».
3. **Given** un presupuesto ya aceptado, **When** el cliente vuelve a abrir el enlace, **Then**
   ve la decisión tomada y su fecha y hora, sin botones para cambiarla.

---

### User Story 2 - El cliente rechaza el presupuesto (Priority: P1)

El cliente abre el enlace, pulsa «Rechazar» y confirma. Queda guardada la decisión con su fecha
y hora, y el taller lo ve en la orden.

**Why this priority**: El cliente debe poder decir que no con la misma comodidad.

**Independent Test**: Rechazar un presupuesto desde el enlace y comprobar en la orden el
rechazo con su fecha y hora.

**Acceptance Scenarios**:

1. **Given** un presupuesto vigente, **When** el cliente lo rechaza, **Then** se guarda
   «Rechazado» con fecha y hora exactas y la orden vuelve a «en diagnóstico» para que el
   mecánico pueda preparar otro presupuesto.
2. **Given** una ampliación (spec-005) vigente, **When** el cliente la rechaza, **Then** se guarda
   el rechazo y la orden sigue «pendiente de conformidad» hasta que se acepte otra ampliación.

---

### User Story 3 - Presupuesto caducado (Priority: P2)

Si el cliente abre el enlace después de la fecha de caducidad, ve el presupuesto marcado como
caducado y no puede aceptarlo.

**Why this priority**: Protege al taller de precios desactualizados.

**Independent Test**: Con un presupuesto cuya fecha de caducidad ya ha pasado, abrir el enlace
y comprobar que no aparece el botón de aceptar.

**Acceptance Scenarios**:

1. **Given** un presupuesto caducado ayer, **When** el cliente abre el enlace, **Then** ve
   «Este presupuesto caducó el dd/mm/aaaa. Contacte con el taller» y no puede aceptarlo.

---

### User Story 4 - Sin presupuesto aceptado no se trabaja (Priority: P1)

Mientras la orden no tenga un presupuesto aceptado, la orden no permite imputar tiempo ni
piezas.

**Why this priority**: Es la regla de oro de Paco.

**Independent Test**: En una orden con presupuesto emitido pero sin respuesta, comprobar que
las opciones de imputar tiempo y piezas no están disponibles.

**Acceptance Scenarios**:

1. **Given** una orden con presupuesto pendiente, rechazado o caducado, **When** el mecánico la
   abre, **Then** no puede imputar tiempo ni piezas.

### Edge Cases

- El cliente pulsa «Aceptar» dos veces o desde dos pestañas: solo cuenta la primera respuesta.
- Alguien modifica el enlace a mano: no ve ningún presupuesto que no sea el suyo.
- El presupuesto caduca mientras el cliente lo tiene abierto: al pulsar «Aceptar» se le indica
  que ha caducado y no se guarda la aceptación.
- El correo no llega: el personal puede ver y copiar el enlace desde la orden para enviárselo
  por otro medio.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Al emitirse un presupuesto, el sistema MUST enviar al correo del cliente un enlace
  único a ese presupuesto.
- **FR-002**: El enlace MUST permitir ver el presupuesto completo sin registrarse ni iniciar
  sesión, y MUST NOT permitir ver ningún otro dato del taller ni de otros clientes.
- **FR-003**: Desde el enlace, el cliente MUST poder aceptar o rechazar el presupuesto una sola
  vez, con un paso de confirmación.
- **FR-004**: El sistema MUST guardar la decisión con la fecha y hora exactas de la respuesta y
  mostrarlas en la orden.
- **FR-005**: El sistema MUST impedir aceptar un presupuesto caducado.
- **FR-006**: Al aceptarse el presupuesto, la orden MUST pasar a «en reparación».
- **FR-007**: Mientras la orden no tenga un presupuesto aceptado, el sistema MUST NOT permitir
  imputar tiempo ni piezas.
- **FR-008**: El personal MUST poder ver en la orden el enlace del presupuesto para reenviarlo.
- **FR-009**: Este mismo mecanismo de enlace MUST usarse para que el cliente acepte las
  ampliaciones de presupuesto (spec-005).

### Key Entities

- **Presupuesto**: (definido en spec-003) se le añaden la decisión del cliente (aceptado o
  rechazado) y la fecha y hora de la respuesta.
- **Enlace de presupuesto**: dirección única y difícil de adivinar que da acceso a un único
  presupuesto.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: El cliente acepta o rechaza un presupuesto en menos de 1 minuto desde que abre el
  enlace, sin crear ninguna cuenta.
- **SC-002**: El 100 % de las respuestas quedan registradas con fecha y hora visibles en la orden.
- **SC-003**: Ningún presupuesto caducado puede aceptarse (comprobable abriendo el enlace de un
  presupuesto caducado).
- **SC-004**: Ninguna orden sin presupuesto aceptado permite imputar tiempo ni piezas.

## Assumptions

- El enlace se envía por correo electrónico, usando el correo recogido en la cita (spec-001).
- La hora registrada es la hora oficial de España peninsular.
- Si el cliente no responde, el presupuesto simplemente caduca a los 15 días; no se envían
  recordatorios en la v1.
- Una firma manuscrita o digital certificada no es necesaria; basta la decisión con fecha y hora.
- Cerrar una orden sin reparar (el cliente rechaza todo y se lleva el coche) queda fuera de la v1
  (decisión Q1-A, 2026-10-15).
