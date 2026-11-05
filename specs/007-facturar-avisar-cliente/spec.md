# Feature Specification: spec-007 · Facturar la orden y avisar al cliente (HU-07)

**Feature Branch**: `007-facturar-avisar-cliente`

**Created**: 2026-10-15

**Status**: Draft

**Input**: HU-07 · Como recepcionista, quiero generar la factura a partir de lo imputado en la orden y
avisar al cliente, para no volver a teclear datos y que el cliente sepa cuándo recoger el coche.
Prioridad Alta · 3 pts.

**Responsable**: Persona B

## Clarifications

### Session 2026-10-29

- Q: ¿Cómo recibe el cliente su factura? → A: Recepción la imprime desde la app (vista de factura
  con botón «Imprimir» del navegador, que también permite guardarla como PDF) y se la entrega al
  recoger el coche. El correo de aviso no lleva la factura.
- Q: ¿Cómo se numeran las facturas? → A: Un único número que sigue siempre (1, 2, 3…), sin
  reiniciar cada año; se muestra tal cual («Factura nº 12»).
- Q: ¿Puede la factura llevar el logo del taller? (petición de la Persona B, 29/10) → A: Sí, es
  opcional: si el taller tiene configurado un logo, sale en la cabecera de la factura; si no lo
  tiene, la factura sale sin logo y sin dejar hueco.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Generar la factura de una orden terminada (Priority: P1)

La recepcionista abre una orden «terminada» y pulsa «Facturar». La factura se genera sola con las
horas y piezas imputadas, la base imponible, el IVA desglosado y el total, sin escribir nada. La
orden pasa a «facturada».

**Why this priority**: Acaba con las facturas discutidas y con volver a teclear datos.

**Independent Test**: Facturar una orden terminada con imputaciones conocidas y comprobar que la
factura las recoge todas con los importes correctos.

**Acceptance Scenarios**:

1. **Given** una orden «terminada» con 2 h a 40,00 € y 2 uds de PF-123 a 25,00 €, **When** la
   recepcionista pulsa «Facturar», **Then** la factura muestra esas líneas, base 130,00 €, IVA
   (21 %) 27,30 € y total 157,30 €.
2. **Given** la factura emitida, **When** se consulta, **Then** tiene número de factura, fecha,
   datos del cliente y matrícula del vehículo.
3. **Given** la factura emitida, **When** la recepcionista pulsa «Imprimir», **Then** se abre la
   impresión del navegador con la factura sola (sin menú ni botones de la aplicación), lista para
   imprimir o guardar como PDF.
4. **Given** el taller tiene configurado su logo, **When** se consulta o se imprime la factura,
   **Then** el logo sale en la cabecera, junto a los datos del taller; **Given** no lo tiene
   configurado, **Then** la factura sale igual pero sin logo.
5. **Given** una orden en cualquier estado distinto de «terminada», **When** la recepcionista la
   abre, **Then** la opción «Facturar» no está disponible.

---

### User Story 2 - Avisar al cliente para que recoja el coche (Priority: P1)

Al emitirse la factura, el cliente recibe un aviso de que puede pasar a recoger su vehículo.

**Why this priority**: El cliente sabe cuándo ir sin tener que llamar.

**Independent Test**: Facturar una orden y comprobar que llega un correo al cliente indicando que
puede recoger el vehículo.

**Acceptance Scenarios**:

1. **Given** una orden de la matrícula 1234BCD, **When** se emite su factura, **Then** el
   cliente recibe un correo «Su vehículo 1234BCD está listo para recoger» con el total.

---

### User Story 3 - Historial del vehículo por matrícula (Priority: P2)

El personal escribe una matrícula y ve todas las reparaciones facturadas de ese vehículo, de la
más reciente a la más antigua, con su fecha, kilómetros, diagnóstico y líneas facturadas.

**Why this priority**: Los clientes preguntan a menudo cuándo se cambió por última vez la correa o
las pastillas.

**Independent Test**: Tras facturar dos órdenes del mismo vehículo, buscar su matrícula y
comprobar que aparecen ambas con sus líneas.

**Acceptance Scenarios**:

1. **Given** un vehículo con dos reparaciones facturadas, **When** se busca su matrícula,
   **Then** aparecen las dos, la más reciente primero, con fecha, km, diagnóstico y líneas.
2. **Given** una matrícula sin reparaciones facturadas, **When** se busca, **Then** se indica que
   no hay historial.

### Edge Cases

- Se pulsa «Facturar» dos veces: solo se genera una factura por orden.
- Una orden terminada sin ninguna imputación: no se puede facturar y se indica el motivo.
- El correo del aviso no se puede enviar: la factura se emite igual y la orden muestra que el
  aviso no se envió para que recepción llame al cliente.
- Las facturas emitidas no se pueden modificar.
- El logo está configurado pero su archivo no existe o no se puede leer: la factura sale sin
  logo, igual que si no estuviera configurado.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: La opción de facturar MUST estar disponible solo en órdenes «terminadas».
- **FR-002**: La factura MUST generarse con las horas y piezas imputadas en la orden, sin
  introducir datos a mano.
- **FR-003**: La factura MUST mostrar cada línea (concepto, cantidad, precio unitario, importe),
  la base imponible, el IVA desglosado y el total, en euros con formato español. MUST poder
  imprimirse desde la aplicación con la impresión del navegador (que permite guardarla como PDF);
  no se genera PDF en el servidor ni se envía la factura por correo.
- **FR-004**: Cada factura MUST tener un número correlativo propio, sin huecos ni repeticiones, y
  la fecha de emisión. Es una única serie (1, 2, 3…) que no se reinicia al cambiar de año y se
  muestra tal cual («Factura nº 12»).
- **FR-005**: Al emitir la factura, la orden MUST pasar a «facturada» y dejar de estar abierta.
- **FR-006**: Al emitir la factura, el sistema MUST enviar al cliente un aviso por correo de que
  puede recoger el vehículo.
- **FR-007**: La reparación facturada MUST incorporarse al historial del vehículo, consultable
  por matrícula.
- **FR-008**: Una orden MUST tener como máximo una factura.
- **FR-009**: Si el taller tiene configurado un logo, la factura MUST mostrarlo en la cabecera
  (también al imprimirla); si no lo tiene, MUST mostrarse sin logo y sin hueco.

### Key Entities

- **Factura**: número correlativo, fecha, orden, cliente, vehículo, líneas, base imponible, IVA y
  total. No modificable una vez emitida.
- **Aviso al cliente**: mensaje enviado al cliente al emitirse la factura; queda constancia de si
  se envió.
- **Historial del vehículo**: lista de reparaciones facturadas de una matrícula.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: La recepcionista emite una factura en menos de 30 segundos sin escribir ningún dato.
- **SC-002**: El 100 % de las líneas imputadas aparecen en la factura con el importe correcto,
  comprobable con una calculadora.
- **SC-003**: Ninguna orden que no esté «terminada» puede facturarse.
- **SC-004**: El historial de un vehículo se consulta en menos de 10 segundos escribiendo su
  matrícula.

## Assumptions

- El cobro (forma de pago, cobrado o pendiente) y la entrega del coche quedan fuera de la v1.
- La factura es simplificada: datos del taller, nombre del cliente y matrícula; no se pide NIF ni
  dirección del cliente (decisión Q2-A, 2026-10-15; principio V). La factura completa para
  empresas queda fuera de la v1.
- El envío de facturas a la Agencia Tributaria (VeriFactu u otros) queda fuera de la v1.
- El historial solo recoge reparaciones facturadas; el historial anterior en papel no se migra.
- El logo lo configura el equipo al instalar la aplicación (igual que el nombre, el NIF y la
  dirección del taller); cambiarlo desde una pantalla queda fuera de la v1.
