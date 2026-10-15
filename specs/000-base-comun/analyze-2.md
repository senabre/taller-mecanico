# Análisis de coherencia 2 · spec-000 Base común

**Fecha**: 2026-10-15 · **Alcance y criterios**: los mismos que en [analyze-1.md](analyze-1.md):
`spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/api.md`, `quickstart.md` y
`tasks.md` de `specs/000-base-comun/`, contrastados con la constitución (ahora **v1.2.0**) y con
lo que los `spec.md`/`plan.md` de 001–008 (y `research.md`/`tasks.md` de 006) esperan de la base
común. Los IDs de tarea son los del `tasks.md` actual (43 tareas).
**Modo**: solo lectura sobre los documentos analizados. Este informe y
[hallazgos.md](hallazgos.md) son los únicos ficheros escritos.

## Comprobación de los hallazgos de analyze-1

| Resultado | Hallazgos |
|-----------|-----------|
| **Cerrados y verificados** (12) | K1, I1, I2, I4, G1, G2, G3, G4, G5 (justificado), T1, T2, T3 |
| **Siguen abiertos, sin cambios** (13) | I3, I5, I6, I7, I8, I9, I10, D1, A1, A2, G6, G7, G8 |

Qué se ha comprobado en los cerrados:
- **K1**: la excepción del principio V está en la constitución y cubre `seed/demo.sql`, `quickstart.md` y `.env.example`.
- **I1**: ya no queda en ningún documento `presupuesto_abierto_unico` ni «borrador o emitido». La migración (T007), data-model, el contrato, spec-003 FR-008 y el plan de 003 dicen lo mismo.
- **I2**: ningún documento dice ya que `cambiar_estado` publica el evento: R14, data-model, el plan de 008 y el research de 006 coinciden.
- **I4**: el plan de 002 y T017 publican la apertura de una orden.
- **G1**: `normalizar_matricula` solo aparece en `matricula.rs` (T014) y en los planes de 001 y 007 como uso.
- **G2**: `exigir_mecanico_de` aparece en T027, en T032 y en los planes de 003 y 005.
- **G3**: `numero()` está en T021, en T026 y en T015 (`formato.rs`), y el plan de 002 lo usa.
- **G4**: el pedido pendiente está en T039 y las imputaciones por estado en T040.
- **G5**: está la tarea T035, que crea `+error.svelte`.
- **T1**: T016 → T017, sin ciclo.
- **T2**: las utilidades de prueba están divididas en T023 y T031.
- **T3**: T033 ya no lleva `[P]`.

## Hallazgos

Se mantienen los IDs de analyze-1 para los abiertos. Los nuevos llevan IDs nuevos (K2, A3, I11,
G9, D2).

| ID | Categoría | Gravedad | Ubicación | Resumen | Recomendación |
|----|-----------|----------|-----------|---------|---------------|
| K2 | Constitución | **CRITICAL** | tasks T031 (`auth::hash_contrasena("demo1234")` en `backend/tests/comun/mod.rs`) y T042; `tasks.md` (T033, T040), `analyze-1.md` y `hallazgos.md` citan `demo1234` y `demo-presupuesto-emitido` | **Nuevo**, consecuencia de K1 y T2. La excepción v1.2.0 solo permite credenciales ficticias en `seed/demo.sql`, `quickstart.md` y `.env.example`. T031 escribiría la contraseña demo en código de pruebas, fuera de esos tres ficheros. Además, la revisión de secretos (T042) fallaría tal como está escrita, porque `git grep` también encontrará esos valores en `tasks.md`, `analyze-1.md`, `hallazgos.md` y el código de pruebas. | Recomendada: que las pruebas no usen literales. `crear_usuario` genera una contraseña aleatoria con `auth::nuevo_token()` y la devuelve junto al id, y `entrar` la recibe. Además, una enmienda PATCH (1.2.1) que aclare que los documentos de `specs/` pueden **citar** los valores demo ya publicados en `quickstart.md`, y ajustar T042 a esa lista. Lo tiene que decidir el equipo. |
| I3 | Inconsistencia | MEDIUM | research R11 (`{"tipo":"orden","id":…}`) frente al contrato y T017/T030 (evento `orden` con `{id, estado}`) | Sigue igual que en analyze-1. | Dejar en R11 el formato del contrato. |
| A1 | Ambigüedad | MEDIUM | spec US1 escenario 1; quickstart §0.1; Independent Test de la fase 3 de tasks | Sigue igual: «no ve Facturación» no comprueba nada, porque no existe esa entrada de menú. | «`luis` ve «Mis órdenes» e «Historial» y no ve Citas, Almacén, Pedidos ni Tablero». |
| A2 | Ambigüedad | MEDIUM | quickstart §0 frente a SC-002 | Sigue igual: faltan `ana` y `pepe`. T043 los recorre, pero no figuran en el quickstart, que es el criterio de terminado. | Añadir un paso 4 al quickstart §0 con el menú esperado de `ana` y de `pepe`. |
| I5 | Inconsistencia | LOW | data-model y T016 (`en_diagnostico → en_diagnostico`) frente al plan de 004 («sin transición») | Sigue igual. | Quitar la autotransición de data-model y de T016. |
| I6 | Inconsistencia | LOW | plan de 002, «Endpoints» | Sigue igual: `GET /api/mecanicos` figura en 002 y en la base común (T029). | Marcarlo como «(base común)» en el plan de 002. |
| I7 | Inconsistencia | LOW | spec-008 US2 | Sigue igual: «Ana» aparece como mecánica. | Cambiarla por «Marta». |
| I8 | Inconsistencia | LOW | plan.md: «Scale/Scope» y ficheros compartidos | Sigue igual: dice «unas 12 tablas» (hay 14). Faltan en la estructura `lib.rs`, `rutas/mod.rs` y `tests/comun/mod.rs`, y la lista de ficheros compartidos no incluye `matricula.rs`, `formato.rs`, `eventos.rs`, `auth.rs`, `correo.rs`, `demo.sql` ni `+layout.svelte`. La sección «Después de la fase 0» de tasks.md sí los nombra, así que plan y tasks no coinciden. | Actualizar plan.md con la misma lista que tasks.md. |
| I9 | Inconsistencia | LOW | contrato, «Rutas de pantalla» | Sigue igual: faltan `/` y `/facturas/[id]`. | Añadirlas. |
| I10 | Inconsistencia | LOW | research R3 frente a T001 | Sigue igual: faltan crates en R3 y sobra la feature `uuid`. | Actualizar R3 y quitar `uuid`. |
| I11 | Inconsistencia | LOW | T014 (`normalizar_matricula -> Option<String>`, 422 si `None`) frente al plan de 001 y el contrato (`GET /api/publico/vehiculos/{matricula}` solo prevé 404) | **Nuevo**. El contrato no dice qué responde una matrícula inválida (vacía o con símbolos) en las rutas de 001, ni el plan de 001 recoge el caso `None`. | Añadir «422 si la matrícula no es válida» al contrato en `GET /api/publico/vehiculos/{matricula}` y `POST /api/publico/citas`, y una línea al plan de 001. |
| D1 | Duplicación | LOW | research R9 frente al plan de 004 | Sigue igual: dos versiones del `UPDATE` de la primera respuesta. | Dejar una sola en R9. |
| D2 | Duplicación | LOW | tasks de 006, T007 («utilidades de prueba: crear usuario con rol…, iniciar sesión y devolver la cookie») frente a T023/T031 de 000 (`tests/comun/mod.rs`) | **Nuevo** desde que existe `tests/comun`: 006 vuelve a escribir utilidades que la base común ya da. | Cambiar 006/T007 para que use `tests/comun` (`crear_usuario`, `entrar`, `crear_orden`) y solo añada las de pieza y proveedor. |
| A3 | Ambigüedad | LOW | T035 (`+error.svelte`: «si el mensaje viene del servidor en español, ese mensaje») | **Nuevo**. No hay forma de saber si un mensaje «viene en español», así que la tarea no se puede comprobar tal cual. | Simplificar: 404 → «Esta página no existe»; cualquier otro error → «Ha ocurrido un error», siempre con texto fijo. |
| G6 | Hueco | LOW | plan.md, `componentes/EstadoOrden.svelte` | Sigue igual: no tiene dueño. | Asignarlo a 002 o quitarlo del plan. |
| G7 | Hueco | LOW | `correo.rs` frente a los planes de 003 y 007 | Sigue igual: no está en la lista de ficheros compartidos del plan (tasks.md sí la recoge en «Después de la fase 0»). | Resolverlo junto con I8. |
| G8 | Hueco | LOW | T019 (`ServeDir("../frontend/build")`) | Sigue igual. | Sin acción en la fase 0; anotarlo para el despliegue. |
| G9 | Hueco | LOW | T039 (pedido pendiente demo de `PF-123`) frente a quickstart §5.1 (stock de PF-123 «de 4 a 2») | **Nuevo**, consecuencia de G4. Si alguien recibe ese pedido demo antes del §5, el stock de PF-123 pasa a 5 y el paso §5.1 del quickstart falla. | Usar en el pedido demo una segunda pieza de ejemplo (por ejemplo, `BJ-7 Bujía`) añadida en T038, y dejar PF-123 solo para el quickstart. |

## Cobertura

| Requisito | ¿Tiene tareas? | Tareas | Notas |
|-----------|----------------|--------|-------|
| FR-001 usuario + contraseña, un rol | Sí | T005, T027, T028, T029, T032, T033, T037 | |
| FR-002 el jefe puede todo | Sí | T027 (`exigir_rol`, `exigir_mecanico_de`), T032, T034 | Antes parcial; G2 cerrado |
| FR-003 páginas públicas | Sí | T022, T034, T043 | Solo se puede comprobar del todo cuando existan 001 y 004 |
| FR-004 configuración fuera del código | Sí | T004, T009 | |
| FR-005 estados únicos | Sí | T005, T016, T021, T025 | I5 abierto |
| FR-006 español, € y dd/mm/aaaa | Sí | T010, T015, T021, T026, T035 | G3 cerrado |
| SC-001 entrar en < 15 s | Sí | T037 | |
| SC-002 ningún rol ve opciones ajenas | Sí | T034, T043 | A2 abierto |
| Caso límite: sesión de 12 h | Sí | T027, T032 | |
| Caso límite: usuario desactivado | Sí | T027, T028, T029, T032 | |
| Supuesto: alta y restablecimiento por orden de administración | Sí | T028, T041 | |

**Tareas sin requisito de spec-000**: T014 (`matricula.rs`), T015 (`formato.rs`) y T038–T042.
Todas se justifican en las specs 001–008, en el plan maestro o en los principios II y V. No hay
alcance fantasma.

**Problemas con la constitución**: K2 (principio V). No queda ningún otro.

## Métricas

- Requisitos analizados: 8 (6 FR + 2 SC), más 2 casos límite y 1 supuesto
- Tareas: 43
- Cobertura: 100 % (FR-002 pasa de parcial a completa)
- Ambigüedades: 3 · Duplicaciones: 2 · Inconsistencias: 7 · Huecos: 4 · Constitución: 1
- Hallazgos abiertos: 18 (13 de analyze-1 + 5 nuevos) · **CRITICAL: 1** · HIGH: 0 · MEDIUM: 3 · LOW: 14
- Evolución: de 25 hallazgos (1 CRITICAL, 1 HIGH) a 18 (1 CRITICAL, 0 HIGH)

## Siguientes pasos

1. **Antes de `/speckit-implement`**: resolver **K2**. Lo recomendado son las contraseñas de prueba aleatorias en T031 y una aclaración PATCH 1.2.1 sobre las citas en `specs/` con `/speckit-constitution`, ajustando T042.
2. **En la misma pasada** (son baratos y evitan dudas al implementar):
   - A1 y A2 (quickstart §0, que es el criterio de terminado).
   - A3 (T035).
   - G9 (datos demo).
   - D2 (tasks de 006).
3. **Documentación (no bloquea)**: I3, I5–I11, D1, G6 y G7. Se pueden agrupar en un solo commit de revisión entre A y B.
4. G8 queda anotado para cuando se planifique el despliegue.
5. Después, `analyze-3.md`.

¿Quieres que te proponga los cambios concretos para K2 y los hallazgos del punto 2?
