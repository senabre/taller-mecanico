# Taller Mecánico Constitution

Taller Mecánico es una herramienta web para gestionar un taller mecánico de vehículos.
Estas reglas mandan sobre cualquier otra práctica del proyecto.

## Core Principles

### I. Simplicidad ante todo

- Entre dos soluciones que cumplen la spec, se elige SIEMPRE la más simple.
- Es una versión 1: PROHIBIDO añadir capas, abstracciones, configuraciones o dependencias
  "por si acaso".
- Cualquier complejidad extra DEBE justificarse por escrito en el plan, explicando por qué
  la opción simple no sirve.

Motivo: una v1 simple se entrega antes, se entiende mejor y se cambia con menos riesgo.

### II. Español de España y euro

- Todo el producto visible para el usuario (textos, mensajes, errores, correos) DEBE estar
  en español de España.
- Si se muestran importes, la moneda es el euro (€) con el formato español (p. ej. 1.234,56 €).
- Las fechas siguen el formato español (dd/mm/aaaa).

### III. Cero alcance fantasma

- NO se implementa ninguna funcionalidad que no esté escrita en la spec.
- Si surge una idea nueva durante el trabajo, se apunta y se propone; no se construye
  hasta que se añada a la spec.

Motivo: lo que no está pedido no se puede verificar y solo añade mantenimiento.

### IV. Verificable por una persona no técnica

- Cada criterio de éxito de una spec DEBE poder comprobarse usando la aplicación, sin leer
  código, consultar la base de datos ni usar herramientas técnicas.
- Un criterio que no cumpla esto se reescribe antes de planificar.

### V. Respeto por los datos del usuario

- Solo se piden los datos imprescindibles para la tarea; cada dato pedido DEBE tener un uso
  descrito en la spec.
- PROHIBIDO escribir claves, contraseñas, tokens u otros secretos en el código o en el
  repositorio. Se leen de la configuración del entorno.
- Única excepción: credenciales y tokens **ficticios de demostración**, documentados como tales,
  y solo en `backend/seed/demo.sql` y en `quickstart.md`, más los valores de ejemplo de
  `.env.example`. Nunca se cargan ni se usan en producción: cada entorno pone los suyos en su
  `.env`, que no se versiona.
- Los documentos de `specs/` pueden **citar** los valores demo que ya publica `quickstart.md`. El
  código, incluidas las pruebas automáticas, nunca los contiene.

## Restricciones del producto

- Producto: aplicación web para la gestión de un taller mecánico, versión 1.
- Idioma único: español de España. No hay soporte multiidioma salvo que la spec lo pida.
- Moneda única: euro.

## Flujo de trabajo

1. Toda funcionalidad empieza con una spec (`/speckit-specify`).
2. El plan (`/speckit-plan`) DEBE comprobar los cinco principios antes de diseñar y
   justificar cualquier complejidad.
3. Una funcionalidad está terminada cuando una persona no técnica comprueba en la app
   todos sus criterios de éxito.

## Governance

- Esta constitución prevalece sobre cualquier otra guía del proyecto.
- Cambiarla requiere editar este archivo, explicar el motivo y actualizar la versión:
  - MAJOR: se elimina o redefine un principio.
  - MINOR: se añade un principio o sección, o se amplía de forma relevante.
  - PATCH: aclaraciones y correcciones de redacción.
- Cada spec, plan y revisión DEBE verificar que cumple estos principios.

**Version**: 1.2.1 | **Ratified**: 2026-10-15 | **Last Amended**: 2026-10-15

**Cambios**:
- 1.2.1 (2026-10-15) · Principio V (aclaración): los documentos de `specs/` pueden citar los
  valores demo publicados en `quickstart.md`; el código y las pruebas nunca los contienen. Motivo:
  los análisis y las tareas tienen que poder nombrar esos datos (analyze-2, K2).
- 1.2.0 (2026-10-15) · Principio V: se permiten credenciales y tokens ficticios de demostración,
  solo en `backend/seed/demo.sql`, `quickstart.md` y los valores de ejemplo de `.env.example`, y
  nunca en producción. Motivo: los datos
  demo permiten probar cada spec y el quickstart sin dar de alta usuarios a mano
  (analyze-1, K1).
