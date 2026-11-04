# Quickstart: validar spec-004 · Aceptar o rechazar el presupuesto desde un enlace

La guía de validación es **única**: los pasos están en
[000-base-comun/quickstart.md § §4 spec-004 · Respuesta del cliente](../000-base-comun/quickstart.md#4-spec-004--respuesta-del-cliente),
y el arranque, los usuarios demo y los presupuestos demo están en su apartado «Arrancar». Aquí solo
se indica qué escenario de la [spec](spec.md) comprueba cada paso, para que quien valide sepa que no
se ha quedado ninguno sin probar.

## Antes de empezar

- Base de datos recién cargada con `seed/demo.sql` **el mismo día** del recorrido. Incluye la
  ampliación caducada de la orden 7, con el enlace `/presupuesto/demo-presupuesto-caducado`
  ([research](research.md) D7).
- Se recorre después del §3, que emite el presupuesto de la orden de `1313MMM`. Los correos se ven
  en Mailpit (`http://localhost:8025`).
- El §4.2 se **cronometra**: hace falta una persona con un reloj (SC-001).

## Correspondencia de escenarios y pasos

| Escenario o caso límite de la spec | Paso del quickstart §4 |
|---|---|
| US1-1 · el enlace muestra el presupuesto completo | 1 |
| US1-2 · aceptar: fecha y hora en la orden, y «En reparación» | 2, 3 |
| US1-3 · volver a abrir el enlace: la decisión, sin botones | 2 |
| US1-4 · dos pestañas: solo cuenta la primera | 2; a la vez de verdad, en las pruebas automáticas |
| US1-5 · el enlace en la orden, solo mientras está vigente | 3, 4 |
| US2-1 · rechazar: la orden sigue «En diagnóstico» | 5 |
| US2-2 · rechazar una ampliación: sigue «Pendiente de conformidad» | 6 |
| US2-3 · la ficha muestra todas las respuestas | 6 |
| US3-1 · presupuesto caducado: aviso, sin botones | 7 |
| US3-2 · caduca con la página abierta | pruebas automáticas (`spec004_respuesta.rs`) |
| US4-1 · sin presupuesto aceptado no se imputa | 4 |
| Caso límite · enlace modificado a mano | 8 |
| Caso límite · el correo no llega | 4 |
| FR-002 y principio V · sin datos del cliente | 1 |
| SC-001 · responder en menos de 1 minuto | 2 (cronometrado) |
| SC-002 · las respuestas se ven en la orden | 3, 6 |
| SC-003 · ningún caducado se acepta | 7 |
| SC-004 · sin presupuesto aceptado no se imputa | 4 |
| Móvil · `/presupuesto/[token]` a 390 px | 1 |

## Pruebas automáticas

`cd backend && cargo test --test spec004_respuesta` (la lista de pruebas está en [plan.md](plan.md)).
Lo que una persona no puede hacer a mano a la vez, dos respuestas en el mismo instante, y que un
presupuesto caduque con la página abierta solo se comprueban aquí.
