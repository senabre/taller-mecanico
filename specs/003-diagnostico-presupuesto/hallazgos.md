# Seguimiento de hallazgos · spec-003 Registrar diagnóstico y preparar presupuesto (HU-03)

**Origen**: [analyze-1.md](analyze-1.md) (2026-11-12) · **Revisión**: [analyze-2.md](analyze-2.md)
(2026-11-12). Decisiones de la Persona B del 2026-11-12.

**Decisión**: *corregido* = se ha cambiado el código, el documento o la tarea; *justificado* = no se
cambia, con un motivo explícito; *registrado* = se resuelve fuera de esta spec.

**Estado**: *Cerrado (corregido; pendiente de verificar en analyze-2)* = cambio aplicado y todavía sin
comprobar en un nuevo análisis; *Cerrado (verificado en analyze-2)* = analyze-2 ha comprobado el
cambio; *Cerrado (corregido)* = cambio posterior a analyze-2, sin un nuevo análisis; *Cerrado
(justificado)* = no se cambia, con su motivo.

## Hallazgos de analyze-1

| ID | Historia o criterio | Gravedad | Evidencia | Decisión | Responsable | Estado |
|----|---------------------|----------|-----------|----------|-------------|--------|
| C1 | Principio II · US2, US3 · T030 · `Presupuesto.svelte`, `FormularioLinea.svelte` · quickstart común §3.4 · V2 de spec-008 | CRITICAL | «Emitir presupuesto» confirmaba con `confirm()`, cuyos botones salen en el idioma del navegador, y el formulario de líneas dejaba que el navegador validara (unidades con `type="number"`, campos `required`) con sus propios mensajes; en la regresión T030 de spec-008 salieron en inglés. | Corregido (2026-11-12, T036): confirmación dentro de la página, con «Sí, emitir» y «Volver», como spec-004 D5; el formulario lleva `novalidate` y las unidades van en un campo de texto, así que los mensajes son los del servidor, en español. Comprobado en el navegador: rojo 0 de 3 y verde 4 de 4, también a 390 px ([T036](evidencias/T036/informe.md)). `FormularioLinea` lo usa también spec-005 (imputaciones), que se beneficia del mismo cambio. Los `confirm()` de «Terminar reparación» (spec-005) y «Facturar» (spec-007) se corrigen en sus specs; al terminar, se cierra el V2 de spec-008. | Persona B | Cerrado (verificado en analyze-2) |
| G1 | SC-001 · principio IV · plan (Constitution Check, fila IV) · quickstart común §3 · T035 | MEDIUM | SC-001 (5 líneas en menos de 5 minutos) no tiene un paso de cronometraje en la guía ni una tarea que lo mida, y el plan decía que cada SC tenía su paso. | Justificado (2026-11-12): se acepta la medición con un navegador automático de la [matriz](../matriz-trazabilidad.md) (5 líneas en 1,02 s, con los campos y clics que necesita una persona) como desviación: el margen frente a 5 minutos es muy grande. Corregida la fila IV del plan para que lo diga (T037). | Persona B | Cerrado (justificado) |
| I1 | FR-001a, D8 · plan (dependencias) · T006, T029 | LOW | `PresupuestoResumen` se describía sin `respondido_en`, que añadió spec-004 T014, y el plan decía que la Persona A no modifica `presupuestos.rs`, aunque lo hizo con su acuerdo. | Corregido (2026-11-12, T037): nota en research D8 y excepción acordada en el plan. Las tareas ya hechas (T006, T029) no se reescriben. | Persona B | Cerrado (verificado en analyze-2) |
| I2 | SC-003 · T027, T029 | LOW | T027 llamaba a `presupuestos_de_orden` con dos argumentos; T029, D8 y el código llevan tres. | Corregido (2026-11-12, T037): T027 con `iva_porcentaje`. | Persona B | Cerrado (verificado en analyze-2) |
| I3 | Key Entities de la spec | LOW | «Código» de pieza en la spec frente a «referencia» en el resto. | Corregido (2026-11-12, T037): «referencia». | Persona B | Cerrado (verificado en analyze-2) |
| G2 | US2-5, US3-2 · quickstart común §3.5 | LOW | El §3.5 quita una línea «Prueba» en lugar de la mano de obra de US2-5, y US3-2 no tiene paso. | Justificado (2026-11-12): el §3.5 comprueba la misma regla (quitar una línea recalcula los totales) con otra línea, y US3-2 se ve en pantalla porque sin líneas no aparece «Emitir presupuesto»; las dos reglas las cubren `quitar_una_linea_recalcula_los_totales` y `no_se_emite_sin_lineas_ni_por_otro_mecanico`. No se cambia el quickstart común. | Persona B | Cerrado (justificado) |

## Hallazgos nuevos de analyze-2

| ID | Historia o criterio | Gravedad | Evidencia | Decisión | Responsable | Estado |
|----|---------------------|----------|-----------|----------|-------------|--------|
| N1 | US2 · T024 · `FormularioLinea.svelte` | LOW | T024 describía horas y unidades con `step`, y las dos son campos de texto (las unidades, desde T036). | Corregido (2026-11-12): nota en T024. | Persona B | Cerrado (corregido) |
| N2 | US3 · quickstart común §3.6 | LOW | El paso 6 no mencionaba la confirmación en la página. | Corregido (2026-11-12): «Emitir y confirmar con «Sí, emitir» →». Cambio en un documento común (su sección de spec-003): lo revisa también la Persona A. | Persona B | Cerrado (corregido) |

## Resumen

- **analyze-1**: 6 hallazgos (1 CRITICAL, 1 MEDIUM y 4 LOW). C1 se corrige en el código (T036), I1, I2
  e I3 en los documentos (T037), y G1 y G2 se justifican.
- **analyze-2**: verifica las correcciones de analyze-1 y encuentra 2 hallazgos nuevos LOW (N1 y N2),
  corregidos en la documentación. Sin CRITICAL ni HIGH.
- **converge-1**: 2 hallazgos en el código (F1 CRITICAL, el `required` del diagnóstico sin
  `novalidate`; F2 MEDIUM, el diagnóstico editable con un presupuesto emitido, que era el V2 de
  spec-004), resueltos en T038 y T039. **converge-2**: sin hallazgos ([converge-2.md](converge-2.md)).
