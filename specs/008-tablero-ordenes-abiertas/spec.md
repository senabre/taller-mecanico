# Feature Specification: spec-008 · Tablero de órdenes abiertas (HU-08)

**Feature Branch**: `008-tablero-ordenes-abiertas`

**Created**: 2026-10-15

**Status**: Draft

**Input**: HU-08 · Como jefe de taller, quiero ver en un tablero todas las órdenes abiertas agrupadas
por estado, para sustituir la pizarra y detectar retrasos y sobrecargas de un vistazo.
Prioridad Media · 5 pts.

**Responsable**: Persona A

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Ver las órdenes abiertas agrupadas por estado (Priority: P1)

El jefe de taller abre el tablero y ve una columna por estado (recibida, en diagnóstico, en
reparación, en espera de piezas, pendiente de conformidad, terminada). Cada orden aparece como
una tarjeta con su número, matrícula, mecánico y fecha de entrega prometida.

**Why this priority**: Es lo que sustituye a la pizarra.

**Independent Test**: Con órdenes en varios estados, abrir el tablero y comprobar que cada orden
aparece en la columna de su estado.

**Acceptance Scenarios**:

1. **Given** una orden en cada estado abierto, **When** el jefe abre el tablero, **Then** ve las
   6 columnas y cada orden en la suya.
2. **Given** una orden «facturada», **When** se abre el tablero, **Then** no aparece.

---

### User Story 2 - Carga de cada mecánico (Priority: P1)

El tablero muestra cuántas órdenes abiertas tiene asignadas cada mecánico.

**Why this priority**: Permite repartir mejor el trabajo y detectar sobrecargas.

**Independent Test**: Con 3 órdenes abiertas de Luis y 1 de Ana, comprobar que el tablero muestra
«Luis: 3» y «Ana: 1».

**Acceptance Scenarios**:

1. **Given** Luis con 3 órdenes abiertas y Ana con 1, **When** se abre el tablero, **Then** se ve
   «Luis 3 · Ana 1», y los mecánicos sin órdenes aparecen con 0.

---

### User Story 3 - Órdenes con retraso destacadas (Priority: P1)

Las órdenes cuya fecha de entrega prometida ya ha pasado aparecen bien destacadas.

**Why this priority**: Es lo que más le interesa a Paco detectar de un vistazo.

**Independent Test**: Con una orden abierta cuya fecha prometida fue ayer, comprobar que su
tarjeta aparece destacada y con la indicación «Retrasada».

**Acceptance Scenarios**:

1. **Given** una orden abierta con entrega prometida ayer, **When** se abre el tablero, **Then**
   su tarjeta aparece destacada con el texto «Retrasada».
2. **Given** una orden con entrega prometida hoy, **When** se abre el tablero, **Then** no
   aparece como retrasada.

---

### User Story 4 - Actualización automática (Priority: P2)

Cuando cualquier orden cambia de estado, el tablero se actualiza solo, sin recargar la página.

**Why this priority**: El tablero se deja abierto todo el día en el taller.

**Independent Test**: Con el tablero abierto en un ordenador, cambiar el estado de una orden
desde otro y comprobar que la tarjeta cambia de columna sin tocar el tablero.

**Acceptance Scenarios**:

1. **Given** el tablero abierto, **When** desde otro puesto una orden pasa a «terminada»,
   **Then** en menos de 5 segundos la tarjeta aparece en la columna «terminada».

### Edge Cases

- Un estado sin órdenes: la columna aparece vacía, no desaparece.
- Una orden terminada pero no facturada cuya fecha prometida ya pasó: también aparece como
  retrasada.
- Muchas órdenes en una columna: todas siguen siendo visibles desplazándose dentro de la
  columna.
- Se pierde la conexión: el tablero indica que no está actualizado y se recupera solo al volver
  la conexión.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El sistema MUST mostrar un tablero con todas las órdenes abiertas (todas excepto
  las facturadas) agrupadas en columnas por estado: recibida, en diagnóstico, en reparación, en
  espera de piezas, pendiente de conformidad y terminada.
- **FR-002**: Cada tarjeta MUST mostrar número de orden, matrícula, mecánico asignado y fecha de
  entrega prometida.
- **FR-003**: El tablero MUST mostrar el número de órdenes abiertas asignadas a cada mecánico.
- **FR-004**: Las órdenes abiertas cuya fecha de entrega prometida sea anterior a hoy MUST
  aparecer destacadas visualmente y con el texto «Retrasada».
- **FR-005**: El tablero MUST actualizarse solo cuando cambie el estado de cualquier orden, sin
  recargar la página.
- **FR-006**: El tablero MUST estar disponible para el jefe de taller.

### Key Entities

- **Orden de reparación**: (definida en spec-002) número, estado, mecánico asignado, matrícula y
  fecha de entrega prometida.
- **Mecánico**: miembro del personal al que se asignan órdenes.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: El jefe de taller identifica en menos de 10 segundos qué órdenes van con retraso.
- **SC-002**: Un cambio de estado aparece en el tablero en menos de 5 segundos sin recargar.
- **SC-003**: El número de órdenes por mecánico coincide con el recuento manual de las tarjetas.
- **SC-004**: El 100 % de las órdenes abiertas aparecen en el tablero y ninguna facturada.

## Assumptions

- La fecha de entrega prometida se registra al abrir la orden (spec-002).
- El tablero es solo de consulta: no se cambian estados arrastrando tarjetas en la v1.
- Se considera retrasada una orden cuando hoy es posterior a la fecha prometida (sin hora).
- Cada empleado entra con usuario y contraseña propios y un rol (recepción, mecánico, almacén,
  jefe); el jefe puede ver todo. Es base común de todas las specs (decisión Q3-A, 2026-10-15;
  ver `specs/000-base-comun/`).
