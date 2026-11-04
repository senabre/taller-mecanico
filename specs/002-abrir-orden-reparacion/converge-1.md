# Converge 1 · spec-002 · Recibir el vehículo y abrir la orden de reparación

**Fecha**: 2026-11-04 · **Rama**: `spec-002` (último commit `584cfcb`) · **Resultado**:
`tasks_appended` (4 tareas, Phase 6: Convergence, T032–T035) · **Responsable**: Persona A ·
**Revisor**: Persona B

## Alcance

- **Código revisado**: `backend/src/rutas/ordenes.rs`, `backend/tests/spec002_ordenes.rs`,
  `frontend/src/routes/ordenes/` (`+page.svelte`, `nueva/+page.svelte`, `[id]/+page.svelte`,
  `[id]/Datos.svelte`), la columna «Orden» de `frontend/src/routes/citas/+page.svelte`, la parte 5
  de `backend/seed/demo.sql`, y los cambios en `frontend/src/lib/tipos.ts` y
  `backend/src/rutas/imputaciones.rs`. También `orden_numero` en `backend/src/rutas/citas.rs`.
- **Documentos**: `spec.md`, `plan.md`, `research.md` (D1–D8), `data-model.md`, `contracts/api.md` y
  `tasks.md` de spec-002; el contrato común (sección spec-002 con su tabla de validación), el
  data-model común y el quickstart común §2; la constitución 1.2.1.
- **Método**: cada requisito se contrastó con el código y con lo que comprueban de verdad las
  pruebas y los guiones del navegador, sin dar nada por cumplido solo porque la tarea esté marcada.
  Las comprobaciones de T030 (2026-11-04) estaban en verde y el código no ha cambiado desde
  entonces: `spec002_ordenes` 12/12 y 118 pruebas en total, sin ignoradas.
- **Fuera de este informe** (ya tiene tarea abierta): T031, revisión de la Persona B.

## Convergence Findings

| ID | Tipo | Gravedad | Origen | Evidencia | Trabajo pendiente |
|----|------|----------|--------|-----------|-------------------|
| F1 | partial | MEDIUM | US1-1, US2-1, FR-003 · quickstart común §2.4–§2.6 · principio IV | **Lo que el quickstart manda escribir se rechaza.** El §2 dice «Guardar con 85.000 km», «23.500 km» y «30.000 km», y la spec escribe los km igual. `leer_km` (`ordenes.rs`) solo acepta cifras: «85.000» con punto da 422 «El kilometraje debe ser un número entero igual o mayor que 0». En el paso 6 el efecto es peor: en vez del rechazo por la visita anterior, sale un error que no explica nada. No lo detectó la verificación de T029 porque los guiones (`t029.mjs`, `t029b.mjs`) teclean `85000`, `23500` y `30000`; prueba 7 tampoco lo cubre. Una persona que siga el quickstart al pie de la letra fallará en el paso 4, y la app muestra los km con punto en la ficha, así que lo natural es escribirlos igual. | Decidir y aplicar una de las dos: (a) **recomendada**: aceptar el punto de miles («85.000» → 85000; solo grupos de tres cifras, «85.0» sigue siendo inválido), con la regla en la tabla del contrato común y un caso en la prueba 7; (b) escribir en el quickstart común `85000`, `23500` y `30000`, y añadir al campo un ejemplo sin punto. |
| F2 | partial | MEDIUM | Data-model común (`retrasada`) · hallazgo I1 · T016, T019 · FR-006 | La regla «estado ≠ `facturada`» de `retrasada` está en la consulta (`COLUMNAS_RESUMEN`), pero **ninguna prueba ni paso la comprueba**: `spec002_ordenes.rs` no contiene ningún caso con una orden facturada, y la verificación de T029 solo mira la orden 8 (terminada). Es justo la diferencia que resolvió I1: sin esa prueba, quitar la condición no haría fallar nada, y `/ordenes` mostraría «Retrasada» en órdenes ya facturadas. | Añadir a la prueba 12 una orden `facturada` con la entrega de ayer → `retrasada: false`, en la lista y en la ficha; comprobar que falla al quitar la condición. |
| F3 | partial | MEDIUM | SC-001 · principios II y IV · T029 | **Dos verificaciones pendientes sin ninguna tarea abierta.** T029 está marcada, y lo pendiente solo consta en su nota y en el informe: (a) el cronometraje humano de SC-001: los tiempos medidos son de máquina; (b) que la fecha de entrega salga en dd/mm/aaaa: `<input type="date">` usa el idioma del navegador, no el `lang="es"` de la página, y las capturas la muestran como `11/07/2026` y `mm/dd/yyyy`. No se ha observado en ningún navegador en español. Si en uno en español sale bien, en uno en inglés seguirá saliendo mal; hay que decidir si eso basta para el principio II. Afecta también al selector de `/citas` de spec-001, ya aceptado así. | Abrir tarea propia: cronometrar SC-001 con una persona, comprobar el formato en un navegador en español, anotarlo en el informe y decidir si el control nativo cumple el principio II. |
| F4 | partial | LOW | Contrato común (tabla de validación, `km`) · FR-003 · T012 | El contrato fija el límite superior («de 0 a 2.147.483.647»), que `leer_km` aplica con `i32::try_from`, pero ninguna prueba lo ejercita: `spec002_ordenes.rs` no tiene ningún valor cercano a ese límite. Si se cambiara, un km fuera de rango daría un 500 de la base de datos en lugar del 422. | Añadir a la prueba 7: `2147483647` → 201 y `2147483648`, como número y como texto → 422 del km. |

### Observaciones (no generan tarea)

- **`/ordenes/abc`** (id no numérico): el extractor `Path<i64>` responde un 400 de axum sin el
  formato del contrato, y la ficha muestra «Ha ocurrido un error inesperado» en lugar de «No existe
  la orden». Nadie llega ahí desde la aplicación, y las rutas de B con `{id}` hacen lo mismo. Si se
  quiere cambiar, es una decisión de la base común.
- **`backend/tests/spec006_pedidos.rs`**: el comentario de `la_ficha_de_la_orden_trae_sus_pedidos`
  aún dice «quitar el `#[ignore]` cuando exista (T031)». Anotado para la Persona B en T020; lo ve
  en T031.

### Comprobado sin hallazgos

- **FR-001 y Q2**: el servidor compara con `fechas::hoy()` (409, prueba 5) y `/citas` solo ofrece
  «Abrir orden» hoy; por URL, el formulario lo rechaza (T029, fila 4).
- **FR-002 y SC-002**: el número se pide después de todas las comprobaciones, dentro de la
  transacción (pruebas 2, 3 y 4; §2.3–§2.5 en el navegador).
- **FR-003 y Q4**: daños con `trim`, sin convertir el vacío en «Sin daños visibles» (prueba 7). La
  parte del km, ver F1 y F4.
- **FR-004 y D3**: visita anterior por `abierta_en`, para otra cita; una mutación confirma que la
  prueba 6 vigila `cita_id <> $2`. Vehículo sin visitas aceptado.
- **FR-005 y Q3**: misma condición que `GET /api/mecanicos` (prueba 8); el `vehiculo_id` sale de la
  cita, no de la petición (prueba 1). La carrera con una baja simultánea está aceptada en D4.
- **FR-006, FR-007 y D5**: entrega ≥ hoy, estado `recibida`, y evento después del `commit`, que no
  se publica si hay error (prueba 1).
- **FR-008, Q5 y D2**: `FOR UPDATE` antes de mirar la orden existente; una mutación confirma que la
  prueba 4 falla sin él (`[201, 500]`). «Orden Nº N» en `/citas` y 409 con `detalle` y enlace.
- **Orden de los 422 y 409**: como dice la tabla del contrato; el km anterior es el último 422
  (prueba 6).
- **US1-1 … US1-9, US2-1 … US2-3, SC-003 y SC-004**: pruebas y verificación de T029 (US1-6, con
  spec-001 FR-006). La excepción es lo escrito en F1.
- **D6 y secciones de B**: `orden_completa` llama a las tres funciones sin modificarlas; los
  componentes de B no se han tocado y reciben sus props; `factura_id` por subconsulta. La prueba
  de spec-006 ya no está ignorada.
- **D7 y D8**: `orden_numero`, formulario y parte 5 de `demo.sql` (comprobada con consultas).
- **Contrato**: textos propios de 002 alineados; `tipos.ts` e `imputaciones.rs` hacen solo lo que
  piden T002 y T003.
- **Código sin justificación**: ninguno. Lo que no sale de un FR de spec-002 (lista de órdenes,
  E1) o es accesorio (`resumenes` y `OrdenResumen` públicos, para spec-008) consta en los
  documentos.
- **Constitución**: I, sin dependencias ni migraciones nuevas. II, textos en español; fechas, ver
  F3. III, sin alcance sin justificar. IV, ver F1 y F3. V, sin datos del cliente en la ficha ni en
  la lista, sin secretos, y las pruebas no usan datos demo.

## Métricas

- **Requisitos y criterios revisados**: 8 FR, 4 SC, 12 escenarios (US1-1 … US1-9, US2-1 … US2-3) y
  7 casos límite.
- **Decisiones del plan revisadas**: research D1–D8, las reglas clave del plan, la lista de pruebas
  1–12 y la tabla de validación del contrato común.
- **Principios de la constitución revisados**: 5 (I–V).
- **Hallazgos por tipo**: missing 0 · partial 4 (F1, F2, F3, F4) · contradicts 0 · unrequested 0.
- **Hallazgos por gravedad**: CRITICAL 0 · HIGH 0 · MEDIUM 3 (F1, F2, F3) · LOW 1 (F4).

## Tareas añadidas

En `tasks.md`, «Phase 6: Convergence», T032–T035, una por hallazgo: T032 (F1), T033 (F2), T034 (F4)
y T035 (F3), que va la última porque necesita a una persona. Responsable: Persona A; revisor:
Persona B.

## Siguiente paso

Decidir F1 (aceptar el punto de miles o cambiar el quickstart) y lanzar `/speckit-implement` con
T032–T034. T035 (F3) necesita a una persona con un navegador en español. Después, un
`/speckit-converge` de comprobación debería dar «Converged».
