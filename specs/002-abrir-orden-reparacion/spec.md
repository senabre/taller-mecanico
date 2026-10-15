# Feature Specification: spec-002 · Recibir el vehículo y abrir la orden de reparación (HU-02)

**Feature Branch**: `002-abrir-orden-reparacion`

**Created**: 2026-10-15

**Status**: Draft

**Input**: HU-02 · Como recepcionista, quiero abrir la orden de reparación al recibir el coche
anotando kilometraje y daños visibles, para evitar discusiones posteriores sobre el estado en que
se entregó. Prioridad Alta · 3 pts.

**Responsable**: Persona A

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
   «rayón puerta trasera izquierda», mecánico «Luis» y entrega prometida 20/10/2026,
   **Then** la orden se guarda en estado «recibida» con todos esos datos.
2. **Given** la última orden abierta tiene el número 41, **When** se abre una nueva orden,
   **Then** recibe el número 42.
3. **Given** una orden abierta, **When** se consulta, **Then** muestra la cita, el vehículo
   (matrícula) y el mecánico asignado a los que está vinculada.

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
- Se intenta abrir una segunda orden para la misma cita: no se permite; se indica que la cita ya
  tiene orden.
- El coche no tiene daños visibles: se puede guardar indicando «Sin daños visibles».
- La fecha de entrega prometida es anterior a hoy: no se permite.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: La recepción MUST poder abrir una orden de reparación a partir de una cita.
- **FR-002**: El sistema MUST asignar a cada orden un número correlativo automático, sin huecos
  ni repeticiones, también cuando se abren órdenes a la vez.
- **FR-003**: La orden MUST registrar los kilómetros del cuentakilómetros (número entero, mayor
  o igual a 0) y una descripción en texto de los golpes o arañazos visibles.
- **FR-004**: El sistema MUST rechazar un kilometraje menor que el de la visita anterior del
  mismo vehículo y MUST mostrar el valor previo en el mensaje.
- **FR-005**: La orden MUST quedar vinculada a la cita, al vehículo y a un mecánico asignado.
- **FR-006**: La orden MUST registrar la fecha de entrega prometida al cliente (necesaria para
  el tablero, spec-008).
- **FR-007**: Una orden recién abierta MUST quedar en estado «recibida».
- **FR-008**: Una cita MUST tener como máximo una orden.

### Key Entities

- **Orden de reparación**: número correlativo, fecha de apertura, kilómetros, daños visibles,
  fecha de entrega prometida, estado; vinculada a una cita, un vehículo y un mecánico.
- **Mecánico**: miembro del personal del taller al que se asignan órdenes.
- **Estados de la orden** (comunes a todas las specs): recibida, en diagnóstico, en reparación,
  en espera de piezas, pendiente de conformidad, terminada y facturada.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: La recepcionista abre una orden en menos de 2 minutos desde la lista de citas.
- **SC-002**: Tras abrir 10 órdenes seguidas, sus números son consecutivos sin huecos ni
  repetidos.
- **SC-003**: El 100 % de los intentos de anotar un kilometraje menor que el anterior se
  rechazan mostrando el valor previo.
- **SC-004**: Toda orden muestra a qué cita, vehículo y mecánico pertenece.

## Assumptions

- Toda orden nace de una cita. Si un cliente llega sin cita, la recepción le registra primero
  la cita (spec-001) y después abre la orden.
- Los daños se describen solo con texto; las fotos quedan fuera de la v1.
- La lista de mecánicos ya existe en el sistema (personal del taller dado de alta en la
  configuración inicial).
- La fecha de entrega prometida es un requisito derivado de la narrativa («fecha de entrega que
  se le prometió al cliente»); se pide aquí porque es el momento en que se recibe el coche.
- Por «visita anterior» se entiende la orden más reciente del mismo vehículo.
