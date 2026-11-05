# Comprobación parcial de la página pública · spec-004 (T015, T022 y T026)

**Fecha**: 2026-11-04 · **Responsable**: Persona A · **Alcance**: parcial. **No sustituye a T027.**

## Cómo se hizo

Chrome sin interfaz, dirigido con Playwright, a 1280 px y a 390 px, con la aplicación compilada y una
base de datos temporal (`taller_verif`), con las migraciones y `backend/seed/demo.sql`. La salida
completa está en [salida.txt](salida.txt): 58 comprobaciones correctas y 0 fallos.

## Con datos reales

| Caso | Resultado | Capturas |
|---|---|---|
| Presupuesto aceptado | «Aceptado el … a las …», total en €, sin botones | `escritorio-aceptado.png`, `movil-aceptado.png` |
| Presupuesto rechazado | «Rechazado el … a las …», total en €, sin botones | `escritorio-rechazado.png`, `movil-rechazado.png` |
| Enlace inválido | Solo «Este enlace no es válido» | `escritorio-enlace-invalido.png`, `movil-enlace-invalido.png` |

En los tres casos y en los dos anchos: sin desplazamiento horizontal, sin el nombre, el teléfono ni el
correo del cliente, y sin el menú del personal.

## Simulado (no cuenta como verificación)

El estado vigente y el caducado necesitan el `GET` de T012: hoy un presupuesto sin responder enseña
«No se pudo cargar el presupuesto», por el 501 provisional. Para encontrar errores antes de tiempo se
sustituyeron las respuestas del servidor por otras inventadas (líneas «SIM» de la salida): preguntas de
confirmación, «Volver» sin llamar al servidor, sin `confirm()`, hora de Madrid, los dos 409, el aviso
de caducidad una sola vez y el error al responder. No hay capturas de esta parte.

## Pendiente

Tras T012: comprobar con el servidor real el estado vigente, la confirmación y el aviso de caducidad,
y marcar T015, T022 y T026. El recorrido completo es T027.
