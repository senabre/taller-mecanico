# Revisión cruzada entre las dos personas

Revisión de los cambios compartidos de cada persona por la otra, como piden las specs: los
`tasks.md` de 003, 005, 006 y 007 terminan con «los cambios en ficheros compartidos … los revisa
también la Persona A», y los de 001, 002, 004 y 008 tienen una tarea final de revisión de la
Persona B. También recoge la revisión de las filas de la
[matriz de trazabilidad](matriz-trazabilidad.md) (apartado 5.2).

- **Fecha**: 12/11/2026.
- **Código revisado**: `origin/main` en `bd330b9`, que contiene todo lo de las dos personas, y la rama
  `entrega` en `4a57c56`, que solo añade documentación.
- **Revisa**: la Persona A, en el apartado 1. La Persona B, en el apartado 2.

**Qué se comprueba en cada cambio**: que no rompe nada de las specs de la Persona A (001, 002, 004 y
008); que respeta la constitución (español de España, importes en € y fechas dd/mm/aaaa, ningún
secreto en el código); y que coincide con lo que dicen su spec y su plan.

**Comprobaciones generales**, válidas para los cuatro apartados:
- **Mis specs siguen funcionando con todo el código de B**: las pruebas de 001, 002, 004, 008 y
  `spec000_estados` pasan sobre el árbol actual (resultado al final de este documento). La regresión
  T030 de spec-008, sobre `02e9868`, recorrió en la aplicación §0–§8 con todo lo de B y no tuvo ningún
  fallo de la aplicación ([verificación](008-tablero-ordenes-abiertas/evidencias/T030/regresion/verificacion.md)).
- **Sin secretos**: en todo lo añadido por B (`git log --author=Andres -p` en `main`) no hay claves,
  contraseñas ni tokens reales (las coincidencias de «secret» o «contraseña» son texto de
  documentos), y no hay ningún `.env` versionado.
- **Formato español**: las pantallas de B (secciones de la ficha, `/almacen`, `/pedidos`,
  `/facturas/[id]`, `/historial` y `componentes/`) muestran los importes con `euros()` y las fechas
  con `fecha()`, `fechaDe()` o `fechaHora()` de `formato.ts`. No usan `toFixed`, `toLocale…` ni «EUR». Los textos
  están en español.
- **Ficheros que B no ha cambiado** aunque sus listas los nombren: `backend/src/rutas/ordenes.rs` y
  `frontend/src/routes/+layout.svelte` (y `estados.rs` y `dinero.rs`). Los cambios que necesitaban
  sus specs en esos ficheros los hizo la Persona A, con su propia revisión (por ejemplo, la ficha de
  spec-002 y `test(006)` `0913e16`). No hay nada que revisar de B en ellos.

## 1. Revisión de la Persona A sobre los cambios compartidos de la Persona B

**Estado**: hecha el 12/11/2026 por la Persona A, para 003, 005, 006 y 007. **Correcta**: ningún
cambio rompe las specs de la Persona A y todos respetan la constitución y coinciden con su spec y su
plan. Lo único pendiente de decisión es O1, que es de la Persona B.

### spec-003 · Diagnóstico y presupuesto

Lista del [tasks.md](003-diagnostico-presupuesto/tasks.md): migración `0002`, `tipos.ts`,
`componentes/`, `ordenes/[id]/+page.svelte`, `ordenes.rs`. Documentos del
[plan.md](003-diagnostico-presupuesto/plan.md): data-model, contrato y quickstart §3.

| Fichero compartido | Qué cambió | Commit | Qué he comprobado | Resultado |
|---|---|---|---|---|
| `backend/migrations/0002_spec003_lineas.sql` | Nueva: dos `CHECK` en `linea_presupuesto` (horas en múltiplos de 0,25; piezas enteras) | `dd77530` | Coincide con D1 de 003 y con el data-model. Los presupuestos de mis specs y de los datos demo cumplen las dos reglas (1,5 h y 2 uds; la ampliación demo de 004, 1 h); las pruebas de 004 pasan con la migración | Correcto |
| `frontend/src/lib/tipos.ts` | Sección «spec-003»: `EstadoPresupuesto`, `TipoPresupuesto`, `Presupuesto`, `PresupuestoResumen` | `dd77530` | Solo añade. Coincide con el contrato. 004 amplió después `PresupuestoResumen` con `respondido_en` (clarify Q1 de 004, revisado por B) | Correcto |
| `frontend/src/lib/componentes/TablaLineas.svelte` y `Totales.svelte` | Nuevos: tabla de líneas con «Quitar» opcional y bloque de base, IVA y total | `dd77530` | Importes con `euros()` y cantidades con `numero()` («1,5 h», «2 uds»). Los usa mi página pública de 004 sin `onquitar`; verificado en T027 y en T030 §4 | Correcto |
| `frontend/src/routes/ordenes/[id]/Diagnostico.svelte` y `Presupuesto.svelte` | Nuevas secciones de la ficha (contenedor de spec-002) | `dd77530` | Cada una en su componente, como acordó el plan maestro. `Presupuesto.svelte` lo amplió después 004 (T016), con aviso previo y revisión de B. **`confirm()`** al emitir: ver la observación O1 | Correcto, con O1 |
| `ordenes/[id]/+page.svelte`, `ordenes.rs` | Sin cambios de B (las secciones las monta la ficha de 002) | — | `git log --author=Andres` no da ningún commit en esos ficheros | Nada que revisar |
| `specs/000-base-comun/contracts/api.md` | Filas de spec-003: respuesta del diagnóstico y su 409, tipo del presupuesto según el estado, totales en borrador, cantidades, `aviso_enviado`, `PresupuestoResumen` | `dd77530` | Solo cambia sus filas. Lo que usa 004 (`token`, `estado`, `caduca_el`, `respondido_en`, `enlace`) coincide con lo implementado | Correcto |
| `specs/000-base-comun/data-model.md` | `linea_presupuesto`: cantidades (migración `0002`) y concepto de la pieza puesto por el servidor | `dd77530` | Coincide con D1 y D3 de 003 | Correcto |
| `specs/000-base-comun/quickstart.md` | §3: cambiar el diagnóstico, cantidades no válidas, quitar una línea, emitir con correo y diagnóstico fijo tras emitir | `dd77530` | Solo cambia el §3. Mi regresión T030 lo recorrió: 7 de 7 | Correcto |

### spec-005 · Imputar tiempo y piezas

Lista del [tasks.md](005-imputar-tiempo-piezas/tasks.md): migración `0003`, `lineas.rs`,
`presupuestos.rs`, `tipos.ts`, `componentes/FormularioLinea.svelte`, `Presupuesto.svelte`,
`ordenes/[id]/+page.svelte`, `ordenes.rs`. Documentos del [plan.md](005-imputar-tiempo-piezas/plan.md):
data-model, contrato y quickstart §5.

| Fichero compartido | Qué cambió | Commit | Qué he comprobado | Resultado |
|---|---|---|---|---|
| `backend/migrations/0003_spec005_imputaciones.sql` | Nueva: `CHECK` de piezas enteras en `imputacion` | `09c887f` | Coincide con D2 de 005 y con el data-model. Las imputaciones demo cumplen (2 uds) | Correcto |
| `backend/src/lineas.rs` (nuevo) y `backend/src/lib.rs` | `Linea`, `NuevaLinea` y la validación de una línea, comunes a presupuesto e imputaciones; `pub mod lineas` | `09c887f` | Coincide con D1 de 005 (una sola validación). Mis `ordenes.rs` y `publico_presupuesto.rs` usan `lineas::Linea`, y sus pruebas pasan. Mensajes de error en español | Correcto |
| `backend/src/rutas/presupuestos.rs` | Ampliaciones y uso de `lineas.rs` | `09c887f` | Es un fichero de B que consume 004. Lo que usa 004 (`token`, `estado`, `caduca_el`, `respondido_en`, el correo con el enlace) no cambia; las pruebas de 004 pasan, también las de ampliación (`la_ampliacion_se_acepta_y_se_rechaza_igual`) | Correcto |
| `backend/seed/demo.sql` | Dos comentarios: «§5.1» pasa a «§5.2» | `09c887f` | Solo comentarios; no cambia ningún dato que usen mis specs | Correcto |
| `frontend/src/lib/tipos.ts` | Sección «spec-005»: `ResumenImputaciones` y `NuevaLinea` | `09c887f` | Solo añade. Coincide con el contrato. La ficha de 002 usa `ResumenImputaciones` | Correcto |
| `frontend/src/lib/componentes/FormularioLinea.svelte` (nuevo) y `ordenes/[id]/Presupuesto.svelte` | Formulario de línea común; `Presupuesto.svelte` pasa a usarlo | `09c887f` | Textos y cantidades en español. Mis pantallas no usan `FormularioLinea` | Correcto |
| `frontend/src/routes/ordenes/[id]/Imputaciones.svelte` (nuevo) | Sección de imputaciones de la ficha | `09c887f` | En su propio componente. **`confirm()`** al terminar: ver O1 | Correcto, con O1 |
| `ordenes/[id]/+page.svelte`, `ordenes.rs` | Sin cambios de B | — | Ningún commit de B en esos ficheros | Nada que revisar |
| `specs/000-base-comun/contracts/api.md` | Filas de imputar y terminar con sus errores; `ResumenImputaciones` | `09c887f` | Solo cambia sus filas. «Lo da `imputaciones::resumen_imputaciones`, que spec-002 usa» coincide con mi ficha | Correcto |
| `specs/000-base-comun/data-model.md` | `imputacion`: piezas enteras (`0003`), concepto puesto por el servidor, regla del 10 % | `09c887f` | Coincide con D1, D2 y D4 de 005 | Correcto |
| `specs/000-base-comun/quickstart.md` | §5: terminar sin imputar, stock insuficiente, 110 %, ampliación desde Mailpit, terminar | `09c887f` | Solo cambia el §5. Mi regresión T030 lo recorrió: 6 de 6 | Correcto |

### spec-006 · Pedido de pieza al proveedor

Lista del [tasks.md](006-pedido-pieza-proveedor/tasks.md): migración, `demo.sql`, `tipos.ts`,
`+layout.svelte`, `ordenes.rs`, `ordenes/[id]/+page.svelte`. Documentos del
[plan.md](006-pedido-pieza-proveedor/plan.md): data-model, contrato y quickstart §6.

| Fichero compartido | Qué cambió | Commit | Qué he comprobado | Resultado |
|---|---|---|---|---|
| Migración, `demo.sql`, `+layout.svelte`, `ordenes.rs`, `ordenes/[id]/+page.svelte` | Sin cambios de B: lo que pide 006 (stock por defecto 0, proveedores precargados, índice de pedidos, menú Almacén y Pedidos, pedidos en la ficha) ya estaba en la base común o lo añadió la Persona A (`0913e16`) | — | Ningún commit de B en esos ficheros | Nada que revisar |
| `frontend/src/lib/tipos.ts` | Sección «spec-006»: `Pieza`, `Proveedor`, `EstadoPedido`, `Pedido` | `52448f3` | Solo añade. Coincide con el contrato | Correcto |
| `frontend/src/routes/ordenes/[id]/Pedidos.svelte` (nuevo) | Sección de pedidos de la ficha | `52448f3` | En su propio componente; fechas con `fechaDe()` (dd/mm/aaaa) | Correcto |
| `backend/src/rutas/mod.rs` | `pub mod almacen` y `.merge(almacen::rutas())` | `52448f3` | Solo registra su módulo. Lo mismo para 003, 005 y 007 (`presupuestos`, `imputaciones`, `facturas`) | Correcto |
| `specs/000-base-comun/data-model.md`, `contracts/api.md`, `quickstart.md` | Ajustes de 006 antes de implementar: `pieza.stock` por defecto 0, proveedor precargado, índice parcial de `pedido`, sin `POST /api/proveedores`, §6 | `c7e6a02` | Coincide con el plan de 006. El §6 lo recorrió mi regresión T030: 9 de 9, con la orden 3 como «R» | Correcto |

### spec-007 · Facturar y avisar al cliente

Lista del [tasks.md](007-facturar-avisar-cliente/tasks.md): `formato.rs`, `tipos.ts`,
`componentes/TablaLineas.svelte`, `ordenes/[id]/+page.svelte`, `ordenes.rs`. Documentos del
[plan.md](007-facturar-avisar-cliente/plan.md): data-model, contrato y quickstart §7.

| Fichero compartido | Qué cambió | Commit | Qué he comprobado | Resultado |
|---|---|---|---|---|
| `backend/src/formato.rs` | Nueva `euros()` para los correos: «1.234,56 €», con su prueba | `e2a2060` | Formato español con punto de miles, coma decimal y «€». No cambia `numero()`, que usa mi 002 en el mensaje del km | Correcto |
| `frontend/src/lib/tipos.ts` | Sección «spec-007»: `LineaFactura`, `Factura` (con `taller.logo`), `Historial` | `e2a2060` | Solo añade. Coincide con el contrato | Correcto |
| `frontend/src/lib/componentes/TablaLineas.svelte` | Acepta también líneas de factura (sin `tipo` ni `id`): la cantidad sale sin «h» ni «uds» y la clave de cada fila pasa a ser el índice | `e2a2060` | **Me afecta**: mi página pública de 004 lo usa. Con `Linea` (con `tipo` e `id`) se comporta como antes; verificado después en T027 y en T030 §4 | Correcto |
| `frontend/src/routes/ordenes/[id]/Factura.svelte` (nuevo) y `+page.svelte` | Sección de la factura; la ficha la monta con `puedeFacturar` (recepción o jefe) | `e2a2060`, `54bdbe7` | Cuatro líneas en `+page.svelte`, que es el contenedor de 002: solo importa y monta su sección. **`confirm()`** al facturar: ver O1 | Correcto, con O1 |
| `ordenes.rs` | Sin cambios de B (`factura_id` lo llena la ficha de 002 con `facturas::factura_de_orden`) | — | Ningún commit de B en el fichero | Nada que revisar |
| `specs/000-base-comun/contracts/api.md` | Facturar, ver la factura (con `logo`), historial, `GET /api/taller/logo`, `LineaFactura` | `e2a2060` | Solo cambia sus filas | Correcto |
| `specs/000-base-comun/data-model.md` | `factura`: una sola serie de números; `aviso_enviado` después del `commit` | `e2a2060` | Coincide con D2 y D4 de 007 | Correcto |
| `specs/000-base-comun/quickstart.md` | §7: «Ver e imprimir», logo, aviso con el total, historial | `e2a2060` | Solo cambia el §7. Mi regresión T030 lo recorrió: 5 de 5 | Correcto |

## 2. Revisiones de la Persona B sobre las specs de la Persona A

| Spec | Tarea | Estado | Detalle |
|---|---|---|---|
| 001 | T031 | **Hecha** | Revisión de todo lo compartido de 001, marcada en su [tasks.md](001-pedir-cita-en-linea/tasks.md) |
| 002 | T031 | **Hecha** (05/11/2026) | «Código y documentos compartidos revisados sin problemas; 130 pruebas en verde y las secciones de 003, 005, 006 y 007 comprobadas en pantalla en `/ordenes/[id]`» ([tasks.md](002-abrir-orden-reparacion/tasks.md)) |
| 004 | T031 | **Pendiente** | Pendiente de su PR (#4, ya fusionado el 12/11/2026 por la Persona B, pero sin marcar T031). Cambios en ficheros de B (`presupuestos.rs`, `Presupuesto.svelte`) con su OK del 05/11/2026 (I1); el resto, en la tabla de compartidos del [plan](004-aceptar-presupuesto-enlace/plan.md) |
| 008 | T032 | **Pendiente** | Pendiente de su PR (#5, fusionado el 12/11/2026 por la Persona B, sin marcar T032). Lista en la tabla de compartidos del [plan](008-tablero-ordenes-abiertas/plan.md), al día (T040), y ejecutar aparte la prueba 9 (`cargo test --test spec008_tablero -- --ignored`) |

## 3. Revisión de las filas de la matriz de trazabilidad

| Revisión | Estado actual |
|---|---|
| **La Persona B sobre las filas de la Persona A** (68 filas) | Hecha por muestreo el 12/11/2026: un criterio de cada historia (HU-01 US1-4, HU-02 US2-1, HU-04 US4-1 y HU-08 US1-2), los cuatro «Correcta», con la sugerencia de citar también la prueba de spec-005 en HU-04 US4-1 ([matriz](matriz-trazabilidad.md), «Revisión de la Persona B sobre las filas de la Persona A»). En la tabla, esas 4 filas dicen «Persona B, revisado el 12/11/2026» y las otras 64, «Persona B (pendiente)» |
| **La Persona A sobre las filas de la Persona B** (58 filas) | Hecha por muestreo el 12/11/2026 (ver abajo) y marcada en la matriz: esas 4 filas dicen «Persona A, revisado el 12/11/2026» y las otras 54, «Persona A (pendiente)». La tabla del muestreo está también en la matriz («Revisión de la Persona A sobre las filas de la Persona B») |
| **Sugerencia de la Persona B en HU-04 US4-1** | **Aplicada** el 12/11/2026: la fila cita también `solo_se_imputa_en_reparacion_con_presupuesto_aceptado` de spec-005, que cubre la orden «en reparación» sin presupuesto aceptado |

### Muestreo de la Persona A sobre las filas de la Persona B

Para cada historia se eligió un criterio, se leyó la prueba citada, se comprobó que sus aserciones
cubren lo que dice el criterio y que la evidencia enlazada la da por superada.

| Historia · criterio | Qué se comprobó | Resultado |
|---|---|---|
| HU-03 · US2-5 (quitar una línea recalcula los totales) | `quitar_una_linea_recalcula_los_totales` parte del presupuesto del ejemplo (1,5 h + PF-123 × 2), quita la mano de obra y comprueba que queda solo la pieza con base 50,00, IVA 10,50 y total 60,50, que quitarla otra vez da 404 y que otro mecánico recibe 403. En la [evidencia](003-diagnostico-presupuesto/evidencias/matriz/pruebas.txt), `ok` (20 de 20) | Correcta |
| HU-05 · US2-1 (el 110 % justo no bloquea) | `el_limite_del_10_por_ciento_es_exacto_al_centimo`: con 100,00 € presupuestados, una pieza de 110,00 € deja la orden «en reparación» y otra de 110,01 €, en otra orden, la pasa a «pendiente de conformidad». En la [evidencia](005-imputar-tiempo-piezas/evidencias/matriz/pruebas.txt), `ok` (12 de 12) | Correcta |
| HU-06 · US3-2 (con dos pedidos, la orden espera al último) | `con_dos_pedidos_la_orden_espera_al_ultimo`: al recibir el primero la orden sigue «en espera de piezas» y el tablero no recibe ningún evento; al recibir el segundo vuelve a «en reparación» y se publica `{id, estado: "en_reparacion"}`. En la [evidencia](006-pedido-pieza-proveedor/evidencias/matriz/pruebas.txt), `ok` (23 de 23) | Correcta; además comprueba el evento que usa mi spec-008 |
| HU-07 · US1-1 (factura de 157,30 €) | `facturar_copia_las_imputaciones_y_cierra_la_orden`: las dos líneas (2 h × 40,00 = 80,00 y PF-123 × 2 × 25,00 = 50,00), base 130,00, IVA 21 % 27,30, total 157,30; número 1, fecha de hoy, cliente, matrícula y taller; la orden pasa a «facturada» y se publica el evento. En la [evidencia](007-facturar-avisar-cliente/evidencias/matriz/pruebas.txt), `ok` (11 de 11) | Correcta |

**Columna «Revisor» de las filas de la Persona B** (aplicado el 12/11/2026): HU-03 US2-5, HU-05 US2-1,
HU-06 US3-2 y HU-07 US1-1, «Persona A, revisado el 12/11/2026»; las otras 54, «Persona A
(pendiente)», como la revisión por muestreo que hizo la Persona B de las filas de la Persona A.

## Observaciones

| ID | Observación | Decisión | Responsable | Estado |
|---|---|---|---|---|
| O1 | `confirm()` del navegador en **tres** sitios: «Emitir presupuesto» (`Presupuesto.svelte`, spec-003), «Terminar reparación» (`Imputaciones.svelte`, spec-005) y «Facturar» (`Factura.svelte`, spec-007). Los botones del diálogo salen en el idioma del navegador (principio II); spec-004 lo resolvió con una confirmación dentro de la página (D5) | Comunicada a la Persona B. **Respuesta de la Persona B (12/11/2026)**: queda abierto como trabajo futuro y lo añade a su V2 (hallazgos de spec-008), con los tres sitios: «Emitir presupuesto» (003), «Terminar reparación» (005) y «Facturar» (007) | Persona B | Decidido; lo aplica la Persona B |
| O2 | Muchas filas de la Persona B en la matriz dicen «en la app, anotado en T0xx», sin informe ni capturas propios. La regresión T030 de la Persona A recorrió en la app los §3, §5, §6 y §7 con capturas ([verificación](008-tablero-ordenes-abiertas/evidencias/T030/regresion/verificacion.md), carpetas `s3`, `s5`, `s6` y `s7`) | Sugerencia a la Persona B. **Respuesta de la Persona B (12/11/2026)**: aceptada; citará en sus filas de la matriz las capturas de la regresión T030 (§3, §5, §6 y §7) | Persona B | Decidido; lo aplica la Persona B |
| O3 | HU-05 US3-1 está «con desviación» porque la aceptación de la ampliación se simuló con SQL; la regresión T030 la hizo de verdad desde el enlace del correo (§5.5: aceptada, «en reparación», acumulado 130,00 € de 150,00 €) | Sugerencia a la Persona B. **Respuesta de la Persona B (12/11/2026)**: aceptada; pasará HU-05 US3-1 a «Verificado» con la evidencia de `s5` de la regresión T030 | Persona B | Decidido; lo aplica la Persona B |
| O4 | Las listas de ficheros compartidos de 003, 005, 006 y 007 nombran ficheros que la Persona B no ha tocado (`ordenes.rs`, `+layout.svelte`, la migración de 006 y `ordenes/[id]/+page.svelte`, salvo 4 líneas de 007) | Sin acción: solo significa que no hay nada que revisar en ellos | — | Cerrada |
| O5 | T031 de 004 y T032 de 008 (revisión de la Persona B de los compartidos de la Persona A) sin marcar, aunque sus PR ya están fusionados | Pedido a la Persona B el 12/11/2026. **Respuesta de la Persona B (12/11/2026)**: marcará T031 de 004 y T032 de 008 y añadirá su aprobación en el PR #5 | Persona B | Decidido; lo aplica la Persona B |

## Resultado de las pruebas de la Persona A sobre el árbol actual

`cd backend && cargo test --test spec000_estados --test spec001_citas --test spec002_ordenes
--test spec004_respuesta --test spec008_tablero`, sobre la rama `entrega` en `4a57c56`, con todo el
código de la Persona B, el 12/11/2026:

| Fichero de pruebas | Resultado |
|---|---|
| `spec000_estados` | 7 de 7 |
| `spec001_citas` | 15 de 15 |
| `spec002_ordenes` | 12 de 12 |
| `spec004_respuesta` | 16 de 16 |
| `spec008_tablero` | 8 de 8 (la 9 lleva `#[ignore]` y se lanza aparte; pasó el 12/11/2026 sobre el mismo código de B: [salida](008-tablero-ordenes-abiertas/evidencias/T033-prueba9-ignored.txt)) |

**58 pruebas, 0 fallos.** Los cambios compartidos de la Persona B no rompen nada de las specs de la
Persona A.
