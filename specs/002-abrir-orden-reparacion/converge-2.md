# Converge 2 · spec-002 · Recibir el vehículo y abrir la orden de reparación

**Fecha**: 2026-11-04 · **Rama**: `spec-002` (último commit `b592531`) · **Resultado**: `converged`.
No se añade ninguna tarea: `tasks.md` queda sin cambios · **Responsable**: Persona A · **Revisor**:
Persona B

## Alcance

- **Mismos criterios que [converge-1](converge-1.md)**. Se revisa el mismo código:
  `backend/src/rutas/ordenes.rs`, `backend/tests/spec002_ordenes.rs`, `frontend/src/routes/ordenes/`,
  la columna «Orden» de `frontend/src/routes/citas/+page.svelte`, la parte 5 de
  `backend/seed/demo.sql`, y los cambios en `tipos.ts`, `imputaciones.rs` y `citas.rs`.
- **Mismos documentos**: `spec.md`, `plan.md`, `research.md` (D1–D9), `data-model.md`,
  `contracts/api.md` y `tasks.md` de spec-002, el contrato común y la constitución 1.2.1.
- **Además**: se verifican las correcciones T032–T034 y se busca si han abierto brechas nuevas.
- **Comprobaciones ejecutadas en esta pasada**: `spec002_ordenes` 12/12, `spec001_citas` 15/15,
  `spec006_pedidos` 23/23 (sin ignoradas) y `npm run check` con 0 errores y 0 avisos. La batería
  completa, `clippy` y `npm test` se ejecutaron al cerrar T032–T034, sin cambios de código desde
  entonces.
- **Tareas abiertas, que no son brechas nuevas y no se duplican**:
  - T031: revisión de la Persona B.
  - T035: comprobación manual de la responsable (ver «Lo que este resultado no cubre»).

## Verificación de las correcciones de converge-1

| Hallazgo | Tarea | Estado | Evidencia |
|----------|-------|--------|-----------|
| F1 · km con punto | T032 | **Resuelto** | `sin_punto_de_miles` acepta «85.000» y solo grupos de tres cifras detrás de un primer grupo de una a tres. La prueba 7 lo comprueba con 3 casos aceptados y 8 rechazados, y falló en rojo antes del cambio. La tabla del contrato común y research D4 lo recogen. Verificado en el navegador escribiendo los km con punto: 23/23 y 3/3 (sección «Kilómetros con punto de miles» de `evidencias/T029/verificacion.md`). Revisada la función: el texto vacío, «.000», «85.» y un número de cifras enorme no pasan, y el límite del `INTEGER` se sigue aplicando después. |
| F2 · facturada no retrasada | T033 | **Resuelto** | La prueba 12 comprueba `retrasada: false` en una orden facturada con la entrega de ayer, en la lista y en la ficha. Sin la condición, falla (`retrasada: true`). |
| F3 · verificaciones pendientes | T035 y research D9 | **Decidido; verificación abierta en T035** | D9 mantiene el control de fecha nativo, con sus motivos y su consecuencia (en un navegador en otro idioma sale en ese formato) y dice que afecta a `/citas`. Lo pendiente ya tiene tarea propia. |
| F4 · límite del km | T034 | **Resuelto** | La prueba 7 cubre `2147483647` (201) y `2147483648` como número y como texto (422). Recortando el km en lugar de rechazarlo, falla. |

## Convergence Findings

No hay hallazgos accionables.

### Brechas nuevas revisadas

- **T032 no introduce nada contradictorio**: el campo sigue siendo texto, el formulario envía lo
  escrito y `km` ya admitía texto en `tipos.ts`. El km anterior compara el valor ya convertido, así
  que «30.000» da el rechazo con «(39.800 km)» (captura `c03`).
- **Los textos no cambian**: el mensaje del km es el mismo en la tabla del contrato y en el código.
- **T033 y T034 solo añaden pruebas**; no tocan código de la aplicación.
- **Referencias a D9** al día en `plan.md` y `tasks.md`.

### Observaciones (no generan tarea)

Siguen las dos de converge-1, sin cambios:

- **`/ordenes/abc`** (id no numérico): error genérico en lugar de «No existe la orden», igual que
  las rutas de B con `{id}`. Si se quiere cambiar, es una decisión de la base común.
- **`backend/tests/spec006_pedidos.rs`**: el comentario de la prueba aún dice «quitar el `#[ignore]`
  cuando exista (T031)». Lo ve la Persona B en T031.

### Comprobado sin hallazgos

- **FR-001 a FR-008, US1-1 … US1-9, US2-1 … US2-3, SC-002, SC-003 y SC-004**: siguen cumpliéndose
  como en converge-1, con sus pruebas en verde y la verificación automatizada de T029.
- **Reglas de D1–D5 en el código**: comparación con `fechas::hoy()`, `FOR UPDATE` antes de mirar la
  orden existente, `contador::siguiente` después, `cita_id <> $2` en el km anterior, condición de no
  facturada en `retrasada`, y evento después del `commit`.
- **D6–D9**: composición de la ficha, `orden_numero`, parte 5 de `demo.sql` y control de fecha
  nativo.
- **Código sin justificación**: ninguno.
- **Constitución**: I, sin dependencias ni migraciones nuevas. II, textos en español; fechas, según
  D9, con la comprobación en T035. III, sin alcance sin justificar. IV, ver abajo. V, sin datos del
  cliente en la ficha ni en la lista, y sin secretos.

## Lo que este resultado no cubre

«Converged» significa que no queda trabajo sin tarea, no que spec-002 esté cerrada:

- **SC-001 (menos de 2 minutos) no se ha medido con una persona**: los tiempos registrados son de
  máquina. Hasta hacerlo, el principio IV no está cumplido del todo para ese criterio (T035).
- **El formato dd/mm/aaaa no se ha visto en un navegador en español**: las capturas van en inglés
  (T035).
- **Nada de lo compartido está revisado por la Persona B** (T031): `demo.sql`, `imputaciones.rs`,
  `spec006_pedidos.rs`, `tipos.ts`, `rutas/mod.rs`, `ordenes/[id]` y los documentos comunes, que
  incluyen el contrato con el punto de miles, el plan maestro y las secciones §3, §4, §5, §7 y §9
  del quickstart.
- Fuera de spec-002 siguen asignados I5 (spec-004 y spec-003) y N1 (plan de spec-008).

## Métricas

- **Requisitos y criterios revisados**: 8 FR, 4 SC, 12 escenarios y 7 casos límite.
- **Decisiones del plan revisadas**: research D1–D9, reglas clave del plan, pruebas 1–12 y tabla de
  validación del contrato común.
- **Correcciones verificadas**: 3 resueltas (T032, T033, T034) y 1 decidida con su verificación
  abierta (F3: D9 y T035).
- **Principios de la constitución revisados**: 5 (I–V).
- **Hallazgos por tipo**: missing 0 · partial 0 · contradicts 0 · unrequested 0.
- **Hallazgos por gravedad**: CRITICAL 0 · HIGH 0 · MEDIUM 0 · LOW 0.

## Resultado

✅ **Converged**: la implementación cumple la spec, el plan y las tareas.

## Siguiente paso

No hace falta otra pasada de `/speckit-implement`. Para cerrar spec-002 quedan T035 (comprobación
manual) y T031 (revisión de la Persona B). Después, la rama queda lista para el PR.
