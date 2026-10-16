---

description: "Tareas de la fase 0 · Base común del sistema (spec-000)"
---

# Tasks: spec-000 · Base común del sistema

**Input**: [spec.md](spec.md), [plan.md](plan.md) (plan maestro), [research.md](research.md),
[data-model.md](data-model.md), [contracts/api.md](contracts/api.md) y [quickstart.md](quickstart.md).

**Alcance**: solo la fase 0 del plan maestro («Base común», A + B juntos): esqueleto del backend y
del frontend, `docker-compose.yml`, `.env.example`, la migración 0001 con **todas** las tablas,
`config`, `error`, `auth` + `/entrar`, `estados`, `eventos`, `correo`, `dinero`, `contador`,
`fechas`, layout, `api.ts`, `formato.ts`, `tipos.ts` y `seed/demo.sql`.
**Fuera de alcance**: los endpoints y las pantallas de las specs 001–008 (`rutas/citas.rs`,
`rutas/ordenes.rs`, …, `routes/cita/`, `routes/ordenes/`, …) y los componentes
`TablaLineas`/`Totales`/`EstadoOrden`, que crea cada spec. El layout puede enlazar a esas rutas
aunque todavía no existan.

**Tests**: se incluyen. Research R13 exige `#[sqlx::test]` para las reglas de negocio (aquí:
correlativos sin huecos, transiciones de estado, sesión y roles) y Vitest para `formato.ts`.

**Criterio de terminado**: el [quickstart §0](quickstart.md) funciona con la app compilada y
servida por el backend en `http://localhost:3000`.

**Reglas para todas las tareas**: todo texto visible y todo mensaje de error, en español de España
(principio II); ningún secreto en el código ni en git (principio V, R7); nada que no pidan las
specs (principio III); la opción más simple que cumpla (principio I).

## Format: `[ID] [P?] [Story] Description`

- **[P]**: se puede hacer en paralelo (otro fichero, sin depender de tareas pendientes)
- **[Story]**: historia de usuario (spec-000 solo tiene US1)

## Path Conventions

Aplicación web: `backend/` (Cargo, `src/`, `migrations/`, `seed/`, `tests/`) y `frontend/`
(SvelteKit, `src/lib/`, `src/routes/`), más `docker-compose.yml` y `.env.example` en la raíz.

---

## Phase 1: Setup (esqueleto del repositorio)

**Purpose**: que `cargo build`, `npm run build` y `docker compose up -d` funcionen en vacío.

- [X] T001 Crear `backend/Cargo.toml` (paquete `taller`, `edition = "2024"`, binario por defecto `src/main.rs` más librería `src/lib.rs` para que las pruebas de `backend/tests/` usen el router) con las dependencias de research R3: `axum = "0.8"` (feature `macros`), `tokio` (`full`), `sqlx = "0.8"` (`postgres`, `runtime-tokio`, `tls-rustls`, `macros`, `migrate`, `chrono`, `rust_decimal`; sin `uuid`, porque ningún id lo es), `tower-http` (`fs`, `trace`), `chrono` (`serde`), `chrono-tz`, `rust_decimal` (`serde-with-str`), `rust_decimal_macros`, `argon2`, `sha2`, `rand`, `base64`, `lettre` (`tokio1`, `tokio1-rustls-tls`, `smtp-transport`, `builder`, sin `native-tls`), `serde` (`derive`), `serde_json`, `tracing`, `tracing-subscriber` (`env-filter`), `tokio-stream` (`sync`), `futures-util`, `dotenvy`; `[dev-dependencies]`: `tower` (`util`) y `http-body-util`. Crear `backend/src/main.rs` y `backend/src/lib.rs` mínimos que compilen
- [X] T002 [P] Crear el esqueleto de `frontend/` con SvelteKit 2 + Svelte 5 + TypeScript (Node 22): `frontend/package.json` (scripts `dev`, `build`, `check` = `svelte-kit sync && svelte-check`, `test` = `vitest run`; dependencias `@sveltejs/kit`, `@sveltejs/adapter-static`, `svelte`, `vite`, `typescript`, `svelte-check`, `vitest`, `tailwindcss@4`, `@tailwindcss/vite`, `@skeletonlabs/skeleton` v3 y `@skeletonlabs/skeleton-svelte`), `frontend/svelte.config.js` (`adapter-static` con `fallback: 'index.html'`, salida en `build/`), `frontend/vite.config.ts` (plugins `tailwindcss()` y `sveltekit()`, `server.proxy` de `/api` a `http://localhost:3000`, `test.include: ['src/**/*.test.ts']`), `frontend/tsconfig.json`, `frontend/src/app.html` con `<html lang="es" data-theme="cerberus">` y `frontend/src/app.css` con `@import 'tailwindcss'; @import '@skeletonlabs/skeleton'; @import '@skeletonlabs/skeleton/themes/cerberus';`. Ejecutar `npm install` para generar `frontend/package-lock.json`
- [X] T003 [P] Crear `docker-compose.yml` (solo desarrollo) con dos servicios con etiqueta de versión fija: `postgres` (imagen `postgres:16`; usuario, contraseña y base desde `${POSTGRES_USER}`, `${POSTGRES_PASSWORD}`, `${POSTGRES_DB}` del `.env`, puerto `5432`, volumen con nombre `datos-postgres`, `healthcheck` con `pg_isready`) y `mailpit` (imagen `axllent/mailpit`; puertos `1025` SMTP y `8025` web)
- [X] T004 [P] Crear `.env.example` sin secretos reales, con comentarios en español y los valores demo del quickstart: `POSTGRES_USER=taller`, `POSTGRES_PASSWORD=cambiar-en-local`, `POSTGRES_DB=taller`, `DATABASE_URL=postgres://taller:cambiar-en-local@localhost:5432/taller`, `SMTP_URL=smtp://localhost:1025`, `CORREO_REMITENTE="Talleres Rueda <taller@talleres-rueda.test>"`, `URL_PUBLICA=http://localhost:3000`, `TALLER_CUPO_DIARIO=2`, `TALLER_PRECIO_HORA=40.00`, `TALLER_IVA_PORCENTAJE=21.00`, `TALLER_NOMBRE="Talleres Rueda"`, `TALLER_NIF=B00000000` (ficticio), `TALLER_DIRECCION="Calle Ejemplo 1, 03001 Alicante"` (ficticia) y `RUST_LOG=info,taller=debug`. Comprobar que `.gitignore` ya ignora `.env` y deja pasar `.env.example`

**Checkpoint**: `docker compose up -d` levanta Postgres y Mailpit (bandeja en http://localhost:8025);
`cargo build` y `npm run build` terminan sin errores.

---

## Phase 2: Foundational (módulos comunes que bloquean todo)

**Purpose**: base de datos completa y módulos compartidos que usan US1 y las specs 001–008.

**⚠️ CRITICAL**: ni US1 ni ninguna spec 001–008 puede empezar sin esta fase.

### Migración 0001 (todas las tablas de data-model.md)

Convenciones para las cuatro tareas: nombres en español, `snake_case` y sin tildes; claves
primarias `id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY` (salvo `sesion` y `contador`);
importes `NUMERIC(12,2)`, cantidades `NUMERIC(10,2)`, instantes `TIMESTAMPTZ`, fechas `DATE`; todas
las FK `NOT NULL` salvo las marcadas como NULL. Las reglas que dependen de otras filas o de «hoy»
(km ≥ anterior, rol del mecánico, días de cita, cupo…) **no** van en la migración: son de cada spec.


- [X] T005 Crear `backend/migrations/0001_esquema_inicial.sql`, parte 1: los seis tipos enumerados exactamente como en data-model — `rol` (`recepcion`, `mecanico`, `almacen`, `jefe`), `estado_orden` (`recibida`, `en_diagnostico`, `en_reparacion`, `en_espera_piezas`, `pendiente_conformidad`, `terminada`, `facturada`), `tipo_presupuesto` (`inicial`, `ampliacion`), `estado_presupuesto` (`borrador`, `emitido`, `aceptado`, `rechazado`; *caducado* no es un valor, se calcula), `tipo_linea` (`mano_obra`, `pieza`), `estado_pedido` (`pendiente`, `recibido`); tabla `usuario` (`nombre TEXT NOT NULL`, `login TEXT NOT NULL UNIQUE CHECK (login = lower(login))`, `hash_contrasena TEXT NOT NULL`, `rol rol NOT NULL`, `activo BOOLEAN NOT NULL DEFAULT true`); tabla `sesion` (`token_hash TEXT PRIMARY KEY` = SHA-256 del token, `usuario_id BIGINT NOT NULL REFERENCES usuario ON DELETE CASCADE`, `expira_en TIMESTAMPTZ NOT NULL`); tabla `contador` (`nombre TEXT PRIMARY KEY`, `ultimo BIGINT NOT NULL DEFAULT 0`) e `INSERT INTO contador (nombre) VALUES ('orden'), ('factura')` (research R5)
- [X] T006 Añadir a `backend/migrations/0001_esquema_inicial.sql`, parte 2: `cliente` (`nombre`, `telefono`, `email` todos `TEXT NOT NULL`; ningún otro dato personal, principio V); `vehiculo` (`matricula TEXT NOT NULL UNIQUE CHECK (matricula ~ '^[A-Z0-9]+$')` — normalizada en mayúsculas y sin espacios ni guiones, `marca TEXT NOT NULL`, `modelo TEXT NOT NULL`, `cliente_id` FK `cliente`); `cita` (`vehiculo_id` FK, `fecha DATE NOT NULL`, `motivo TEXT NOT NULL CHECK (btrim(motivo) <> '')`, `creada_en TIMESTAMPTZ NOT NULL DEFAULT now()`, índice por `fecha`); `orden` (`numero BIGINT NOT NULL UNIQUE`, `cita_id` FK `UNIQUE`, `vehiculo_id` FK, `mecanico_id` FK `usuario`, `km INTEGER NOT NULL CHECK (km >= 0)`, `danos TEXT NOT NULL`, `fecha_entrega_prometida DATE NOT NULL`, `diagnostico TEXT NULL`, `estado estado_orden NOT NULL DEFAULT 'recibida'`, `abierta_en TIMESTAMPTZ NOT NULL DEFAULT now()`, `terminada_en TIMESTAMPTZ NULL`; índices por `estado` y por `vehiculo_id`)
- [X] T007 Añadir a `backend/migrations/0001_esquema_inicial.sql`, parte 3: `pieza` (`referencia TEXT NOT NULL UNIQUE CHECK (referencia = btrim(referencia) AND referencia <> '')`, `descripcion TEXT NOT NULL`, `precio_venta NUMERIC(12,2) NOT NULL CHECK (precio_venta >= 0)`, `stock INTEGER NOT NULL DEFAULT 0 CHECK (stock >= 0)`); `presupuesto` (`orden_id` FK, `tipo tipo_presupuesto NOT NULL`, `estado estado_presupuesto NOT NULL DEFAULT 'borrador'`, `token TEXT NULL UNIQUE`, `iva_porcentaje NUMERIC(5,2) NULL`, `base`/`iva`/`total NUMERIC(12,2) NULL` — se fijan al emitir, `emitido_en TIMESTAMPTZ NULL`, `caduca_el DATE NULL`, `respondido_en TIMESTAMPTZ NULL`) con el índice único parcial `CREATE UNIQUE INDEX presupuesto_borrador_unico ON presupuesto (orden_id) WHERE estado = 'borrador'` (como mucho un borrador por orden; que no haya además un `emitido` sin caducar lo comprueba spec-003 en el código, porque depende de hoy: data-model y spec-003 FR-008); `linea_presupuesto` (`presupuesto_id` FK `ON DELETE CASCADE`, `tipo tipo_linea NOT NULL`, `concepto TEXT NOT NULL`, `pieza_id` FK `pieza` NULL, `cantidad NUMERIC(10,2) NOT NULL CHECK (cantidad > 0)`, `precio_unitario NUMERIC(12,2) NOT NULL`, `importe NUMERIC(12,2) NOT NULL`, `CHECK (tipo <> 'pieza' OR pieza_id IS NOT NULL)`); `imputacion` (`orden_id` FK, `tipo tipo_linea NOT NULL`, `concepto TEXT NOT NULL`, `pieza_id` FK NULL, `cantidad NUMERIC(10,2) NOT NULL CHECK (cantidad > 0)`, `CHECK (tipo <> 'mano_obra' OR cantidad * 4 = trunc(cantidad * 4))` — horas en múltiplos de 0,25, `CHECK (tipo <> 'pieza' OR pieza_id IS NOT NULL)`, `precio_unitario NUMERIC(12,2) NOT NULL`, `importe NUMERIC(12,2) NOT NULL`, `usuario_id` FK `usuario`, `creada_en TIMESTAMPTZ NOT NULL DEFAULT now()`; índice por `orden_id`)
- [X] T008 Añadir a `backend/migrations/0001_esquema_inicial.sql`, parte 4: `proveedor` (`nombre TEXT NOT NULL UNIQUE`); `pedido` (`proveedor_id` FK, `pieza_id` FK, `cantidad INTEGER NOT NULL CHECK (cantidad > 0)`, `orden_id` FK, `estado estado_pedido NOT NULL DEFAULT 'pendiente'`, `pedido_en TIMESTAMPTZ NOT NULL DEFAULT now()`, `recibido_en TIMESTAMPTZ NULL`) con `CREATE UNIQUE INDEX pedido_pendiente_unico ON pedido (orden_id, pieza_id) WHERE estado = 'pendiente'`; `factura` (`numero BIGINT NOT NULL UNIQUE`, `orden_id` FK `UNIQUE`, `fecha DATE NOT NULL`, `cliente_nombre TEXT NOT NULL`, `matricula TEXT NOT NULL`, `iva_porcentaje NUMERIC(5,2) NOT NULL`, `base`/`iva`/`total NUMERIC(12,2) NOT NULL`, `aviso_enviado BOOLEAN NOT NULL DEFAULT false`); `linea_factura` (`factura_id` FK `ON DELETE CASCADE`, `concepto TEXT NOT NULL`, `cantidad NUMERIC(10,2) NOT NULL`, `precio_unitario NUMERIC(12,2) NOT NULL`, `importe NUMERIC(12,2) NOT NULL`). Comprobar con `cargo sqlx migrate run` sobre una base vacía que se crean las 15 tablas y las 2 filas de `contador`

### Módulos del backend

- [X] T009 [P] Crear `backend/src/config.rs`: `struct Config { database_url, smtp_url, correo_remitente, url_publica: String, cupo_diario: u32, precio_hora: Decimal, iva_porcentaje: Decimal, taller_nombre, taller_nif, taller_direccion: String }` con `Config::desde_entorno() -> Result<Config, String>` que llama a `dotenvy::dotenv().ok()` y lee las variables de research R7; si falta una o no se puede interpretar, error en español con el nombre de la variable («Falta la variable de entorno TALLER_PRECIO_HORA»). Método `cookie_segura(&self) -> bool` = `url_publica` empieza por `https://` (R8: `Secure` solo en producción)
- [X] T010 [P] Crear `backend/src/error.rs`: `enum ErrorApi { PeticionMala(String) /*400*/, SinSesion /*401*/, Prohibido /*403*/, NoEncontrado(String) /*404*/, Conflicto(String) /*409*/, Validacion { mensaje: String, detalle: Option<serde_json::Value> } /*422*/, Interno(String) /*500*/ }` con `IntoResponse` que devuelve el código del contrato y el cuerpo `{"error": "...", "detalle": {...}}` (`detalle` solo si existe); textos por defecto: 401 «Tienes que entrar con tu usuario», 403 «No tienes permiso para hacer esto», 500 «Ha ocurrido un error interno» (el detalle técnico solo va a `tracing::error!`, nunca al cliente). `From<sqlx::Error>` → `NoEncontrado` para `RowNotFound` e `Interno` para el resto. Extractor `Json<T>` propio con `#[derive(FromRequest)] #[from_request(via(axum::Json), rejection(ErrorApi))]` y `From<JsonRejection> for ErrorApi` → 400 «Los datos enviados no son válidos», para que todas las rutas devuelvan errores JSON en español. Alias `type Resultado<T> = Result<T, ErrorApi>`
- [X] T011 [P] Crear `backend/src/fechas.rs` (research R12): `const ZONA: chrono_tz::Tz = Europe::Madrid`, `fn fecha_en_madrid(instante: DateTime<Utc>) -> NaiveDate` y `fn hoy() -> NaiveDate` (= `fecha_en_madrid(Utc::now())`). Pruebas `#[cfg(test)]` en el mismo fichero: `2026-10-15T22:30:00Z` → `2026-10-16` (verano, UTC+2) y `2026-12-31T23:30:00Z` → `2027-01-01` (invierno, UTC+1)
- [X] T012 [P] Crear `backend/src/dinero.rs` (research R4): `fn redondear(x: Decimal) -> Decimal` (2 decimales, `RoundingStrategy::MidpointAwayFromZero`, siempre con escala 2 para que se serialice `"60.00"` y no `"60"`); `fn importe_linea(cantidad: Decimal, precio_unitario: Decimal) -> Decimal` = `redondear(cantidad × precio)`; `#[derive(Serialize)] struct Totales { base, iva_porcentaje, iva, total: Decimal }` (serializados como string con 2 decimales y punto, `"133.10"`, según el contrato); `fn totales(importes: impl IntoIterator<Item = Decimal>, iva_porcentaje: Decimal) -> Totales` con base = Σ importes, IVA = `redondear(base × % / 100)` calculado **una vez** sobre la base, total = base + IVA. Pruebas `#[cfg(test)]`: 1,5 h × 40,00 = `60.00`; 2 × 25,00 = `50.00`; totales de [60, 50] al 21 % = base `110.00`, IVA `23.10`, total `133.10` (quickstart §3); redondeo de 0,005 → 0,01; serialización JSON de `Totales` con strings de 2 decimales
- [X] T013 [P] Crear `backend/src/contador.rs` (research R5): `async fn siguiente(tx: &mut PgConnection, nombre: &str) -> Resultado<i64>` que ejecuta `UPDATE contador SET ultimo = ultimo + 1 WHERE nombre = $1 RETURNING ultimo` dentro de la transacción del llamador (nunca `SEQUENCE`/`SERIAL`); si no hay fila, `ErrorApi::Interno`. Constantes `ORDEN = "orden"` y `FACTURA = "factura"`
- [X] T014 [P] Crear `backend/src/matricula.rs` (base común; la usan spec-001 y spec-007, contrato «el servidor la normaliza siempre»): `pub fn normalizar_matricula(texto: &str) -> Option<String>`, que quita espacios y guiones y pasa a mayúsculas, y devuelve `None` si el resultado está vacío o tiene caracteres fuera de `[A-Z0-9]` (el mismo criterio que el `CHECK` de `vehiculo`, T006); cada spec convierte `None` en un 422. Pruebas `#[cfg(test)]`: «1234-bcd», « 1234 BCD » y «1234BCD» → `1234BCD`; «» y «12#4» → `None`
- [X] T015 [P] Crear `backend/src/formato.rs`, el equivalente en el backend de `numero()` de `formato.ts`, para los mensajes de error en español: `pub fn numero(n: i64) -> String` con punto de miles (85000 → «85.000», 1500 → «1.500», 0 → «0», -1234 → «-1.234»). Lo usa spec-002 en «…de la visita anterior (85.000 km)». Pruebas `#[cfg(test)]` con esos cuatro casos
- [X] T016 Crear `backend/src/estados.rs` (research R14; depende de T010): `#[derive(sqlx::Type, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)] #[sqlx(type_name = "estado_orden", rename_all = "snake_case")] #[serde(rename_all = "snake_case")] enum Estado` con los 7 valores; `Estado::texto()` en minúsculas para los mensajes de error del backend («recibida», «en diagnóstico», «en reparación», «en espera de piezas», «pendiente de conformidad», «terminada», «facturada»); `const TRANSICIONES: &[(Estado, Estado)]` con exactamente las 9 transiciones de la máquina de estados de data-model (incluida `en_diagnostico → en_diagnostico`; la creación `— → recibida` es un `INSERT` de spec-002 y no pasa por aquí); `#[must_use] async fn cambiar_estado(tx: &mut PgConnection, orden_id: i64, desde: &[Estado], hacia: Estado) -> Resultado<CambioEstado>` que: (1) devuelve `Interno` si algún par `(desde[i], hacia)` no está en `TRANSICIONES`; (2) ejecuta `UPDATE orden SET estado = $hacia, terminada_en = CASE WHEN $hacia = 'terminada' THEN now() ELSE terminada_en END WHERE id = $1 AND estado = ANY($desde)`; (3) si no se actualiza ninguna fila: 404 «No existe la orden» si la orden no existe, o 409 «La orden está {estado.texto()}; no se puede pasar a {hacia.texto()}» si existe. Definir aquí `#[must_use] #[derive(Clone, Copy, Debug, Serialize)] pub struct CambioEstado { pub id: i64, pub estado: Estado }`, que también es el contenido del evento del tablero; su método `publicar` está en `eventos.rs` (T017), así `estados.rs` no depende de `eventos.rs`
- [X] T017 Crear `backend/src/eventos.rs` (research R11 y R14; depende de T016), parte sin HTTP: `#[derive(Clone)] struct Eventos(broadcast::Sender<CambioEstado>)` con `Eventos::nuevo()` (capacidad 64) y `suscribir(&self) -> broadcast::Receiver<CambioEstado>`; e `impl CambioEstado { pub fn publicar(self, eventos: &Eventos) }`, que envía el cambio (ignora el error si no hay nadie escuchando). Quien llama a `cambiar_estado` lo invoca **después** del `commit`, así el tablero nunca relee un estado sin confirmar. Spec-002 lo usa también al abrir una orden: `CambioEstado { id, estado: Estado::Recibida }.publicar(…)` (data-model)
- [X] T018 [P] Crear `backend/src/correo.rs` (research R10): `#[derive(Clone)] struct Correo { transporte: AsyncSmtpTransport<Tokio1Executor>, remitente: Mailbox }` construido desde `Config` (`AsyncSmtpTransport::from_url(&config.smtp_url)`), y `async fn enviar(&self, para: &str, asunto: &str, cuerpo: &str) -> bool` (texto plano UTF-8) que nunca entra en pánico: si la dirección no es válida o el SMTP falla, registra `tracing::warn!` y devuelve `false` para que el llamador guarde `aviso_enviado = false`. Documentar en el propio fichero que se llama **después** del `commit` y que un fallo no deshace nada
- [X] T019 Crear el arranque en `backend/src/lib.rs`, `backend/src/main.rs` y `backend/src/rutas/mod.rs` (depende de T009–T018): `lib.rs` declara los módulos (`auth`, `config`, `contador`, `correo`, `dinero`, `error`, `estados`, `eventos`, `fechas`, `formato`, `matricula`, `rutas`) y define `#[derive(Clone)] struct EstadoApp { db: PgPool, config: Arc<Config>, eventos: Eventos, correo: Correo }` y `fn app(estado: EstadoApp) -> Router` que monta `rutas::api()` bajo `/api`, un `fallback` dentro de `/api` que responde 404 JSON `{"error":"No existe"}` (las rutas `/api/*` desconocidas nunca devuelven `index.html`), y fuera de `/api` `ServeDir::new("../frontend/build").fallback(ServeFile::new("../frontend/build/index.html"))` (research R2), con `TraceLayer`. `rutas/mod.rs` expone `pub fn api() -> Router<EstadoApp>` (vacío por ahora; cada spec añade aquí su `módulo::rutas()`). `main.rs`: inicia `tracing_subscriber` con `RUST_LOG`, lee `Config::desde_entorno()` (si falla, imprime el mensaje y termina con código 1), crea el `PgPool`, construye `EstadoApp` y escucha en `0.0.0.0:3000`. No ejecuta migraciones al arrancar (se hacen con `cargo sqlx migrate run`, quickstart)

### Módulos del frontend

- [X] T020 [P] Crear `frontend/src/lib/tipos.ts` con los tipos comunes del contrato: `type Importe = string` ("133.10"), `type Fecha = string` ("2026-10-20"), `type Instante = string` (ISO 8601 con zona); `type Rol = 'recepcion' | 'mecanico' | 'almacen' | 'jefe'`; `type EstadoOrden = 'recibida' | 'en_diagnostico' | 'en_reparacion' | 'en_espera_piezas' | 'pendiente_conformidad' | 'terminada' | 'facturada'`; `type TipoLinea = 'mano_obra' | 'pieza'`; `interface Yo { id: number; nombre: string; rol: Rol }`; `interface Mecanico { id: number; nombre: string }`; `interface OrdenResumen { id; numero; matricula; estado: EstadoOrden; mecanico: Mecanico; fecha_entrega_prometida: Fecha; retrasada: boolean }`; `interface Linea { id; tipo: TipoLinea; concepto; pieza_id: number | null; cantidad: string; precio_unitario: Importe; importe: Importe }`; `interface Totales { base; iva_porcentaje; iva; total: Importe }`; `interface RespuestaError { error: string; detalle?: Record<string, unknown> }`; `interface EventoOrden { id: number; estado: EstadoOrden }`. Los tipos propios de cada spec (Orden, Presupuesto, Pieza, Pedido, Factura…) los añade cada spec
- [X] T021 [P] Crear `frontend/src/lib/formato.ts` (research R4 y R12, FR-006): `euros(importe: Importe | number): string` con `Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR', useGrouping: 'always' })` (sin `useGrouping: 'always'`, es-ES escribe «1234,56 €» sin punto de miles); `fecha(f: Fecha): string` → `dd/mm/aaaa` partiendo el string `YYYY-MM-DD` (sin `new Date`, para no mover el día por la zona horaria); `hora(i: Instante): string` → `HH:mm` y `fechaHora(i: Instante): string` → `dd/mm/aaaa HH:mm`, ambos con `timeZone: 'Europe/Madrid'`; `numero(n: number | string): string` con `Intl.NumberFormat('es-ES', { useGrouping: 'always', maximumFractionDigits: 2 })` para km, horas y unidades (85000 → «85.000», "1.25" → «1,25»); `NOMBRE_ESTADO: Record<EstadoOrden, string>` con «Recibida», «En diagnóstico», «En reparación», «En espera de piezas», «Pendiente de conformidad», «Terminada», «Facturada»; `ESTADOS_ABIERTOS: EstadoOrden[]` (los 6 distintos de `facturada`, en el orden de la máquina de estados); `NOMBRE_ROL: Record<Rol, string>` con «Recepción», «Mecánico», «Almacén», «Jefe de taller»; `INICIO_ROL: Record<Rol, string>` con la página de inicio de cada rol (`recepcion` → `/citas`, `mecanico` → `/ordenes`, `almacen` → `/almacen`, `jefe` → `/tablero`)
- [X] T022 Crear `frontend/src/lib/api.ts` (depende de T020): `class ErrorApi extends Error { estado: number; detalle?: Record<string, unknown> }` y `async function api<T>(metodo: 'GET' | 'POST' | 'PUT' | 'DELETE', ruta: string, cuerpo?: unknown): Promise<T>` que llama a `fetch('/api' + ruta)` con `credentials: 'same-origin'` y `Content-Type: application/json` si hay cuerpo; 204 → `undefined`; respuesta no 2xx → lee `{error, detalle}` y lanza `ErrorApi` con ese mensaje (si el cuerpo no es JSON: «Ha ocurrido un error inesperado»); fallo de red → `ErrorApi` con estado 0 y «No se pudo conectar con el servidor». Si la respuesta es 401, la ruta no es `/sesion` y la página actual no es pública (`/entrar`, `/cita`, `/presupuesto/…`), redirige con `goto('/entrar')` (sesión caducada tras 12 h). Exportar atajos `get`, `post`, `put`, `del`

### Pruebas de la fase

- [X] T023 Crear `backend/tests/comun/mod.rs` (depende de T019), parte 1, con utilidades reutilizables por todas las specs y que no necesitan `auth.rs`: `fn estado_prueba(pool: PgPool) -> EstadoApp` (config de prueba fija: cupo 2, precio hora 40.00, IVA 21.00, SMTP a `smtp://localhost:1025`), `async fn peticion(app, metodo, ruta, cookie: Option<&str>, cuerpo: Option<Value>) -> (StatusCode, Value, Option<String /*set-cookie*/>)` usando `tower::ServiceExt::oneshot`, `async fn insertar_usuario(pool, login, rol) -> i64` (SQL directo, con un `hash_contrasena` que no sirve para entrar; para asignar órdenes en las pruebas) y `async fn crear_orden(pool, numero, estado, mecanico_id) -> i64` (inserta cliente, vehículo, cita y orden por SQL directo). `crear_usuario` y `entrar` se añaden en US1 (T031)
- [X] T024 [P] Crear `backend/tests/spec000_contador.rs` con `#[sqlx::test]`: `siguiente(tx, "orden")` devuelve 1, 2, 3 seguidos; si una transacción pide un número y hace `rollback`, la siguiente obtiene el mismo número (sin huecos, spec-002 FR-002 y spec-007 FR-004); `"orden"` y `"factura"` llevan cuentas independientes
- [X] T025 [P] Crear `backend/tests/spec000_estados.rs` con `#[sqlx::test]`: `recibida → en_diagnostico` actualiza la orden y devuelve `CambioEstado`; pedir el cambio desde un estado que no es el actual → 409 y la orden no cambia; un par que no está en `TRANSICIONES` (p. ej. `en_espera_piezas → terminada`) → error y la orden no cambia; orden inexistente → 404; `en_reparacion → terminada` rellena `terminada_en`; `CambioEstado::publicar` llega a un receptor de `Eventos::suscribir` con `{id, estado}`
- [X] T026 [P] Crear `frontend/src/lib/formato.test.ts` (Vitest): `euros("1234.56")` = `1.234,56 €` y `euros("133.10")` = `133,10 €` (normalizar el espacio no separable ` ` antes de comparar); `fecha("2026-10-20")` = `20/10/2026`; `hora("2026-10-16T10:32:00+02:00")` = `10:32`; `fechaHora("2026-10-15T22:30:00Z")` = `16/10/2026 00:30`; `numero(85000)` = `85.000`, `numero(1500)` = `1.500` y `numero("1.25")` = `1,25`; `NOMBRE_ESTADO.en_espera_piezas` = «En espera de piezas»; `ESTADOS_ABIERTOS` tiene 6 elementos y no incluye `facturada`

**Checkpoint**: `cargo test` y `npm run check && npm test` pasan; `cargo run` arranca y
`GET /api/cualquiera` devuelve 404 JSON en español.

---

## Phase 3: User Story 1 - El personal entra con su usuario (Priority: P1) 🎯 MVP

**Goal**: cada empleado entra con su usuario y contraseña y solo ve las opciones de su rol; las
páginas de pedir cita y de responder al presupuesto quedan públicas (FR-001…FR-003).

**Independent Test**: quickstart §0 — entrar como `luis` (mecánico) y comprobar que ve «Mis
órdenes» e «Historial» y no ve Citas, Almacén, Pedidos ni Tablero; contraseña errónea → «Usuario o contraseña incorrectos»; entrar como
`paco` (jefe) y ver todo el menú, incluido el Tablero; `ana` y `pepe` solo ven su menú.

### Backend de User Story 1

- [X] T027 [US1] Crear `backend/src/auth.rs` (research R8): `#[derive(sqlx::Type, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)] enum Rol { Recepcion, Mecanico, Almacen, Jefe }` (`type_name = "rol"`, `rename_all = "snake_case"`); `fn hash_contrasena(&str) -> String` y `fn verificar_contrasena(&str, hash) -> bool` con argon2id (`Argon2::default()`, sal aleatoria); `fn nuevo_token() -> String` (32 bytes aleatorios en base64url sin relleno) y `fn hash_token(&str) -> String` (SHA-256 en hex); `const DURACION_SESION = 12 h`; `fn cookie_sesion(token, segura: bool) -> HeaderValue` (`sesion=…; HttpOnly; SameSite=Lax; Path=/; Max-Age=43200` + `; Secure` si `config.cookie_segura()`) y `fn cookie_borrada()` (`Max-Age=0`); extractor `struct Usuario { id: i64, nombre: String, rol: Rol }` con `FromRequestParts<EstadoApp>` que lee la cookie `sesion` de la cabecera `Cookie` (sin añadir `axum-extra`), busca por `hash_token` uniendo `usuario` con `activo = true` y `expira_en > now()`, renueva `expira_en = now() + interval '12 hours'` (caduca tras 12 h **sin actividad**) y si falla devuelve `ErrorApi::SinSesion`; `impl Usuario { fn exigir_rol(&self, roles: &[Rol]) -> Resultado<()> }` que deja pasar siempre a `Rol::Jefe` y si no → `ErrorApi::Prohibido`; y `fn exigir_mecanico_de(&self, mecanico_id: i64) -> Resultado<()>` (comprobación común de «mecánico asignado» para spec-003 y spec-005): pasa si `self.id == mecanico_id` o si el rol es `Jefe` (FR-002), y si no → `ErrorApi::Prohibido` (403 del contrato «la orden no es del mecánico»)
- [X] T028 [P] [US1] Crear `backend/src/bin/crear-usuario.rs` (depende de T027; Assumptions de la spec): `cargo run --bin crear-usuario -- --login luis --nombre "Luis" --rol mecanico [--desactivar]` lee `DATABASE_URL` del entorno y la contraseña de la entrada estándar (nunca como argumento, para que no quede en el historial), y hace `INSERT … ON CONFLICT (login) DO UPDATE` del nombre, rol, `hash_contrasena` y `activo` (sirve para dar de alta, restablecer la contraseña y desactivar). Al desactivar, borra las sesiones del usuario. Mensajes en español («Usuario luis guardado (mecánico)»)
- [X] T029 [US1] Crear `backend/src/rutas/sesion.rs` con `pub fn rutas() -> Router<EstadoApp>` y registrarlo en `backend/src/rutas/mod.rs` (contrato, sección «Base común»): `POST /api/sesion` 🔓 (`{login, contrasena}`; normaliza `login` con `trim().to_lowercase()`; si el usuario no existe, está inactivo o la contraseña no coincide → 401 «Usuario o contraseña incorrectos», el mismo texto en los tres casos; si es correcto, borra las sesiones caducadas, inserta la nueva en `sesion` y responde 204 con la cookie); `DELETE /api/sesion` (todos; borra la fila y responde 204 con `cookie_borrada()`); `GET /api/yo` (todos → `{id, nombre, rol}`); `GET /api/mecanicos` (todos → `[{id, nombre}]` de usuarios activos con rol `mecanico`, por nombre)
- [X] T030 [US1] Añadir a `backend/src/eventos.rs` el endpoint `GET /api/eventos` (todos, exige `Usuario`; research R11) y registrarlo en `backend/src/rutas/mod.rs`: `Sse` sobre `BroadcastStream` de `eventos.suscribir()` que envía cada `CambioEstado` como evento SSE con nombre `orden` y datos JSON `{id, estado}`, ignora los avisos de `Lagged` y usa `KeepAlive::default()` para que el proxy no corte la conexión

### Tests for User Story 1

- [X] T031 [US1] Añadir a `backend/tests/comun/mod.rs`, parte 2 (depende de T027 y T029): `async fn crear_usuario(pool, login, rol, activo) -> (i64, String)`, que genera una contraseña aleatoria con `auth::nuevo_token()`, guarda su `auth::hash_contrasena` y devuelve el id y la contraseña; y `async fn entrar(app, login, contrasena) -> String`, que hace `POST /api/sesion` y devuelve la cookie. Las pruebas nunca contienen la contraseña demo ni ninguna otra escrita a mano (principio V, constitución v1.2.1)
- [X] T032 [US1] Crear `backend/tests/spec000_sesion.rs` con `#[sqlx::test]` (depende de T031): `POST /api/sesion` con `{login, contrasena}` correctos → 204 y `Set-Cookie: sesion=…; HttpOnly; SameSite=Lax; Path=/` (sin `Secure` con `URL_PUBLICA` http); el login se acepta en mayúsculas (`LUIS`); contraseña errónea, usuario inexistente y usuario con `activo = false` → 401 con el texto exacto «Usuario o contraseña incorrectos»; `GET /api/yo` sin cookie → 401 y con cookie → `{id, nombre, rol}` sin `hash_contrasena`; sesión con `expira_en` en el pasado → 401; usuario desactivado después de entrar → 401; `DELETE /api/sesion` → 204 y la misma cookie ya da 401; `GET /api/mecanicos` devuelve solo los usuarios activos con rol `mecanico`, ordenados por nombre; `exigir_rol(&[Rol::Recepcion])` → 403 para un mecánico y OK para `jefe` (FR-002); `exigir_mecanico_de(id_de_luis)` → OK para `luis`, 403 para `marta` y OK para `paco` (jefe); `GET /api/eventos` sin cookie → 401

### Datos y frontend de User Story 1

- [X] T033 [US1] Crear `backend/seed/demo.sql` (solo desarrollo; cabecera en comentario: «Datos de demostración. Las credenciales y tokens de este fichero son ficticios (constitución v1.2.0, principio V). Nunca cargar en producción. Se carga sobre una base recién migrada») dentro de `BEGIN; … COMMIT;`, parte 1: los cinco usuarios demo del quickstart con contraseña `demo1234` — `ana` («Ana», `recepcion`), `luis` («Luis», `mecanico`), `marta` («Marta», `mecanico`), `pepe` («Pepe», `almacen`) y `paco` («Paco», `jefe`). El valor de `hash_contrasena` (argon2id de `demo1234`, el mismo para los cinco) se obtiene una vez con `crear-usuario` contra la base local y se copia; no se escribe ninguna otra contraseña en el repositorio
- [X] T034 [US1] Crear `frontend/src/routes/+layout.ts` y `frontend/src/routes/+layout.svelte` (depende de T021 y T022): `+layout.ts` con `export const ssr = false; export const prerender = false;` y `load` que pide `GET /api/yo` y devuelve `{ yo: Yo | null }` (401 → `null`, sin redirigir desde `api.ts` en esta llamada). `+layout.svelte` (Svelte 5 runes, importa `../app.css`): las rutas públicas (`/entrar`, `/cita`, `/presupuesto/[token]`, FR-003) se muestran sin menú y sin pedir sesión; en las demás, sin `yo` → `goto('/entrar')`; con `yo`, barra superior (Skeleton `AppBar` o equivalente) con «Talleres Rueda», el menú filtrado por rol, `nombre` + `NOMBRE_ROL[rol]` y el botón «Salir» (`DELETE /api/sesion` → `/entrar`). Menú, definido en una sola lista `{texto, ruta, roles}` dentro del propio layout: «Tablero» `/tablero` (jefe), «Citas» `/citas` (recepcion), «Órdenes» `/ordenes` (recepcion), «Mis órdenes» `/ordenes` (mecanico), «Almacén» `/almacen` (almacen), «Pedidos» `/pedidos` (almacen), «Historial» `/historial` (todos); `jefe` ve todas las entradas (FR-002) sin duplicar «Órdenes»/«Mis órdenes» (ve «Órdenes»). No crear las páginas de esas rutas: son de las specs 001–008
- [X] T035 [US1] Crear `frontend/src/routes/+error.svelte` (Svelte 5, `page` de `$app/state`; depende de T034): se muestra dentro del layout, así que el menú del rol sigue visible. Si es 404: «Esta página no existe»; con cualquier otro error: «Ha ocurrido un error». Siempre texto fijo, nunca el mensaje técnico del error. En la fase 0 es lo que se ve al entrar con cualquier rol, porque las páginas de inicio de cada rol (`/citas`, `/ordenes`, `/almacen`, `/tablero`) llegan con sus specs; no se crean páginas de relleno (decisión sobre G5, analyze-1)
- [X] T036 [US1] Crear `frontend/src/routes/+page.svelte` (ruta `/`, depende de T034): redirige con `goto` a la página de inicio del rol — `recepcion` → `/citas`, `mecanico` → `/ordenes`, `almacen` → `/almacen`, `jefe` → `/tablero` — (usando `INICIO_ROL` de `formato.ts`), o a `/entrar` si no hay sesión
- [X] T037 [US1] Crear `frontend/src/routes/entrar/+page.svelte` (depende de T022 y T036): formulario centrado con «Usuario» (`autocomplete="username"`, con foco al cargar), «Contraseña» (`type="password"`, `autocomplete="current-password"`) y botón «Entrar» (deshabilitado mientras se envía); al enviar, `POST /api/sesion` y, si va bien, `invalidateAll()` y `goto(INICIO_ROL[yo.rol])` tras pedir `GET /api/yo`; si falla, muestra el `error` del servidor («Usuario o contraseña incorrectos») sin borrar el usuario escrito. Si ya hay sesión al abrir `/entrar`, va directamente a la página de inicio (SC-001: entrar en menos de 15 s)

**Checkpoint**: con `npm run build` y `cargo run`, en http://localhost:3000 se recorren los pasos
1–3 del quickstart §0. Tras entrar se ve el menú del rol y, en el contenido, «Esta página no
existe» hasta que lleguen las pantallas de 001–008. `/cita` y `/presupuesto/abc` se abren sin
usuario, sin menú y sin redirigir a `/entrar`.

---

## Phase 4: Polish & datos demo para el trabajo en paralelo

**Purpose**: datos de ejemplo que permiten a A y B empezar sus specs sin esperarse (plan maestro,
«Reparto y orden de trabajo»), documentación de arranque y verificación final.

- [ ] T038 Añadir a `backend/seed/demo.sql`, parte 2 (catálogo): la pieza `PF-123` «Pastillas freno», `precio_venta` 25.00, `stock` 4 (quickstart); la pieza `BJ-7` «Bujía», `precio_venta` 8.00, `stock` 0, que solo usa el pedido demo pendiente (T039), para que recibirlo no cambie el stock de `PF-123` que espera el quickstart §5.1; y los proveedores precargados «Recambios Sur» y «Autopartes Norte» (spec-006 FR-010)
- [ ] T039 Añadir a `backend/seed/demo.sql`, parte 3 (órdenes): clientes y vehículos de ejemplo con matrículas distintas de `1234BCD` (el quickstart §1 la da de alta como nueva), con citas en **fechas pasadas** (`current_date - n`, para no gastar el cupo de los próximos días, quickstart §1.4) y una orden por cita con `numero` 1, 2, 3…, cubriendo: al menos una orden en **cada uno** de los 7 estados; para B, una `recibida` y dos `en_reparacion` de `luis` (además de una `en_diagnostico` de `luis`, que pide spec-006); para A, órdenes repartidas entre `luis` y `marta` y al menos una abierta con `fecha_entrega_prometida` en el pasado (`current_date - 1`, «Retrasada» en el tablero, quickstart §8.4). La orden `en_espera_piezas` lleva un `pedido` `pendiente` (proveedor «Recambios Sur», pieza `BJ-7`, 1 ud; nunca `PF-123`, ver T038): sin él no podría salir nunca de ese estado, porque solo sale al recibir su último pedido pendiente (data-model). Al final, `UPDATE contador SET ultimo = (SELECT max(numero) FROM orden) WHERE nombre = 'orden'` para que la primera orden real siga sin huecos
- [ ] T040 Añadir a `backend/seed/demo.sql`, parte 4 (presupuestos, imputaciones y factura): un presupuesto `aceptado` (con líneas, `iva_porcentaje` 21.00 y totales que cuadren con `dinero.rs`) para cada orden de estado `en_reparacion` o posterior; un presupuesto `emitido` con `token` conocido `demo-presupuesto-emitido` y `caduca_el = current_date + 15` en la orden `en_diagnostico` de A (para spec-004 mientras B termina 003); imputaciones (mano de obra a 40.00 y `PF-123` a 25.00, con importes de `dinero.rs` y `usuario_id` del mecánico asignado) coherentes con el estado: en las `en_reparacion`, un acumulado ≤ 110 % del presupuestado; en la `pendiente_conformidad`, > 110 %; en la `terminada`, al menos dos (para facturarla en spec-007); en la `facturada`, las mismas que copia su `factura` en `linea_factura`. Las imputaciones demo no descuentan el stock: `PF-123` se queda con 4 (quickstart §5.1); y `UPDATE contador … WHERE nombre = 'factura'` con el máximo `numero` de factura
- [ ] T041 [P] Actualizar `README.md` con una sección «Arrancar en local» que remita al [quickstart](specs/000-base-comun/quickstart.md) y añada los requisitos que no figuran allí: `cargo install sqlx-cli --no-default-features --features postgres,rustls`, `cargo sqlx database reset` para empezar con la base vacía, cómo cargar los datos demo sin tener `psql` instalado (`docker compose exec -T postgres sh -c 'psql -v ON_ERROR_STOP=1 -U "$POSTGRES_USER" -d "$POSTGRES_DB"' < backend/seed/demo.sql`), y el uso de `cargo run --bin crear-usuario` para dar de alta, restablecer y desactivar usuarios
- [ ] T042 [P] Revisar que no hay secretos en el repositorio (principio V): `git grep -n -i -E "password|contrasena|secret|token"` solo encuentra nombres de campo, las credenciales y el token ficticios de demostración en `backend/seed/demo.sql` y `quickstart.md` y los valores de ejemplo de `.env.example`; `git grep -n -E "demo1234|demo-presupuesto-emitido"` solo los encuentra en `backend/seed/demo.sql`, `quickstart.md` y documentos de `specs/` que los citan, y nunca en `backend/src/`, `backend/tests/` ni `frontend/src/` (excepción del principio V, constitución v1.2.1); `.env` no está versionado
- [ ] T043 Verificación final: sobre una base vacía (`docker compose up -d`, `cargo sqlx database reset` o `migrate run`, y después `psql "$DATABASE_URL" -f backend/seed/demo.sql` o, sin `psql` instalado, `docker compose exec -T postgres sh -c 'psql -v ON_ERROR_STOP=1 -U "$POSTGRES_USER" -d "$POSTGRES_DB"' < backend/seed/demo.sql`; comprobar que las dos formas cargan los datos), `cargo test`, `cargo clippy -- -D warnings`, `npm run check` y `npm test` pasan; con `npm run build` y `cargo run`, recorrer a mano el [quickstart §0](quickstart.md) completo como `luis`, con contraseña errónea y como `paco`, entrar también como `ana` y `pepe` y comprobar que cada uno ve solo su menú (SC-002), y comprobar que `/cita` se abre sin usuario. Anotar el resultado en el PR

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: sin dependencias; T001–T004 en paralelo.
- **Foundational (Phase 2)**: depende de Setup. **Bloquea US1 y las specs 001–008.**
  - Migración: T005 → T006 → T007 → T008 (mismo fichero, en orden).
  - Backend: T009–T015 y T018 en paralelo; T016 (`estados.rs`) tras T010; T017 (`eventos.rs`)
    tras T016; T019 (arranque) tras todos ellos.
  - Frontend: T020 y T021 en paralelo; T022 tras T020.
  - Pruebas: T023 tras T019; T024 y T025 tras T023; T026 tras T021.
- **US1 (Phase 3)**: depende de Foundational. T027 (`auth.rs`) → T028 (`crear-usuario`) y T029
  (`rutas/sesion.rs`) → T030 (SSE). T031 (utilidades de prueba, parte 2) tras T027 y T029 → T032
  (pruebas de sesión). T033 (usuarios demo) tras T028. Frontend: T034 → T035 y T036 → T037.
- **Polish (Phase 4)**: T038 → T039 → T040 tras T033 (mismo fichero, en orden); T041 y T042 en
  paralelo; T043 al final.

### Después de la fase 0

Con T043 verde, se fusiona en `main` y A (001 → 002 → 004 → 008) y B (006 → 003 → 005 → 007)
crean sus ramas. Cualquier cambio posterior a la migración, `estados.rs`, `eventos.rs`,
`dinero.rs`, `matricula.rs`, `formato.rs`, `auth.rs`, `tipos.ts`, `formato.ts`, `+layout.svelte`
o `demo.sql` se revisa entre los dos (plan maestro).

### Parallel Opportunities

- Setup: T001, T002, T003, T004.
- Foundational: T009–T015, T018, T020 y T021 a la vez (11 ficheros distintos), mientras otra
  persona escribe la migración T005–T008.
- Pruebas: T024, T025 y T026.
- US1: T028 junto a T029; el frontend T034–T037 puede avanzar contra el backend en cuanto T029
  exista.

---

## Parallel Example: fase 0 en pareja

```bash
# Persona A (base de datos y backend de sesión):
Task: "T005–T008 Migración 0001 en backend/migrations/0001_esquema_inicial.sql"
Task: "T027 auth.rs" → "T029 rutas/sesion.rs" → "T031–T032 pruebas de sesión" → "T033 seed/demo.sql (usuarios)"

# Persona B (módulos comunes y frontend):
Task: "T012 dinero.rs", "T013 contador.rs", "T011 fechas.rs", "T014 matricula.rs", "T015 formato.rs"
Task: "T016 estados.rs" → "T017 eventos.rs"
Task: "T020 tipos.ts", "T021 formato.ts" → "T022 api.ts"
Task: "T034 +layout.svelte" → "T035 +error.svelte" → "T037 entrar/+page.svelte"
```

---

## Implementation Strategy

### MVP (= toda la fase 0)

1. Phase 1: Setup → `docker compose up -d`, `cargo build`, `npm run build`.
2. Phase 2: Foundational → todas las tablas y módulos comunes, con sus pruebas en verde.
3. Phase 3: US1 → entrar con cada rol y ver solo su menú.
4. **STOP and VALIDATE**: quickstart §0 completo.
5. Phase 4: datos demo para el trabajo en paralelo y verificación final → fusionar en `main`.

### Notas

- [P] = otro fichero y sin dependencias pendientes.
- Hacer un commit por tarea o grupo lógico.
- Si durante la fase aparece algo que no está en las specs, se apunta y no se construye
  (principio III).

---

## Desviaciones durante la implementación

Diferencias entre el texto de las tareas y lo implementado. Cada cambio de un fichero compartido
se revisa entre los dos (plan maestro).

| Tarea | Desviación | Motivo |
|-------|------------|--------|
| T008 | La migración crea **15 tablas**, no 14 (corregido en el texto de T008 y en `plan.md`). | Error de recuento en analyze-1. Son las 15 entidades de data-model: usuario, sesion, contador, cliente, vehiculo, cita, orden, pieza, presupuesto, linea_presupuesto, imputacion, proveedor, pedido, factura y linea_factura. |
| T019 | `lib.rs` no declara todavía el módulo `auth`; se declara con T027. | `auth.rs` es de la fase 3 (US1). Declararlo en la fase 2 impediría compilar. |
| T016 | `estados.rs` no implementa a mano `PgHasArrayType` para `Estado`. | La derivación `sqlx::Type` de SQLx 0.8 ya lo implementa, y repetirlo da un error de implementaciones en conflicto (E0119). Sigue siendo posible pasar `&[Estado]` en `estado = ANY($3)`. |
| T016 | `cambiar_estado` no lleva `#[must_use]`; lo lleva solo `CambioEstado`. | `Resultado<CambioEstado>` ya obliga a usar el valor y `CambioEstado` avisa si no se publica. Repetirlo en la función hace saltar `clippy::double_must_use`, y T043 exige clippy sin avisos. |
| T018 | Añadido `Correo::desde_config(&Config) -> Result<Correo, String>`. | La tarea dice que `Correo` se construye desde `Config`, pero no nombra la función. Devuelve un error en español si `SMTP_URL` o `CORREO_REMITENTE` no son válidos, igual que `Config::desde_entorno`. |
| T022 | Añadido y exportado `esRutaPublica(ruta)` en `api.ts`. | `api.ts` necesita saber si la página es pública para no redirigir a `/entrar` con un 401. El layout (T034) usará la misma función, así la lista de páginas públicas (FR-003) está en un solo sitio. |
| T023 | Añadido `app_prueba(pool) -> Router` en `tests/comun/mod.rs`. | Atajo de `app(estado_prueba(pool))`, que repetirán todas las pruebas HTTP de las specs. |
| T027 | La cookie `sesion` **no lleva `Max-Age`**: es una cookie de sesión del navegador. | Con `Max-Age=43200` fijo, el navegador borraría la cookie 12 h después de entrar aunque el usuario siguiera activo, en contra de «caduca tras 12 h **sin actividad**» (spec, casos límite). La caducidad la decide el servidor con `expira_en`, que se renueva en cada petición. |
| T027 | `DURACION_SESION` es un `std::time::Duration` y la nueva caducidad se calcula en Rust (`nueva_expiracion()`) en vez de con `now() + interval '12 hours'` en SQL. Se exportan además `nueva_expiracion()` y `token_de_cookie()`. | La duración se escribe en un solo sitio y la usan el login (T029) y la renovación del extractor. `token_de_cookie` la necesita también `DELETE /api/sesion` para borrar la sesión. |
| T029 | Nueva variante `ErrorApi::CredencialesIncorrectas` en `error.rs` (401 «Usuario o contraseña incorrectos»). | `ErrorApi::SinSesion` tiene un texto fijo («Tienes que entrar con tu usuario»), y el contrato pide otro texto para el 401 al entrar. |
| T028 | `--desactivar` solo desactiva un usuario existente (y borra sus sesiones): no pide contraseña, nombre ni rol y no hace `INSERT … ON CONFLICT`. Si la entrada es un terminal, pide «Contraseña:», pero lo que se escribe se ve. | Para desactivar no tiene sentido pedir una contraseña nueva. Ocultar lo que se escribe exigiría otra dependencia (principio I); se puede pasar la contraseña por una tubería (`echo … \| crear-usuario …`). |
| T031 | `crear_usuario` pone como nombre el login con la inicial en mayúscula («luis» → «Luis»). | Las pruebas de `GET /api/yo` y `GET /api/mecanicos` (orden por nombre) necesitan un nombre conocido. |
| T032 | Prueba adicional `cada_peticion_renueva_la_sesion`. `exigir_rol` y `exigir_mecanico_de` se prueban como pruebas unitarias (`#[test]`), sin pasar por HTTP. | La renovación de la sesión es la regla de los casos límite de la spec y no tenía prueba. Ningún endpoint de la base común usa todavía esas dos funciones; las specs 003, 005 y 007 las probarán por HTTP. |
| T033 | La cabecera de `demo.sql` cita la constitución **v1.2.1** en vez de v1.2.0. | Es la versión vigente; la excepción del principio V viene de la 1.2.0 y la 1.2.1 la aclara. |
| T001 | `backend/Cargo.toml` lleva `default-run = "taller"` en `[package]`. | Con dos binarios (`taller` y `crear-usuario`, T028), `cargo run` a secas falla porque no sabe cuál arrancar, y el quickstart usa `cargo run`. |
| T002 | `frontend/src/app.css` importa también `@skeletonlabs/skeleton/optional/presets`. | En Skeleton v3 las clases `preset-*` (`preset-filled-primary-500`, `preset-tonal`…) son un import opcional. Sin él, el botón «Entrar» de `/entrar` (T037) y «Salir» del layout (T034) se veían como texto gris. |

