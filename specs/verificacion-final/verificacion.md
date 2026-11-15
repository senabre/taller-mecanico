# Verificación final sobre `main` antes de la entrega

- **Fecha**: 14/11/2026. Persona A, con el navegador automático (Playwright, Chromium en `es-ES`, zona horaria de Madrid).
- **Código**: `main` en `1c347b0` (incluye las correcciones de la Persona B del 12/11: confirmación dentro de la página en «Emitir presupuesto», «Terminar reparación» y «Facturar», el diagnóstico de solo lectura tras emitir y los mensajes en español).
- **Motivo**: la Persona B cambió pantallas de 003, 005 y 007 después de la regresión T030 de spec-008, y esos cambios los atraviesan las historias de la Persona A.

## Pruebas automáticas

| Comprobación | Resultado |
|---|---|
| `cargo test --no-fail-fast` | **155 de 155** en verde (`salida/cargo-main.txt`). La prueba `el_aviso_se_anota_si_sale_el_correo` (007) necesita un servidor de correo en el puerto 1025, como Mailpit: en la primera pasada no estaba arrancado y falló (154 de 155 en esa salida); con él, `spec007_facturas` da 11 de 11 (`salida/cargo-spec007-con-correo.txt`) |
| `cargo test --test spec008_tablero -- --ignored` (prueba 9) | 1 de 1 |
| `cargo clippy --all-targets -- -D warnings` | Sin avisos |
| `npm run check` | 0 errores, 0 avisos |
| `npm test` (Vitest) | 21 de 21 |
| `npm run build` | Correcto |

## Regresión del quickstart común y recorrido R2

Base recién cargada con `demo.sql`, un solo recorrido §1 → §2 → §0 → §3 → §4 → D7 → §5 → §6 → §7 → §8, y después R2 sobre otra base limpia.

| Sección | Resultado | Evidencias |
|---|---|---|
| §0 Base común | 5/5 | `s0/` |
| §1 Pedir cita (001) | 31/31 | `s1/` |
| §2 Abrir orden (002) | 24/24 | `s2/` |
| §3 Diagnóstico y presupuesto (003) | 7/7 | `s3/` |
| §4 Respuesta del cliente (004) | 20/20 | `s4/` |
| D7 Orden demo 7 | 3/3 | `d7/` |
| §5 Imputar (005) | 6/6 | `s5/` |
| §6 Almacén y pedidos (006) | 9/9 | `s6/` |
| §7 Factura e historial (007) | 5/5 | `s7/` |
| §8 Tablero (008) | 13/14 | `s8/` |
| R2 Recorrido integrado de error, permiso y estado | 9/9 | `r2/` |

- **Sin ningún fallo de la aplicación.** La única comprobación que falla en el §8 es la parte opcional del paso 9, que se quitó del quickstart en el converge-1 de spec-008 (V1); el script la conserva para dejar constancia.
- **`estres.sh`** (V3): 0 errores y 30 de 30 inicios de sesión correctos (`salida/estres.txt`).
- **Scripts adaptados a los cambios de la Persona B**: tras «Emitir presupuesto», «Terminar reparación» y «Facturar» se pulsa la confirmación de la página («Sí, emitir», «Sí, terminar», «Sí, facturar»), y el §3.7 comprueba ahora que el diagnóstico se ve sin «Guardar diagnóstico», con el aviso, y que la API lo rechaza (409), como dice el quickstart actualizado. Las versiones anteriores de los scripts están en las evidencias de T030 de spec-008.

## Cómo repetirlo

Los scripts están en `scripts/`, con el mismo orden y las mismas variables que en T030 de spec-008 (`SIN_MOVIL=1 SOLO_RECHAZO=1` en `t029b.mjs`, `REUSAR=1` en `t027.mjs`, `CARGA_LUIS=6` en `t028.mjs`).
