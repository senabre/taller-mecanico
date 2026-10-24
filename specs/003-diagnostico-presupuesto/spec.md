# Feature Specification: spec-003 · Registrar diagnóstico y preparar presupuesto (HU-03)

**Feature Branch**: `003-diagnostico-presupuesto`

**Created**: 2026-10-15

**Status**: Draft

**Input**: HU-03 · Como mecánico, quiero registrar el diagnóstico y elaborar un presupuesto línea a
línea con mano de obra y piezas, para que el cliente sepa exactamente qué se va a hacer y cuánto le
va a costar. Prioridad Alta · 5 pts.

**Responsable**: Persona B

## Clarifications

### Session 2026-10-22

- Q: ¿Qué cantidades se permiten en una línea del presupuesto? → A: Horas de mano de obra en
  múltiplos de 0,25 (0,25, 0,5, 1,5…); piezas en unidades enteras (1, 2, 3…).
- Q: Mientras el presupuesto está en borrador, ¿puede el mecánico quitar una línea? → A: Sí; para
  corregir una línea se quita y se añade de nuevo (no hay edición de líneas).
- Q: Después de guardar el diagnóstico, ¿puede el mecánico cambiarlo? → A: Sí, mientras la orden
  esté «en diagnóstico» y aún no se haya emitido ningún presupuesto para ella.
- Q: ¿Qué concepto lleva cada línea? → A: Mano de obra: concepto obligatorio escrito por el
  mecánico. Pieza: se pone solo, «referencia descripción» del almacén (p. ej. «PF-123 Pastillas
  freno»), y no se puede cambiar.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Registrar el diagnóstico (Priority: P1)

El mecánico abre una de sus órdenes en estado «recibida», revisa el coche y escribe el
diagnóstico. Al guardarlo, la orden pasa a «en diagnóstico».

**Why this priority**: El diagnóstico justifica el presupuesto y queda en el historial.

**Independent Test**: Abrir una orden «recibida», guardar un diagnóstico y comprobar que se ve
en la orden y que su estado es «en diagnóstico».

**Acceptance Scenarios**:

1. **Given** una orden «recibida» asignada a Luis, **When** Luis guarda el diagnóstico «Pastillas
   delanteras desgastadas», **Then** el diagnóstico queda en la orden y esta pasa a «en
   diagnóstico».
2. **Given** esa orden «en diagnóstico» sin presupuesto emitido, **When** Luis cambia el
   diagnóstico a «Pastillas y discos delanteros desgastados», **Then** la orden muestra el nuevo
   diagnóstico y sigue «en diagnóstico».
3. **Given** esa orden con un presupuesto ya emitido, **When** Luis intenta cambiar el
   diagnóstico, **Then** no se permite y se indica que el presupuesto ya se ha emitido.

---

### User Story 2 - Preparar el presupuesto línea a línea (Priority: P1)

El mecánico añade líneas al presupuesto. Una línea de mano de obra indica las horas; una línea
de pieza se elige de las referencias del almacén e indica las unidades. Cada línea muestra
concepto, cantidad, precio unitario e importe. El presupuesto muestra la base imponible, el IVA
desglosado y el total.

**Why this priority**: Es lo que el cliente va a aceptar; sin presupuesto no se toca el coche.

**Independent Test**: Crear un presupuesto con 1,5 h de mano de obra y 2 unidades de una pieza
y comprobar a mano los importes, la base, el IVA y el total.

**Acceptance Scenarios**:

1. **Given** un precio por hora de 40,00 €, **When** el mecánico añade «Cambio de pastillas»
   con 1,5 h, **Then** la línea muestra cantidad 1,5 h, precio 40,00 € e importe 60,00 €.
2. **Given** la referencia «PF-123 Pastillas freno» con precio de 25,00 €, **When** el mecánico la
   elige con 2 unidades, **Then** la línea muestra el concepto «PF-123 Pastillas freno», 2 uds,
   25,00 € e importe 50,00 €.
3. **Given** esas dos líneas, **When** se consulta el presupuesto, **Then** muestra base
   imponible 110,00 €, IVA (21 %) 23,10 € y total 133,10 €.
4. **Given** el mecánico busca una pieza, **When** escribe parte de la referencia o de la
   descripción, **Then** solo puede elegir entre referencias dadas de alta en el almacén.
5. **Given** un presupuesto en borrador con las dos líneas anteriores, **When** el mecánico quita
   la de mano de obra, **Then** el presupuesto queda solo con la pieza y muestra base 50,00 €,
   IVA 10,50 € y total 60,50 €.

---

### User Story 3 - Emitir el presupuesto (Priority: P1)

Cuando el presupuesto está completo, el mecánico lo emite. A partir de ese momento ya no se
puede modificar, se fija su fecha de caducidad a 15 días y queda listo para enviarse al cliente
(spec-004).

**Why this priority**: Fija lo que se ofrece al cliente y su validez.

**Independent Test**: Emitir un presupuesto el 15/10/2026 y comprobar que caduca el 30/10/2026
y que ya no se pueden editar sus líneas.

**Acceptance Scenarios**:

1. **Given** un presupuesto con al menos una línea, **When** se emite el 15/10/2026, **Then**
   muestra «Válido hasta 30/10/2026» y sus líneas no se pueden modificar.
2. **Given** un presupuesto sin líneas, **When** se intenta emitir, **Then** no se permite.

### Edge Cases

- Cantidad 0 o negativa en una línea: no se permite.
- Horas que no son múltiplo de 0,25 (p. ej. 1,3 h) o unidades de pieza no enteras (p. ej. 2,5):
  no se permite y se indica el motivo.
- Línea de mano de obra sin concepto (vacío o solo espacios): no se permite.
- La pieza elegida no tiene stock: se permite presupuestarla (el stock se comprueba al montarla,
  spec-005; si falta se pide, spec-006).
- Redondeo: los importes se muestran con 2 decimales y la suma de las líneas coincide con la
  base imponible mostrada.
- Una orden no asignada al mecánico: no puede diagnosticarla ni presupuestarla.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El mecánico asignado MUST poder registrar el diagnóstico de una orden «recibida»;
  al guardarlo, la orden MUST pasar a «en diagnóstico».
- **FR-001a**: El mecánico asignado MUST poder cambiar el diagnóstico mientras la orden esté «en
  diagnóstico» y no se haya emitido ningún presupuesto para ella; después, el diagnóstico MUST
  quedar fijo.
- **FR-002**: El mecánico MUST poder crear un presupuesto con líneas de mano de obra (horas, en
  múltiplos de 0,25) y de piezas (unidades enteras).
- **FR-002a**: Mientras el presupuesto esté en borrador, el mecánico MUST poder quitar cualquiera
  de sus líneas. Las líneas no se editan: para corregir una, se quita y se añade de nuevo.
- **FR-003**: Cada línea MUST mostrar concepto, cantidad (horas o unidades), precio unitario e
  importe (cantidad × precio unitario). En las de mano de obra el concepto lo escribe el mecánico
  y es obligatorio; en las de pieza el sistema MUST ponerlo solo como «referencia descripción» de
  la referencia del almacén, sin que se pueda cambiar.
- **FR-004**: Las líneas de mano de obra MUST usar el precio por hora del taller; las de piezas
  MUST usar el precio de venta de la referencia del almacén.
- **FR-005**: Las piezas MUST elegirse entre las referencias dadas de alta en el almacén.
- **FR-006**: El presupuesto MUST mostrar base imponible, IVA desglosado (porcentaje e importe) y
  total, en euros con formato español.
- **FR-007**: Al emitirse, el presupuesto MUST fijar su fecha de caducidad en 15 días naturales
  desde su emisión y MUST quedar bloqueado para cambios.
- **FR-008**: Una orden MUST tener como máximo un presupuesto abierto a la vez: en borrador o
  emitido sin caducar. Un presupuesto caducado no cuenta, y el mecánico puede preparar otro.

### Key Entities

- **Presupuesto**: pertenece a una orden; estado (borrador, emitido, aceptado, rechazado; un
  emitido pasada su fecha de caducidad se considera caducado), fecha de emisión, fecha de caducidad, base imponible, IVA y total.
- **Línea de presupuesto**: tipo (mano de obra o pieza), concepto, cantidad, precio unitario,
  importe; las de pieza apuntan a una referencia del almacén.
- **Referencia de pieza**: código, descripción, precio de venta y stock (se da de alta en
  spec-006).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Un mecánico prepara un presupuesto de 5 líneas en menos de 5 minutos.
- **SC-002**: En el 100 % de los presupuestos, base + IVA = total, y cada importe es cantidad ×
  precio, comprobable con una calculadora.
- **SC-003**: Todo presupuesto emitido muestra una fecha de caducidad 15 días posterior a su
  emisión.
- **SC-004**: No es posible añadir a un presupuesto una pieza que no exista en el almacén.

## Assumptions

- Los 15 días cuentan desde la emisión, que es cuando se crea la versión que verá el cliente.
- El IVA es el general del 21 % para todas las líneas.
- El precio por hora de mano de obra es un único valor del taller fijado en la configuración del
  sistema; su edición desde pantalla queda fuera de esta spec.
- Se precisan referencias de piezas dadas de alta (spec-006) para probar las líneas de pieza; para
  probar esta spec por separado se pueden cargar algunas referencias de ejemplo.
- Los descuentos quedan fuera de la v1.
