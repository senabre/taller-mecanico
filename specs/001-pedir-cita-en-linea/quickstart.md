# Quickstart: validar spec-001 · Pedir cita en línea

La guía de validación es **única**: los pasos están en
[000-base-comun/quickstart.md § §1 spec-001 · Pedir cita](../000-base-comun/quickstart.md#1-spec-001--pedir-cita),
y el arranque, los usuarios demo y la configuración (cupo diario **2**) están en su apartado
«Arrancar». Aquí solo se indica qué escenario de la [spec](spec.md) comprueba cada paso, para que
quien valide sepa que no se ha quedado ninguno sin probar.

## Antes de empezar

- Base de datos recién creada con `seed/demo.sql`. Las citas demo son de días pasados, así que no
  gastan el cupo y los vehículos demo no tienen ninguna cita pendiente.
- Ninguna matrícula demo es `1234BCD` ni `5678CDF`: el recorrido las da de alta como nuevas.

## Correspondencia de escenarios y pasos

| Escenario de la spec | Paso del quickstart §1 |
|---|---|
| US1-1 · matrícula conocida muestra marca y modelo | 6 |
| US1-2 · confirmación con fecha dd/mm/aaaa y motivo | 3, 6 |
| US1-3 · la lista muestra matrícula, marca, modelo, motivo, nombre y teléfono, sin correo | 7 |
| US1-4 · aviso exacto de cita pendiente, sin fecha y sin poder confirmar | 4 |
| US1-5 · matrícula inválida (`12#4`, `A`, 11 caracteres) y límites válidos | 1 |
| US1-6 · nueva cita cuando la anterior ya pasó | 6 (`1111BBB` tiene una cita de ayer) |
| US1-7 · vehículo conocido: solo día y motivo; el propietario no cambia | 6, 7 |
| US1-8 · lista de hoy por defecto, otro día a elegir, orden en que se pidieron | 7, 9 (`paco`) |
| US1-9 · día sin citas | 8 |
| US1-10 · otros roles no ven la lista | 9 |
| US2-1 · matrícula nueva pide los cinco datos | 2 |
| US2-2 · alta de vehículo y cita | 3 |
| US2-3 · teléfono o correo incorrectos | 3 |
| US2-4 · teléfonos y correo válidos | 3, 5 |
| US3-1 · un día completo deja de ofrecerse | 5 |
| US3-2 · dos clientes a la vez por la última plaza | pruebas automáticas (`spec001_citas.rs`) |

## Pruebas automáticas

`cd backend && cargo test --test spec001_citas` y `cargo test matricula` (la lista de pruebas está
en [plan.md](plan.md)). Las dos cosas que una persona no puede hacer a mano a la vez, la última
plaza y la misma matrícula para dos días, solo se comprueban aquí.
