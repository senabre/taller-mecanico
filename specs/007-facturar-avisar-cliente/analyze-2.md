# Analyze 2 · spec-007 · Facturar la orden y avisar al cliente

**Fecha**: 2026-11-12 · **Rama**: `spec-007` (sobre `076f626`, con T033 y T034 sin commit todavía) ·
**Artefactos**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md) y
[tasks.md](tasks.md) (T001–T034) · **Contexto**: el mismo que [analyze-1](analyze-1.md) ·
**Constitución**: 1.2.1.

Análisis de solo lectura. Comprueba las correcciones de [analyze-1](analyze-1.md) y repite los mismos
controles sobre los documentos cambiados.

## Verificación de analyze-1

| ID | Comprobación | Resultado |
|----|--------------|-----------|
| C1 | `Factura.svelte` ya no llama a `confirm()` («Sí, facturar» y «Volver» en la página) y `/historial` lleva `novalidate` con su propio mensaje para la matrícula vacía. En el navegador, verde 4 de 4, también a 390 px ([T033](evidencias/T033/informe.md)). En toda la interfaz ya no queda ningún `confirm()`. T016, T024, research D7 y el §7.2 de la guía lo dicen. | Verificado |
| G1 | La fila IV del plan deja SC-001 y SC-004 como desviación aceptada, enlazando a hallazgos. | Verificado (justificado) |
| I1 | Research D7 explica que `GET /api/ordenes/{id}` usa su propia subconsulta y que la función se queda por T014 y T012. | Verificado |
| I2 | El plan dice «D1–D8». | Verificado |

## Specification Analysis Report (hallazgos nuevos)

Ninguno en spec-007.

**Observación fuera de spec-007**: el formulario de `/entrar` (base común) es el único de la interfaz
que sigue sin `novalidate`; con el usuario o la contraseña vacíos, el mensaje lo pone el navegador. No
es de esta spec; se comenta a la Persona A para la base común.

### Metrics

- **Total Requirements**: 13 (9 FR + 4 SC) · **Total Tasks**: 34
- **Coverage**: 100 % (SC-001 y SC-004 con desviación aceptada, G1)
- **Ambiguity Count**: 0 · **Duplication Count**: 0 · **Critical Issues Count**: 0

## Next Actions

Lanzar `/speckit-converge`.
