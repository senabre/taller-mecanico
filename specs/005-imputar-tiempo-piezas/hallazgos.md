# Seguimiento de hallazgos · spec-005 Imputar tiempo y piezas durante la reparación (HU-05)

**Origen**: [analyze-1.md](analyze-1.md) (2026-11-12) · **Revisión**: [analyze-2.md](analyze-2.md)
(2026-11-12). Decisiones de la Persona B del 2026-11-12.

**Decisión**: *corregido* = se ha cambiado el código, el documento o la tarea; *justificado* = no se
cambia, con un motivo explícito.

**Estado**: *Cerrado (verificado en analyze-2)* = cambio aplicado y comprobado en un nuevo análisis;
*Cerrado (corregido)* = cambio posterior a analyze-2, sin un nuevo análisis; *Cerrado (justificado)*
= no se cambia, con su motivo; *Registrado* = lo resuelve otra persona en su spec.

## Hallazgos de analyze-1

| ID | Historia o criterio | Gravedad | Evidencia | Decisión | Responsable | Estado |
|----|---------------------|----------|-----------|----------|-------------|--------|
| C1 | Principio II · US4 · T029 · `Imputaciones.svelte` · V2 de spec-008 | CRITICAL | «Terminar reparación» confirmaba con `confirm()`, cuyos botones salen en el idioma del navegador. | Corregido (2026-11-12, T035): confirmación en la página, con «Sí, terminar» y «Volver», como spec-003 T036 y spec-004 D5. Rojo 0 de 3, verde 3 de 3, también a 390 px ([T035](evidencias/T035/informe.md)). El formulario de imputación ya llegó corregido de spec-003 (T036). | Persona B | Cerrado (verificado en analyze-2) |
| U1 | FR-008, US4-2 · T029 · quickstart común §5.1 | MEDIUM | Sin imputaciones, la pantalla escondía «Terminar reparación» sin decir por qué, y el §5.1 solo se podía comprobar con la API (principio IV). | Corregido (2026-11-12, T035): en lugar del botón se ve «Primero hay que imputar el trabajo realizado.»; el §5.1 de la guía dice lo que se ve. | Persona B | Cerrado (verificado en analyze-2) |
| G1 | SC-003 · principio IV · plan (fila IV) · T034 | MEDIUM | SC-003 no tiene paso de cronometraje y el plan decía que todos los SC lo tenían. | Justificado (2026-11-12), igual que G1 de spec-003: se acepta la medición con un navegador automático de la [matriz](../matriz-trazabilidad.md) (0,24 s frente a 30 s) como desviación. Corregida la fila IV del plan (T036). | Persona B | Cerrado (justificado) |
| G2 | US3-1, FR-007 · T034 | LOW | T034 decía que la aceptación de la ampliación se simuló por SQL; ya se ha hecho desde el enlace del correo en la regresión T030 de spec-008. | Corregido (2026-11-12, T036): nota en T034 con las capturas de la regresión. | Persona B | Cerrado (verificado en analyze-2) |
| I1 | Key Entities de la spec | LOW | «tipo (horas o pieza)» frente a «mano de obra» en el resto. | Corregido (2026-11-12, T036). | Persona B | Cerrado (verificado en analyze-2) |

## Hallazgos nuevos de analyze-2

| ID | Historia o criterio | Gravedad | Evidencia | Decisión | Responsable | Estado |
|----|---------------------|----------|-----------|----------|-------------|--------|
| N1 | Quickstart común §8.6 (spec-008) y §9.6 (común) | LOW | Los pasos dicen «pulsar «Terminar reparación» →» sin la confirmación, que ahora es «Sí, terminar» en la página (antes, el `confirm()`). El tiempo de SC-002 de spec-008 no cambia. | Registrado (2026-11-12): el §8 es de la Persona A; se le avisa para que añada «y «Sí, terminar»» en el §8.6 y en el §9.6. | Persona A | Registrado |

## Resumen

- **analyze-1**: 5 hallazgos (1 CRITICAL, 2 MEDIUM y 2 LOW). C1 y U1 se corrigen en el código (T035),
  G2 e I1 en los documentos (T036) y G1 se justifica.
- **analyze-2**: verifica las correcciones; 1 hallazgo nuevo LOW (N1), registrado para la Persona A.
- **converge-1**: sin hallazgos ([converge-1.md](converge-1.md)).
