# Feature Specification: spec-008 · Tablero de órdenes abiertas (HU-08)

**Feature Branch**: `008-tablero-ordenes-abiertas`

**Created**: 2026-10-15

**Status**: Draft

**Input**: HU-08 · Como jefe de taller, quiero ver en un tablero todas las órdenes abiertas agrupadas
por estado, para sustituir la pizarra y detectar retrasos y sobrecargas de un vistazo.
Prioridad Media · 5 pts.

**Responsable**: Persona A

## Clarifications

### Session 2026-11-12

- Q: Si un mecánico inactivo todavía tiene órdenes abiertas, ¿cómo aparece en la carga por
  mecánico? → A: Los activos salen siempre, también con 0. Los inactivos salen solo si tienen
  órdenes abiertas, marcados como «(de baja)».
- Q: ¿Quién puede ver el tablero, y qué le pasa a otro rol que escriba `/tablero` a mano? → A:
  Solo el jefe. Recepción, mecánico y almacén no ven «Tablero» en el menú, `GET /api/tablero` les
  responde 403 y `/tablero` les muestra, dentro de la página y sin ninguna tarjeta, el mensaje del
  servidor «No tienes permiso para hacer esto», como las demás pantallas protegidas (corregido en
  el plan, 2026-11-12: ninguna usa `+error.svelte` para el 403). Hay una prueba del 403 para cada
  uno de esos tres roles.
- Q: ¿Qué debe hacer que el tablero se actualice solo, sin recargar? → A: Abrir una orden,
  cualquier cambio de estado (también facturar, con lo que la tarjeta desaparece), el cambio de
  día a las 00:00 Europe/Madrid (recalcula las retrasadas) y la vuelta de la conexión (los
  eventos enviados mientras no había conexión se han perdido). En todos los casos se vuelve a
  pedir el tablero entero. La medianoche se calcula en Europe/Madrid con `Intl`, no con la zona
  del navegador, y se prueba en el frontend con reloj simulado (`vi.useFakeTimers`), porque en
  el quickstart no se puede comprobar a mano.
- Q: ¿En qué orden van las tarjetas dentro de cada columna? → A: Por fecha de entrega prometida,
  de la más antigua a la más reciente, y a igual fecha por número de orden ascendente. Así las
  retrasadas quedan arriba.

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
3. **Given** un usuario de recepción, mecánico o almacén con sesión, **When** escribe a mano la
   dirección del tablero, **Then** no tiene «Tablero» en el menú, ve el mensaje «No tienes permiso
   para hacer esto» sin ninguna tarjeta, y el sistema le niega los datos del tablero.
4. **Given** en una misma columna una orden con entrega pasado mañana, otra con entrega ayer y
   dos con entrega hoy (números 12 y 7), **When** se abre el tablero, **Then** aparecen en este
   orden: la de ayer, la nº 7, la nº 12 y la de pasado mañana.

---

### User Story 2 - Carga de cada mecánico (Priority: P1)

El tablero muestra cuántas órdenes abiertas tiene asignadas cada mecánico.

**Why this priority**: Permite repartir mejor el trabajo y detectar sobrecargas.

**Independent Test**: Con 3 órdenes abiertas de Luis y 1 de Marta, comprobar que el tablero
muestra «Luis: 3» y «Marta: 1».

**Acceptance Scenarios**:

1. **Given** Luis con 3 órdenes abiertas y Marta con 1, **When** se abre el tablero, **Then** se ve
   «Luis 3 · Marta 1», y los mecánicos activos sin órdenes aparecen con 0.
2. **Given** un mecánico inactivo con 1 orden abierta y otro inactivo sin ninguna, **When** se
   abre el tablero, **Then** el primero aparece en la carga con 1 y marcado «(de baja)», el
   segundo no aparece, y la suma de la carga coincide con el número de tarjetas.

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

Cuando se abre una orden, cualquier orden cambia de estado (también al facturarla) o cambia el
día, el tablero se actualiza solo, sin recargar la página.

**Why this priority**: El tablero se deja abierto todo el día en el taller.

**Independent Test**: Con el tablero abierto en un ordenador, cambiar el estado de una orden
desde otro y comprobar que la tarjeta cambia de columna sin tocar el tablero.

**Acceptance Scenarios**:

1. **Given** el tablero abierto, **When** desde otro puesto una orden pasa a «terminada»,
   **Then** en menos de 5 segundos la tarjeta aparece en la columna «terminada».
2. **Given** el tablero abierto, **When** recepción abre una orden nueva, **Then** en menos de 5
   segundos su tarjeta aparece en «recibida» y la carga de su mecánico sube en 1.
3. **Given** el tablero abierto, **When** se factura una orden terminada, **Then** en menos de 5
   segundos su tarjeta desaparece y la carga de su mecánico baja en 1.
4. **Given** el tablero abierto con una orden cuya entrega prometida es hoy, **When** llegan las
   00:00 en Europe/Madrid (sea cual sea la zona horaria del ordenador), **Then** el tablero se
   vuelve a pedir y la tarjeta aparece como «Retrasada» sin que cambie ninguna orden.
5. **Given** el tablero sin conexión y una orden que cambia de estado mientras tanto, **When**
   vuelve la conexión, **Then** el aviso de desconexión desaparece y el tablero se vuelve a pedir
   entero, con esa tarjeta ya en su columna nueva.

### Edge Cases

- Un estado sin órdenes: la columna aparece vacía, no desaparece.
- Una orden terminada pero no facturada cuya fecha prometida ya pasó: también aparece como
  retrasada.
- Muchas órdenes en una columna: todas siguen siendo visibles desplazándose dentro de la
  columna.
- Se pierde la conexión: el tablero indica que no está actualizado y, al volver la conexión, se
  vuelve a pedir entero, porque los cambios ocurridos mientras tanto no le han llegado.
- Pasa la medianoche con el tablero abierto: las órdenes cuya entrega era el día anterior pasan a
  «Retrasada» sin esperar a ningún cambio de estado.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El sistema MUST mostrar un tablero con todas las órdenes abiertas (todas excepto
  las facturadas) agrupadas en columnas por estado: recibida, en diagnóstico, en reparación, en
  espera de piezas, pendiente de conformidad y terminada. Dentro de cada columna, las tarjetas
  van por fecha de entrega prometida, de la más antigua a la más reciente, y a igual fecha por
  número de orden ascendente.
- **FR-002**: Cada tarjeta MUST mostrar número de orden, matrícula, mecánico asignado y fecha de
  entrega prometida.
- **FR-003**: El tablero MUST mostrar el número de órdenes abiertas asignadas a cada mecánico:
  todos los mecánicos activos, también con 0, y los inactivos solo si tienen órdenes abiertas,
  marcados como «(de baja)».
- **FR-004**: Las órdenes abiertas cuya fecha de entrega prometida sea anterior a hoy MUST
  aparecer destacadas visualmente y con el texto «Retrasada».
- **FR-005**: El tablero MUST actualizarse solo, sin recargar la página, cuando se abra una
  orden, cuando cualquier orden cambie de estado (incluido facturarla), al cambiar el día a las
  00:00 Europe/Madrid y al recuperar la conexión tras perderla.
- **FR-006**: El tablero MUST estar disponible solo para el jefe de taller. Los demás roles
  (recepción, mecánico, almacén) no lo ven en el menú, el sistema les niega los datos del
  tablero y, si abren su dirección a mano, ven «No tienes permiso para hacer esto» sin ninguna
  tarjeta, igual que en las demás pantallas protegidas.

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
- Se considera retrasada una orden no facturada cuando hoy (Europe/Madrid) es posterior a la
  fecha prometida (sin hora). Es la regla común del data-model de la base común, la misma en
  todas las pantallas (hallazgo I1 de spec-002); por eso una orden terminada sin facturar también
  puede salir como retrasada.
- Cada empleado entra con usuario y contraseña propios y un rol (recepción, mecánico, almacén,
  jefe); el jefe puede ver todo. Es base común de todas las specs (decisión Q3-A, 2026-10-15;
  ver `specs/000-base-comun/`).
