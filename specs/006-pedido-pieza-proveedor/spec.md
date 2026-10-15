# Feature Specification: spec-006 · Pedir al proveedor una pieza sin stock (HU-06)

**Feature Branch**: `006-pedido-pieza-proveedor`

**Created**: 2026-10-15

**Status**: Draft

**Input**: HU-06 · Como responsable del almacén de recambios, quiero pedir al proveedor una pieza que
falta dejándola vinculada a la orden que la necesita, para que la reparación continúe en cuanto
llegue sin seguimientos manuales. Prioridad Media · 3 pts.

**Responsable**: Persona B

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Dar de alta referencias de piezas (Priority: P1)

El responsable del almacén da de alta a mano cada referencia de pieza con su código,
descripción, precio de venta y stock inicial. No hay conexión con catálogos de fabricantes.

**Why this priority**: Sin referencias no se pueden presupuestar, imputar ni pedir piezas.

**Independent Test**: Dar de alta la referencia «PF-123 Pastillas freno», 25,00 €, stock 4, y
comprobar que aparece en la lista del almacén y al presupuestar (spec-003).

**Acceptance Scenarios**:

1. **Given** el almacén, **When** se da de alta «PF-123», «Pastillas freno», 25,00 €, stock 4,
   **Then** la referencia aparece en la lista con esos datos.
2. **Given** ya existe «PF-123», **When** se intenta dar de alta otra con el mismo código,
   **Then** no se permite.

---

### User Story 2 - Pedir una pieza vinculada a una orden (Priority: P1)

Falta una pieza para una orden «en reparación». El responsable registra el pedido indicando
proveedor, referencia, cantidad y orden. La orden pasa sola a «en espera de piezas».

**Why this priority**: Es el núcleo de la historia: dejar la pieza ligada a la orden.

**Independent Test**: Registrar un pedido para una orden «en reparación» y comprobar que la
orden pasa a «en espera de piezas» y muestra el pedido pendiente.

**Acceptance Scenarios**:

1. **Given** una orden 42 «en reparación», **When** se registra el pedido «Recambios Sur»,
   «PF-123», 2 uds, orden 42, **Then** el pedido queda «pendiente» y la orden 42 pasa a «en
   espera de piezas».
2. **Given** la orden 42, **When** se consulta, **Then** muestra sus pedidos pendientes.

---

### User Story 3 - Recibir el pedido (Priority: P1)

Cuando llega la pieza, el responsable marca el pedido como recibido. La cantidad entra en stock
y, si la orden no espera ningún otro pedido, vuelve sola a «en reparación».

**Why this priority**: Evita que la orden se quede olvidada en espera.

**Independent Test**: Marcar como recibido el pedido de la orden 42 y comprobar que el stock sube
y que la orden vuelve a «en reparación» sin ninguna otra acción.

**Acceptance Scenarios**:

1. **Given** «PF-123» con stock 0 y un pedido pendiente de 2 uds para la orden 42, **When** se
   marca como recibido, **Then** el stock pasa a 2 y la orden 42 vuelve a «en reparación».
2. **Given** la orden 42 con dos pedidos pendientes, **When** solo se recibe uno, **Then** la
   orden sigue «en espera de piezas» hasta recibir el otro.

### Edge Cases

- Cantidad del pedido 0 o negativa: no se permite.
- Pedido para una orden que no está «en reparación» ni «en espera de piezas»: no se permite.
- Un pedido ya recibido no puede volver a marcarse como recibido (no se suma dos veces al stock).
- La referencia pedida no existe: primero se da de alta (User Story 1).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El responsable del almacén MUST poder dar de alta referencias de piezas a mano con
  código (único), descripción, precio de venta y stock inicial.
- **FR-002**: El responsable del almacén MUST poder dar de alta proveedores indicando su nombre.
- **FR-003**: El responsable del almacén MUST poder registrar un pedido con proveedor, referencia,
  cantidad y la orden de reparación asociada.
- **FR-004**: Al registrar el pedido, la orden MUST pasar automáticamente a «en espera de piezas».
- **FR-005**: El responsable del almacén MUST poder marcar un pedido como recibido; al hacerlo,
  la cantidad MUST sumarse al stock de la referencia.
- **FR-006**: Cuando una orden ya no tenga pedidos pendientes, MUST volver automáticamente a «en
  reparación».
- **FR-007**: El sistema MUST mostrar la lista de pedidos pendientes y, en cada orden, sus
  pedidos.
- **FR-008**: El sistema MUST NOT conectarse con catálogos electrónicos de fabricantes.

### Key Entities

- **Referencia de pieza**: código único, descripción, precio de venta y stock.
- **Proveedor**: nombre.
- **Pedido a proveedor**: proveedor, referencia, cantidad, orden asociada, estado (pendiente o
  recibido), fecha de pedido y de recepción.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: El responsable registra un pedido en menos de 1 minuto.
- **SC-002**: El 100 % de las órdenes con pedido pendiente aparecen «en espera de piezas».
- **SC-003**: Al recibir el último pedido de una orden, esta vuelve a «en reparación» sin que
  nadie la cambie a mano.
- **SC-004**: Tras recibir un pedido, el stock mostrado sube exactamente en la cantidad pedida.

## Assumptions

- El sistema registra el pedido; la comunicación con el proveedor (teléfono, correo) se hace por
  fuera del sistema en la v1.
- Se recibe siempre el pedido completo; las recepciones parciales quedan fuera de la v1.
- La pieza recibida no se imputa sola a la orden: el mecánico la imputa al montarla (spec-005).
- Editar o dar de baja referencias y proveedores queda fuera de esta spec.
