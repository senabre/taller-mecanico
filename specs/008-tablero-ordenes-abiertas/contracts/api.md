# Contrato de API: spec-008 · Tablero de órdenes abiertas

El contrato es **único** y está en
[000-base-comun/contracts/api.md § spec-008 · Tablero](../../000-base-comun/contracts/api.md#spec-008--tablero);
`GET /api/eventos` está en la tabla común del mismo documento y `OrdenResumen` en la sección de
spec-002. Las convenciones (errores, fechas) son las del mismo documento. Aquí solo se explica qué
hace la pantalla con cada llamada y qué requisito cumple cada respuesta. Tras el clarify y el plan,
se han hecho estos cambios en el contrato común (los revisa la Persona B):

- `carga_mecanicos` añade `activo`: activos aunque tengan 0, inactivos solo con órdenes abiertas,
  ordenados por nombre, y la suma es el número de tarjetas (Q1, research D2).
- Las tarjetas de cada columna van por `fecha_entrega_prometida` y `numero` ascendentes (Q4).
- 401 sin sesión y 403 «No tienes permiso para hacer esto» para los demás roles (Q2, research D4).
- Cuándo vuelve a pedir el tablero la pantalla (Q3, research D5 y D6).

## Ejemplo de respuesta de `GET /api/tablero`

Con los datos demo recién cargados y el cambio de research D8 (orden 3 con entrega hoy). Se
abrevian los `OrdenResumen` y se omiten las columnas sin interés para el ejemplo.

```json
{
  "columnas": [
    { "estado": "recibida", "ordenes": [ { "numero": 1, "matricula": "1111BBB", "retrasada": false, "…": "…" } ] },
    { "estado": "en_diagnostico", "ordenes": [ { "numero": 5, "…": "entrega +3" }, { "numero": 4, "…": "entrega +5" } ] },
    { "estado": "en_reparacion", "ordenes": [ { "numero": 3, "retrasada": false, "…": "entrega hoy" }, { "numero": 2, "…": "entrega +2" } ] },
    { "estado": "en_espera_piezas", "ordenes": [ "…" ] },
    { "estado": "pendiente_conformidad", "ordenes": [ "…" ] },
    { "estado": "terminada", "ordenes": [ { "numero": 8, "matricula": "8888KKK", "retrasada": true, "…": "entrega ayer" } ] }
  ],
  "carga_mecanicos": [
    { "id": 2, "nombre": "Luis", "activo": true, "ordenes_abiertas": 5 },
    { "id": 3, "nombre": "Marta", "activo": true, "ordenes_abiertas": 3 }
  ]
}
```

Siempre hay 6 elementos en `columnas`, en este orden, aunque `ordenes` esté vacío. Los `id` del
ejemplo son orientativos.

## Uso desde la pantalla `/tablero` (jefe)

| Momento | Llamada | Qué muestra | Requisito |
|---|---|---|---|
| Al abrir | `GET /api/tablero` | **200**: la carga arriba («Luis 5 · Marta 3», con «(de baja)» si `activo` es `false`) y las 6 columnas con sus tarjetas: «Nº», matrícula, mecánico y «Entrega dd/mm/aaaa»; con `retrasada`, borde destacado e insignia «Retrasada»; una columna vacía, «Sin órdenes» · **403**: solo «No tienes permiso para hacer esto», sin columnas ni tarjetas · **otro error**: su mensaje, o «No se pudo cargar el tablero» si no hay conexión | FR-001 a FR-004, FR-006, US1, US2, US3 |
| Tras la primera carga, salvo si dio 403 (también si falló por la red: así se recupera sola) | `EventSource('/api/eventos')`, con `conectarEventos` de `lib/eventos.ts` | Nada por sí mismo | FR-005, FR-006, US4-5 (research D5; E2, N2) |
| `onopen`: el primero y cada vez que la conexión se abre de nuevo tras un error (reconecte el navegador solo o se cree otra tras un cierre) | `GET /api/tablero` | Quita el aviso y muestra el tablero entero: lo enviado mientras no había conexión se ha perdido | FR-005, US4-5 (Q3, N3) |
| Con cada evento `orden` (`{id, estado}`) | `GET /api/tablero` | El tablero entero, sin mirar el contenido del evento. Si ya hay una petición en curso, se hace una más al terminar, y solo una | FR-005, US4-1 a US4-3, SC-002 |
| `onerror`, con la conexión reintentando (`CONNECTING`) | ninguna: el tablero se pide en el `onopen` | «Sin conexión, reintentando…» encima del tablero, que se queda como estaba. Desaparece en el siguiente `onopen` | Caso límite «Se pierde la conexión» |
| `onerror`, con la conexión cerrada (`CLOSED`: la respuesta no fue un 200 de eventos, p. ej. 401) | `GET /api/tablero` **una vez** (única excepción: con un 401 la conexión no volvería a abrirse) y otra conexión pasados 5 s | El mismo aviso. Si `GET /api/tablero` da 401, `api.ts` lleva a `/entrar`; si no, la conexión nueva abre, el aviso desaparece y su `onopen` pide el tablero | Caso límite «Se pierde la conexión», US4-5 (E1, N3) |
| 00:01 de Madrid (medianoche, con un minuto de margen) | `GET /api/tablero` | Las órdenes con entrega ayer pasan a «Retrasada» sin que cambie ninguna orden | FR-005, US4-4 (research D6) |
| Al salir de la página | ninguna | Se cierran la conexión de eventos, su espera pendiente y el temporizador de medianoche | — |

Las tarjetas no son enlaces y no se arrastran: el tablero es solo de consulta (principio III). Si
`GET /api/tablero` responde 401 (sesión caducada), `api.ts` lleva a `/entrar`, como en todas las
pantallas.

## Eventos que mueven el tablero

Los publica cada spec con `CambioEstado::publicar` después del `commit` (research común R14); la
lista, con el fichero y la línea, está en [plan.md](../plan.md). El contenido del evento no se usa:
basta con que llegue.
