# Registro de decisiones del clarify · spec-002 · Recibir el vehículo y abrir la orden de reparación (HU-02)

**Sesión**: 2026-11-02 · **Spec**: [spec.md](spec.md) (sección «Clarifications», sesión 2026-11-02)
· **Constitución**: 1.2.1

Cada pregunta se hizo de una en una, con una opción recomendada. La respuesta de cada una está
integrada en la spec, en los requisitos y escenarios que se indican. Q1 y Q2 cierran los
hallazgos I2 e I6 de spec-001 ([hallazgos](../001-pedir-cita-en-linea/hallazgos.md)).

| ID | Historia y criterio afectados | Pregunta | Opciones consideradas | Respuesta | Justificación | Autora | Revisor |
|----|-------------------------------|----------|-----------------------|-----------|---------------|--------|---------|
| Q1 | HU-02 · FR-001, escenario 6 de la US1, Assumptions · hallazgo I2 de spec-001 | Si un cliente llega al taller sin cita, ¿qué puede hacer la recepción ese mismo día, dado que spec-001 solo ofrece días desde mañana (FR-006) y no admite una segunda cita pendiente (FR-010)? | A) en la v1 no se atiende sin cita el mismo día: el cliente pide cita (desde mañana) y la orden se abre ese día; se quita la suposición y queda como trabajo futuro; B) recepción, con sesión, puede pedir en `/cita` una cita para hoy (cambia FR-006 de spec-001 solo para recepción y jefe) y abrir la orden enseguida; C) la orden se puede abrir sin cita, directamente con la matrícula (`orden.cita_id` deja de ser obligatoria) | **A** (la recomendada) | La descripción del sistema dice que «La entrada natural es la cita» y que «El día acordado, la persona de recepción recibe el vehículo y abre la orden de reparación»: la orden se abre al recibir un coche que tiene cita, y ninguna HU pide atender sin ella (principio III). **C se descarta** porque contradice el criterio de HU-02 «La orden queda vinculada a la cita, al vehículo y al mecánico asignado»: una orden sin cita no cumpliría ese vínculo. **B se descarta** porque introduce citas para el mismo día, que ninguna HU pide, y obliga a reabrir FR-006 de spec-001 para un caso que la descripción no contempla. La decisión se comprueba en la aplicación: el vehículo sin cita no aparece en la lista de hoy y el formulario solo ofrece días desde mañana (principio IV). Cierra I2 | Persona A | Persona B (pendiente) |
| Q2 | HU-02 · FR-001, escenario 5 de la US1, casos límite · hallazgo I6 de spec-001 | ¿Se puede abrir la orden de una cita cuya fecha no es hoy (futura, o pasada y sin orden)? | A) solo el mismo día de la cita; en otro día no hay «Abrir orden» y el servidor lo rechaza con un mensaje propio; B) el día de la cita o después (el cliente llegó tarde), nunca antes; C) cualquier día, antes o después | **A** (la recomendada). Mensaje: «Solo se puede abrir la orden el día de la cita» | La descripción del sistema sitúa la recepción del coche y la apertura de la orden en «el día acordado», y dice que «El taller solo puede recibir cada día un número determinado de coches»: el cupo de spec-001 reserva una plaza para ese día concreto. Abrir la orden otro día haría entrar un coche en un día cuyo cupo no lo contaba, así que se descartan B y C. Un cliente que no vino el día de su cita no queda bloqueado: una cita pasada deja de contar como pendiente (spec-001 FR-010) y puede pedir otra. El rechazo tiene un texto fijo que se comprueba en pantalla (principio IV). Cierra I6; el quickstart común §2 necesitará una cita de hoy (se resuelve en el plan) | Persona A | Persona B (pendiente) |
| Q3 | HU-02 · FR-005, escenario 7 de la US1, entidad Mecánico, casos límite | ¿A quién se puede asignar como mecánico al abrir la orden? | A) solo usuarios con rol mecánico y activos; con otro valor, «Elige un mecánico de la lista»; B) usuarios activos con rol mecánico o jefe de taller; C) cualquier usuario con rol mecánico, activo o de baja | **A** (la recomendada) | La descripción del sistema distingue a los mecánicos del jefe: el taller tiene «seis mecánicos» y Paco «lo dirige»; el jefe no es uno de los mecánicos a los que se asigna el trabajo, así que **B se descarta**: además, una orden asignada al jefe falsearía la carga por mecánico del tablero (spec-008). **C se descarta** porque un mecánico dado de baja no puede entrar en la aplicación y, por tanto, no podría imputar tiempo ni piezas en su orden (spec-005): la orden quedaría bloqueada. Con A, la lista y el mensaje se comprueban en pantalla (principio IV) | Persona A | Persona B (pendiente) |
| Q4 | HU-02 · FR-003, escenario 8 de la US1, casos límite | ¿Es obligatorio rellenar los daños visibles, y cómo se registra un coche que no tiene ninguno? | A) obligatorio; sin daños se escribe «Sin daños visibles»; vacío, «Describe los daños visibles o escribe “Sin daños visibles”»; B) opcional; vacío se guarda como «Sin daños visibles»; C) una casilla «Sin daños visibles» que, marcada, hace innecesario el texto | **A** (la recomendada) | La descripción del sistema dice que en la orden se anotan «los golpes o arañazos visibles, para evitar discusiones posteriores». Para que sirva de prueba, la orden tiene que dejar constancia expresa también cuando no hay daños. **B se descarta** porque un olvido de la recepcionista quedaría registrado como «sin daños», justo la constancia que luego se discutiría. **C se descarta** porque añade un control que la HU no pide para obtener lo mismo que A (principios I y III). El error y su texto se comprueban en pantalla (principio IV) | Persona A | Persona B (pendiente) |
| Q5 | HU-02 · FR-008, escenario 4 de la US1, casos límite | En la lista de citas del día, ¿qué se ve en una cita que ya tiene orden, y qué ocurre si alguien intenta abrirle una segunda? | A) la cita muestra «Orden Nº 42» como enlace a la orden; un segundo intento no se guarda, no consume número y muestra «Esta cita ya tiene una orden (Nº 42)»; B) solo se oculta «Abrir orden»; un segundo intento muestra «Esta cita ya tiene una orden»; C) el botón sigue visible y, si la cita ya tiene orden, lleva a esa orden sin error | **A** (la recomendada) | Que un intento rechazado no consuma número protege FR-002 (número correlativo sin huecos), también cuando dos recepcionistas intentan abrir la orden de la misma cita a la vez. Mostrar el número de la orden existente, en la lista y en el mensaje, dice a recepción qué coches ya están dentro. **C se descarta** porque ocultaría a la segunda recepcionista que la cita ya estaba atendida: creería haber abierto la orden ella. **B se descarta** porque obliga a buscar la orden existente sin decir cuál es. El enlace, el mensaje y la numeración se comprueban en la aplicación (principio IV) | Persona A | Persona B (pendiente) |

## Criterios de aceptación resultantes

Cada criterio está escrito igual que el escenario de [spec.md](spec.md) que se indica entre
corchetes; los escenarios marcados como «nuevo» se añadieron a la spec al preparar este registro,
porque hasta entonces la decisión solo figuraba como caso límite, requisito o suposición. Los casos
límite equivalentes siguen en la spec y no contradicen ningún criterio.

### Q1 · Cliente sin cita (FR-001)

- **Dado** un cliente llega hoy sin cita, **cuando** la recepcionista consulta las citas de hoy y
  después pide una cita para su vehículo en el formulario público, **entonces** ese vehículo no
  aparece en la lista de hoy, así que no hay ninguna orden que abrir, y el formulario solo ofrece
  días a partir de mañana. [US1, escenario 6, nuevo]

### Q2 · Solo el día de la cita (FR-001)

- **Dado** una cita para mañana o una cita de ayer sin orden, **cuando** la recepcionista consulta
  las citas de ese día, **entonces** no ve «Abrir orden» en esa cita; y si se intenta abrir la
  orden de todas formas, no se guarda y aparece «Solo se puede abrir la orden el día de la cita».
  [US1, escenario 5]

### Q3 · Mecánico asignado (FR-005)

- **Dado** los mecánicos activos Luis y Marta, y el mecánico Pedro dado de baja, **cuando** la
  recepcionista abre la orden de una cita de hoy, **entonces** la lista de mecánicos ofrece a Luis
  y a Marta, sin Pedro ni usuarios de otros roles (recepción, almacén o jefe de taller); y si Marta
  se da de baja mientras el formulario la mostraba y se guarda con ella, la orden no se guarda, no
  se consume ningún número y aparece «Elige un mecánico de la lista». [US1, escenario 7, nuevo]

### Q4 · Daños visibles (FR-003)

- **Dado** una cita de hoy, **cuando** la recepcionista deja vacía la descripción de los daños (o
  solo con espacios) y guarda, **entonces** la orden no se guarda, no se consume ningún número y
  aparece «Describe los daños visibles o escribe “Sin daños visibles”»; y si escribe «Sin daños
  visibles», la orden se guarda con ese texto. [US1, escenario 8, nuevo]

### Q5 · Cita que ya tiene orden (FR-008)

- **Dado** una cita de hoy que ya tiene la orden Nº 42, **cuando** la recepcionista consulta las
  citas de hoy, **entonces** esa cita muestra «Orden Nº 42» como enlace a la orden, en lugar de
  «Abrir orden»; y si en otra pestaña que aún mostraba «Abrir orden» se intenta abrir otra, no se
  guarda, no se consume ningún número y aparece «Esta cita ya tiene una orden (Nº 42)». [US1,
  escenario 4]

## Comando y versión

- **Comando**: `/speckit-clarify` con este argumento:

  > Revisa las ambigüedades de HU-02 en specs/002-abrir-orden-reparacion/spec.md, especialmente:
  > qué pasa con un cliente que llega sin cita, dado que spec-001 FR-006 solo ofrece días desde
  > mañana (hallazgo I2 de specs/001-pedir-cita-en-linea/hallazgos.md); si se puede abrir la orden
  > de una cita de un día distinto al de hoy, futuro o pasado (hallazgo I6); quién puede abrir la
  > orden; qué datos son válidos (kilómetros, daños, fecha de entrega prometida, mecánico
  > asignado); y qué ocurre ante estados incompatibles (cita que ya tiene orden, vehículo sin
  > visitas anteriores). Formula las preguntas de una en una para concretar criterios de
  > aceptación verificables. Respeta la constitución (principios III y V).

  Quién puede abrir la orden no se preguntó: ya lo fija la base común (recepción, y el jefe de
  taller, que puede hacer todo lo que hacen los demás roles). Los textos de los mensajes de Q2–Q5
  los propuso el asistente al integrar cada respuesta. Después de la sesión, a petición de la
  autora, se cambió la fecha del ejemplo del escenario 1 de la US1 por una relativa («dentro de 3
  días»), porque la fecha fija ya era anterior a hoy, y se añadieron a la spec los escenarios 6, 7
  y 8 de la US1 para que cada decisión tenga su criterio «Dado… Cuando… Entonces…». Quedan para el
  plan los textos de los errores de km vacío o negativo y de fecha de entrega anterior a hoy.
- **Spec Kit**: plantillas del proyecto inicializadas con la versión 1.0.6
  (`.specify/init-options.json` e `.specify/integration.json`). La CLI `specify` no estaba en el
  PATH durante esta sesión, así que no se pudo consultar su versión instalada; el registro de
  spec-001 (2026-10-22) anotó 1.0.8.dev0.
