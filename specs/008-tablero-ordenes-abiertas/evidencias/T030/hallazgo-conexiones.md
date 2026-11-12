# Hallazgo de la regresión (T030): `GET /api/tablero` deja conexiones de la base dentro de una transacción

- **Fecha**: 12/11/2026. Rama `spec-008` en `c01fb8e`. Encontrado por la Persona A al empezar T030.
- **Gravedad propuesta**: HIGH. Afecta a toda la aplicación, no solo al tablero.

## Qué pasó

En el primer intento de la regresión, a los 30 s de empezar el §1, el servidor empezó a responder 500 en peticiones que no tienen nada que ver con el tablero (entrar, ver citas…). En el registro del servidor:

```
error returned from database: cannot execute UPDATE in a read-only transaction
error returned from database: current transaction is aborted, commands ignored until end of transaction block   (×9, durante 3 minutos)
```

El §1 se quedó en «§1.9 paco» y todo lo de después falló (`regresion-primer-intento-*.log`). Justo antes, `paco` había entrado: su página de inicio es el tablero, y el script se fue a otra pantalla mientras el tablero aún cargaba.

## Causa

Cuando el navegador corta una petición a `/api/tablero` a mitad (al cambiar de pantalla, al cerrar la pestaña, al reconectar los eventos…), Axum cancela el manejador **mientras tiene abierta la transacción** `BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY` (research D1). A veces esa conexión vuelve a la reserva de conexiones (el *pool*) **todavía dentro de la transacción**. La siguiente petición que la recibe hace su `UPDATE` (por ejemplo, alargar la sesión) dentro de una transacción de solo lectura: falla, la transacción queda abortada y la conexión sigue fallando hasta que se descarta.

## Cómo se reproduce

`estres.sh`: entra como `paco` y lanza 200 peticiones a `/api/tablero` que se cortan a los 0,5–4,5 ms; después cuenta las conexiones que quedan en transacción, entra 10 veces como `ana` y cuenta los errores del servidor. Resultados (3 tandas cada uno, base recién cargada):

| Versión de `tablero.rs` | Errores del servidor | Conexiones «idle in transaction» |
|---|---|---|
| La actual (`begin_with` … `READ ONLY`) | 18 en la 2.ª tanda (`estres-codigo-actual.txt`); en una prueba anterior, 10 errores y un inicio de sesión con 500 | Sí, en una de las pruebas |
| Con `begin()` normal (prueba aparte, no está en la rama) | 0, pero quedó una conexión «idle in transaction» y 5 `UPDATE sesion` bloqueados más de 45 s | Sí |
| **Sin transacción** (las dos consultas con una conexión normal; prueba aparte) | **0 en las 3 tandas** | **No** |

Con `begin()` normal el fallo no da errores, pero es peor: la conexión se queda con la transacción abierta y bloquea las filas de `sesion`, así que las peticiones esperan sin respuesta.

## Propuesta para el converge (Persona A)

Quitar la transacción de `GET /api/tablero` sin perder lo que buscaba D1 (que la carga cuadre siempre con las tarjetas, SC-003): **calcular la carga en Rust a partir de la misma lista de órdenes** que se usa para las columnas, y pedir a la base solo la lista de mecánicos (id, nombre, activo, rol). Así hay una sola consulta de órdenes, la carga suma lo mismo que las tarjetas por construcción y no queda ninguna transacción que se pueda cancelar a mitad.

- Prueba: `estres.sh` sin errores ni conexiones en transacción, además de las pruebas que ya hay (la 5 y la 6 comprueban que la carga suma lo mismo que las tarjetas).
- Después, repetir T030 entera.

## Para la Persona B y la base común (no bloquea)

Las demás transacciones (`POST` de citas, órdenes, presupuestos, imputaciones, facturas, sesión…) podrían sufrir lo mismo si el navegador cortara la petición a mitad. Es mucho menos probable, porque esas peticiones no se repiten solas ni se cortan al navegar, pero conviene anotarlo en el research común.
