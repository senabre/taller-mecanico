# Converge 1 · spec-001 · Pedir cita en línea

**Fecha**: 2026-10-29 · **Rama**: `spec-001` · **Resultado**: `tasks_appended` (5 tareas, Phase 7:
Convergence, T035–T039) · **Responsable**: Persona A · **Revisor**: Persona B

## Alcance

- **Código revisado**: `backend/src/rutas/citas.rs`, `backend/src/matricula.rs`,
  `backend/tests/spec001_citas.rs`, `frontend/src/routes/cita/+page.svelte` y
  `frontend/src/routes/citas/+page.svelte`. También se leyeron `frontend/src/lib/api.ts`,
  `frontend/src/routes/+layout.svelte` y `+layout.ts`, para la redirección sin sesión y el menú
  por rol.
- **Documentos**: `spec.md`, `plan.md`, `research.md` (D1–D7), `contracts/api.md`, `data-model.md`
  y `tasks.md` de spec-001; la sección «spec-001 · Citas» del contrato común, con su tabla de
  validación; y la constitución 1.2.1.
- **Método**: se contrastó cada requisito con el código, sin dar nada por cumplido solo porque la
  tarea esté marcada. Las pruebas automáticas se dieron por buenas solo cuando comprueban de
  verdad el criterio. El último `cargo test` (2026-10-29, sin cambios en el código desde entonces)
  pasa entero: 12/12 en `spec001_citas` y 4 unitarias de `citas.rs`.
- **Fuera de este informe**: el trabajo que ya tiene tarea abierta y no hay que duplicar.
  - T030: la pantalla de US3 está hecha; falta la prueba en el navegador.
  - T031: revisión de la Persona B.
  - T032: prueba manual del quickstart §1, con SC-001 y SC-002 cronometrados y los pasos de US3.
  - T033: ejecución final de todas las comprobaciones.
  - T034: quickstart §2.3.

## Convergence Findings

| ID | Tipo | Gravedad | Origen | Evidencia | Trabajo pendiente |
|----|------|----------|--------|-----------|-------------------|
| F1 | partial | MEDIUM | FR-010, US1-4 (research D3, D4) | `cita/+page.svelte`, `confirmar()`: con un 409 «Este vehículo ya tiene una cita pendiente.» solo se actúa `if (vehiculo)`. Si el formulario trató la matrícula como nueva (404 → `nuevo = true`, `vehiculo = null`), no se muestra nada: no hay aviso, ni error general, y Confirmar sigue activo. Ocurre cuando otra persona da de alta la misma matrícula nueva y pide su cita mientras tanto: el servidor aplica D4 en la segunda búsqueda y responde el 409 de cita pendiente. FR-010 exige mostrar el aviso y no dejar confirmar. | Con ese 409, mostrar siempre el aviso y desactivar Confirmar, aunque la matrícula se hubiera tratado como nueva. Por ejemplo, volviendo a buscar el vehículo, de modo que el formulario pase a «conocido con cita pendiente» y oculte los campos de vehículo nuevo. |
| F2 | partial | MEDIUM | FR-004, FR-010 (research D3 y D4; plan, «Reglas clave», pasos 2, 4 y 7) | Ninguna prueba ejercita la segunda búsqueda con una matrícula **nueva**. La prueba 10 usa un vehículo ya insertado y la prueba 1 una sola petición. D3 justifica el bloqueo por matrícula también porque «evita que dos altas simultáneas de la misma matrícula nueva choquen en el `UNIQUE`», y D4 describe ese caso de carrera. Según el código, una de las dos peticiones recibiría 201 y la otra 409 de cita pendiente, con 1 cliente y 1 vehículo. Pero no hay ninguna prueba que lo compruebe ni que detecte una regresión (por ejemplo, un 500 por el `UNIQUE`). | Añadir una prueba `#[sqlx::test]` con dos `POST` a la vez con la misma matrícula nueva y datos de cliente distintos. Debe dar un 201 y un 409 «Este vehículo ya tiene una cita pendiente.», con exactamente 1 cliente, 1 vehículo y 1 cita, y el cliente guardado debe ser el de la petición que ha ganado. Repetir 20 veces, como las pruebas 9 y 10. Comprobar que falla si se quita el bloqueo por matrícula. |
| F3 | partial | LOW | FR-009, US1-8 | `citas/+page.svelte`: cada cambio del selector lanza `cargar(dia)` sin descartar las respuestas antiguas. Si se cambia de día rápido y una respuesta anterior llega después, la tabla muestra las citas de otro día distinto del que indican el selector y el título. | Ignorar las respuestas que no correspondan al día seleccionado en ese momento. Por ejemplo, comparando el día pedido con `dia` antes de asignar `citas`. |
| F4 | contradicts | LOW | `contracts/api.md` de 001 («Los únicos textos propios del formulario son…»); contrato común (`GET /api/publico/vehiculos`, `GET /api/citas`) | (a) El contrato de 001 dice que los únicos textos propios de `/cita` son «No quedan fechas libres…» y «Ese día se acaba de completar; elige otro». La pantalla tiene además «No conocemos este vehículo. Cuéntanos qué coche es y cómo contactarte.», que ya está en el plan (aprobado el 2026-10-29) pero no en el contrato, y también «Buscando el vehículo…», «No se pudieron cargar los días» y «Ha ocurrido un error inesperado». (b) Hay dos desviaciones justificadas en tasks.md (T015, T017) que no están en el contrato común: el 422 del `GET` público lleva `detalle.campo`, y `GET /api/citas?fecha=ayer` da 400 «La fecha no es válida». | Alinear el contrato de 001 y el común con lo implementado (o cambiar el código si la Persona B lo rechaza): lista completa de textos propios de `/cita`, `detalle.campo` en el 422 del `GET` y el 400 por fecha mal escrita. Lo revisa la Persona B, porque el contrato común es compartido. |
| F5 | unrequested | LOW | Constitución III; spec FR-008, FR-009 | Dos botones que ni la spec ni el plan piden y que solo están justificados en notas de tasks.md (T018 nota 4, T019 ajuste 1): «Pedir otra cita» tras la confirmación, en `/cita`, y «Hoy», en `/citas`. Son pequeños y coherentes con la spec: recepción usa el mismo formulario según Assumptions, y FR-009 dice que la lista abre con hoy. Aun así, el principio III pide que estén escritos. | Justificarlos en el plan (sección del frontend) o quitarlos. |

### Comprobado sin hallazgos

- **FR-001**: `/cita` está en `esRutaPublica` y el layout no pide sesión.
- **FR-002 y US1-5**: `normalizar_matricula` (2–10, `[A-Z0-9]`) tiene pruebas unitarias. En
  `citas.rs`, la matrícula es lo primero que se valida en el `GET` y en el `POST`, antes de
  consultar la base de datos. La pantalla muestra el error bajo la matrícula y no deja confirmar.
  El commit propio de T001 existe (`224daaf`); la revisión de la Persona B sigue en T031.
- **FR-003, US1-1, US1-7 y D4**: el `GET` público no selecciona campos de `cliente`, y con un
  vehículo conocido el `POST` no hace ningún `UPDATE` (prueba 6).
- **FR-004, FR-004a, FR-005, US2-1 a US2-4 y el orden de I3**: los 422 llegan antes de los
  bloqueos (pruebas 1, 4 y 5, y unitarias). Una mutación confirmó que el caso «día lleno → 422»
  de la prueba 4 detecta el orden.
- **FR-006**: prueba 11, con los límites `hoy`, `hoy + 31`, el sábado y el último día laborable
  ≤ `hoy + 30`.
- **FR-007, US3-1, US3-2 y SC-003**: una sola consulta en `dias-disponibles`, y el cupo se cuenta
  con el bloqueo por fecha tomado (pruebas 8 y 9). La prueba 9 falla si se quita el bloqueo.
- **FR-008 y US1-2**: el 201 devuelve `{fecha, motivo, matricula}` y la pantalla muestra la fecha
  en dd/mm/aaaa con el motivo.
- **FR-009, US1-3, US1-8 a US1-10 y SC-004**: 401, 403, el jefe pasa, nunca se devuelve `email`,
  orden por `creada_en, id`, sin `fecha` se usa hoy (prueba 12). La pantalla abre con hoy en
  Madrid (D5, con su cambio documentado), muestra «No hay citas para este día» y, con un 403,
  solo el mensaje común. El menú enseña «Citas» solo a recepción y jefe.
- **FR-010 y US1-4, US1-6**: `cita_pendiente` sin la fecha, el 409 en el `POST`, y se puede pedir
  otra cita si la anterior ya pasó (prueba 7). La concurrencia con un vehículo conocido está
  cubierta (prueba 10). Las carencias están en F1 y F2.
- **Constitución**:
  - I: sin dependencias ni migraciones nuevas.
  - II: textos en español de España y fechas dd/mm/aaaa.
  - IV: los escenarios se verifican en T032.
  - V: no se devuelve el correo, no hay secretos en el código y las pruebas no usan datos demo.
  - III: ver F5.

## Métricas

- **Requisitos y criterios revisados**: 11 FR (FR-001 a FR-010 y FR-004a), 4 SC, 16 escenarios
  (US1-1 a US1-10, US2-1 a US2-4, US3-1 y US3-2) y 9 casos límite.
- **Decisiones del plan revisadas**: research D1–D7; reglas clave del plan (`GET` público, los
  8 pasos del `POST`, `GET /api/citas`, `/cita` y `/citas`); la lista de pruebas 1–12.
- **Principios de la constitución revisados**: 5 (I–V).
- **Hallazgos por tipo**: missing 0 · partial 3 (F1, F2, F3) · contradicts 1 (F4) ·
  unrequested 1 (F5).
- **Hallazgos por gravedad**: CRITICAL 0 · HIGH 0 · MEDIUM 2 (F1, F2) · LOW 3 (F3, F4, F5).

## Tareas añadidas

En `tasks.md`, «Phase 7: Convergence», T035–T039 (una por hallazgo, en el orden de la tabla).
Responsable: Persona A; revisor: Persona B.

## Siguiente paso

Ejecutar `/speckit-implement` para T035–T039, preferiblemente antes de T032, para que la prueba
manual se haga sobre el código definitivo. Después, un `/speckit-converge` de comprobación
debería dar «Converged».
