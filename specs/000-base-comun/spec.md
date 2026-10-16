# Feature Specification: spec-000 · Base común del sistema

**Feature Branch**: `000-base-comun`

**Created**: 2026-10-15

**Status**: Draft

**Input**: Requisitos transversales que necesitan las specs 001–008 y que ninguna historia cubre por
sí sola: acceso del personal, roles, valores de configuración y estados de la orden. No añade
funcionalidad nueva para el cliente (principio III); solo recoge lo que las 8 specs ya exigen.

**Responsable**: Persona A y Persona B (se hace en pareja, antes de repartirse las specs)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - El personal entra con su usuario (Priority: P1)

Cada empleado (recepción, mecánico, almacén, jefe de taller) entra con su usuario y contraseña y
solo ve las opciones de su rol. Las páginas de pedir cita y de responder al presupuesto son
públicas.

**Why this priority**: Las specs necesitan saber quién es el mecánico asignado, quién imputa y
quién puede facturar (decisión Q3-A).

**Independent Test**: Entrar como «luis» (mecánico) y comprobar que en el menú no aparecen Citas,
Almacén, Pedidos ni Tablero; entrar como «paco» (jefe) y comprobar que ve el tablero.

**Acceptance Scenarios**:

1. **Given** el usuario «luis» con rol mecánico, **When** entra con su contraseña, **Then** ve en el
   menú «Mis órdenes» e «Historial» y no ve Citas, Almacén, Pedidos ni Tablero.
2. **Given** una contraseña incorrecta, **When** intenta entrar, **Then** ve «Usuario o contraseña
   incorrectos».
3. **Given** nadie ha entrado, **When** se abre la página de pedir cita, **Then** se puede usar sin
   usuario.

### Edge Cases

- Una sesión sin actividad durante 12 horas caduca y pide entrar de nuevo.
- Un usuario desactivado no puede entrar.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: El personal MUST entrar con usuario y contraseña propios; cada usuario tiene un único
  rol: recepción, mecánico, almacén o jefe.
- **FR-002**: El jefe de taller MUST poder hacer todo lo que hacen los demás roles.
- **FR-003**: Las páginas de pedir cita (spec-001) y de responder al presupuesto (spec-004) MUST
  ser públicas.
- **FR-004**: El cupo diario, el precio por hora de mano de obra, el porcentaje de IVA y los datos
  del taller MUST leerse de la configuración del sistema, nunca escribirse en el código.
- **FR-005**: Los estados de la orden MUST ser los mismos en todo el sistema: recibida, en
  diagnóstico, en reparación, en espera de piezas, pendiente de conformidad, terminada y
  facturada.
- **FR-006**: Todo el texto visible MUST estar en español de España, con importes en euros
  (1.234,56 €) y fechas dd/mm/aaaa.

### Key Entities

- **Usuario**: nombre, usuario de acceso, contraseña (nunca guardada en claro), rol, activo.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Un empleado entra en el sistema en menos de 15 segundos.
- **SC-002**: Ningún empleado ve opciones de un rol que no es el suyo (comprobable entrando con un
  usuario de cada rol).

## Assumptions

- Los usuarios se crean con una orden de administración al instalar; no hay pantalla de gestión
  de usuarios en la v1.
- Recuperar la contraseña olvidada queda fuera de la v1: el jefe la restablece con la misma orden
  de administración.
