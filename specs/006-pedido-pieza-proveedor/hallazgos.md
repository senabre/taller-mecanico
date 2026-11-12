# Seguimiento de hallazgos · spec-006 Pedir al proveedor una pieza sin stock (HU-06)

**Origen**: [analyze-1.md](analyze-1.md) (2026-11-12) · **Revisión**: [analyze-2.md](analyze-2.md)
(2026-11-12). Decisiones de la Persona B del 2026-11-12.

**Decisión**: *corregido* = se ha cambiado el código, el documento o la tarea; *justificado* = no se
cambia, con un motivo explícito.

**Estado**: *Cerrado (verificado en analyze-2)* = cambio aplicado y comprobado en un nuevo análisis;
*Cerrado (justificado)* = no se cambia, con su motivo.

## Hallazgos de analyze-1

| ID | Historia o criterio | Gravedad | Evidencia | Decisión | Responsable | Estado |
|----|---------------------|----------|-----------|----------|-------------|--------|
| C1 | Principio II · FR-001, FR-003 y casos límite · T013, T019 · `/almacen`, `/pedidos` | CRITICAL | Los dos formularios dejaban que validara el navegador (`required`, `type="number"` con `min` y `step`), así que la cantidad 0, el stock negativo o los campos vacíos se rechazaban con el mensaje del navegador y en su idioma, sin llegar al servidor. | Corregido (2026-11-12, T033): `novalidate` y campos de texto que viajan como número si son enteros; los mensajes son los del servidor, en español. Rojo 3 de 7 (los 4 fallos, de C1), verde 7 de 7, también a 390 px ([T033](evidencias/T033/informe.md)). | Persona B | Cerrado (verificado en analyze-2) |
| G1 | SC-001 · principio IV · plan (fila IV) · T032 | MEDIUM | SC-001 no tiene paso de cronometraje y el plan decía que todos los SC lo tenían. | Justificado (2026-11-12), igual que en spec-003 y spec-005: se acepta la medición con un navegador automático de la [matriz](../matriz-trazabilidad.md) (0,56 s frente a 1 minuto) como desviación. Corregida la fila IV del plan (T034). | Persona B | Cerrado (justificado) |
| G2 | US3-1, SC-003 · T032 | LOW | T032 decía que la vuelta de la orden a «en reparación» se comprobó en la base de datos. | Corregido (2026-11-12, T034): nota en T032 con las capturas de la regresión T030 de spec-008, donde se ve en la ficha. | Persona B | Cerrado (verificado en analyze-2) |
| I1 | US1, FR-001, FR-002, SC-006, Key Entities | LOW | «Código» de pieza en la spec frente a «referencia» en el resto y en el mensaje. | Corregido (2026-11-12, T034): «referencia» en la spec. | Persona B | Cerrado (verificado en analyze-2) |

## Resumen

- **analyze-1**: 4 hallazgos (1 CRITICAL, 1 MEDIUM y 2 LOW). C1 se corrige en el código (T033), G2 e
  I1 en los documentos (T034) y G1 se justifica.
- **analyze-2**: verifica las correcciones; sin hallazgos nuevos.
- **converge-1**: sin hallazgos ([converge-1.md](converge-1.md)).
