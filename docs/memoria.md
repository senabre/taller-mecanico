# Memoria del proyecto · Talleres Rueda

## 1. Portada y resumen

**Talleres Rueda · Gestión de un taller mecánico** (práctica SI-26)

| | |
|---|---|
| Autores | Persona A · @evm60-ua (HU-01, HU-02, HU-04 y HU-08) · Persona B · @senabre (HU-03, HU-05, HU-06 y HU-07) |
| Fecha | 14/11/2026 |
| Versión del código | `main`, commit `1c347b0`; la verificación final, en la rama `cierre` (`52c7024`) |
| Repositorio | https://github.com/senabre/taller-mecanico |

**Resumen.** Talleres Rueda es una aplicación web que acompaña a cada vehículo desde que el cliente
pide cita hasta que se lo lleva reparado y facturado. La usan el cliente, que pide cita y responde al
presupuesto desde un enlace sin entrar en la aplicación, y el personal del taller: recepción, mecánicos,
almacén y el jefe de taller, que ve todas las órdenes abiertas en un tablero que se actualiza solo. Se
ha construido con desarrollo guiado por especificaciones (Spec Kit): una base común y **8
especificaciones**, una por historia de usuario, repartidas entre dos personas. El resultado:
- **336 tareas, todas hechas**, incluidas las revisiones finales de la Persona B.
- **177 pruebas automáticas en verde** según la verificación final: 156 de backend (155 con `cargo
  test` y una más que se lanza aparte porque tarda unos 100 s) y 21 de frontend (Vitest).
- La guía de validación (quickstart común) se recorrió en un navegador real automatizado. La
  verificación final sobre `main` (§0–§8, 123 de 124) no tuvo ningún fallo de la aplicación. El
  recorrido de la cita a la factura (§9) dio 10 de 10 y el de error, permiso y estado (R2), 9 de 9.
- De los **126 criterios** de aceptación y de éxito de las 8 historias, **119 están verificados** y
  **7 verificados con desviación** (ninguno pendiente), según la
  [matriz de trazabilidad](../specs/matriz-trazabilidad.md).
- **135 hallazgos** registrados en los analyze y converge: 128 cerrados, 7 asignados y ninguno
  abierto.

## 2. Repositorio

- **URL**: https://github.com/senabre/taller-mecanico
- **Rama principal**: `main` (`1c347b0`), con las 8 specs y la documentación final (matriz,
  recorridos, agrupamiento, revisión cruzada y esta memoria), que entró con el PR #6. La rama `cierre`
  añade la verificación final (`52c7024`), todavía sin fusionar.

## 3. Introducción

**Contexto.** Un taller mecánico gestiona hoy cada reparación con llamadas, papeles y una pizarra.
El cliente tiene que llamar o ir al taller para pedir cita y para aprobar el presupuesto. Recepción
anota a mano el estado del coche al recibirlo. Los mecánicos apuntan tiempos y piezas que después
hay que volver a teclear en la factura. Y el jefe de taller sigue las órdenes en una pizarra (las HU
de cada spec, enlazadas en el apartado 4).

**Problema.** Sin un sistema común se pierden datos y tiempo:
- discusiones sobre el estado en que se entregó el coche;
- presupuestos que no constan como aceptados;
- trabajo que no llega a la factura;
- órdenes paradas esperando una pieza sin que nadie lo vea;
- retrasos que solo se detectan cuando el cliente reclama.

**Objetivos**, uno por historia de usuario:
1. Que el cliente pida cita en línea, sin llamar (HU-01).
2. Que recepción abra la orden con el kilometraje y los daños visibles (HU-02).
3. Que el mecánico registre el diagnóstico y un presupuesto línea a línea (HU-03).
4. Que el cliente acepte o rechace el presupuesto desde un enlace (HU-04).
5. Que el mecánico impute el tiempo y las piezas reales, con un tope sobre lo presupuestado (HU-05).
6. Que el almacén pida al proveedor las piezas que faltan, con la orden parada mientras tanto (HU-06).
7. Que la factura salga de lo imputado y el cliente reciba el aviso (HU-07).
8. Que el jefe vea todas las órdenes abiertas por estado, con los retrasos y la carga de cada
   mecánico, en un tablero que se actualiza solo (HU-08).

Todo bajo la [constitución](../.specify/memory/constitution.md) (v1.2.1): simplicidad, español de
España y euro, cero alcance fantasma, verificable por una persona no técnica y respeto por los datos.

## 4. Análisis

### Actores y roles

| Actor | Rol en la app | Qué hace |
|---|---|---|
| Cliente | Sin sesión | Pide cita (`/cita`) y responde al presupuesto desde el enlace del correo (`/presupuesto/[token]`) |
| Recepcionista | `recepcion` | Ve las citas, abre las órdenes, factura y consulta el historial |
| Mecánico | `mecanico` | Ve sus órdenes, diagnostica, presupuesta, imputa y termina la reparación |
| Responsable de almacén | `almacen` | Da de alta piezas, pide al proveedor y recibe los pedidos |
| Jefe de taller | `jefe` | Puede hacer todo lo anterior y es el único que ve el tablero |

Cada empleado entra con su usuario y su contraseña (base común, research R8).

### Historias de usuario

El reparto entre las dos personas y su porqué están en [agrupamiento.md](agrupamiento.md).

| HU | Historia | Prioridad · pts | Persona | Spec | Historias / escenarios / FR / SC |
|---|---|---|---|---|---|
| HU-01 | Como cliente, pedir cita indicando la matrícula y el motivo, para no tener que llamar ni ir al taller | Alta · 3 | A | [spec-001](../specs/001-pedir-cita-en-linea/spec.md) | 3 / 16 / 11 / 4 |
| HU-02 | Como recepcionista, abrir la orden al recibir el coche, anotando el kilometraje y los daños visibles | Alta · 3 | A | [spec-002](../specs/002-abrir-orden-reparacion/spec.md) | 2 / 12 / 8 / 4 |
| HU-03 | Como mecánico, registrar el diagnóstico y un presupuesto línea a línea con mano de obra y piezas | Alta · 5 | B | [spec-003](../specs/003-diagnostico-presupuesto/spec.md) | 3 / 10 / 10 / 4 |
| HU-04 | Como cliente, aceptar o rechazar el presupuesto desde un enlace | Alta · 5 | A | [spec-004](../specs/004-aceptar-presupuesto-enlace/spec.md) | 4 / 11 / 9 / 4 |
| HU-05 | Como mecánico, cargar en la orden el tiempo real y las piezas montadas | Alta · 5 | B | [spec-005](../specs/005-imputar-tiempo-piezas/spec.md) | 4 / 9 / 8 / 4 |
| HU-06 | Como responsable de almacén, pedir al proveedor una pieza que falta, vinculada a su orden | Media · 3 | B | [spec-006](../specs/006-pedido-pieza-proveedor/spec.md) | 4 / 13 / 13 / 6 |
| HU-07 | Como recepcionista, generar la factura a partir de lo imputado y avisar al cliente | Alta · 3 | B | [spec-007](../specs/007-facturar-avisar-cliente/spec.md) | 3 / 8 / 9 / 4 |
| HU-08 | Como jefe de taller, ver en un tablero las órdenes abiertas agrupadas por estado | Media · 5 | A | [spec-008](../specs/008-tablero-ordenes-abiertas/spec.md) | 4 / 13 / 6 / 4 |
| **Total** | | **32 pts** (16 por persona) | | | **27 / 92 / 74 / 34** |

### Requisitos funcionales (resumen por HU)

- **HU-01**:
  - matrícula de 2 a 10 letras o cifras, sin importar espacios ni guiones;
  - un vehículo conocido muestra marca y modelo sin datos del cliente, y uno nuevo pide sus datos;
  - días laborables desde mañana hasta dentro de 30 días, con un cupo diario;
  - como mucho una cita pendiente por matrícula;
  - recepción y el jefe ven la lista del día.
- **HU-02**:
  - la orden solo se abre el día de la cita, una por cita;
  - número correlativo sin huecos;
  - el kilometraje no puede ser menor que el de la visita anterior;
  - mecánico activo y daños obligatorios;
  - la ficha muestra la cita, el vehículo y el mecánico.
- **HU-03**:
  - el diagnóstico pasa la orden a «en diagnóstico»;
  - líneas de mano de obra (en múltiplos de 0,25 h) y de pieza (enteras, solo del almacén);
  - totales con IVA;
  - al emitir, el presupuesto caduca a los 15 días y se envía el enlace al cliente.
- **HU-04**:
  - página pública por enlace, sin datos del cliente;
  - una sola respuesta, con confirmación;
  - aceptar pasa la orden a «en reparación» y rechazar la deja como estaba;
  - un presupuesto caducado no admite respuesta;
  - sin presupuesto aceptado no se imputa;
  - el personal ve el enlace mientras está vigente.
- **HU-05**:
  - imputar horas y piezas descuenta el stock;
  - si el coste pasa del 110 % de lo presupuestado, la orden queda pendiente de conformidad hasta una ampliación aceptada;
  - solo se termina con imputaciones.
- **HU-06**:
  - alta de piezas con referencia única;
  - un pedido deja la orden «en espera de piezas»;
  - al recibir el último pedido, la orden vuelve sola a reparación y el stock sube.
- **HU-07**:
  - factura con número único a partir de lo imputado;
  - impresión limpia y logo opcional;
  - correo «listo para recoger»;
  - historial por matrícula.
- **HU-08**:
  - seis columnas, una por estado abierto;
  - carga por mecánico que cuadra con las tarjetas;
  - las retrasadas destacadas;
  - solo el jefe lo ve;
  - se actualiza solo con cada cambio, al reconectar y a medianoche.

### Requisitos no funcionales

De la constitución y del plan maestro:
- Español de España, importes en euros (`1.234,56 €`) y fechas dd/mm/aaaa, con «hoy» en Europe/Madrid (R12).
- Ningún secreto en el repositorio (R7, principio V).
- Usable en el móvil (390 px) en las páginas públicas y en el tablero.
- Rendimiento medido en los criterios de éxito de cada spec. Por ejemplo, una cita en menos de 1 minuto
  (HU-01 SC-001) o un cambio en el tablero en menos de 5 s (HU-08 SC-002).
- Cada criterio comprobable por una persona en la aplicación (principio IV).

## 5. Diseño

### Arquitectura

Un solo proceso y un solo origen (R2). El backend en **Rust con Axum** sirve la API JSON en
`/api/*` y también la SPA compilada de **SvelteKit**, así que no hay CORS ni dos servidores.

```mermaid
flowchart LR
  N[Navegador · SvelteKit SPA] -- "HTTP /api/*" --> B[Backend Rust · Axum + SQLx]
  B -- "SSE /api/eventos" --> N
  B --> P[(PostgreSQL 16)]
  B -- SMTP --> M[Mailpit · correo de pruebas]
```

- **Base de datos**: PostgreSQL 16, con migraciones SQLx.
- **Correo**: `lettre` por SMTP, a **Mailpit** en desarrollo (R10).
- **Tiempo real**: el tablero escucha **Server-Sent Events** en `GET /api/eventos`. Cada cambio de
  estado de una orden se publica después del `commit` (R11, R14).
- **Sesión**: en el servidor, con una cookie y contraseñas con argon2 (R8).

### Modelo de datos

El modelo es único, en el [data-model común](../specs/000-base-comun/data-model.md). Tablas
principales:

```mermaid
erDiagram
  cliente ||--o{ vehiculo : "tiene"
  vehiculo ||--o{ cita : "pide"
  cita ||--o| orden : "abre (una por cita)"
  vehiculo ||--o{ orden : ""
  usuario ||--o{ orden : "mecánico asignado"
  orden ||--o{ presupuesto : "inicial y ampliaciones"
  presupuesto ||--o{ linea_presupuesto : ""
  orden ||--o{ imputacion : ""
  pieza ||--o{ linea_presupuesto : ""
  pieza ||--o{ imputacion : ""
  orden ||--o{ pedido : ""
  pieza ||--o{ pedido : ""
  proveedor ||--o{ pedido : ""
  orden ||--o| factura : "una por orden"
  factura ||--o{ linea_factura : ""
  usuario ||--o{ sesion : ""
```

Además están `contador` (números correlativos de orden y de factura, R5) y `usuario` (personal con su
rol).

**Máquina de estados de la orden.** Todas las transiciones pasan por una sola función,
`estados::cambiar_estado` (R14). Cada una la provoca la spec que se indica:

```mermaid
stateDiagram-v2
  [*] --> recibida: 002 · recepción abre la orden
  recibida --> en_diagnostico: 003 · diagnóstico
  en_diagnostico --> en_reparacion: 004 · el cliente acepta el presupuesto
  en_reparacion --> pendiente_conformidad: 005 · coste > 110 %
  pendiente_conformidad --> en_reparacion: 004/005 · acepta la ampliación
  en_reparacion --> en_espera_piezas: 006 · pedido al proveedor
  en_espera_piezas --> en_reparacion: 006 · llega el último pedido
  en_reparacion --> terminada: 005 · el mecánico termina
  terminada --> facturada: 007 · factura
  facturada --> [*]
```

### Decisiones técnicas y por qué

De la [constitución](../.specify/memory/constitution.md), el
[plan maestro](../specs/000-base-comun/plan.md), el
[research común](../specs/000-base-comun/research.md) y los `research.md` de cada spec:

| Decisión | Por qué |
|---|---|
| Un solo proceso que sirve la API y la SPA (R2) | Lo más simple: un despliegue, sin CORS (principio I) |
| Dinero en `NUMERIC(12,2)` y `rust_decimal`; el IVA se redondea una vez sobre la base (R4) | Sin errores de coma flotante; los totales cuadran con la calculadora |
| Números de orden y de factura con una tabla `contador` dentro de la misma transacción (R5) | Correlativos sin huecos, incluso con intentos rechazados (spec-002 Q6) |
| Cupo diario con `pg_advisory_xact_lock` por fecha (R6) | Nunca más citas que el cupo, aunque dos clientes confirmen a la vez |
| Enlace del presupuesto con un token aleatorio de 32 bytes (R9), y la respuesta guardada con un solo `UPDATE` condicional (spec-004 Q4, D2) | El cliente no necesita cuenta, y de dos respuestas a la vez solo cuenta una |
| SSE en vez de WebSockets para el tablero (R11) | Basta la comunicación del servidor al navegador; se reconecta solo |
| «Hoy» siempre en Europe/Madrid (R12) | Caducidades, retrasos y días de cita dependen del día en España |
| Una sola función para los cambios de estado, que publica el evento después del `commit` (R14) | Ninguna transición se salta el tablero, y este nunca ve un estado sin guardar |
| Cada spec en su módulo de rutas, su fichero de pruebas y su sección de la ficha | Cada persona trabaja en sus ficheros; los compartidos se revisan entre las dos |

## 6. Implementación

### Pila tecnológica

Versiones de `Cargo.lock` y `package-lock.json` (sin cambios entre `2a7dfce` y `1c347b0`):

| Capa | Tecnología | Versión |
|---|---|---|
| Backend | Rust (edición 2024) · Axum · Tokio · SQLx · tower-http | Axum 0.8.9 · Tokio 1.53.3 · SQLx 0.8.6 · tower-http 0.6.11 |
| Dinero, fechas y seguridad | rust_decimal · chrono · chrono-tz · argon2 | 1.43.0 · 0.4.45 · 0.10.4 · 0.5.3 |
| Correo | lettre | 0.11.23 |
| Frontend | SvelteKit (Svelte 5) con adapter-static · TypeScript · Vite | Kit 2.70.3 · Svelte 5.57.2 · TS 5.9.3 · Vite 8.3.4 |
| Estilos | Tailwind CSS · Skeleton | 4.3.3 · Skeleton 3.2.2 (skeleton-svelte 1.5.3) |
| Pruebas | `#[sqlx::test]` (backend) · Vitest (frontend) | Vitest 5.0.5 |
| Servicios | PostgreSQL · Mailpit (Docker Compose) | `postgres:16` · `axllent/mailpit:v1.27` |

Requisitos según el quickstart: Docker, Rust estable y Node 22. En el equipo de desarrollo de la
Persona A se usaron rustc 1.99.0, Node 25.8.0 y Docker 28.5.1.

### Estructura del repositorio

```text
.specify/memory/constitution.md   constitución
specs/000-base-comun/             plan maestro, research común, data-model, contrato y quickstart comunes
specs/00N-…/                      una spec por HU: spec, plan, research, tasks, evidencias…
specs/matriz-trazabilidad.md · specs/revision-cruzada.md · specs/recorridos/ · specs/verificacion-final/
backend/migrations/               0001 esquema · 0002 líneas (003) · 0003 imputaciones (005)
backend/seed/demo.sql             datos demo
backend/src/                      auth, config, contador, correo, dinero, error, estados, eventos,
                                  fechas, formato, lineas, matricula y rutas/ (un módulo por spec)
backend/tests/                    un fichero de pruebas por spec, más tests/comun
frontend/src/routes/              cita, citas, ordenes (con la ficha por secciones), presupuesto,
                                  almacen, pedidos, facturas, historial, tablero, entrar
frontend/src/lib/                 api, formato, tipos, componentes/, medianoche, recarga, eventos
docs/                             agrupamiento.md y esta memoria
```

### Partes más relevantes del código

- **`backend/src/estados.rs`**: la máquina de estados y `cambiar_estado`. Devuelve un `CambioEstado`
  marcado `#[must_use]`, para que quien la llama no olvide publicarlo después del `commit`.
- **`backend/src/rutas/publico_presupuesto.rs`** (004): la respuesta del cliente con un solo `UPDATE`
  que comprueba a la vez que el presupuesto sigue emitido y vigente. El cambio de estado de la orden va
  en la misma transacción.
- **`backend/src/rutas/tablero.rs`** (008): una sola lista de órdenes abiertas para las columnas, y la
  carga por mecánico contada en Rust sobre esa misma lista, sin transacción (V3).
- **`backend/src/lineas.rs`** (005): una sola validación de líneas para el presupuesto y las imputaciones.
- **`frontend/src/lib/eventos.ts`, `recarga.ts` y `medianoche.ts`** (008): la conexión de eventos con
  reconexión, una sola petición del tablero a la vez, y la medianoche de Madrid calculada con `Intl`.

### Decisiones tomadas sobre la marcha

| Decisión | Dónde | Por qué |
|---|---|---|
| **Sin transacción en `GET /api/tablero`** | spec-008, hallazgo V3 · [research D1](../specs/008-tablero-ordenes-abiertas/research.md) · [hallazgos](../specs/008-tablero-ordenes-abiertas/hallazgos.md) | La regresión T030 se paró: si el navegador cortaba la petición mientras esperaba la respuesta del `BEGIN`, SQLx 0.8.6 devolvía la conexión al *pool* dentro de la transacción de solo lectura y fallaban peticiones de toda la app. Se quitó la transacción y la carga pasó a contarse en Rust. Una prueba que corta unas 4600 peticiones falló 3 de 3 antes y pasa después |
| **Un solo `UPDATE` para «solo cuenta la primera respuesta»** | spec-004, clarify Q4 · [research D2](../specs/004-aceptar-presupuesto-enlace/research.md) · D1 de la base común | Había dos versiones de la sentencia; se dejó una, que comprueba y guarda a la vez. Se probó con dos respuestas simultáneas y con mutación |
| **Sin la autotransición en_diagnostico → en_diagnostico** | spec-004, clarify Q2 · [hallazgos de la base común](../specs/000-base-comun/hallazgos.md) (I5) | Rechazar el presupuesto no cambia el estado de la orden, así que no hay transición que registrar |
| **Con cupo 0 no se ofrece ningún día** | spec-001, hallazgo H1 · T041 | La verificación en el navegador encontró días ofrecidos que el servidor rechazaba |
| **La orden solo se abre el día de la cita** | spec-002, clarify Q2 | Evita órdenes sin coche presente; obligó a crear citas de «hoy» en los datos demo |
| **La entrega de la orden demo 3, hoy** | spec-008, research D8 · T001 | Para ver en la app una orden con la entrega de hoy, que no sale retrasada |
| **Confirmación dentro de la página en «Emitir presupuesto», «Terminar reparación» y «Facturar»** | specs 003, 005 y 007 · V2 de [spec-008](../specs/008-tablero-ordenes-abiertas/hallazgos.md) · C1 de [003](../specs/003-diagnostico-presupuesto/hallazgos.md), [005](../specs/005-imputar-tiempo-piezas/hallazgos.md) y [007](../specs/007-facturar-avisar-cliente/hallazgos.md) | Usaban `confirm()` del navegador, cuyos botones salen en el idioma del navegador (principio II), lo que spec-004 había descartado en D5. Lo detectó la verificación de spec-008 (V2) y la revisión cruzada (O1); los analyze de la Persona B lo clasificaron como CRITICAL y lo corrigió con «Sí, emitir», «Sí, terminar» y «Sí, facturar» dentro de la página (T036 de 003, T035 de 005, T033 de 007). V2 queda cerrado |
| **Formularios sin la validación del navegador y mensajes del servidor en español** | specs 003, 006 y 007 · C1 de [006](../specs/006-pedido-pieza-proveedor/hallazgos.md) | Los avisos de validación del navegador salen en su idioma; los formularios llevan `novalidate` y muestran los mensajes del servidor |

## 7. Pruebas y validación

### Estrategia

- **Pruebas de backend** con `#[sqlx::test]`: una base limpia por prueba, una prueba por regla de
  negocio, y los datos creados en cada prueba, sin valores demo (R13, principio V).
- **Pruebas de frontend** con Vitest para la lógica sin pantalla: el formato, la medianoche con reloj
  simulado, una sola petición a la vez y la reconexión con un `EventSource` simulado.
- **Rojo antes del verde**: cada prueba se escribió antes que el código y se vio fallar.
- **Mutaciones**: cuando una regla ya existía, se rompió a propósito, se comprobó que la prueba fallaba
  y se restauró con `cmp`. Por ejemplo, la condición de la carga y `<` por `<=` en «retrasada» (008), o
  `estado = 'emitido'` en el `UPDATE` (004).
- **Navegador automático** (Playwright, Chromium en español y con la zona horaria de Madrid), que sigue
  los pasos de una persona en el quickstart común, a 1280–1920 px y a 390 px.
- **Cronometrajes humanos** de los criterios de tiempo, con una persona y un reloj.

### Resultados de las pruebas automáticas

Según la [verificación final](../specs/verificacion-final/verificacion.md) del 14/11/2026 sobre `main`
(`1c347b0`), con `cd backend && cargo test --no-fail-fast`, aparte `cargo test --test spec008_tablero
-- --ignored`, y `cd frontend && npm test`. Los recuentos por fichero coinciden con los del 12/11 sobre
`2a7dfce`:

| Fichero | Spec | Persona | Pruebas | Resultado |
|---|---|---|---|---|
| Unitarias de `backend/src` (`dinero`, `fechas`, `formato`, `matricula` y `rutas/citas`) | Base común y specs | Las dos | 16 | 16 pasan |
| `spec000_contador.rs` | 000 | Base común | 3 | 3 pasan |
| `spec000_estados.rs` | 000 | Base común | 7 | 7 pasan |
| `spec000_sesion.rs` | 000 | Base común | 12 | 12 pasan |
| `spec001_citas.rs` | 001 | A | 15 | 15 pasan |
| `spec002_ordenes.rs` | 002 | A | 12 | 12 pasan |
| `spec003_presupuestos.rs` | 003 | B | 20 | 20 pasan |
| `spec004_respuesta.rs` | 004 | A | 16 | 16 pasan |
| `spec005_imputaciones.rs` | 005 | B | 12 | 12 pasan |
| `spec006_pedidos.rs` | 006 | B | 23 | 23 pasan |
| `spec007_facturas.rs` | 007 | B | 11 | 11 pasan con el servidor de correo arrancado (ver la nota de abajo) |
| `spec008_tablero.rs` | 008 | A | 9 | 8 pasan; la 9 lleva `#[ignore]` y pasa al lanzarla aparte (unos 158 s) |
| **Backend** | | | **156** | **156 pasan** (155 con `cargo test` y 1 aparte) |
| Vitest: `formato.test.ts`, `medianoche.test.ts`, `recarga.test.ts` y `eventos.test.ts` | Base común y 008 | Las dos | 21 | 21 pasan |
| **Total** | | | **177** | **177 pasan, 0 fallan** |

Además, `cargo clippy --all-targets -- -D warnings` sin avisos, y `npm run check` y `npm run build`
correctos ([verificación final](../specs/verificacion-final/verificacion.md)).

**Nota sobre `spec007_facturas`**: la prueba `el_aviso_se_anota_si_sale_el_correo` necesita un
servidor de correo en el puerto 1025, como Mailpit. En la primera pasada de la verificación final no
estaba arrancado y falló
([`cargo-main.txt`](../specs/verificacion-final/salida/cargo-main.txt), 154 de 155). Con él,
`spec007_facturas` da 11 de 11
([`cargo-spec007-con-correo.txt`](../specs/verificacion-final/salida/cargo-spec007-con-correo.txt)).

### Validación en el navegador (Persona A)

| Spec | Verificación | Resultado | Evidencia |
|---|---|---|---|
| 001 | Quickstart §1 (T032), con la repetición tras T041 | 31/31 | [T032](../specs/001-pedir-cita-en-linea/evidencias/T032/verificacion.md) |
| 002 | Quickstart §2 (T029) | 23/23 (US1) y 3/3 (US2 y móvil) | [T029](../specs/002-abrir-orden-reparacion/evidencias/T029/verificacion.md) |
| 004 | Quickstart §4 (T027) | 20/20 | [T027](../specs/004-aceptar-presupuesto-enlace/evidencias/T027/verificacion.md) |
| 008 | Quickstart §8 (T028), su complemento y las comprobaciones de la matriz | 13/14 (la que falla es la parte opcional del §8.9, que se quitó por V1), 6/6 y 2/2 | [T028](../specs/008-tablero-ordenes-abiertas/evidencias/T028/verificacion.md), [complemento](../specs/008-tablero-ordenes-abiertas/evidencias/T028/complemento.md), [matriz](../specs/008-tablero-ordenes-abiertas/evidencias/matriz/informe.md) |

**Cronometrajes humanos**:
- HU-01: SC-001 en 47 s y SC-002 en 1 min 27 s.
- HU-02: SC-001 en 31 s.
- HU-04: SC-001 en 13 s, con la orden Nº 4.
- HU-08: SC-001 en 5 s y SC-002 en 4 s.

**Validación de la Persona B**:
- Las verificaciones en pantalla de 003, 005, 006 y 007 constan como notas en sus `tasks.md` (T035, T034,
  T032 y T028), y sus secciones del quickstart (§3, §5, §6 y §7) las recorrió en el navegador la
  regresión T030 y la verificación final, con capturas: 7/7, 6/6, 9/9 y 5/5.
- Los criterios de tiempo se midieron con el navegador automático
  ([matriz B](../specs/recorridos/matriz-B/informe.md), 7/7). Sus analyze los registraron como G1 de
  cada spec y los justificaron.
- No consta un informe de verificación en el navegador propio de la Persona B, aparte de la matriz B.

### Recorridos integrados (apartado 5.4)

Detalle en [recorridos-integrados.md](../specs/recorridos/recorridos-integrados.md). Los dos conectan
historias de las dos personas y están revisados por las dos.

| Recorrido | Tipo | Historias | Resultado |
|---|---|---|---|
| R1 · `3579KLM` de la cita a la factura (§9) | Satisfactorio | 001, 002, 004, 008 · 003, 005, 007 | 10/10 |
| R2 · Presupuesto rechazado: la orden queda bloqueada | Error, permiso y estado | 002, 004, 008 · 003, 005, 007 | 9/9 |

### Regresión

| Regresión | Sobre | Resultado | Evidencia |
|---|---|---|---|
| T029 de spec-004 | Quickstart §0–§7 con los datos demo de 004 (`8d64285`) | 111 comprobaciones: 109 a la primera y las 2 de §7.3 y §7.4 al repetirlas (eran fallos del script). §8 y §9 quedaron pendientes de spec-008 | [T029 de 004](../specs/004-aceptar-presupuesto-enlace/evidencias/T029/verificacion.md) |
| T030 de spec-008 | Quickstart §0–§8 con todo el código (`02e9868`), después de arreglar V3 | 123/124, sin ningún fallo de la aplicación (la que falla es la parte opcional del §8.9, que se quitó del quickstart). `estres.sh`: 0 errores | [T030 de 008](../specs/008-tablero-ordenes-abiertas/evidencias/T030/regresion/verificacion.md) |
| **Verificación final** | Quickstart §0–§8 y R2 sobre `main` (`1c347b0`), después de los cambios de la Persona B del 12/11 (confirmación dentro de la página y mensajes en español) | §0–§8: 123/124, sin ningún fallo de la aplicación (otra vez la parte opcional del §8.9). R2: 9/9. `estres.sh`: 0 errores | [verificación final](../specs/verificacion-final/verificacion.md) |

La verificación final es la última regresión: la Persona B cambió pantallas de 003, 005 y 007 después
de T030, y esos cambios los atraviesan las historias de la Persona A. El primer intento de T030 se paró en el §1 por V3
([hallazgo](../specs/008-tablero-ordenes-abiertas/evidencias/T030/hallazgo-conexiones.md)).

### Balance de la iteración

**Ambigüedades resueltas en los clarify** (preguntas con respuesta integrada en la spec):

| Spec | Preguntas | Registro |
|---|---|---|
| 000 · base común | 3 (R1: rechazo del presupuesto, datos de la factura, acceso del personal) | [research](../specs/000-base-comun/research.md) |
| 001 | 5, y 1 aclaración posterior en la spec (el aviso de FR-010) | [decisiones](../specs/001-pedir-cita-en-linea/decisiones-clarify.md) |
| 002 | 6 | [decisiones](../specs/002-abrir-orden-reparacion/decisiones-clarify.md) |
| 004 | 5 | [decisiones](../specs/004-aceptar-presupuesto-enlace/decisiones-clarify.md) |
| 008 | 4 | [decisiones](../specs/008-tablero-ordenes-abiertas/decisiones-clarify.md) |
| 003, 005, 006 y 007 | 4, 3, 5 y 3 | Sección «Clarifications» de cada spec ([003](../specs/003-diagnostico-presupuesto/spec.md), [005](../specs/005-imputar-tiempo-piezas/spec.md), [006](../specs/006-pedido-pieza-proveedor/spec.md), [007](../specs/007-facturar-avisar-cliente/spec.md)); sus specs no tienen `decisiones-clarify.md` aparte |
| **Total** | **38** (más la aclaración de 001) | |

**Hallazgos de los analyze y converge**, en los `hallazgos.md`:

| Registro | Hallazgos | Cerrados | Asignados a otra spec o persona | Abiertos |
|---|---|---|---|---|
| [Base común](../specs/000-base-comun/hallazgos.md) | 33 | 31 | 2 (I9 → 007, D2 → 006) | 0 |
| [spec-001](../specs/001-pedir-cita-en-linea/hallazgos.md) | 21 | 20 | 1 (I4 → 007) | 0 |
| [spec-002](../specs/002-abrir-orden-reparacion/hallazgos.md) | 17 | 15 | 2 (I5(b) → 003; N3 → T031 de 004) | 0 |
| [spec-003](../specs/003-diagnostico-presupuesto/hallazgos.md) | 8 | 8 | 0 | 0 |
| [spec-004](../specs/004-aceptar-presupuesto-enlace/hallazgos.md) | 25 | 23 | 2 (G1 e I8 → Persona B) | 0 |
| [spec-005](../specs/005-imputar-tiempo-piezas/hallazgos.md) | 6 | 6 | 0 | 0 |
| [spec-006](../specs/006-pedido-pieza-proveedor/hallazgos.md) | 4 | 4 | 0 | 0 |
| [spec-007](../specs/007-facturar-avisar-cliente/hallazgos.md) | 4 | 4 | 0 | 0 |
| [spec-008](../specs/008-tablero-ordenes-abiertas/hallazgos.md) | 17 | 17 | 0 | 0 |
| **Total** | **135** | **128** | **7** | **0** |

- **V2** (los `confirm()` del navegador) está cerrado en spec-004 y en spec-008: la Persona B los cambió
  por una confirmación dentro de la página (C1 de 003, 005 y 007).
- **I7** y **G6** de la base común se cerraron el 12/11/2026: I7 en el clarify de spec-008 («Luis 3 ·
  Marta 1», `200b4db`), y G6 porque se usa `NOMBRE_ESTADO` de `formato.ts` y el plan maestro ya no
  menciona `EstadoOrden.svelte` (cambio que revisa la Persona B).
- **N1 de 005** (pasos del quickstart sin la confirmación dentro de la página) lo cerró la Persona A el
  14/11/2026: el quickstart común dice «confirmar con «Sí, terminar»» en el §8.6, el §8.9 y el §9.6, y «confirmar
  con «Sí, facturar»» en el §8.8 y el §9.7. Es un documento compartido; el cambio lo revisa la Persona B.
- **Los 7 asignados a la Persona B**, comprobados en el repositorio el 14/11/2026:

  | Hallazgo | Qué pedía | Estado |
  |---|---|---|
  | D2 (base común → 006) | Que las pruebas de 006 usen `tests/comun` | Resuelto; falta actualizarlo en su registro (`spec006_pedidos.rs` usa `comun::crear_usuario` y `comun::entrar`) |
  | I8 (004 → Persona B) | Quitar `confirm()` de 003 y 005 | Resuelto; falta actualizarlo en su registro (no queda ningún `confirm()`) |
  | N3 (002 → T031 de 004) | Que la Persona B revise el §4.5 en T031 | Resuelto; falta actualizarlo en su registro (T031 de 004, marcada, incluye el §4.5) |
  | I4 (001 → 007) | El 422 de la matrícula del historial, en el contrato y en la spec de 007 | Abierto, trabajo futuro de la Persona B (el contrato lo tiene; la spec de 007 no tiene el caso límite y el texto no coincide con el de spec-001) |
  | I5(b) (002 → 003) y G1 (004 → Persona B) | Un paso de diagnóstico antes de emitir el presupuesto del §4.5 | Abierto, trabajo futuro de la Persona B |
  | I9 (base común → 007) | Añadir `/` y `/facturas/[id]` a «Rutas de pantalla» del contrato | Abierto, trabajo futuro de la Persona B |

**Brechas de implementación pendientes**: 0 tareas de 336. Las dos que quedaban, las revisiones finales
de la Persona B (T031 de 004 y T032 de 008), están hechas. El último converge de cada spec no dejó
tareas sin hacer (por ejemplo, [converge-2 de 008](../specs/008-tablero-ordenes-abiertas/converge-2.md)
y [converge-2 de 003](../specs/003-diagnostico-presupuesto/converge-2.md)). Lo único que queda son
documentos: los asignados abiertos de la Persona B (I4, I5(b)/G1 e I9).

**Criterios verificados**, de la [matriz](../specs/matriz-trazabilidad.md):

| | Criterios | Verificados | Con desviación | Pendientes |
|---|---|---|---|---|
| Persona A (HU-01, 02, 04 y 08) | 68 | 66 | 2 | 0 |
| Persona B (HU-03, 05, 06 y 07) | 58 | 53 | 5 | 0 |
| **Grupo** | **126** | **119 (94,4 %)** | **7 (5,6 %)** | **0** |

Las desviaciones son criterios de tiempo: dos de la Persona A (HU-04 SC-001, con otra orden, y HU-08
SC-002, sin cronometrar abrir y facturar) y cinco de la Persona B, medidos con el navegador automático en
lugar de con una persona. La Persona B pasó HU-05 US3-1 a verificado con la evidencia de la regresión
T030 ([revisión cruzada](../specs/revision-cruzada.md), O3).

**Revisión de las filas de la matriz** (apartado 5.2): el enunciado pide revisar al menos un criterio
por historia de la otra persona. Es un muestreo: cada persona revisó 4 filas de la otra, una por
historia, y las 8 son correctas ([revisión cruzada](../specs/revision-cruzada.md)). Las demás filas
llevan «pendiente» en la columna de revisión porque no entran en el muestreo, no porque falte
revisarlas.

### Registro mínimo

Las filas más relevantes, al menos una por historia. El resto está en la
[matriz](../specs/matriz-trazabilidad.md) y en los `hallazgos.md` de cada spec.

| Historia y criterio | Hallazgo o decisión | Comando | Documentos o código afectados | Tarea y commit | Prueba y resultado | Responsable | Revisor | Estado |
|---|---|---|---|---|---|---|---|---|
| HU-01 · US1-5 (matrícula válida) | Clarify Q1: de 2 a 10 letras o cifras (cierra I11 de la base común) | `/speckit-clarify` | spec y research D1 de 001; `backend/src/matricula.rs` (compartido) | T001, T010 · `22f08be`, `224daaf` | `prueba_3_matricula_invalida` y unitarias de `matricula.rs`: pasan; T032 filas 1–4 | Persona A | Persona B (pendiente) | Verificado |
| HU-01 · US3-1 (día completo) | H1: con cupo 0 se ofrecían días que el servidor rechazaba | `/speckit-implement` (verificación T032) | `backend/src/rutas/citas.rs`, `cita/+page.svelte` | T041 · `00e6436` | `con_cupo_0_no_se_ofrece_ningun_dia`: pasa; T032 repetido 31/31 | Persona A | Persona B (pendiente) | Verificado |
| HU-02 · US1-9 y SC-002 (números consecutivos) | Clarify Q6: los rechazos no gastan número (C1 de analyze-1) | `/speckit-analyze`, `/speckit-clarify` | spec y research D2 de 002; `ordenes.rs` | T008 · `2516faf`, `d5106aa` | `prueba_2_correlativo_sin_huecos`, `prueba_3_concurrencia_del_numero`: pasan; T029 fila 12 | Persona A | Persona B (pendiente) | Verificado |
| HU-03 · US2-5 (quitar una línea recalcula) | Clarify Q2 de 003 (sesión del 22/10/2026 en su spec) | `/speckit-clarify` | `presupuestos.rs`, `Presupuesto.svelte` | `dd77530` | `quitar_una_linea_recalcula_los_totales`: pasa | Persona B | Persona A, revisado el 12/11/2026 | Verificado |
| HU-04 · US1-4 (solo cuenta la primera respuesta) | Clarify Q4 y D1 de la base común: un solo `UPDATE` (R9) | `/speckit-clarify` | research R9; `publico_presupuesto.rs` | T010, T013 · `76dcdda`, `49c3188`, `7e66bad` | `dos_respuestas_a_la_vez_solo_cuenta_una` con mutación: pasa; T027 filas 15–16 | Persona A | Persona B (pendiente) | Verificado |
| HU-05 · US2-1 (el 110 % justo no bloquea) | Research D4 de 005 | `/speckit-plan` (deducido: la decisión está en su `research.md`; sus documentos no lo registran) | `imputaciones.rs` | `09c887f` | `el_limite_del_10_por_ciento_es_exacto_al_centimo`: pasa | Persona B | Persona A, revisado el 12/11/2026 | Verificado |
| HU-06 · US3-2 (la orden espera al último pedido) | Research D4 de 006 | `/speckit-plan` (deducido, como la fila anterior) | `almacen.rs` | `52448f3` | `con_dos_pedidos_la_orden_espera_al_ultimo`: pasa | Persona B | Persona A, revisado el 12/11/2026 | Verificado |
| HU-07 · US1-1 (factura de 157,30 €) | Research D1 y D6 de 007 | `/speckit-plan` (deducido, como la fila anterior) | `facturas.rs`, `Factura.svelte` | `e2a2060` | `facturar_copia_las_imputaciones_y_cierra_la_orden`: pasa | Persona B | Persona A, revisado el 12/11/2026 | Verificado |
| HU-08 · SC-003 (la carga cuadra con las tarjetas) | V3: transacción cancelada a mitad | `/speckit-converge` (converge-1) | research D1 y D2 de 008; `tablero.rs`; R14 común | T033, T034 · `c4588e9` | `prueba9_peticiones_cortadas_no_dejan_conexiones_en_transaccion`: rojo 3/3 antes y verde después; `estres.sh`: 0 errores | Persona A | Persona B (T032 hecha) | Verificado |
| HU-03 · US3 (emitir) · HU-05 · US4 (terminar) · HU-07 · US1 (facturar) | C1 de [003](../specs/003-diagnostico-presupuesto/hallazgos.md), [005](../specs/005-imputar-tiempo-piezas/hallazgos.md) y [007](../specs/007-facturar-avisar-cliente/hallazgos.md) (CRITICAL, principio II): `confirm()` del navegador; cierra V2 de 008 | `/speckit-analyze` (analyze-1), `/speckit-converge` y `/speckit-implement` | `Presupuesto.svelte`, `Imputaciones.svelte`, `Factura.svelte`; quickstart común §3.6 | T036 de 003, T035 de 005, T033 de 007 · `2c207bd`, `7bf2648`, `037f0da` | En el navegador (sus registros): 003 rojo 0 de 3 y verde 4 de 4; 005 rojo 0 de 3 y verde 3 de 3; 007 rojo 1 de 4 y verde 4 de 4. Verificación final §3, §5 y §7 con «Sí, emitir», «Sí, terminar» y «Sí, facturar»: 7/7, 6/6 y 5/5 | Persona B | Persona A (verificación final) | Verificado |
| HU-06 · US1 y US2 (alta y pedidos) | C1 de [006](../specs/006-pedido-pieza-proveedor/hallazgos.md) (CRITICAL, principio II): avisos de validación del navegador | `/speckit-analyze` (analyze-1), `/speckit-converge` y `/speckit-implement` | `/almacen`, `/pedidos` | T033 de 006 · `076f626` | Rojo 3 de 7 y verde 7 de 7 (su registro); verificación final §6: 9/9 | Persona B | Persona A (verificación final) | Verificado |
| HU-08 · US2-2 (mecánico de baja con órdenes) | Clarify Q1: los inactivos solo salen si tienen órdenes, marcados «(de baja)» | `/speckit-clarify` | spec, research D2 y contrato de 008 | T014, T015 · `200b4db`, `67f75d1` | `prueba6_inactivos_solo_con_ordenes_abiertas` con mutación: pasa; en la app, «Pedro (de baja) 1» | Persona A | Persona B (pendiente) | Verificado |

## 8. Despliegue y uso

### Instalación y ejecución

Según el apartado «Arrancar» del [quickstart común](../specs/000-base-comun/quickstart.md):

```bash
cp .env.example .env                       # ajustar los valores; ningún secreto en git
docker compose up -d                       # PostgreSQL y Mailpit
cd backend && cargo sqlx migrate run       # crea las tablas
psql "$DATABASE_URL" -f seed/demo.sql      # datos demo (o con el psql del contenedor, ver el quickstart)
cd ../frontend && npm ci && npm run build  # compila la SPA
cd ../backend && cargo run                 # la app, en http://localhost:3000
```

- **Variables de `.env.example`**: `POSTGRES_USER`, `POSTGRES_PASSWORD`, `POSTGRES_DB`,
  `DATABASE_URL`, `SMTP_URL`, `CORREO_REMITENTE`, `URL_PUBLICA`, `TALLER_CUPO_DIARIO`,
  `TALLER_PRECIO_HORA`, `TALLER_IVA_PORCENTAJE`, `TALLER_NOMBRE`, `TALLER_NIF`, `TALLER_DIRECCION`
  y `RUST_LOG`. Solo trae valores de ejemplo, y el `.env` real no se versiona (principio V). El logo de
  la factura es opcional, con `TALLER_LOGO` (spec-007).
- **Correos**: se ven en Mailpit, en http://localhost:8025.
- **Pruebas**: `cd backend && cargo test` (y, aparte, `cargo test --test spec008_tablero --
  --ignored`); `cd frontend && npm test`.
- **Datos demo**: se cargan el mismo día en que se usan, porque traen citas de «hoy».

### Usuarios demo

Los publica el quickstart común, con una contraseña ficticia solo para la demostración:

| Usuario | Rol |
|---|---|
| `ana` | recepción |
| `luis`, `marta` | mecánicos |
| `pepe` | almacén |
| `paco` | jefe de taller |
| `pedro` | mecánico de baja (no puede entrar ni recibir órdenes) |

### Manual breve por rol

- **Cliente**:
  - en `/cita`, escribe la matrícula, el motivo y elige un día;
  - si el vehículo es nuevo, da marca, modelo, nombre, teléfono y correo;
  - cuando el taller emite el presupuesto, recibe un correo con un enlace para aceptarlo o rechazarlo,
    con una confirmación antes de responder.
- **Recepción (`ana`)**:
  - en «Citas» ve las de hoy y abre la orden con el kilometraje, los daños, el mecánico y la fecha de
    entrega;
  - en «Órdenes» consulta la ficha;
  - con la orden terminada, pulsa «Facturar» y puede imprimir la factura;
  - en «Historial» busca por matrícula.
- **Mecánico (`luis`, `marta`)**: en «Mis órdenes», en la ficha de cada orden:
  - registra el diagnóstico;
  - prepara y emite el presupuesto;
  - imputa horas y piezas una vez aceptado;
  - termina la reparación.
- **Almacén (`pepe`)**:
  - en «Almacén» da de alta piezas y ve el stock;
  - en «Pedidos» pide al proveedor para una orden en reparación y marca los pedidos como recibidos.
- **Jefe (`paco`)**:
  - ve todo el menú;
  - en «Tablero» ve las órdenes abiertas por estado, la carga de cada mecánico y las retrasadas
    destacadas, y el tablero se actualiza solo.

## 9. Planificación y seguimiento

Del [plan maestro](../specs/000-base-comun/plan.md), de [agrupamiento.md](agrupamiento.md) y del
historial de git:

| Fase | Quién | Qué | Fechas |
|---|---|---|---|
| 0 · Base común | Las dos | Constitución, specs y plan (B, 15/10); tareas, análisis e implementación (A, 15–22/10) | 15/10 – 22/10/2026 |
| 1 · En paralelo | A: 001 → 002 → 004 → 008 · B: 006 → 003 → 005 → 007 | Una rama por spec | 22/10 – 12/11/2026 |
| 2 · Integración | Las dos | Quickstart §9 (R1), R2, matriz, revisión cruzada, analyze y converge de las specs de B y verificación final | 12/11 – 14/11/2026 |

**Ramas y entrada en `main`**:

| Fecha | Qué entra | Cómo |
|---|---|---|
| 22/10/2026 | Base común (A) | PR #1 (`spec-000`), fusionado por B |
| 22/10/2026 | spec-006 (B) | PR #2 (`spec-006`), fusionado por A |
| 22/10/2026 | spec-003 (B) | Commit directo |
| 29/10/2026 | spec-005 y spec-007 (B) | Commits directos |
| 05/11/2026 | spec-001 y spec-002 (A) | PR #3 (`spec-001`); `main` avanzó hasta la rama de spec-002, que llevaba spec-001 dentro |
| 05/11/2026 | Integración en la ficha de 003, 005, 006 y 007 (B) | Commits directos |
| 12/11/2026 | spec-004 y spec-008 (A) | PR #5 (`spec-008`, que lleva spec-004 dentro) y PR #4, fusionados por B |
| 12/11/2026 | Analyze, hallazgos y converge de 003, 005, 006 y 007, con sus correcciones (B) | PR #8 (`spec-007`) y PR #7 (`spec-003`), abiertos por B y fusionados por A |
| 12/11/2026 | Documentación final: matriz, recorridos, agrupamiento, revisión cruzada y memoria (las dos) | PR #6 (`entrega`), abierto por B y fusionado por A |
| 14/11/2026 | Verificación final sobre `main` (A) | Rama `cierre` (`52c7024`), pendiente de fusionar |

**Commits**, por autor y por las rutas de sus specs, recontados el 14/11/2026 con el criterio de
[agrupamiento.md](agrupamiento.md), que tiene los recuentos del 12/11:

| Persona | En sus specs | En las de la otra | Total en el repositorio |
|---|---|---|---|
| A (@evm60-ua) | 82 | 12 | 130 |
| B (@senabre, firma «Andres») | 17 | 6 | 23 (22 como «Andres» y una fusión en GitHub) |

Los recuentos no miden el trabajo de la misma forma: B subió cada spec en pocos commits grandes, y A,
en muchos commits pequeños por tarea.

**Fusiones con conflictos**: integrar `main` en las ramas de la Persona A obligó a resolver
conflictos en los ficheros donde cada spec registra lo suyo. Por ejemplo, el 29/10/2026, «Merge
origin/main (spec-005) en spec-001; conflicto en rutas/mod.rs resuelto conservando citas e
imputaciones».

## 10. Conclusiones

**Qué se consiguió.**
- Las 8 historias están implementadas, con las 336 tareas hechas.
- Hay 177 pruebas automáticas y todas pasan según la verificación final: 156 de backend, contando la
  que se lanza aparte, y 21 de frontend.
- El quickstart común se recorre entero sin fallos de la aplicación, también después de los últimos
  cambios de la Persona B (verificación final).
- De los 126 criterios, 119 están verificados y 7 verificados con desviación.
- De los 135 hallazgos, 128 están cerrados y ninguno abierto.
- El flujo completo de la cita a la factura funciona con las dos personas y con el tablero
  actualizándose solo.

**Problemas encontrados.**
- **V3, transacción cancelada a mitad (spec-008)**: el fallo más grave. Lo encontró la regresión y
  hacía fallar peticiones de toda la aplicación. Costó tres versiones de la prueba conseguir
  reproducirlo, porque la ventana es de una décima de milisegundo. Se arregló quitando la transacción.
- **Conflictos al fusionar** `main` en las ramas (`rutas/mod.rs`, `tipos.ts`), por las specs que se
  desarrollaban en paralelo.
- **Cambios de una persona en ficheros de la otra**: por ejemplo, `presupuestos.rs` (de B) cambiado
  por 004 (de A). Se resolvieron con aviso previo y revisión en commits propios.
- **Criterios de tiempo**: para medirlos hacía falta una persona con un reloj. Varios se midieron
  solo con el navegador automático y quedan como desviación.
- **Diálogos del navegador en inglés**: tres confirmaciones con `confirm()` y los avisos de validación de
  los formularios salían en el idioma del navegador (principio II). Se detectó tarde, en la verificación
  de spec-008 (V2) y en los analyze de la Persona B, y se corrigió el 12/11/2026, lo que obligó a repetir
  la regresión (verificación final).
- **El quickstart dependía del día**: los datos demo traen citas de «hoy», y el §9 necesita dos días.
  El día B se hizo adelantando el reloj.

**Trabajo futuro**, de los hallazgos que siguen asignados:
- **Persona B**:
  - abiertos: I5(b) de 002 y G1 de 004 (un paso de diagnóstico antes de emitir en el §4.5 del
    quickstart común); I9 de la base común (`/` y `/facturas/[id]` en «Rutas de pantalla» del
    contrato); I4 de 001 (el caso límite de la matrícula no válida en la spec de 007, con el mismo
    texto que spec-001);
  - D2, I8 y N3 están resueltos; falta actualizarlos en su registro;
  - revisar el cambio del quickstart común con el que se cerró N1 de 005 (§8.6, §8.8, §8.9, §9.6 y §9.7).
- **Persona A**: la limitación aceptada de V1(b): una caída silenciosa de la red no se detecta hasta el
  siguiente evento o la medianoche.

## 11. Demo

Pendiente: se completa cuando esté DemoSSD_GXX.zip, que entrega la Persona B.

Estructura prevista:
- Nombre y contenido del fichero `DemoSSD_GXX.zip` (vídeo y guion).
- Guion de la demo, siguiendo los recorridos R1 (de la cita a la factura) y R2 (presupuesto rechazado).
- Duración, quién presenta cada parte y versión del código usada.

## 12. Anexos

- **Repositorio**: https://github.com/senabre/taller-mecanico
- **Constitución**: [.specify/memory/constitution.md](../.specify/memory/constitution.md)
- **Matriz de trazabilidad**: [specs/matriz-trazabilidad.md](../specs/matriz-trazabilidad.md)
- **Revisión cruzada**: [specs/revision-cruzada.md](../specs/revision-cruzada.md)
- **Recorridos integrados**: [specs/recorridos/recorridos-integrados.md](../specs/recorridos/recorridos-integrados.md)
- **Agrupamiento**: [docs/agrupamiento.md](agrupamiento.md)
- **Verificación final**: [specs/verificacion-final/verificacion.md](../specs/verificacion-final/verificacion.md)

### Specs

| Spec | Ruta | Ficheros |
|---|---|---|
| 000 · Base común | [specs/000-base-comun/](../specs/000-base-comun/plan.md) | spec, plan (maestro), research, data-model, contracts/api, quickstart, tasks, analyze-1 a 3, hallazgos |
| 001 · Pedir cita | [specs/001-pedir-cita-en-linea/](../specs/001-pedir-cita-en-linea/spec.md) | spec, decisiones-clarify, plan, research, data-model, contracts, quickstart, tasks, analyze-1 y 2, converge-1 a 3, hallazgos, checklists, evidencias |
| 002 · Abrir orden | [specs/002-abrir-orden-reparacion/](../specs/002-abrir-orden-reparacion/spec.md) | spec, decisiones-clarify, plan, research, data-model, contracts, quickstart, tasks, analyze-1 y 2, converge-1 y 2, hallazgos, checklists, evidencias |
| 003 · Diagnóstico y presupuesto | [specs/003-diagnostico-presupuesto/](../specs/003-diagnostico-presupuesto/spec.md) | spec, plan, research, tasks, analyze-1 y 2, converge-1 y 2, hallazgos, checklists, evidencias |
| 004 · Aceptar presupuesto | [specs/004-aceptar-presupuesto-enlace/](../specs/004-aceptar-presupuesto-enlace/spec.md) | spec, decisiones-clarify, plan, research, data-model, contracts, quickstart, tasks, analyze-1 y 2, converge-1 a 3, hallazgos, checklists, evidencias |
| 005 · Imputar | [specs/005-imputar-tiempo-piezas/](../specs/005-imputar-tiempo-piezas/spec.md) | spec, plan, research, tasks, analyze-1 y 2, converge-1, hallazgos, checklists, evidencias |
| 006 · Pedido al proveedor | [specs/006-pedido-pieza-proveedor/](../specs/006-pedido-pieza-proveedor/spec.md) | spec, plan, research, tasks, analyze-1 y 2, converge-1, hallazgos, checklists, evidencias |
| 007 · Facturar | [specs/007-facturar-avisar-cliente/](../specs/007-facturar-avisar-cliente/spec.md) | spec, plan, research, tasks, analyze-1 y 2, converge-1, hallazgos, checklists, evidencias |
| 008 · Tablero | [specs/008-tablero-ordenes-abiertas/](../specs/008-tablero-ordenes-abiertas/spec.md) | spec, decisiones-clarify, plan, research, data-model, contracts, quickstart, tasks, analyze-1 a 3, converge-1 y 2, hallazgos, checklists, evidencias |

### Glosario

| Término | Significado |
|---|---|
| HU | Historia de usuario |
| Spec | Especificación de una historia (Spec Kit): `spec.md` con sus escenarios, FR y SC |
| FR / SC | Requisito funcional / criterio de éxito |
| US*n*-*m* | Escenario *m* de la historia *n* de una spec |
| Clarify / analyze / converge | Pasos de Spec Kit: aclarar la spec, analizar la coherencia de los documentos y comparar el código con lo pedido |
| Quickstart | Guía de validación en la aplicación, común a todas las specs (§0–§9) |
| Orden | Orden de reparación de un vehículo, con su estado |
| Ampliación | Presupuesto adicional cuando el coste supera el 110 % de lo presupuestado |
| Carga | Número de órdenes abiertas de cada mecánico, en el tablero |
| Retrasada | Orden no facturada con la fecha de entrega prometida anterior a hoy |
| SSE | Server-Sent Events: avisos del servidor al navegador por una conexión abierta |
| Mutación | Romper a propósito una regla para comprobar que la prueba que la protege falla |
| Datos demo | `backend/seed/demo.sql`: usuarios, piezas, citas y órdenes de ejemplo, con valores ficticios |
