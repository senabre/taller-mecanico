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

## Arrancar en local

Los pasos completos, con los usuarios y datos demo, están en el
[quickstart](specs/000-base-comun/quickstart.md) («Arrancar»). Requisitos: Docker, Rust estable y
Node 22. Además necesitas `sqlx-cli`, que da la orden `cargo sqlx` para las migraciones:

```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls
```

### Base de datos vacía

Para borrar la base, volver a crearla y aplicar todas las migraciones (pide confirmación; `-y` la
salta):

```bash
cd backend && cargo sqlx database reset -y
```

### Cargar los datos demo

Los datos demo (`backend/seed/demo.sql`) se cargan sobre una base recién migrada, desde la raíz del
repositorio. Si tienes `psql` instalado:

```bash
psql "$DATABASE_URL" -f backend/seed/demo.sql
```

Si no lo tienes, usa el `psql` que trae el contenedor de Postgres de `docker-compose.yml`:

```bash
docker compose exec -T postgres sh -c 'psql -v ON_ERROR_STOP=1 -U "$POSTGRES_USER" -d "$POSTGRES_DB"' < backend/seed/demo.sql
```

Para volver a cargarlos, vacía antes la base (`cargo sqlx database reset -y`). Usuarios y
contraseña demo: ver el quickstart.

### Usuarios del personal

No hay pantalla de usuarios: se gestionan con una orden de administración, que lee
`DATABASE_URL` del `.env` y la contraseña de la entrada estándar (nunca como argumento):

```bash
cd backend
cargo run --bin crear-usuario -- --login luis --nombre "Luis" --rol mecanico  # alta o nueva contraseña
cargo run --bin crear-usuario -- --login luis --desactivar                    # baja (cierra sus sesiones)
```

Los roles son `recepcion`, `mecanico`, `almacen` y `jefe`.
