# T029 · Cronometraje humano de SC-001 y SC-002

- **Fecha**: 12/11/2026. Hecho por la Persona A, con una persona que **no había visto antes el
  tablero** para SC-001.
- **Navegador**: Chrome en español. `paco` en una ventana normal con `/tablero`; `luis` en una
  ventana de incógnito (la sesión va en una cookie, así que son dos sesiones distintas).
- **Datos**: los datos demo, en los que la única orden retrasada es `8888KKK` y `2222CCC` está «En
  reparación», asignada a Luis.

## Resultados

| Criterio | Qué se cronometra | Límite | Tiempo | Resultado |
|---|---|---|---|---|
| **SC-001** · quickstart común §8.4 | Desde que se ve el tablero hasta que la persona dice qué órdenes van con retraso | < 10 s | **5 s**. Nombra solo `8888KKK`; no nombra `3333DDD` (entrega hoy) | ✅ |
| **SC-002** · quickstart común §8.6 | Como `luis`, en `2222CCC`: desde «Terminar reparación» → «Aceptar» hasta que la tarjeta aparece en «Terminada» en el tablero de `paco`, sin tocarlo | < 5 s | **4 s** | ✅ |

## Lo que no se cronometró a mano

- T029 pedía repetir SC-002 en los pasos 7 (abrir una orden) y 8 (facturar). No se hizo con reloj:
  en la verificación de T028 (Playwright) esas dos actualizaciones llegaron en 108 ms y 111 ms, y la
  de terminar, en 109 ms. Son tiempos de máquina, que confirman el mecanismo, pero no son el
  cronometraje humano.
- El cronometraje se hizo en Chrome real, en un recorrido aparte del de T028 (que fue con
  Playwright), y no en el mismo recorrido, como proponía la tarea.
- El «Aceptar» es el del diálogo `confirm()` de spec-005, con los botones en el idioma del navegador
  (hallazgo V2 de [hallazgos.md](../../hallazgos.md)).
