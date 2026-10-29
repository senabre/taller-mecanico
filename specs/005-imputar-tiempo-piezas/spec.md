# Feature Specification: spec-005 · Imputar tiempo y piezas durante la reparación (HU-05)

**Feature Branch**: `005-imputar-tiempo-piezas`

**Created**: 2026-10-15

**Status**: Draft

**Input**: HU-05 · Como mecánico, quiero cargar en la orden el tiempo real dedicado y las piezas que
monto, para que la factura refleje el trabajo realmente realizado. Prioridad Alta · 5 pts.

**Responsable**: Persona B

## Clarifications

### Session 2026-10-29

- Q: ¿Qué concepto lleva cada imputación? → A: Horas: concepto obligatorio escrito por el
  mecánico. Pieza: se pone solo, «referencia descripción» del almacén, igual que en el
  presupuesto (spec-003).
- Q: ¿Se puede marcar como «terminada» una orden sin ninguna imputación? → A: No; para terminar
  debe tener al menos una imputación, y si no se indica el motivo.
- Q: Si el cliente rechaza la ampliación y no quiere ninguna otra, ¿cómo sale la orden de
  «pendiente de conformidad»? → A: No sale: sigue «pendiente de conformidad». Resolverlo queda
  fuera de la v1 (límite conocido).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Imputar tiempo y piezas (Priority: P1)

En una orden «en reparación», el mecánico anota las horas que dedica y las piezas que monta. Cada
pieza imputada se descuenta del stock del almacén en ese momento. La orden muestra siempre el
coste acumulado frente al presupuestado.

**Why this priority**: Es lo que después se factura; sin imputaciones no hay factura fiable.

**Independent Test**: En una orden «en reparación», imputar 1 h y 2 unidades de una pieza con
stock 10 y comprobar que el stock pasa a 8 y que el coste acumulado se actualiza.

**Acceptance Scenarios**:

1. **Given** una pieza con stock 10, **When** el mecánico imputa 2 unidades, **Then** el stock
   pasa a 8 en ese momento.
2. **Given** un presupuesto aceptado de 110,00 € (base), **When** el mecánico imputa 1 h a
   40,00 € y 2 uds a 25,00 €, **Then** la orden muestra «Acumulado 90,00 € de 110,00 €
   presupuestados».
3. **Given** una pieza con stock 1, **When** el mecánico intenta imputar 2 unidades, **Then** no
   se permite y se indica que solo hay 1 en stock y que puede pedirse al proveedor (spec-006).

---

### User Story 2 - Bloqueo por desviación de más del 10 % (Priority: P1)

Si con una imputación el coste acumulado supera en más de un 10 % lo presupuestado, la orden
pasa a «pendiente de conformidad» y se bloquean nuevas imputaciones.

**Why this priority**: Evita las quejas de clientes a los que se cobró más de lo dicho.

**Independent Test**: Con 100,00 € presupuestados, imputar hasta 110,00 € (se permite seguir) y
después hasta 110,01 € o más (la orden se bloquea).

**Acceptance Scenarios**:

1. **Given** 100,00 € presupuestados, **When** el acumulado llega a 110,00 €, **Then** la orden
   sigue «en reparación».
2. **Given** 100,00 € presupuestados, **When** una imputación lleva el acumulado a 115,00 €,
   **Then** la imputación queda registrada, la orden pasa a «pendiente de conformidad» y no se
   permiten más imputaciones.

---

### User Story 3 - Ampliación del presupuesto y reanudación (Priority: P1)

Con la orden «pendiente de conformidad», el mecánico prepara una ampliación del presupuesto con
las nuevas líneas y la emite. El cliente la recibe por enlace (spec-004). Si la acepta, la orden
vuelve a «en reparación» y el nuevo total presupuestado es la suma del presupuesto original y la
ampliación.

**Why this priority**: Sin ella, una orden bloqueada no podría terminarse.

**Independent Test**: Bloquear una orden, emitir una ampliación, aceptarla desde el enlace y
comprobar que la orden vuelve a «en reparación» con el nuevo total.

**Acceptance Scenarios**:

1. **Given** una orden «pendiente de conformidad» con 100,00 € presupuestados, **When** el cliente
   acepta una ampliación de 50,00 €, **Then** la orden vuelve a «en reparación» y muestra
   150,00 € presupuestados.
2. **Given** la ampliación sin respuesta, **When** el mecánico intenta imputar, **Then** no se
   permite.

---

### User Story 4 - Terminar la reparación (Priority: P1)

Cuando el coche está listo, el mecánico marca la orden como «terminada».

**Why this priority**: Solo una orden terminada se puede facturar (spec-007).

**Independent Test**: Marcar como terminada una orden «en reparación» y comprobar su estado y que
ya no admite imputaciones.

**Acceptance Scenarios**:

1. **Given** una orden «en reparación» con imputaciones, **When** el mecánico la marca como
   terminada, **Then** pasa a «terminada» y no admite más imputaciones.
2. **Given** una orden «en reparación» sin ninguna imputación, **When** el mecánico intenta
   marcarla como terminada, **Then** no se permite y se indica que primero hay que imputar el
   trabajo realizado.

### Edge Cases

- Horas: se imputan en fracciones de 0,25 h; no se permiten 0 ni valores negativos.
- Imputar una pieza que no estaba en el presupuesto: se permite (cuenta para el acumulado).
- Imputación de horas sin concepto (vacío o solo espacios): no se permite.
- Una orden «en espera de piezas», «pendiente de conformidad» o sin presupuesto aceptado no
  admite imputaciones.
- Solo el mecánico asignado a la orden puede imputar en ella (y el jefe de taller, que puede hacer
  todo lo de los demás roles, spec-000 FR-002).
- Una orden «pendiente de conformidad» no puede marcarse como terminada, tampoco si el cliente
  ha rechazado la ampliación (ver el límite conocido en Assumptions).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El mecánico asignado MUST poder imputar horas y piezas en una orden «en
  reparación». En las horas el concepto lo escribe el mecánico y es obligatorio; en las piezas el
  sistema MUST ponerlo solo como «referencia descripción» de la referencia del almacén.
- **FR-002**: Cada pieza imputada MUST descontar su cantidad del stock del almacén en el mismo
  momento; el sistema MUST NOT permitir imputar más unidades de las que hay en stock.
- **FR-003**: El sistema MUST mostrar en todo momento el coste acumulado (sin IVA) frente al
  total presupuestado (sin IVA).
- **FR-004**: Las horas MUST valorarse al precio por hora del taller y las piezas a su precio de
  venta, igual que en el presupuesto (spec-003).
- **FR-005**: Si el coste acumulado supera en más de un 10 % lo presupuestado, la orden MUST pasar
  a «pendiente de conformidad» y el sistema MUST bloquear nuevas imputaciones.
- **FR-006**: El mecánico MUST poder preparar y emitir una ampliación del presupuesto, que el
  cliente acepta o rechaza por enlace (spec-004).
- **FR-007**: Al aceptarse la ampliación, la orden MUST volver a «en reparación» y el total
  presupuestado MUST pasar a ser la suma del original y de las ampliaciones aceptadas.
- **FR-008**: El mecánico MUST poder marcar como «terminada» una orden «en reparación» que tenga
  al menos una imputación; si no tiene ninguna, el sistema MUST impedirlo e indicar el motivo.

### Key Entities

- **Imputación**: pertenece a una orden; tipo (horas o pieza), concepto, cantidad, precio
  unitario, importe, fecha y hora, mecánico que la hace.
- **Ampliación de presupuesto**: presupuesto adicional de una orden, con las mismas reglas de
  líneas, IVA, caducidad y respuesta por enlace que el presupuesto inicial.
- **Stock**: unidades disponibles de cada referencia de pieza.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Tras cada imputación de piezas, el stock mostrado en el almacén refleja el descuento
  al instante.
- **SC-002**: El 100 % de las órdenes cuyo acumulado supera el 110 % del presupuesto quedan
  «pendientes de conformidad» y no admiten más imputaciones.
- **SC-003**: Un mecánico registra una imputación en menos de 30 segundos.
- **SC-004**: El coste acumulado mostrado coincide con la suma de las imputaciones, comprobable
  con una calculadora.

## Assumptions

- La comparación del 10 % se hace sobre importes sin IVA; como el IVA es el mismo para todo, el
  resultado es igual que con IVA.
- La imputación que hace superar el umbral sí se registra; lo que se bloquea son las siguientes.
- Si el cliente rechaza la ampliación, la orden sigue «pendiente de conformidad» y el mecánico
  puede emitir otra ampliación (spec-004).
- **Límite conocido de la v1**: si el cliente no acepta ninguna ampliación, la orden se queda
  «pendiente de conformidad»: no se puede terminar ni facturar desde la aplicación. Resolver ese
  caso (cerrarla, facturar solo lo aceptado…) queda fuera de esta spec.
- Corregir o anular una imputación hecha por error queda fuera de la v1.
