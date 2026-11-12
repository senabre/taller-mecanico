# T039 · §9 Integración: 3579KLM de la cita a la factura

- **Quién**: la Persona A, con el navegador automático (Playwright, Chromium en `es-ES`, zona horaria de Madrid), haciendo todos los papeles: el cliente, `ana`, `luis` y `paco`. **La Persona B no participó**, aunque el §9 dice «los dos juntos».
- **Código**: rama `spec-008` en `02e9868`.
- **Base**: la misma en la que se acababa de recorrer §0–§8 (T030), sin recargar nada entre el día A y el día B. Por eso la orden nueva es la **Nº 13**, como dice el quickstart.
- **Resultado**: **10/10**.

## Los dos días

- **Día A, 12/11/2026 (real)**: el cliente pide cita para `3579KLM` en `/cita`, sin entrar. El primer día que ofrecía la página era el **18/11/2026** (el §1 había llenado algunos días antes).
- **Día B, 18/11/2026 (simulado)**: el día de la cita queda seis días después y el entorno donde se ejecuta la prueba no se mantiene tanto tiempo. Para no recargar la base (borraría la cita), se movió el reloj al 18/11/2026 a primera hora (08:26 de Madrid): el del servidor y el de PostgreSQL con `libfaketime` (`arrancar_b.sh`) y el de cada navegador con el reloj de Playwright. Todo lo demás es igual que en el día real: los datos del día A, sin tocar. Si se quiere la prueba con dos días reales, basta con repetir el día B el 18/11 sobre esa misma base.

## Resultados

| Paso | Esperado | Observado | | Capturas |
|---|---|---|---|---|
| §9.1 Pedir cita para 3579KLM (día A) | Pide los datos del vehículo y del cliente; con el primer día que ofrece la página se confirma | pide datos=1,1,1,1,1; primer día ofrecido=2026-11-18 («18/11/2026»); confirmada=true; en la base=2026-11-18/Revisión de frenos | ✅ | a01-formulario.png, a02-cita-confirmada.png |
| §9.1 El tablero no cambia con la cita | Las citas no salen en el tablero | igual=true | ✅ | a00-tablero-antes.png, a03-tablero-despues.png |
| §9.2 Tablero de paco (día B) | La carga coincide con las tarjetas de cada mecánico; alguna orden demo sale «Retrasada» (normal: los datos son del día A) | carga=«Luis 5 · Marta 4»; en la base=«Luis 5 · Marta 4»; retrasadas=[8888KKK,1111BBB,1414NNN,5555GGG,4444FFF,6666HHH,7777JJJ,8888KKK,3333DDD] | ✅ | b01-tablero-inicio.png |
| §9.3 ana abre la orden (15.000 km, Luis, entrega en 3 días) | Orden Nº 13; en menos de 5 s sale en «Recibida» y Luis suma 1 | en Citas de hoy con «Abrir orden»=true; Orden Nº 13; tablero en 106 ms; carga=«Luis 6 · Marta 4» | ✅ | b03-p3-recibida.png |
| §9.4 luis guarda el diagnóstico | La tarjeta pasa a «En diagnóstico» en menos de 5 s | 108 ms | ✅ | b05-p4-en-diagnostico.png |
| §9.4 luis crea el presupuesto (1 h «Cambio de pastillas») y lo emite | La tarjeta no se mueve; llega el correo con el enlace | 48,40 €=true; columna=En diagnóstico; correo con el enlace=true | ✅ | b06-p4-presupuesto-emitido.png |
| §9.5 El cliente acepta desde el enlace | «Aceptado el … a las …» y la tarjeta pasa a «En reparación» en menos de 5 s | «Aceptado el 18/11/2026 a las 08:26»; tablero en 104 ms | ✅ | b09-p5-aceptado-movil.png |
| §9.6 luis imputa 1 h y termina | Sin pasar del 110 %; la tarjeta pasa a «Terminada» en menos de 5 s | «Acumulado 40,00 € de 40,00 € presupuestados»; tablero en 105 ms | ✅ | b11-p6-terminada.png |
| §9.7 ana factura | «Factura nº N» con su total; correo «Su vehículo 3579KLM está listo para recoger»; la tarjeta desaparece y Luis resta 1 en menos de 5 s | «Factura nº 4», total 48.40 €; correo=true; tablero en 105 ms; Luis 6 → 5 | ✅ | b12-p7-factura.png |
| §9.8 Historial de 3579KLM | La reparación con su diagnóstico y sus líneas | diagnóstico=true; línea=true; 15.000 km=true | ✅ | b14-p8-historial.png |

## Observaciones

- El día B, el tablero muestra como «Retrasada» casi todas las órdenes demo (`8888KKK` dos veces: la orden demo 8 y la del §8.7). Es lo que avisa el §9: los datos demo son del día A.
- Cada cambio de la orden nueva llegó al tablero de `paco` en unos 105 ms, sin recargar. Son tiempos de máquina; el cronometraje humano de SC-002 está en T029.
- El cliente aceptó desde un móvil de 390 px (`b08` y `b09`).
- La factura es la **nº 4**: la 1 es de los datos demo, la 2 del §7 y la 3 del §8.8.

## Revisión conjunta

- **Fecha**: 12/11/2026.
- **Quiénes**: la Persona A y la Persona B, juntas.
- **Qué**: el recorrido entero y sus capturas, pasos 1 a 8 (día A y día B). La Persona B comprobó
  además las partes de sus specs: el diagnóstico y el presupuesto (spec-003, paso 4), la imputación
  y el terminar (spec-005, paso 6) y la factura con su correo y el historial (spec-007, pasos 7 y 8).
- **Resultado**: sin observaciones.
- **Decisión**: se acepta el día B con el reloj adelantado (servidor, base y navegador en el
  18/11/2026, sin recargar los datos del día A) en lugar de esperar al día real de la cita.

## Cómo repetirlo

Día A: `node t039a.mjs <carpeta>` sobre la base al final del §8. Día B: arrancar el servidor y la base con el reloj del día de la cita (`arrancar_b.sh`) y `node t039b.mjs <carpeta> <correos> <hora UTC del día B>`; en un día B real no hace falta mover ningún reloj.
