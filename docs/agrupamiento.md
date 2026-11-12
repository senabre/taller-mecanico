# Agrupamiento: reparto de las historias

Cómo se repartieron las 8 historias del taller entre las dos personas del grupo, por qué, qué
necesita cada una de la otra y cómo queda trazado el trabajo. El reparto está fijado en el
[plan maestro](../specs/000-base-comun/plan.md) («Reparto y orden de trabajo»), y este documento lo
resume con los datos del repositorio (rama `entrega`, 12/11/2026).

## 1. Reparto

| Persona | GitHub | Historias | Specs | Tema |
|---|---|---|---|---|
| Persona A | @evm60-ua | HU-01, HU-02, HU-04, HU-08 | [001 · Pedir cita](../specs/001-pedir-cita-en-linea/spec.md), [002 · Abrir orden](../specs/002-abrir-orden-reparacion/spec.md), [004 · Aceptar presupuesto](../specs/004-aceptar-presupuesto-enlace/spec.md), [008 · Tablero](../specs/008-tablero-ordenes-abiertas/spec.md) | Lo que ve el cliente y el seguimiento de la orden (16 pts) |
| Persona B | @senabre | HU-03, HU-05, HU-06, HU-07 | [003 · Diagnóstico y presupuesto](../specs/003-diagnostico-presupuesto/spec.md), [005 · Imputar](../specs/005-imputar-tiempo-piezas/spec.md), [006 · Pedido al proveedor](../specs/006-pedido-pieza-proveedor/spec.md), [007 · Facturar](../specs/007-facturar-avisar-cliente/spec.md) | Piezas, presupuesto, imputaciones y factura (16 pts) |
| Las dos | — | Base común | [000 · Base común](../specs/000-base-comun/plan.md) | Constitución, modelo de datos, contrato, estados, eventos, layout y datos demo |

**Base común (000).** El plan maestro la asigna a «A + B juntos» (fase 0). En el historial:
- la Persona B escribió la primera versión de los documentos comunes: la constitución, las specs
  001–008 y el plan compartido (`a0e1209`, 15/10/2026);
- la Persona A escribió las tareas, los análisis y sus correcciones, e hizo la implementación (los 19
  commits `feat(000)`), y abrió el PR #1, que fusionó la Persona B el 22/10/2026.

En los documentos de `specs/000-base-comun/` hay 5 commits de la Persona B y 29 de la Persona A.

## 2. Por qué así

Lo dice el plan maestro: **el reparto agrupa por cohesión**.

- **La Persona B** se queda con todo lo que trabaja con **líneas, piezas y dinero** (`dinero.rs`,
  `TablaLineas`, `Totales`, `pieza`). Las cuatro specs forman una cadena: 006 da de alta las piezas
  que usan 003 y 005, y 005 genera las imputaciones que factura 007.
- **La Persona A** se queda con lo que **ve el cliente y el seguimiento de la orden**: las páginas
  públicas (001 y 004), la apertura de la orden (002) y el tablero (008), que solo lee estados.
- **Cada persona tiene 16 puntos** de historia.
- **Para no esperar a la otra**, los datos demo (`backend/seed/demo.sql`) crean lo que cada una
  necesita antes de que exista: órdenes `recibida` y `en_reparacion` para B, mientras A termina 002;
  y un presupuesto `emitido` con token conocido y órdenes en todos los estados para A, mientras B
  termina 003.

## 3. Dependencias entre personas

Sacadas del apartado «Dependencias» del `plan.md` de cada spec.

| Spec (persona) | Necesita de la otra persona | Lo que la otra usa de ella |
|---|---|---|
| 001 (A) | Nada: solo la base común | spec-007 usa `normalizar_matricula` |
| 002 (A) | Las funciones de la ficha de 003, 005 y 006 | 003, 005, 006 y 007 añaden su sección a la ficha de la orden (`GET /api/ordenes/{id}`) |
| 003 (B) | La ficha de la orden de 002 (para la pantalla) | 004 usa `presupuesto.token`, `estado`, `caduca_el` y `respondido_en` |
| 004 (A) | El presupuesto emitido, su token y su correo de 003; las imputaciones de 005 | 005 necesita la aceptación de 004 (y de las ampliaciones) para imputar |
| 005 (B) | La aceptación de presupuestos de 004 y la ficha de 002. Sus pruebas simulan 004 con SQL; la pantalla espera a 002 | 007 factura sus imputaciones (spec de la misma persona) |
| 006 (B) | Nada de A: solo la base común y órdenes `en_reparacion` de los datos demo | — |
| 007 (B) | La ficha de la orden de 002 (para su sección) | — |
| 008 (A) | Los eventos de cambio de estado de 003, 005, 006 y 007 (B), además de los de 002 y 004 | — |

**Puntos de contacto** que fija el plan maestro (se acuerdan antes de tocar):
- **003 → 004**: B emite el presupuesto y A gestiona la respuesta. A no modifica `presupuestos.rs`
  y B no modifica `publico_presupuesto.rs`. (Al implementar 004, A necesitó cambiar
  `presupuestos.rs`; avisó a B antes y B lo revisó en commits propios: hallazgo I1 de spec-004.)
- **002 → 003, 005, 006 y 007**: A crea la orden y su ficha; B añade sus campos al objeto `Orden`
  siguiendo el contrato. La ficha (`ordenes/[id]`) tiene una sección por spec, cada una en su
  componente, para que cada persona edite solo el suyo.
- **`estados.rs`**: no se añaden transiciones sin actualizar antes el data-model.

## 4. Acuerdos comunes

Lo que fija la base común y vale para las dos personas:

- **[Constitución](../.specify/memory/constitution.md)** (v1.2.1): simplicidad, español y euro,
  cero alcance fantasma, verificable por una persona no técnica y respeto por los datos.
- **[Modelo de datos](../specs/000-base-comun/data-model.md)**: tablas y **máquina de estados de la
  orden**. Recibida → en diagnóstico (003) → en reparación (004); desde en reparación, a pendiente de
  conformidad (005) o a en espera de piezas (006), y de las dos se vuelve a en reparación (004 y
  006); en reparación → terminada (005) → facturada (007). Cada transición dice qué spec la provoca,
  y todas pasan por `estados::cambiar_estado` ([research R14](../specs/000-base-comun/research.md)).
- **Eventos del tablero**: cada cambio de estado se publica con `CambioEstado::publicar` después del
  `commit` (research R11 y R14). Así se conectan las specs de B con el tablero de A.
- **[Contrato de la API](../specs/000-base-comun/contracts/api.md)**: un solo documento con los
  endpoints, tipos y mensajes de error de todas las specs.
- **[Quickstart común](../specs/000-base-comun/quickstart.md)**: una sola guía de validación, con
  una sección por spec (§1–§8) y la integración de las dos personas (§9).

**Ficheros compartidos** (los revisa la otra persona antes de fusionar, con commits propios para
cada cambio compartido): la migración, `estados.rs`, `dinero.rs`, `matricula.rs`, `rutas/mod.rs`,
`tipos.ts`, `formato.ts`, `componentes/`, la ficha `ordenes/[id]` y `backend/seed/demo.sql`, además de
los documentos de `specs/000-base-comun/`. Cada spec los lista en la tabla «Ficheros compartidos» de
su plan.

**Revisiones de lo compartido que constan como hechas**:
- 001, T031: hecha por la Persona B.
- 002, T031: hecha por la Persona B el 05/11/2026; incluye las secciones de 003, 005, 006 y 007 en la
  ficha.

**Pendientes**: 004 (T031) y 008 (T032).

## 5. Orden seguido

El orden previsto era A: 001 → 002 → 004 → 008 y B: 006 → 003 → 005 → 007 (plan maestro), y así se
siguió. Entrada en `main`:

| Fecha | Qué entra en `main` | Cómo |
|---|---|---|
| 15/10/2026 | Documentos de la base común (B) y spec-006 en documentos (B) | Commits directos |
| 22/10/2026 | Base común implementada (A) | PR #1 (`spec-000`), abierto por A y fusionado por B |
| 22/10/2026 | spec-006 (B) | PR #2 (`spec-006`), abierto por B y fusionado por A |
| 22/10/2026 | spec-003 (B) | Commit directo (`feat(003)`) |
| 29/10/2026 | spec-005 y spec-007 (B) | Commits directos (`feat(005)`, `feat(007)`) |
| 05/11/2026 | spec-001 y spec-002 (A) | PR #3 (`spec-001`), abierto y fusionado por A. `main` avanzó hasta la rama de spec-002, que llevaba spec-001 dentro, así que entraron las dos |
| 05/11/2026 | Integración en la ficha de 003, 005, 006 y 007 (B) y la revisión T031 de 002 (B) | Commits directos |
| 12/11/2026 | spec-004 y spec-008 (A) | PR #5 (`spec-008`, que lleva spec-004 dentro) y PR #4 (`spec-004`), abiertos por A y fusionados por B |
| 12/11/2026 | Matriz de trazabilidad y recorridos integrados | Rama `entrega`, todavía sin fusionar en `main` |

## 6. Trazabilidad del reparto

**Commits** (`git log --all --author=…`, por rutas: los documentos `specs/…` y el código y las pruebas
de cada spec):

| Persona | En sus specs | En las specs de la otra | Total en el repositorio |
|---|---|---|---|
| Persona A (@evm60-ua) | 81 | 11 | 123 |
| Persona B (@senabre) | 12 | 2 | 13 |

Los números no miden el trabajo de la misma forma: la Persona B subió cada spec en pocos commits
grandes (por ejemplo, `feat(003): … (T001-T030, T032-T034)`), y la Persona A, en muchos commits
pequeños por tarea. Los 11 commits de A en specs de B son cambios compartidos que pedían sus propias
specs (por ejemplo, `refactor(005)` y `test(006)` para la ficha de 002), marcados «revisa Persona B».

**Autoría en los documentos**: los `tasks.md` y `plan.md` de 001, 002, 004 y 008 dicen
«Responsable: Persona A · Revisor: Persona B»; los de 003, 005, 006 y 007, «Responsable: Persona B».
Los registros de decisiones del clarify (`decisiones-clarify.md`) de las specs de A llevan las
columnas «Autora: Persona A» y «Revisor: Persona B».

**Matriz y recorridos**:
- [Matriz de trazabilidad](../specs/matriz-trazabilidad.md): las 68 filas de la Persona A (HU-01,
  02, 04 y 08), con 66 verificadas, 2 verificadas con desviación y 0 pendientes. Las de la Persona B
  están por completar, y la revisión cruzada de las filas, pendiente.
- [Recorridos integrados](../specs/recorridos/recorridos-integrados.md): R1 (de la cita a la factura,
  el §9) y R2 (presupuesto rechazado), los dos con pasos de las dos personas y revisados por las dos.
