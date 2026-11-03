# Quickstart: validar spec-002 · Recibir el vehículo y abrir la orden de reparación

La guía de validación es **única**: los pasos están en
[000-base-comun/quickstart.md § §2 spec-002 · Abrir orden](../000-base-comun/quickstart.md#2-spec-002--abrir-orden),
y el arranque, los usuarios demo y las citas demo de hoy están en su apartado «Arrancar». Aquí
solo se indica qué escenario de la [spec](spec.md) comprueba cada paso, para que quien valide sepa
que no se ha quedado ninguno sin probar.

## Antes de empezar

- Base de datos recién cargada con `seed/demo.sql` **el mismo día** del recorrido: las citas demo
  de `1313MMM`, `1414NNN` y `8888KKK` son de hoy, y la de `1515PPP` es de ayer y no tiene orden
  ([research](research.md) D8).
- Se recorre después del §1, que deja citas de días futuros (1234BCD y 5678CDF).
- La primera orden nueva es la **Nº 10**: los datos demo tienen las órdenes 1 a 9.
- `pedro` es un mecánico de baja; `luis` y `marta`, los mecánicos activos.

## Correspondencia de escenarios y pasos

| Escenario o caso límite de la spec | Paso del quickstart §2 |
|---|---|
| US1-1 · la orden se guarda en «Recibida» con km, daños, mecánico y entrega | 4 |
| US1-2 · número siguiente al último | 4 (Nº 10 tras las 9 demo), 5 (Nº 11) |
| US1-3 · la orden muestra su cita, su vehículo y su mecánico | 4 |
| US1-4 · cita con orden: «Orden Nº N» en la lista y 409 en una segunda pestaña | 4, 5 |
| US1-5 · cita de otro día sin «Abrir orden» | 1 (día de 1234BCD y ayer con `1515PPP`) |
| US1-6 · cliente sin cita | 1 (no está en la lista; la página pública ofrece días desde mañana, §1) |
| US1-7 · solo mecánicos activos | 2 (Pedro no aparece); la baja con el formulario abierto, en las pruebas automáticas |
| US1-8 · daños obligatorios | 3, 5 («Sin daños visibles») |
| US1-9 · números consecutivos con intentos rechazados entre medias | 3, 4 y 5 (errores, Nº 10, segunda pestaña, Nº 11) |
| US2-1 · km menor que el anterior, con el valor previo | 6 (`8888KKK`, 39.800 km) |
| US2-2 · km igual al anterior | pruebas automáticas (`spec002_ordenes.rs`) |
| US2-3 · vehículo sin visitas anteriores | 4 (`1313MMM`), 5 (`1414NNN`) |
| Caso límite · km vacío o negativo, sin consumir número | 3, 4 (la orden sigue siendo la Nº 10) |
| Caso límite · entrega anterior a hoy | 3 |
| Caso límite · dos recepcionistas a la vez | pruebas automáticas; en la app, la numeración consecutiva de 4 y 5 |
| SC-001 · abrir una orden en menos de 2 minutos | 4 (cronometrado) |
| SC-002 · órdenes seguidas con números consecutivos, sin huecos ni repetidos, aunque haya intentos rechazados (Q6) | 3, 4, 5 y 6 («Orden Nº 10» y «Orden Nº 11» en «Citas»); la apertura simultánea (FR-002), en las pruebas automáticas |
| SC-003 · todo km menor se rechaza con el valor previo | 6 |
| SC-004 · toda orden muestra su cita, su vehículo y su mecánico | 4, 7 |

## Pruebas automáticas

`cd backend && cargo test --test spec002_ordenes` (la lista de pruebas está en [plan.md](plan.md)).
Lo que una persona no puede hacer a mano a la vez (dos órdenes para la misma cita en el mismo
instante, diez órdenes seguidas en paralelo) y la baja de un mecánico con el formulario abierto
solo se comprueban aquí. `cargo test --test spec006_pedidos` comprueba además que la ficha trae los
pedidos de la orden.
