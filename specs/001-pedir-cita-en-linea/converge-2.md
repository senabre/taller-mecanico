# Converge 2 · spec-001 · Pedir cita en línea

**Fecha**: 2026-10-29 · **Rama**: `spec-001` (último commit `d953a62`) · **Resultado**:
`tasks_appended` (1 tarea, Phase 8: Convergence, T040) · **Responsable**: Persona A ·
**Revisor**: Persona B

## Alcance

- **Mismos criterios que [converge-1](converge-1.md)**. Se revisa el mismo código:
  `backend/src/rutas/citas.rs`, `backend/src/matricula.rs`, `backend/tests/spec001_citas.rs`,
  `frontend/src/routes/cita` y `frontend/src/routes/citas`. Para la sesión y el menú se vuelven a
  leer `lib/api.ts` y el layout.
- **Mismos documentos**: `spec.md`, `plan.md`, `research.md`, `contracts/api.md`,
  `data-model.md` y `tasks.md` de spec-001, el contrato común y la constitución 1.2.1. No se ha
  dado nada por cumplido solo por estar marcado.
- **Además**: se verifican las correcciones T035–T039 (commit `abba866`, más `d953a62` para el
  contrato común) y se busca si han abierto brechas nuevas.
- **Comprobaciones ejecutadas**: `cargo test --test spec001_citas` da 14/14; `npm run check`, 0
  errores y 0 avisos; `npm test`, 6/6. La batería completa del backend y `clippy` se ejecutaron al
  cerrar la Phase 7, sin cambios de código desde entonces.
- **Fuera de este informe**: las tareas que ya están abiertas y no se duplican.
  - T030, T035 y T037: implementadas; falta verlas en el navegador, con sus pasos ya en T032.
  - T031: revisión de la Persona B, que ya incluye el contrato común de T038.
  - T032, T033 y T034.

## Verificación de las correcciones de converge-1

| Hallazgo | Tarea | Estado | Evidencia |
|----------|-------|--------|-----------|
| F1 · FR-010, US1-4 | T035 | **Resuelto en código; falta la prueba en el navegador** (T032 d) | `cita/+page.svelte`, `confirmar()`: con el 409 de cita pendiente y `vehiculo = null`, ahora se llama a `buscarVehiculo()`. Esta función pone `nuevo = false`, recibe el 200 con `cita_pendiente: true` y vacía los datos nuevos, así que el formulario muestra el aviso y desactiva Confirmar, como pide FR-010. Si la nueva búsqueda falla, se muestra el error general y Confirmar sigue desactivado (`nuevo` y `vehiculo` quedan vacíos), así que no se puede confirmar a ciegas. No se ha visto nada nuevo que haga falta corregir. |
| F2 · FR-004, FR-010, D3/D4 | T036 | **Resuelto** | Prueba `alta_simultanea_de_la_misma_matricula_nueva` (20 vueltas, días distintos): `[201, 409]`, 1 cliente, 1 vehículo y 1 cita, y el propietario es el ganador. En tasks.md consta que, sin el bloqueo por matrícula, falla con `[201, 500]`. |
| F3 · FR-009, US1-8 | T037 | **Resuelto en código; falta la prueba en el navegador** (T032 e) | `citas/+page.svelte`, `cargar()`: descarta la respuesta y el error si `elegido !== dia`, y solo pone `cargada = true` con la respuesta vigente. Revisados tres casos: A→B con B primero, A→B con A primero, y A→B→A. En los tres, la tabla corresponde al día del selector. Al borrar la fecha, el efecto vuelve a hoy y la respuesta anterior se descarta. |
| F4 · contratos | T038 | **Resuelto, salvo un detalle** (ver G1) | El contrato común (`d953a62`) recoge el `detalle.campo` del 422 público y el 400 «La fecha no es válida»; la prueba `errores_documentados_en_el_contrato` los comprueba. La revisión de la Persona B está anotada en T031. El contrato de 001 enumera los textos propios de `/cita`, pero la lista se presenta como completa y le faltan los títulos (G1). |
| F5 · Constitución III | T039 | **Resuelto** | El plan, en la sección del frontend, justifica «Pedir otra cita» (US1-4, US1-6 y Assumptions) y «Hoy» (FR-009). El código coincide con lo descrito: el título «Citas del dd/mm/aaaa» y el botón «Hoy» solo aparecen con otro día; `otraCita()` vacía el formulario y recarga los días. |

## Convergence Findings

| ID | Tipo | Gravedad | Origen | Evidencia | Trabajo pendiente |
|----|------|----------|--------|-----------|-------------------|
| G1 | contradicts | LOW | `contracts/api.md` de 001 (texto añadido por T038) | El contrato dice: «Además de las etiquetas de los campos y los botones, los únicos textos propios del formulario son: …». En `cita/+page.svelte` hay dos títulos que no son etiquetas ni botones y que no están en la lista: «Pedir cita» (`<h1>`) y «Cita confirmada» (`<h2>` de la confirmación). Los prefijos «Fecha:», «Motivo:» y «Vehículo:» pueden contarse como etiquetas, pero la redacción no lo deja claro. Es una brecha nueva, introducida por la propia corrección de F4 al presentar la lista como completa. | Completar la frase del contrato de 001 para que también excluya de la lista los títulos y las etiquetas de los datos mostrados (p. ej., «Además de los títulos, las etiquetas de los campos y de los datos mostrados, y los botones, …»), o añadir esos textos a la lista. Solo cambia la documentación de 001; el contrato común no se toca. |

### Comprobado sin hallazgos nuevos

Siguen cumpliéndose todos los puntos que converge-1 dio por cumplidos:

- **FR-001 a FR-010 y FR-004a**: las correcciones no han tocado `citas.rs` ni `matricula.rs`
  (solo se han añadido pruebas).
- **US1-1 a US1-10, US2-1 a US2-4, US3-1 y US3-2**: cubiertos por las pruebas automáticas, con
  las verificaciones en el navegador pendientes en T032.
- **SC-003 y SC-004**: pruebas 9 y 12. SC-001 y SC-002 se cronometran en T032.
- **Decisiones D1–D7 y orden de I3**: sin cambios. El caso «día lleno → 422» de la prueba 4 sigue
  pasando.
- **Constitución**:
  - I: sin dependencias nuevas. T035 y T037 se resolvieron sin añadir librerías de pruebas.
  - II: los textos nuevos están en español.
  - III: los botones quedan justificados en el plan.
  - IV: los pasos de pantalla nuevos están en T032.
  - V: sin datos personales nuevos ni secretos.

## Métricas

- **Requisitos y criterios revisados**: 11 FR, 4 SC, 16 escenarios y 9 casos límite.
- **Decisiones del plan revisadas**: research D1–D7, reglas clave del plan, la lista de pruebas
  1–12 y las dos pruebas nuevas.
- **Correcciones verificadas**: 5 (T035–T039). Resueltas: 5, dos de ellas pendientes solo de la
  prueba en el navegador, que ya está en T032.
- **Principios de la constitución revisados**: 5 (I–V).
- **Hallazgos por tipo**: missing 0 · partial 0 · contradicts 1 (G1) · unrequested 0.
- **Hallazgos por gravedad**: CRITICAL 0 · HIGH 0 · MEDIUM 0 · LOW 1 (G1).

## Tareas añadidas

En `tasks.md`, «Phase 8: Convergence», T040. Responsable: Persona A; revisor: Persona B.

## Siguiente paso

Hacer T040 (un cambio de redacción en el contrato de 001). Después, cerrar T030, T035 y T037 con
la prueba manual de T032, y hacer T031, T033 y T034. Un converge-3 tras T040 debería dar
«Converged» para el alcance de código y documentos.
