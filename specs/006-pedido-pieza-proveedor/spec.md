# Feature Specification: spec-006 · Pedir al proveedor una pieza sin stock (HU-06)

**Feature Branch**: `006-pedido-pieza-proveedor`

**Created**: 2026-10-15

**Status**: Draft

**Input**: HU-06 · Como responsable del almacén de recambios, quiero pedir al proveedor una pieza que
falta dejándola vinculada a la orden que la necesita, para que la orden quede parada mientras
espera y vuelva sola a reparación cuando llegue la pieza, sin seguimientos manuales.
Prioridad Media · 3 pts.

**Responsable**: Persona B

## Clarifications

### Session 2026-10-15

- Q: Si una orden ya está «en espera de piezas» y falta otra pieza, ¿se puede añadir otro
  pedido? → A: Sí. Se admite pedido si la orden está «en reparación» o «en espera de piezas»;
  una orden puede tener varios pedidos pendientes.
- Q: ¿Se indica un stock inicial al dar de alta una referencia? → A: Sí, opcional; si no se
  indica, es 0.
- Q: Mientras una orden está «en espera de piezas» con algún pedido pendiente, ¿alguien puede
  cambiarle el estado a mano? → A: No. Nadie puede cambiarla a mano; solo sale de la espera al
  recibir su último pedido pendiente.
- Q: Si una orden ya tiene un pedido pendiente de una referencia y se registra otro de la misma
  referencia para la misma orden, ¿qué hace el sistema? → A: No se permite; se rechaza indicando
  que ya hay un pedido pendiente de esa referencia para la orden.
- Q: Cuando la orden vuelve sola a «en reparación», ¿hay que avisar al mecánico asignado? → A: No
  hay aviso específico; la orden aparece de nuevo «en reparación» en la lista del mecánico y en
  el tablero (spec-008).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Dar de alta referencias de piezas (Priority: P1)

El responsable del almacén da de alta a mano cada referencia de pieza con su código,
descripción, precio de venta y, si quiere, el stock inicial. No hay conexión con catálogos de
fabricantes.

**Why this priority**: Sin referencias no se pueden presupuestar, imputar ni pedir piezas.

**Independent Test**: Dar de alta la referencia «PF-123 Pastillas freno», 25,00 €, stock 4, y
comprobar que aparece en la lista del almacén y al presupuestar (spec-003).

**Acceptance Scenarios**:

1. **Given** el almacén, **When** se da de alta «PF-123», «Pastillas freno», 25,00 €, stock 4,
   **Then** la referencia aparece en la lista con esos datos.
2. **Given** el almacén, **When** se da de alta «FA-77», «Filtro aire», 12,50 € sin indicar
   stock, **Then** la referencia aparece con stock 0.
3. **Given** ya existe «PF-123», **When** se intenta dar de alta otra con el mismo código,
   **Then** no se permite y se indica que ese código ya existe.

---

### User Story 2 - Pedir una pieza vinculada a una orden (Priority: P1)

Falta una pieza para una orden «en reparación». El responsable registra el pedido indicando
proveedor, referencia, cantidad y orden. El pedido queda «pendiente» con la fecha del día y la
orden pasa sola a «en espera de piezas», donde no se puede imputar tiempo ni piezas.

**Why this priority**: Es el núcleo de la historia: dejar la pieza ligada a la orden y parar la
orden mientras falta.

**Independent Test**: Registrar un pedido para una orden «en reparación» y comprobar que la
orden pasa a «en espera de piezas» y muestra el pedido pendiente.

**Acceptance Scenarios**:

1. **Given** una orden 42 «en reparación», **When** se registra el pedido «Recambios Sur»,
   «PF-123», 2 uds, orden 42, **Then** el pedido queda «pendiente» con la fecha de hoy y la
   orden 42 pasa a «en espera de piezas».
2. **Given** la orden 42 «en espera de piezas», **When** se registra otro pedido para ella,
   **Then** se admite y la orden sigue «en espera de piezas».
3. **Given** una orden 50 «en diagnóstico» (o cualquier estado distinto de «en reparación» y
   «en espera de piezas»), **When** se intenta registrar un pedido para ella, **Then** no se
   permite y se indica el motivo (la orden no está en reparación).
4. **Given** la orden 42 «en espera de piezas», **When** el mecánico intenta imputar tiempo o
   piezas, **Then** no se permite (spec-005).
5. **Given** la orden 42 con un pedido pendiente de «PF-123», **When** se intenta registrar otro
   pedido de «PF-123» para la orden 42, **Then** no se permite y se indica que ya hay un pedido
   pendiente de esa referencia para la orden.

---

### User Story 3 - Recibir el pedido (Priority: P1)

Cuando llega la pieza, el responsable marca el pedido como recibido. La cantidad entra en stock
y, si la orden no espera ningún otro pedido, vuelve sola a «en reparación».

**Why this priority**: Evita que la orden se quede olvidada en espera.

**Independent Test**: Marcar como recibido el pedido de la orden 42 y comprobar que el stock sube
y que la orden vuelve a «en reparación» sin ninguna otra acción.

**Acceptance Scenarios**:

1. **Given** «PF-123» con stock 0 y un pedido pendiente de 2 uds para la orden 42, **When** se
   marca como recibido, **Then** el pedido queda «recibido», el stock pasa a 2 y la orden 42
   vuelve a «en reparación».
2. **Given** la orden 42 con dos pedidos pendientes, **When** solo se recibe uno, **Then** la
   orden sigue «en espera de piezas» hasta recibir el otro.
3. **Given** la orden 42 «en espera de piezas» con un pedido pendiente, **When** cualquier
   usuario (incluido el jefe) intenta cambiarle el estado a mano, **Then** no se permite y se
   indica que la orden espera piezas.

---

### User Story 4 - Ver los pedidos de una orden (Priority: P2)

Desde la ficha de una orden se ven todos sus pedidos con proveedor, referencia, cantidad, fecha
y estado. El almacén ve además la lista de pedidos pendientes de todas las órdenes.

**Why this priority**: Da visibilidad, pero el flujo funciona sin ella.

**Independent Test**: Con la orden 42 con un pedido recibido y otro pendiente, abrir la orden y
comprobar que aparecen los dos con su estado.

**Acceptance Scenarios**:

1. **Given** la orden 42 con un pedido «recibido» y otro «pendiente», **When** se consulta la
   orden, **Then** muestra ambos pedidos con su estado.
2. **Given** pedidos pendientes de varias órdenes, **When** el responsable abre la lista de
   pedidos pendientes, **Then** ve todos, cada uno con su orden.

### Edge Cases

- Cantidad del pedido 0, negativa o vacía: no se permite.
- Pedido sin proveedor, sin referencia o sin orden: no se permite.
- Pedido para una orden que no está «en reparación» ni «en espera de piezas»: no se permite y se
  indica el motivo.
- Un pedido ya recibido no puede volver a marcarse como recibido (no se suma dos veces al stock).
- La referencia pedida no existe: primero se da de alta (User Story 1).
- Stock inicial negativo: no se permite.
- Cambio manual de estado de una orden «en espera de piezas» con pedidos pendientes: no se
  permite a nadie, tampoco al jefe.
- Segundo pedido de la misma referencia para la misma orden mientras el primero sigue pendiente:
  no se permite y se indica el motivo. Una vez recibido el primero, sí se puede pedir de nuevo.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El responsable del almacén MUST poder dar de alta referencias de piezas a mano con
  código (único), descripción, precio de venta y stock inicial opcional (0 por defecto).
- **FR-002**: El sistema MUST rechazar el alta de una referencia con un código que ya existe,
  indicando el motivo.
- **FR-003**: El responsable del almacén MUST poder registrar un pedido con proveedor (elegido
  entre los existentes), referencia, cantidad (entero mayor que 0) y orden asociada; el sistema
  MUST guardar la fecha del pedido y dejarlo «pendiente».
- **FR-004**: El sistema MUST admitir pedidos solo para órdenes «en reparación» o «en espera de
  piezas»; en cualquier otro estado MUST impedirlo y mostrar el motivo.
- **FR-004a**: El sistema MUST rechazar un pedido si la orden ya tiene un pedido pendiente de la
  misma referencia, indicando el motivo.
- **FR-005**: Al registrar un pedido para una orden «en reparación», la orden MUST pasar
  automáticamente a «en espera de piezas».
- **FR-006**: En una orden «en espera de piezas» el sistema MUST NOT permitir imputar tiempo ni
  piezas.
- **FR-007**: El responsable del almacén MUST poder marcar un pedido pendiente como recibido; al
  hacerlo, el sistema MUST guardar la fecha de recepción y sumar la cantidad al stock de la
  referencia. Un pedido recibido MUST NOT poder recibirse otra vez.
- **FR-008**: Cuando una orden ya no tenga pedidos pendientes, MUST volver automáticamente a «en
  reparación».
- **FR-008a**: Mientras una orden tenga pedidos pendientes, el sistema MUST NOT permitir a nadie
  (tampoco al jefe) cambiar su estado a mano; solo sale de «en espera de piezas» por FR-008.
- **FR-009**: El sistema MUST mostrar en cada orden sus pedidos con su estado, y una lista de
  pedidos pendientes de todas las órdenes.
- **FR-010**: Los proveedores MUST venir precargados; esta spec no permite crearlos, editarlos
  ni borrarlos.
- **FR-011**: El sistema MUST NOT conectarse con catálogos electrónicos de fabricantes.

### Key Entities

- **Referencia de pieza**: código único, descripción, precio de venta y stock. Compartida con
  spec-003 y spec-005.
- **Proveedor**: nombre. Precargado.
- **Pedido a proveedor**: proveedor, referencia, cantidad, orden asociada, estado (pendiente o
  recibido), fecha de pedido y fecha de recepción.
- **Orden de reparación**: usa los estados compartidos «en reparación» y «en espera de piezas»
  (spec-002, spec-005, spec-008).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: El responsable registra un pedido en menos de 1 minuto.
- **SC-002**: El 100 % de las órdenes con algún pedido pendiente aparecen «en espera de piezas».
- **SC-003**: Al recibir el último pedido de una orden, esta vuelve a «en reparación» sin que
  nadie la cambie a mano.
- **SC-004**: Tras recibir un pedido, el stock mostrado sube exactamente en la cantidad pedida.
- **SC-005**: El 100 % de los intentos de pedido sobre órdenes en un estado no permitido se
  rechazan con un mensaje que explica el motivo.
- **SC-006**: No es posible tener dos referencias con el mismo código.

## Assumptions

- Los proveedores ya existen en el sistema (datos precargados); su gestión queda fuera.
- El sistema registra el pedido; la comunicación con el proveedor (teléfono, correo) se hace por
  fuera del sistema en la v1.
- Se recibe siempre el pedido completo.
- La pieza recibida no se reserva ni se imputa sola a la orden: el mecánico la imputa al
  montarla (spec-005).
- Al volver la orden a «en reparación» no se envía ningún aviso (ni correo ni notificación): el
  mecánico la ve con su nuevo estado en su lista de órdenes y en el tablero (spec-008).
- Editar o dar de baja referencias queda fuera de esta spec.
- Fuera de alcance: avisos al mecánico, recepción parcial y cancelación de pedidos, reserva de stock por orden,
  gestión de proveedores, catálogos de fabricantes y facturación a proveedores.
- Dependencias: estados de la orden compartidos con spec-002, spec-005 y spec-008; modelo de
  piezas y stock compartido con spec-003 y spec-005.
