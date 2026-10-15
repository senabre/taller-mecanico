# Implementation Plan: spec-000 · Base común del sistema (plan maestro)

**Branch**: `000-base-comun` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/000-base-comun/spec.md` y las specs 001–008.

**Note**: Este es el **plan maestro compartido** por las 8 specs. Cada `specs/00X-*/plan.md` solo
recoge lo propio de su historia y remite aquí para todo lo común. Documentos compartidos:
[research.md](research.md) · [data-model.md](data-model.md) · [contracts/api.md](contracts/api.md) ·
[quickstart.md](quickstart.md).

## Summary

Talleres Rueda necesita una aplicación web que acompañe a cada vehículo desde la cita hasta la
factura. Se construye como **un único servicio**: Rust/Axum expone una API JSON en `/api` y sirve
el frontend SvelteKit compilado como SPA. Usa PostgreSQL, una sesión por cookie para el personal y
dos páginas públicas: pedir cita y responder al presupuesto. La base común (este plan) incluye el
esqueleto del repositorio, **toda la base de datos**, la autenticación, la configuración, el
correo, el bus de eventos del tablero, la función única de transición de estados y el layout de
la interfaz. Sobre esa base, cada persona desarrolla sus 4 specs en paralelo.

## Technical Context

**Language/Version**: Rust estable (edición 2024) · TypeScript 5 con Node 22 LTS

**Primary Dependencies**: Axum 0.8, SQLx 0.8, tower-http, rust_decimal, chrono-tz, argon2, lettre ·
SvelteKit 2, Svelte 5 (runes), Skeleton UI v3, Tailwind CSS 4

**Storage**: PostgreSQL 16+ (migraciones SQLx en `backend/migrations/`)

**Testing**: `cargo test` con `#[sqlx::test]` para las reglas de negocio; `svelte-check` y Vitest
para `formato.ts`; aceptación manual con [quickstart.md](quickstart.md)

**Target Platform**: servidor Linux (o Docker) y navegadores de escritorio actuales. La página
pública también se usa desde el móvil.

**Project Type**: aplicación web (backend + frontend en un mismo repositorio y un único proceso
desplegado)

**Performance Goals**: 10 empleados y unas decenas de coches al día; todas las pantallas responden
en menos de 1 s y el tablero se actualiza en menos de 5 s

**Constraints**: todo en español de España, en € y con fechas dd/mm/aaaa; ningún secreto en el
código; nada fuera de las specs

**Scale/Scope**: unas 12 tablas, unos 30 endpoints, unas 10 pantallas y 9 specs (000–008)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principio | Comprobación | Estado |
|---|---|---|
| I. Simplicidad | Un único proceso y un único origen (R2); SPA sin SSR; SSE en vez de WebSockets (R11); sin capas de repositorio ni servicios genéricos: cada módulo de rutas hace sus consultas SQLx directamente; sin E2E con navegador (R13). | ✅ |
| II. Español y euro | Textos en español en el frontend y en los mensajes de error del backend; `formato.ts` con `es-ES` y EUR; «hoy» en Europe/Madrid (R12). | ✅ |
| III. Cero alcance fantasma | La base común solo implementa lo que exigen las specs 001–008. La autenticación por usuario no está en ninguna HU, pero la exige la decisión Q3-A («mecánico asignado», «solo el mecánico asignado imputa»): ver Complexity Tracking. No hay pantallas de configuración ni de usuarios. | ✅ (justificado) |
| IV. Verificable sin código | Cada spec tiene su recorrido en [quickstart.md](quickstart.md) usando solo la app y la bandeja de Mailpit. | ✅ |
| V. Datos con respeto | El cliente solo da nombre, teléfono y correo; la factura es simplificada (Q2-A); la API pública no expone datos personales; los secretos van en variables de entorno (R7); las contraseñas se guardan con argon2. | ✅ |

**Re-check post-diseño (Phase 1)**: el modelo de datos y el contrato no añaden entidades ni
endpoints que no salgan de una spec. ✅

## Project Structure

### Documentation (this feature)

```text
specs/
├── 000-base-comun/          # COMPARTIDO por las 8 specs
│   ├── spec.md
│   ├── plan.md              # este plan maestro
│   ├── research.md
│   ├── data-model.md        # modelo de datos ÚNICO
│   ├── contracts/api.md     # contrato de API ÚNICO
│   └── quickstart.md        # validación de todo el sistema
└── 00X-*/                   # 001…008
    ├── spec.md
    └── plan.md              # solo lo específico; remite a 000
```

### Source Code (repository root)

```text
backend/
├── Cargo.toml
├── migrations/
│   └── 0001_esquema_inicial.sql      # TODAS las tablas (data-model.md) — base común
├── src/
│   ├── main.rs                       # arranque, router, ServeDir del frontend
│   ├── bin/crear-usuario.rs          # alta/restablecer usuarios (base común)
│   ├── config.rs                     # lee variables de entorno (R7)
│   ├── error.rs                      # ErrorApi → {"error": …} + código HTTP
│   ├── auth.rs                       # sesión, extractor Usuario, exigir_rol (R8)
│   ├── estados.rs                    # cambiar_estado + publicar evento (R14)
│   ├── eventos.rs                    # broadcast + GET /api/eventos (R11)
│   ├── correo.rs                     # envío SMTP (R10)
│   ├── dinero.rs                     # importe de línea, totales con IVA (R4)
│   ├── contador.rs                   # siguiente número sin huecos (R5)
│   ├── fechas.rs                     # hoy() en Europe/Madrid (R12)
│   └── rutas/
│       ├── sesion.rs                 # 000
│       ├── citas.rs                  # 001  (Persona A)
│       ├── ordenes.rs                # 002  (Persona A)
│       ├── presupuestos.rs           # 003  (Persona B)
│       ├── publico_presupuesto.rs    # 004  (Persona A)
│       ├── imputaciones.rs           # 005  (Persona B)
│       ├── almacen.rs                # 006  (Persona B)
│       ├── facturas.rs               # 007  (Persona B)
│       └── tablero.rs                # 008  (Persona A)
└── tests/                            # un fichero por spec: spec001_citas.rs, …

frontend/
├── package.json · svelte.config.js (adapter-static, fallback index.html) · vite.config.ts (proxy /api)
└── src/
    ├── lib/
    │   ├── api.ts                    # fetch tipado + manejo de {"error"} (base común)
    │   ├── formato.ts                # euros, fechas, nombres de estado (base común)
    │   ├── tipos.ts                  # tipos TS del contrato (base común)
    │   └── componentes/              # TablaLineas.svelte, Totales.svelte, EstadoOrden.svelte
    └── routes/
        ├── +layout.svelte            # menú por rol (base común)
        ├── entrar/                   # 000
        ├── cita/                     # 001
        ├── citas/                    # 001/002
        ├── ordenes/ · ordenes/[id]/  # 002, 003, 005, 007
        ├── presupuesto/[token]/      # 004
        ├── almacen/ · pedidos/       # 006
        ├── historial/                # 007
        └── tablero/                  # 008

docker-compose.yml                    # postgres + mailpit (solo desarrollo)
.env.example                          # variables sin secretos (R7)
```

**Structure Decision**: aplicación web en dos carpetas, `backend/` y `frontend/`, pero un único
despliegue (R2). Cada spec tiene **su propio módulo de rutas y su fichero de pruebas**, así que
las dos personas apenas tocan los mismos ficheros. Los ficheros compartidos son la migración,
`estados.rs`, `dinero.rs`, `tipos.ts`, `formato.ts`, `componentes/` y `ordenes/[id]`, y se cambian
solo con revisión de los dos.

## Reparto y orden de trabajo

| Fase | Quién | Qué | Hecho cuando |
|---|---|---|---|
| 0. Base común | A + B juntos | Esqueleto, `docker-compose`, migración 0001 completa, `config`, `error`, `auth` + `/entrar`, `estados`, `eventos`, `correo`, `dinero`, `contador`, `fechas`, layout, `api.ts`, `formato.ts`, `tipos.ts`, datos demo | El quickstart §0 funciona: entrar con cada rol |
| 1. Paralelo | A · Cliente y seguimiento de la orden (16 pts) | 001 → 002 → 004 → 008 | quickstart §1, §2, §4, §8 |
| 1. Paralelo | B · Piezas, presupuesto, imputaciones y factura (16 pts) | 006 → 003 → 005 → 007 | quickstart §6, §3, §5, §7 |
| 2. Integración | A + B | Recorrido completo del quickstart §9 | Todo el flujo, de la cita a la factura |

El reparto agrupa por cohesión. B se queda con todo lo que trabaja con **líneas, piezas y dinero**
(`dinero.rs`, `TablaLineas`, `Totales`, `pieza`): 006 da de alta las piezas que usan 003 y 005, y
005 genera las imputaciones que factura 007. A se queda con lo que **ve el cliente y el
seguimiento de la orden**: las páginas públicas (001 y 004), la apertura de la orden (002) y el
tablero (008), que solo lee estados.

Para no esperar al otro, el script de datos demo (`backend/seed/demo.sql`) crea:
- Para B: órdenes `recibida` y `en_reparacion` asignadas a Luis, mientras A termina 002.
- Para A: un presupuesto `emitido` con token conocido, mientras B termina 003, y órdenes en todos
  los estados para el tablero.

**Puntos de contacto entre personas** (acordar antes de tocar):
- **003 (B) → 004 (A)**: B emite el presupuesto y A gestiona la respuesta. El contrato es la tabla
  `presupuesto` (`token`, `estado`, `caduca_el`, `respondido_en`) y los endpoints de spec-004.
  A no modifica `presupuestos.rs` y B no modifica `publico_presupuesto.rs`.
- **002 (A) → 003, 005, 006, 007 (B)**: A crea la orden y el `GET /api/ordenes/{id}`; B añade
  campos al objeto `Orden` siguiendo el contrato.
- `ordenes/[id]/+page.svelte` es una página con secciones: Datos (002), Diagnóstico y presupuesto
  (003/004), Imputaciones (005), Pedidos (006) y Factura (007). Cada sección es un componente
  propio en `routes/ordenes/[id]/`, así cada persona edita solo el suyo.
- `estados.rs`: la tabla de transiciones permitidas es la de data-model.md y no se añaden
  transiciones sin actualizarla.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Autenticación con usuarios y roles (no está en ninguna HU) | spec-002/003/005 hablan del «mecánico asignado» y de que solo él imputa; spec-007 solo para recepción; spec-008 para el jefe. Decisión Q3-A. | Un acceso común no permite saber quién imputa ni filtrar «mis órdenes», y dejaría que cualquiera emita facturas. |
