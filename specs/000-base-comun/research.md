# Research: Base común (Talleres Rueda)

Decisiones técnicas y de alcance que comparten las 8 specs. Formato: Decisión, Motivo y
Alternativas descartadas. Fecha: 2026-10-15.

## R1. Preguntas abiertas de las specs

| Pregunta | Decisión | Motivo |
|---|---|---|
| Q1 · Rechazo de presupuesto (spec-004/005) | Presupuesto inicial rechazado → la orden vuelve a «en diagnóstico». Ampliación rechazada → sigue «pendiente de conformidad». | Es la opción más simple y no necesita estados nuevos (principio I). |
| Q2 · Datos fiscales de la factura (spec-007) | Factura simplificada: datos del taller, nombre del cliente y matrícula. | Así no se piden más datos al cliente (principio V). ⚠️ Antes de usarla de verdad, el taller debe confirmar con su gestoría que la factura simplificada le sirve. |
| Q3 · Acceso del personal (spec-008 y todas) | Usuario y contraseña por empleado, cada uno con un rol. | Las specs exigen saber quién es «el mecánico asignado» y quién imputa. |

Las tres se resolvieron con la opción recomendada porque el equipo no respondió. Se pueden cambiar
editando este archivo y la spec afectada.

## R2. Arquitectura: un solo proceso y un solo origen

- **Decisión**: el backend Axum sirve la API JSON en `/api/*` y también los ficheros estáticos del
  frontend SvelteKit, compilado como SPA con `adapter-static` y `fallback: 'index.html'`. En
  desarrollo, Vite redirige `/api` a `localhost:3000`.
- **Motivo**: se despliega una sola cosa. Al compartir origen, la cookie de sesión funciona sin
  CORS. El taller no necesita SSR ni SEO, salvo quizá la página pública de citas, y ahí basta
  una SPA.
- **Alternativas descartadas**: SvelteKit con `adapter-node` haciendo de proxy, porque supone dos
  procesos y dos despliegues; y orígenes separados con CORS, porque complica la cookie.

## R3. Pila y versiones

- **Backend**: Rust estable (edición 2024), Axum 0.8 (`macros`), SQLx 0.8 (`postgres`,
  `runtime-tokio`, `tls-rustls`, `macros`, `migrate`, `chrono`, `rust_decimal`), tower-http
  (`fs`, `trace`), chrono + chrono-tz, rust_decimal (+ `rust_decimal_macros`), argon2, lettre
  (SMTP con rustls), serde + serde_json, tracing + tracing-subscriber, tokio. Auxiliares: `sha2`
  (hash del token de sesión), `rand` y `base64` (tokens aleatorios), `dotenvy` (`.env` en
  desarrollo), `tokio-stream` y `futures-util` (SSE). Pruebas: `tower` y `http-body-util`.
  Ningún id es UUID, así que no se usa la feature `uuid` de SQLx.
- **Frontend**: Node 22 LTS, SvelteKit 2 con Svelte 5 (runes), Skeleton UI v3 sobre Tailwind CSS 4
  y TypeScript 5 (SvelteKit 3 y TypeScript 7 ya existen, pero no se adoptan en la v1).
- **Base de datos**: PostgreSQL 16 o posterior.
- **Motivo**: es la pila propuesta en el enunciado. Las versiones son las estables actuales al
  empezar; se fijan en `Cargo.lock` y `package-lock.json`.

## R4. Dinero, IVA y redondeo

- **Decisión**: los importes se guardan como `NUMERIC(12,2)` y en Rust se manejan con
  `rust_decimal::Decimal`. Las cantidades son `NUMERIC(10,2)`, en horas o unidades. El importe de
  cada línea es cantidad × precio, redondeado a 2 decimales (mitad hacia arriba). La base es la
  suma de las líneas. El IVA se calcula una vez sobre la base, redondeado, y total = base + IVA.
  El % de IVA se guarda en cada presupuesto y factura.
- **Motivo**: nunca se usan floats, así que las cuentas cuadran con una calculadora (SC de
  spec-003 y spec-007). Guardar el % permite que un cambio de IVA no altere documentos ya
  emitidos.
- **Alternativas descartadas**: guardar céntimos en enteros, porque es igual de exacto pero obliga
  a convertir en cada pantalla.
- **Formato visible**: `Intl.NumberFormat('es-ES', {style:'currency', currency:'EUR'})`, centralizado
  en `frontend/src/lib/formato.ts`.

## R5. Números correlativos sin huecos (orden y factura)

- **Decisión**: se usa una tabla `contador(nombre, ultimo)`. Dentro de la misma transacción que
  crea la orden o la factura se ejecuta `UPDATE contador SET ultimo = ultimo + 1 WHERE nombre = $1
  RETURNING ultimo`.
- **Motivo**: el bloqueo de fila serializa a quienes abren órdenes a la vez. Si la transacción
  falla, el número se devuelve, de modo que no hay huecos (spec-002 FR-002, spec-007 FR-004).
- **Alternativas descartadas**: `SEQUENCE`/`SERIAL`, porque deja huecos cuando hay rollback.

## R6. Cupo diario sin sobreventa

- **Decisión**: al crear una cita se ejecuta `pg_advisory_xact_lock` con la fecha como clave,
  después se cuentan las citas de esa fecha y se inserta solo si quedan plazas. Todo ocurre en una
  transacción.
- **Motivo**: dos clientes que confirman la última plaza a la vez quedan serializados
  (spec-001 US3).
- **Alternativas descartadas**: el aislamiento `SERIALIZABLE` con reintentos, porque es más
  complejo.

## R7. Configuración y secretos

- **Decisión**: todo se lee de variables de entorno, con `.env` solo en desarrollo e ignorado por
  git, y un `.env.example` versionado sin valores secretos: `DATABASE_URL`, `SMTP_URL`,
  `CORREO_REMITENTE`, `URL_PUBLICA`, `TALLER_CUPO_DIARIO`, `TALLER_PRECIO_HORA`,
  `TALLER_IVA_PORCENTAJE`, `TALLER_NOMBRE`, `TALLER_NIF`, `TALLER_DIRECCION`.
- **Motivo**: cumple el principio V (ningún secreto en el código) y deja los valores de negocio
  fuera del código sin crear pantallas que nadie ha pedido (principio III).

## R8. Autenticación y roles

- **Decisión**: la sesión se guarda en el servidor, en la tabla `sesion`. Se usa una cookie
  `sesion` `HttpOnly`, `SameSite=Lax` y `Secure` en producción, que contiene un token aleatorio de
  32 bytes; en la base de datos se guarda su hash SHA-256. Las contraseñas se guardan con
  argon2id. Un extractor de Axum (`Usuario`) exige sesión y una función `exigir_rol(&[Rol])`
  comprueba el rol; el rol `jefe` pasa siempre. Los usuarios se crean con el binario
  `cargo run --bin crear-usuario`.
- **Motivo**: es lo más simple y seguro para una aplicación de un solo origen y unos 10 usuarios.
- **Alternativas descartadas**: JWT, porque complica revocar sesiones sin aportar nada aquí; y
  OAuth/SSO, que el taller no necesita.

## R9. Enlace público del presupuesto

- **Decisión**: cada presupuesto emitido lleva un `token` aleatorio de 32 bytes en base64url, único
  e imposible de adivinar. La URL es `{URL_PUBLICA}/presupuesto/{token}`. La respuesta se guarda
  con `UPDATE ... WHERE id = $1 AND respondido_en IS NULL`, de modo que solo cuenta la primera.
- **Motivo**: cumple «una sola vez» y «solo su presupuesto» (spec-004) sin que el cliente tenga
  que crear una cuenta.

## R10. Correo electrónico

- **Decisión**: se usa `lettre` por SMTP, configurado con `SMTP_URL`. En desarrollo se usa
  **Mailpit** (en docker), cuya bandeja se ve en `http://localhost:8025`, sin enviar correos
  reales. El correo se envía después de confirmar la transacción. Si falla, no se deshace nada:
  queda marcado `aviso_enviado = false` y la pantalla lo muestra.
- **Motivo**: una persona no técnica puede ver los correos en Mailpit (principio IV).

## R11. Tablero en tiempo real

- **Decisión**: se usa **Server-Sent Events** en `GET /api/eventos`. El backend tiene un
  `tokio::sync::broadcast` en el estado de la app, y cada cambio de estado de una orden publica
  un evento SSE `orden` con los datos `{id, estado}` (el mismo formato que el contrato). El tablero escucha con `EventSource`, que se reconecta solo, y vuelve
  a pedir `GET /api/tablero`.
- **Motivo**: basta con comunicación del servidor al navegador. Es lo más simple que cumple
  «sin recargar» (spec-008).
- **Alternativas descartadas**: WebSockets, porque son bidireccionales y aquí no hace falta;
  `LISTEN/NOTIFY` de Postgres, que solo sería necesario con varios procesos; y el *polling*,
  porque añade un retardo de hasta N segundos.

## R12. Fechas y zona horaria

- **Decisión**: los instantes se guardan como `TIMESTAMPTZ` y las fechas sin hora como `DATE`.
  «Hoy» se calcula siempre en `Europe/Madrid` con chrono-tz. En pantalla las fechas se ven como
  dd/mm/aaaa y la hora como HH:mm.
- **Motivo**: la caducidad (15 días), el retraso del tablero y los días de cita dependen del día
  en España (principio II).

## R13. Pruebas

- **Decisión**:
  - **Backend**: pruebas de integración con `#[sqlx::test]`, que crea una base de datos limpia por
    prueba, para cada regla de negocio de las specs: km, cupo, correlativos, caducidad, 10 %,
    stock y transiciones de estado.
  - **Frontend**: `svelte-check` y Vitest solo para `formato.ts`.
  - **Aceptación**: se recorre a mano `quickstart.md` en la aplicación (principio IV).
- **Motivo**: las reglas de negocio son lo crítico. Una batería E2E con navegador sería complejidad
  anticipada (principio I).

## R14. Transiciones de estado en un solo sitio

- **Decisión**: todas las transiciones de la orden pasan por una única función
  `backend/src/estados.rs::cambiar_estado(tx, orden_id, desde: &[Estado], hacia: Estado)`. Esta
  función hace `UPDATE orden SET estado = hacia WHERE id = $1 AND estado = ANY(desde)` y falla con
  409 si no se actualiza ninguna fila. **No publica el evento**: devuelve un `CambioEstado`
  (`#[must_use]`) y quien la llama ejecuta `cambio.publicar(&eventos)` **después del `commit`**.
- **Motivo**: cuatro specs de dos personas cambian estados. Un único punto evita incoherencias y
  garantiza que el tablero se entere siempre. Se publica tras el `commit` porque, si se publicara
  dentro de la transacción, el tablero podría volver a pedir `GET /api/tablero` antes de que el
  cambio esté guardado y seguir viendo el estado antiguo; si la transacción falla, no se publica
  nada. `#[must_use]` hace que el compilador avise si alguien olvida publicar.
- **Alternativas descartadas**: publicar dentro de `cambiar_estado`, por la carrera descrita.
