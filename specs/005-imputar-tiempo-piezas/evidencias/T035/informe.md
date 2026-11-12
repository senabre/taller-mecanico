# T035 · «Terminar reparación»: confirmación en la página y motivo sin imputaciones (C1 y U1)

- **Fecha**: 12/11/2026. Persona B, con Claude Code y un navegador automático (Playwright 1.48,
  Chromium en `es-ES`, zona horaria de Madrid).
- **Código**: rama `spec-005` sobre `2c207bd`; «antes» con ese código y «después» con T035, la
  interfaz recién compilada en los dos casos.
- **Datos**: base aparte `taller_c1`, recién migrada y con `backend/seed/demo.sql` antes de cada
  ejecución. El script prepara por la API, como `luis`, la orden demo 4: presupuesto de 1 h, emitido y
  aceptado desde su enlace, así que queda «en reparación» sin imputaciones.
- **Cómo repetirlo**: con los scripts de [spec-003 T036](../../../003-diagnostico-presupuesto/evidencias/T036/scripts/):
  `ejecutar.sh t-005.mjs antes|despues`.

## Qué se comprueba

| Paso | Esperado |
|---|---|
| (a) U1 · En reparación sin imputaciones | El motivo «Primero hay que imputar el trabajo realizado.» y ningún botón «Terminar reparación» |
| (b) C1 · «Terminar reparación» tras imputar 1 h | La pregunta en la página con «Sí, terminar» y «Volver», sin diálogo del navegador; «Volver» no termina y «Sí, terminar» deja la orden «terminada» |
| (c) A 390 px | La confirmación cabe sin desplazamiento horizontal |

## Resultado

| | Correctas | Fallos | Salida |
|---|---|---|---|
| Antes | 0 | 3 | [salida-005-antes.txt](salida-005-antes.txt) |
| Después | 3 | 0 | [salida-005-despues.txt](salida-005-despues.txt) |

- **Antes**: sin imputaciones no había ni botón ni motivo; «Terminar reparación» abría un `confirm()`
  del navegador.
- **Después**: capturas [sin imputaciones](despues-005-a-sin-imputaciones.png),
  [confirmación](despues-005-b1-confirmacion.png), [terminada](despues-005-b2-terminada.png) y
  [a 390 px](despues-005-c-movil.png).
