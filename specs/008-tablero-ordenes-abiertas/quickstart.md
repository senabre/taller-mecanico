# Quickstart: validar spec-008 · Tablero de órdenes abiertas

La guía de validación es **única**: los pasos están en
[000-base-comun/quickstart.md § §8 spec-008 · Tablero](../000-base-comun/quickstart.md#8-spec-008--tablero)
y, para el recorrido completo con un vehículo nuevo, en el
[§9 Integración](../000-base-comun/quickstart.md#9-integración-los-dos-juntos),
y el arranque y los usuarios demo están en su apartado «Arrancar». Aquí solo se indica qué
escenario de la [spec](spec.md) comprueba cada paso, para que quien valide sepa que no se ha quedado
ninguno sin probar.

## Antes de empezar

- Datos demo cargados **el mismo día** del recorrido, con el cambio de [research](research.md) D8:
  la orden 3 (`3333DDD`) tiene la entrega hoy. Sin ese cambio, los pasos 4, 5 y 9 no se pueden
  comprobar como están escritos.
- Se recorre después del §7, que factura la orden de `1313MMM`. Si se recorre con los datos recién
  cargados, la carga del paso 3 es «Luis 5 · Marta 3» y el paso 2 no tiene `1313MMM`.
- Dos navegadores, o una ventana normal y otra privada: uno con `paco` en `/tablero` y otro para los
  cambios.
- El paso 9 **no necesita herramientas**: se para el servidor (Ctrl+C en `cargo run`), se vuelve a
  arrancar y después se termina `3333DDD` con el tablero conectado. La parte opcional con la red
  cortada en las herramientas de desarrollo se quitó (V1 de [hallazgos.md](hallazgos.md)): no corta
  una conexión de eventos ya abierta. El paso 10 se hace en un móvil de la misma red (C1 de
  [analyze-1](analyze-1.md)).
- El §9 se hace en dos sesiones: el día A, al terminar el §8, se pide la cita de `3579KLM` para el
  primer día que ofrezca la página; el día B, el de la cita, **sin recargar los datos demo** (es
  normal que alguna orden demo salga «Retrasada»), se recorren los pasos 2 a 8.
- Los pasos 4 y 6 se **cronometran**: hace falta una persona con un reloj (SC-001 y SC-002), y para
  el 4, alguien que no haya visto antes el tablero ([research](research.md) D9).

## Correspondencia de escenarios y pasos

| Escenario o caso límite de la spec | Paso del quickstart §8 |
|---|---|
| US1-1 · las 6 columnas y cada orden en la suya | 2 |
| US1-2 · una facturada no aparece | 2, 8 |
| US1-3 · otros roles: sin menú y «No tienes permiso para hacer esto» | 1 (y §0.3–0.4) |
| US1-4 · orden de las tarjetas | 5 |
| US2-1 · «Luis N · Marta N», activos con 0 | 3 (el 0 de un activo, en las pruebas automáticas) |
| US2-2 · mecánico inactivo con órdenes, «(de baja)» | pruebas automáticas: con datos demo no se puede ([research](research.md) D8) |
| US3-1 · entrega ayer → «Retrasada» | 4 |
| US3-2 · entrega hoy → no retrasada | 4, 5 |
| US4-1 · un cambio de estado mueve la tarjeta | 6 |
| US4-2 · una orden nueva aparece en «Recibida» | 7 |
| US4-3 · al facturar, la tarjeta desaparece | 8 |
| US4-4 · medianoche de Madrid | 11 (`medianoche.test.ts`) |
| US4-5 · al volver la conexión, el tablero entero | 9 (servidor parado y arrancado: el aviso se va y el tablero se vuelve a pedir solo). Que una orden que cambió durante el corte aparezca al volver, en `eventos.test.ts` (cada reapertura llama a `alAbrir`, que pide el tablero entero): a mano no se puede cambiar una orden con el servidor parado |
| Caso límite · estado sin órdenes: columna vacía | pruebas automáticas (con datos demo ninguna queda vacía) |
| Caso límite · terminada sin facturar y con la entrega pasada | 4 (`8888KKK`) |
| Caso límite · muchas órdenes en una columna | 10 (desplazamiento vertical propio de cada columna) |
| Caso límite · se pierde la conexión | 9; la conexión cerrada (un 401, por ejemplo), en `eventos.test.ts`. Que la conexión se abra también tras una primera carga fallida (N2) se comprueba en el código de la página (T025), porque a mano no se puede provocar: la página la sirve el mismo servidor |
| Caso límite · pasa la medianoche | 11 |
| SC-001 · retrasadas en menos de 10 s | 4 (cronometrado) |
| SC-002 · un cambio en menos de 5 s | 6 (cronometrado), 7, 8 |
| SC-003 · la carga coincide con las tarjetas | 3, 7, 8 |
| SC-004 · todas las abiertas y ninguna facturada | 2 |
| Pantalla grande y móvil a 390 px | 10 |
| §9 · US4-1, US4-2 y US4-3 con una orden nueva: aparece, cambia de columna en cada paso y desaparece al facturar | §9, pasos 3 a 7 (día B) |
| §9 · SC-002 durante todo el día B | §9, pasos 3 a 7 |
| §9 · SC-003 con datos desfasados | §9, paso 2 |

## Pruebas automáticas

- `cd backend && cargo test --test spec008_tablero`: la lista de pruebas está en [plan.md](plan.md).
- `cd backend && cargo test --test spec008_tablero -- --ignored`: la prueba 9 (peticiones cortadas a
  mitad, hallazgo V3), que tarda unos 100 s y por eso la suite normal la salta. Se lanza aparte.
- `cd frontend && npm test`, con reloj simulado y sin dependencias nuevas:
  - `medianoche.test.ts`: la medianoche de Madrid, también con el proceso en otra zona horaria y en
    los días de cambio de hora ([research](research.md) D6);
  - `recarga.test.ts`: una sola petición del tablero a la vez, más una si llegan avisos mientras
    tanto (D5);
  - `eventos.test.ts`: la conexión de eventos con un `EventSource` simulado, incluida la conexión
    nueva tras quedar cerrada (D5, E1 y N3).

Lo que una persona no puede hacer a mano (esperar a la medianoche, tener un mecánico de baja con
órdenes, una columna vacía con los datos demo o una conexión que el navegador da por cerrada) solo
se comprueba aquí.
