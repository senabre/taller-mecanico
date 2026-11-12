# Seguimiento de hallazgos · spec-007 Facturar la orden y avisar al cliente (HU-07)

**Origen**: [analyze-1.md](analyze-1.md) (2026-11-12) · **Revisión**: [analyze-2.md](analyze-2.md)
(2026-11-12). Decisiones de la Persona B del 2026-11-12.

**Decisión**: *corregido* = se ha cambiado el código, el documento o la tarea; *justificado* = no se
cambia, con un motivo explícito.

**Estado**: *Cerrado (verificado en analyze-2)* = cambio aplicado y comprobado en un nuevo análisis;
*Cerrado (justificado)* = no se cambia, con su motivo.

## Hallazgos de analyze-1

| ID | Historia o criterio | Gravedad | Evidencia | Decisión | Responsable | Estado |
|----|---------------------|----------|-----------|----------|-------------|--------|
| C1 | Principio II · US1, US3 · T016, T024 · `Factura.svelte`, `/historial` · V2 de spec-008 | CRITICAL | «Facturar» confirmaba con `confirm()`, y el buscador del historial dejaba que el navegador validara la matrícula vacía; los dos salen en el idioma del navegador. | Corregido (2026-11-12, T033): confirmación en la página, con «Sí, facturar» y «Volver»; el historial lleva `novalidate` y, sin matrícula, dice «Escribe la matrícula». Rojo 1 de 4 (los 3 fallos, de C1), verde 4 de 4, también a 390 px ([T033](evidencias/T033/informe.md)). Con esto, ningún `confirm()` queda en la aplicación y se cierra el V2 de spec-008. | Persona B | Cerrado (verificado en analyze-2) |
| G1 | SC-001, SC-004 · principio IV · plan (fila IV) · T028 | MEDIUM | SC-001 y SC-004 no tienen paso de cronometraje y el plan decía que todos los SC lo tenían. | Justificado (2026-11-12), igual que en las otras specs de la Persona B: se acepta la medición con un navegador automático de la [matriz](../matriz-trazabilidad.md) (0,24 s frente a 30 s; 0,22 s frente a 10 s) como desviación. Corregida la fila IV del plan (T034). | Persona B | Cerrado (justificado) |
| I1 | D7 · T014, T017 · `factura_de_orden` | LOW | D7 decía que spec-002 llama a `factura_de_orden`, pero `GET /api/ordenes/{id}` usa su propia subconsulta, equivalente. | Corregido (2026-11-12, T034): nota en D7. La función se queda: la pide T014 y la prueba T012. | Persona B | Cerrado (verificado en analyze-2) |
| I2 | Plan (Project Structure) | LOW | «research.md # decisiones D1–D7», pero hay D8. | Corregido (2026-11-12, T034): «D1–D8». | Persona B | Cerrado (verificado en analyze-2) |

## Resumen

- **analyze-1**: 4 hallazgos (1 CRITICAL, 1 MEDIUM y 2 LOW). C1 se corrige en el código (T033), I1 e
  I2 en los documentos (T034) y G1 se justifica.
- **analyze-2**: verifica las correcciones; sin hallazgos nuevos en spec-007 (una observación sobre
  `/entrar`, de la base común, para la Persona A).
- **converge-1**: sin hallazgos ([converge-1.md](converge-1.md)).
