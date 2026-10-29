# Converge 3 · spec-001 · Pedir cita en línea

**Fecha**: 2026-10-29 · **Rama**: `spec-001` (último commit `8d0ef2c`, más T040 aún sin commit en
`contracts/api.md` y `tasks.md`) · **Resultado**: `converged`. No se añade ninguna tarea:
`tasks.md` queda tal como lo dejó `/speckit-implement` de T040.

## Alcance

- **Mismos criterios que [converge-1](converge-1.md) y [converge-2](converge-2.md)**. Se revisa
  el mismo código: `backend/src/rutas/citas.rs`, `backend/src/matricula.rs`,
  `backend/tests/spec001_citas.rs`, `frontend/src/routes/cita` y `frontend/src/routes/citas`.
- **Mismos documentos**: `spec.md`, `plan.md`, `research.md`, `contracts/api.md`,
  `data-model.md` y `tasks.md` de spec-001, el contrato común y la constitución 1.2.1.
- **El código no ha cambiado desde converge-2**: `git diff HEAD -- backend frontend` está vacío.
  En converge-2, con este mismo código, `cargo test --test spec001_citas` dio 14/14,
  `npm run check` 0 errores y 0 avisos, y `npm test` 6/6.
- **Fuera de este informe** (tareas ya abiertas, no son brechas nuevas):
  - T030, T035 y T037: implementadas; falta verlas en el navegador, con sus pasos en T032.
  - T031: revisión de la Persona B, que incluye el contrato común de T038.
  - T032: prueba manual con SC-001 y SC-002 cronometrados.
  - T033: ejecución final de todas las comprobaciones.
  - T034: quickstart §2.3.

## Verificación de T040 (converge-2 G1)

| Comprobación | Resultado |
|--------------|-----------|
| La lista ya no se presenta como completa sin serlo | ✅ El contrato de 001 define qué son **etiquetas**: los nombres de los campos, los textos de los botones, «Elige un día», el ejemplo «1234 BCD» y los rótulos «Vehículo:», «Fecha:» y «Motivo:». Después enumera los textos propios. |
| «Pedir cita» y «Cita confirmada» | ✅ Están en la lista, como títulos. |
| «Fecha:», «Motivo:» y «Vehículo:» | ✅ El contrato dice expresamente que son etiquetas. |
| Cruce con `cita/+page.svelte` | ✅ Se han extraído todos los textos del marcado y de las cadenas de `<script>`. Cada texto visible del formulario está en la lista o es una etiqueta según la definición: matrícula, día, motivo, marca, modelo, nombre, teléfono y correo; «Confirmar», «Confirmando…» y «Pedir otra cita»; «Elige un día» y «1234 BCD»; los títulos; el aviso de FR-010; «No quedan fechas libres…»; «Ese día se acaba de completar; elige otro»; «No conocemos este vehículo…»; «Buscando el vehículo…»; «No se pudieron cargar los días» y «Ha ocurrido un error inesperado». La constante `DIA_COMPLETO` solo sirve para reconocer el 409 y no se muestra. |
| Sin cambios fuera de su alcance | ✅ T040 solo cambia `contracts/api.md` de 001 y su nota en `tasks.md`; no toca el código ni el contrato común. |

## Convergence Findings

No hay hallazgos accionables.

### Observación (no accionable en spec-001)

- **Título de la pestaña del navegador** (`<svelte:head><title>Pedir cita · Talleres Rueda</title>`
  en `cita/+page.svelte`). No está en la lista del contrato de 001, pero no se trata como brecha
  por dos motivos:
  1. La lista se refiere a los textos «del formulario», y el título va en la cabecera de la
     página.
  2. Es una convención que siguen igual las 4 pantallas del frontend (`<pantalla> · Talleres
     Rueda`). Ninguna spec ni la base común la describen, así que, si se quiere documentar,
     corresponde a la base común y no a spec-001.
  
  Queda anotada para quien revise la base común. No genera tarea aquí.

### Comprobado sin hallazgos

- **FR-001 a FR-010 y FR-004a**: la implementación es la misma que se verificó en converge-1 y
  converge-2. Ninguna corrección posterior ha tocado `citas.rs` ni `matricula.rs`.
- **US1-1 a US1-10, US2-1 a US2-4, US3-1 y US3-2**: cubiertos por las 14 pruebas de
  `spec001_citas`, más las unitarias de `matricula.rs` y `citas.rs`. Las verificaciones en el
  navegador están pendientes en T032, que ya incluye los pasos de US3 y de T035 y T037.
- **SC-003 y SC-004**: pruebas 9 y 12. SC-001 y SC-002 se cronometran en T032.
- **Decisiones D1–D7, orden de I3 y reglas clave del plan**, incluida la justificación de
  «Pedir otra cita» y «Hoy» (T039): el código coincide con lo descrito.
- **Contratos**:
  - El común recoge el `detalle.campo` del 422 público y el 400 de `GET /api/citas` (T038),
    pendientes de revisión de la Persona B en T031.
  - El de 001 coincide con la pantalla (T040).
- **Correcciones de converge-1**: F1–F5 siguen resueltas. Sigue pendiente solo lo ya anotado:
  la prueba en el navegador de T035 y T037.
- **Constitución**:
  - I: sin dependencias ni migraciones nuevas.
  - II: textos en español de España y fechas dd/mm/aaaa.
  - III: no hay nada sin justificar en la spec, el plan o las tareas.
  - IV: los escenarios se verifican en T032.
  - V: sin datos personales de más, sin correo en la lista y sin secretos.

## Métricas

- **Requisitos y criterios revisados**: 11 FR, 4 SC, 16 escenarios y 9 casos límite.
- **Decisiones del plan revisadas**: research D1–D7, reglas clave del plan, la lista de pruebas
  1–12 y las pruebas de T036 y T038.
- **Correcciones verificadas**: T040 (converge-2 G1), resuelta. F1–F5 de converge-1 siguen
  resueltas.
- **Principios de la constitución revisados**: 5 (I–V).
- **Hallazgos por tipo**: missing 0 · partial 0 · contradicts 0 · unrequested 0. Hay 1
  observación no accionable.
- **Hallazgos por gravedad**: CRITICAL 0 · HIGH 0 · MEDIUM 0 · LOW 0.

## Resultado

✅ **Converged**: la implementación cumple la spec, el plan y las tareas.

## Siguiente paso

No hace falta otra pasada de `/speckit-implement` para el alcance de código y documentos. Para
cerrar spec-001 quedan las tareas que ya estaban abiertas:

- T032: la prueba manual completa del quickstart §1, con los pasos añadidos para US3, T035 y
  T037. Al cumplirse, se marcan T030, T035 y T037.
- T031: la revisión de la Persona B, que incluye los cambios del contrato común.
- T033 y T034.

Después, la rama queda lista para abrir el PR.
