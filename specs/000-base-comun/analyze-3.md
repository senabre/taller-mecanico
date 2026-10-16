# Análisis de coherencia 3 · spec-000 Base común

**Fecha**: 2026-10-15 · **Alcance y criterios**: los mismos que en [analyze-1.md](analyze-1.md) y
[analyze-2.md](analyze-2.md), con la constitución **v1.2.1**. Este análisis se centra en
verificar los 6 hallazgos corregidos después de analyze-2 y en buscar efectos secundarios de esos
cambios. Los IDs de tarea son los del `tasks.md` actual (43 tareas).
**Modo**: solo lectura sobre los documentos analizados. Este informe y
[hallazgos.md](hallazgos.md) son los únicos ficheros escritos.

## Verificación de los 6 corregidos tras analyze-2

| ID | Resultado | Comprobación |
|----|-----------|--------------|
| K2 | ✅ Cerrado | T031: `crear_usuario` genera la contraseña con `auth::nuevo_token()` y la devuelve; `entrar` la recibe; ningún literal en las pruebas. La constitución 1.2.1 permite citar en `specs/` los valores que publica el quickstart. T042 comprueba que `demo1234` y el token no están en `backend/src/`, `backend/tests/` ni `frontend/src/`. Ninguna spec 001–008 usa las antiguas `crear_usuario`/`entrar` de la base común. |
| I3 | ✅ Cerrado | R11 dice «evento SSE `orden` con `{id, estado}`», igual que el contrato, T017/T030 y el plan de 008. |
| A1 | ✅ Cerrado | Ni spec.md, ni el quickstart, ni tasks.md mencionan ya «Facturación». Lo que comprueban (`luis`: «Mis órdenes» e «Historial») coincide con el menú de T034. |
| A2 | ✅ Cerrado | El paso 4 del quickstart §0 comprueba a `ana` y `pepe`, con el menú de T034 y las rutas de pantalla del contrato. SC-002 y el «Hecho cuando» de la fase 0 del plan quedan cubiertos por el criterio de terminado. |
| A3 | ✅ Cerrado | T035 muestra siempre texto fijo y nunca el mensaje técnico del error. |
| G9 | ✅ Cerrado | T038 añade `BJ-7 Bujía` (stock 0), T039 usa `BJ-7` en el pedido pendiente («nunca `PF-123`») y el quickstart la menciona. El stock de `PF-123` del §5.1 ya no depende del pedido demo. |

## Hallazgos nuevos

| ID | Categoría | Gravedad | Ubicación | Resumen | Recomendación |
|----|-----------|----------|-----------|---------|---------------|
| K3 | Constitución | **CRITICAL** | constitución 1.2.1 (principio V, «pueden citar los valores demo que ya publica `quickstart.md`»); `demo-presupuesto-emitido` en tasks T040, `analyze-1.md` (K1), `analyze-2.md` (K2) y `hallazgos.md` (K1, K2); quickstart: 0 apariciones | **Nuevo**, consecuencia de K2. La aclaración 1.2.1 solo permite citar en `specs/` los valores demo que publica el quickstart, y el quickstart publica la contraseña `demo1234` pero **no** el token del presupuesto demo. Además, T042 tal como está escrito daría por buena esa aparición. | Recomendada: publicar el token en el quickstart, en «Arrancar», con una línea como «Presupuesto demo emitido (ficticio): `/presupuesto/demo-presupuesto-emitido`». Así queda cubierto por 1.2.1 sin tocar la constitución, y además sirve para probar spec-004 sin esperar a 003, como pide el plan maestro. Alternativa: quitar el valor de todos los documentos de `specs/` y dejarlo solo en `demo.sql`. |
| I12 | Inconsistencia | LOW | constitución, principio V | **Nuevo**. El texto dice «Única excepción» y la viñeta siguiente añade el permiso de citar en `specs/`. Parece una segunda excepción. | Justificado: la segunda viñeta no permite ningún valor nuevo; solo deja nombrar en documentos los que ya cubre la excepción. Se puede pulir la redacción en una próxima enmienda PATCH. |
| D3 | Duplicación | LOW | T043 («entrar también como `ana` y `pepe`») frente al quickstart §0.4 | **Nuevo**, consecuencia de A2. La verificación final repite un paso que ya está en el quickstart. | Justificado: T043 consiste en recorrer el quickstart §0; la mención explícita no cambia el resultado. |

**Revisión de efectos secundarios sin hallazgo**:
- La nueva firma de `crear_usuario` (que devuelve `(id, contraseña)`) no afecta a ninguna spec, porque la de 006 escribe sus propias utilidades (D2, asignado a spec-006). Se ha añadido una nota en D2.
- La pieza `BJ-7` no choca con el quickstart §6, que da de alta `FA-9`, ni con el `CHECK` de `pieza`.
- El texto del menú de A1 y A2 coincide con la lista `{texto, ruta, roles}` de T034.

## Estado acumulado (30 + 3 hallazgos)

| Estado | Hallazgos |
|--------|-----------|
| Cerrados y verificados (18) | analyze-2: K1, I1, I2, I4, G1, G2, G3, G4, G5, T1, T2, T3 · analyze-3: K2, I3, A1, A2, A3, G9 |
| Cerrados como justificados (6) | I6, I8, G7, G8 · nuevos: I12, D3 |
| Asignados a una spec (8) | I10 (spec-000, T001), I11 (spec-001), G6 (spec-002), I5 y D1 (spec-004), D2 (spec-006), I9 (spec-007), I7 (spec-008) |
| **Abiertos** (1) | **K3** |

## Cobertura

Sin cambios respecto a analyze-2: los 6 FR, los 2 SC, los 2 casos límite y el supuesto tienen
tareas (100 %). A2 mejora la cobertura de SC-002 en el criterio de terminado (quickstart §0.4).

## Métricas

- Requisitos analizados: 8 (6 FR + 2 SC), más 2 casos límite y 1 supuesto
- Tareas: 43
- Cobertura: 100 %
- Hallazgos nuevos: 3 · **CRITICAL: 1** (K3) · HIGH: 0 · MEDIUM: 0 · LOW: 2 (I12, D3; justificados)
- Ambigüedades: 0 · Duplicaciones: 1 · Problemas con la constitución: 1
- Evolución: analyze-1 → 25 (1 CRITICAL, 1 HIGH); analyze-2 → 18 (1 CRITICAL); analyze-3 → 1 abierto (1 CRITICAL)

## Siguientes pasos

1. **Antes de `/speckit-implement`**: resolver **K3**. Es un cambio de una línea en el quickstart (recomendado) o quitar el token de los documentos de `specs/`. Siguiendo tu indicación, no lo he corregido porque no es un hallazgo bajo.
2. I12 y D3 quedan justificados en [hallazgos.md](hallazgos.md), sin cambios.
3. Con K3 cerrado, la base común puede pasar a `/speckit-implement`. Los 8 asignados se resuelven en las specs correspondientes.

¿Quieres que aplique la corrección recomendada de K3 (publicar el token demo en el quickstart)?
