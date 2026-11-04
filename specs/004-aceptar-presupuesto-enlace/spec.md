# Feature Specification: spec-004 · Aceptar o rechazar el presupuesto desde un enlace (HU-04)

**Feature Branch**: `004-aceptar-presupuesto-enlace`

**Created**: 2026-10-15

**Status**: Draft

**Input**: HU-04 · Como cliente del taller, quiero recibir un enlace desde el que aceptar o rechazar
el presupuesto, para no tener que pasar por el taller a firmarlo. Prioridad Alta · 5 pts.

**Responsable**: Persona A

## Clarifications

### Session 2026-11-04

- Q: ¿Qué respuestas del cliente debe enseñar la ficha de la orden, con su fecha y hora, y de dónde
  sale ese dato? → A: Todas las respuestas de la orden, no solo la del último presupuesto
  («Aceptado el 16/10/2026 a las 10:32», «Rechazado el …»), indicando si es inicial o ampliación. El
  resumen de cada presupuesto de la ficha trae su fecha y hora de respuesta (cierra el apartado a
  del hallazgo I5 de spec-002).
- Q: Cuando el cliente rechaza el presupuesto inicial, ¿debe quedar en la máquina de estados una
  transición de «en diagnóstico» a «en diagnóstico»? → A: No, se elimina. Rechazar no cambia el
  estado de la orden: solo se guarda la decisión, y la orden sigue «en diagnóstico» (cierra el
  hallazgo I5 de la base común).
- Q: Cuando un presupuesto ya ha caducado, ¿puede el cliente rechazarlo desde el enlace? → A: No.
  Un presupuesto caducado no admite ninguna respuesta, ni aceptar ni rechazar, tanto el inicial
  como una ampliación: la página no muestra botones y cualquier intento se rechaza con «Este
  presupuesto caducó el dd/mm/aaaa. Contacte con el taller».
- Q: Para que solo cuente la primera respuesta también con dos pestañas a la vez, ¿cómo se guarda?
  → A: En una sola operación, que comprueba a la vez que el presupuesto sigue emitido y sin caducar;
  la segunda respuesta no se guarda y ve «Este presupuesto ya fue respondido». El cambio de estado
  de la orden se hace junto con la respuesta (cierra el hallazgo D1 de la base común).
- Q: ¿Quién del personal puede ver y copiar el enlace del presupuesto en la orden, y durante cuánto
  tiempo? → A: Todo el personal con sesión, solo mientras el presupuesto está emitido y vigente:
  después de la respuesta o de la caducidad, la orden deja de mostrarlo. El cliente puede seguir
  abriendo el enlace para ver su decisión.

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
4. **Given** un presupuesto vigente abierto en dos pestañas, **When** el cliente confirma en las dos
   a la vez, **Then** solo se guarda una respuesta, la orden cambia una sola vez, y la otra pestaña
   ve «Este presupuesto ya fue respondido».
5. **Given** un presupuesto emitido y vigente, **When** cualquier persona del personal con sesión
   abre la orden, **Then** ve el enlace y «Copiar enlace»; y cuando el presupuesto ya tiene respuesta
   o ha caducado, la orden deja de mostrarlos, mientras el cliente sigue pudiendo abrir el enlace y
   ver la decisión tomada o el aviso de caducidad.

---

### User Story 2 - El cliente rechaza el presupuesto (Priority: P1)

El cliente abre el enlace, pulsa «Rechazar» y confirma. Queda guardada la decisión con su fecha
y hora, y el taller lo ve en la orden.

**Why this priority**: El cliente debe poder decir que no con la misma comodidad.

**Independent Test**: Rechazar un presupuesto desde el enlace y comprobar en la orden el
rechazo con su fecha y hora.

**Acceptance Scenarios**:

1. **Given** un presupuesto vigente, **When** el cliente lo rechaza, **Then** se guarda
   «Rechazado» con fecha y hora exactas y la orden sigue «en diagnóstico» para que el
   mecánico pueda preparar otro presupuesto.
2. **Given** una ampliación (spec-005) vigente, **When** el cliente la rechaza, **Then** se guarda
   el rechazo y la orden sigue «pendiente de conformidad» hasta que se acepte otra ampliación.
3. **Given** un presupuesto inicial rechazado y después otro aceptado en la misma orden, **When** el
   personal consulta la orden, **Then** ve las dos respuestas, cada una con su fecha y hora y
   diciendo si es inicial o ampliación, aunque ya exista un presupuesto más reciente.

---

### User Story 3 - Presupuesto caducado (Priority: P2)

Si el cliente abre el enlace después de la fecha de caducidad, ve el presupuesto marcado como
caducado y no puede responder: ni aceptarlo ni rechazarlo.

**Why this priority**: Protege al taller de precios desactualizados.

**Independent Test**: Con un presupuesto cuya fecha de caducidad ya ha pasado, abrir el enlace
y comprobar que no aparecen los botones de aceptar ni de rechazar.

**Acceptance Scenarios**:

1. **Given** un presupuesto caducado ayer, inicial o ampliación, **When** el cliente abre el enlace,
   **Then** ve «Este presupuesto caducó el dd/mm/aaaa. Contacte con el taller», sin botones, y no
   puede aceptarlo ni rechazarlo.
2. **Given** un presupuesto que caduca mientras el cliente lo tiene abierto, **When** pulsa
   «Aceptar» o «Rechazar» y confirma, **Then** ve el mismo aviso y no se guarda ninguna respuesta.

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

- El cliente pulsa «Aceptar» dos veces o desde dos pestañas: solo cuenta la primera respuesta, y
  la otra ve «Este presupuesto ya fue respondido».
- Alguien modifica el enlace a mano: no ve ningún presupuesto que no sea el suyo.
- El presupuesto caduca mientras el cliente lo tiene abierto: al pulsar «Aceptar» o «Rechazar» se
  le indica que ha caducado y no se guarda ninguna respuesta.
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
  mostrarlas en la orden. La orden MUST mostrar todas las respuestas de sus presupuestos, no solo la
  del más reciente.
- **FR-005**: El sistema MUST impedir responder a un presupuesto caducado: ni aceptarlo ni
  rechazarlo. La regla es la misma para el presupuesto inicial y para las ampliaciones (FR-009).
- **FR-006**: Al aceptarse el presupuesto, la orden MUST pasar a «en reparación».
- **FR-007**: Mientras la orden no tenga un presupuesto aceptado, el sistema MUST NOT permitir
  imputar tiempo ni piezas.
- **FR-008**: Todo el personal con sesión MUST poder ver y copiar en la orden el enlace del
  presupuesto para reenviarlo, mientras esté emitido y vigente. Después de la respuesta o de la
  caducidad, la orden MUST NOT mostrarlo; el enlace MUST seguir funcionando para el cliente, que ve
  su decisión o el aviso de caducidad sin poder cambiar nada.
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
