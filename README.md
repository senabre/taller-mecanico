# Talleres Rueda · Gestión de un taller mecánico

Aplicación web que acompaña a cada vehículo desde que el cliente pide cita hasta que se lo lleva
reparado (SI-26).

**Pila**: SvelteKit (Svelte 5) · Rust (Axum + SQLx) · PostgreSQL · Skeleton UI

## Documentación (Spec Kit)

- Constitución: [.specify/memory/constitution.md](.specify/memory/constitution.md)
- Base común, compartida por todas las specs: [specs/000-base-comun/](specs/000-base-comun/)
  - [Plan maestro](specs/000-base-comun/plan.md) · [Modelo de datos](specs/000-base-comun/data-model.md) ·
    [Contrato de API](specs/000-base-comun/contracts/api.md) · [Quickstart](specs/000-base-comun/quickstart.md)

## Reparto y ramas

`main` contiene la base común. Cada spec se desarrolla en su rama y se fusiona en `main` mediante
pull request revisado por la otra persona.

| Spec | Historia | Rama | Responsable | Pts |
|---|---|---|---|---|
| spec-001 | HU-01 · Pedir cita en línea | `spec-001` | Persona A · @evm60-ua | 3 |
| spec-002 | HU-02 · Recibir el vehículo y abrir la orden | `spec-002` | Persona A · @evm60-ua | 3 |
| spec-003 | HU-03 · Diagnóstico y presupuesto | `spec-003` | Persona B · @senabre | 5 |
| spec-004 | HU-04 · Aceptar/rechazar presupuesto por enlace | `spec-004` | Persona A · @evm60-ua | 5 |
| spec-005 | HU-05 · Imputar tiempo y piezas | `spec-005` | Persona B · @senabre | 5 |
| spec-006 | HU-06 · Pedir pieza al proveedor | `spec-006` | Persona B · @senabre | 3 |
| spec-007 | HU-07 · Facturar y avisar al cliente | `spec-007` | Persona B · @senabre | 3 |
| spec-008 | HU-08 · Tablero de órdenes abiertas | `spec-008` | Persona A · @evm60-ua | 5 |

- **Persona A · @evm60-ua** (cliente y seguimiento de la orden): spec-001 → 002 → 004 → 008 · 16 pts
- **Persona B · @senabre** (piezas, presupuesto, imputaciones y factura): spec-006 → 003 → 005 → 007 · 16 pts

La base común (spec-000) se hace en pareja, directamente sobre `main`, antes de empezar las ramas.

## Cómo trabajar en una spec

```bash
git switch spec-003
# apuntar Spec Kit a la carpeta de la spec (fichero local, no se sube):
echo '{ "feature_directory": "specs/003-diagnostico-presupuesto" }' > .specify/feature.json
# después: /speckit-tasks → /speckit-implement
```
