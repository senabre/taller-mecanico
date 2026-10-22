# Registro de decisiones del clarify · spec-001 · Pedir cita en línea (HU-01)

**Sesión**: 2026-10-22 · **Spec**: [spec.md](spec.md) (sección «Clarifications», sesión 2026-10-22)
· **Constitución**: 1.2.1

Cada pregunta se hizo de una en una, con una opción recomendada. La respuesta de cada una está
integrada en la spec, en los requisitos y escenarios que se indican.

| ID | Historia y criterio afectados | Pregunta | Opciones consideradas | Respuesta | Justificación | Autora | Revisor |
|----|-------------------------------|----------|-----------------------|-----------|---------------|--------|---------|
| Q1 | HU-01 · FR-002, escenario 5 de la US1, casos límite · hallazgo I11 de la base común | ¿Qué matrículas acepta el formulario de cita como válidas, una vez normalizadas (sin espacios ni guiones y en mayúsculas)? | A) solo letras y cifras, de 2 a 10 caracteres; B) solo formato español actual (4 cifras + 3 letras); C) formato actual o provincial antiguo; D) cualquier combinación no vacía de letras y cifras, sin límite (lo que hace hoy `normalizar_matricula`) | **A** (la recomendada). Si no cumple: «La matrícula no es válida», no se busca el vehículo y no se deja confirmar | La descripción del sistema dice que «La matrícula es lo que identifica al vehículo», sin fijar ningún formato, y el taller puede recibir vehículos antiguos o extranjeros. Por eso se descartan B y C, que rechazarían esas matrículas. Pedir solo letras y cifras con una longitud acotada es la regla más simple que admite cualquier vehículo (principio I) sin añadir patrones por país que ninguna HU pide (principio III). D se descarta porque no fija una longitud y no permitiría comprobar el error. Con A, el error se comprueba en pantalla (principio IV). Cierra I11 | Persona A | Persona B (pendiente) |
| Q2 | HU-01 · FR-004, FR-004a (nuevo), escenarios 3 y 4 de la US2, casos límite, Assumptions | ¿Qué teléfono y qué correo electrónico acepta el formulario cuando el vehículo es nuevo? | A) los dos obligatorios; teléfono de 9 a 15 cifras con «+» opcional; correo con una «@» y un dominio con punto; un mensaje por campo; el teléfono sirve para que recepción llame al cliente; B) como A, pero teléfono solo español de 9 cifras; C) solo comprobar que no estén vacíos; D) no pedir el teléfono porque la spec no le daba ningún uso | **A** (la recomendada) | La spec pedía el teléfono sin describir para qué, y el principio V exige que cada dato pedido tenga un uso: ahora lo tiene (recepción llama al cliente si hay algún problema con la cita). El correo ya tenía uso (avisos de spec-004 y spec-007). La validación es simple y admite números extranjeros. Cada mensaje se comprueba en pantalla (principio IV). **D se descarta** porque, aunque la descripción del sistema no menciona el teléfono, el dato ya forma parte del modelo común acordado por las dos personas (tabla `cliente` de [data-model.md](../000-base-comun/data-model.md), teléfono obligatorio) y ahora tiene un uso documentado en la spec. **B se descarta** por coherencia con Q1: el taller puede recibir clientes extranjeros, que no tienen un teléfono español de 9 cifras. C no permitiría comprobar ningún error de formato | Persona A | Persona B (pendiente) |
| Q3 | HU-01 · FR-010 (nuevo), escenarios 4 y 6 de la US1, entidad Cita | ¿Puede una misma matrícula tener más de una cita pendiente (con fecha de hoy o posterior)? | A) una sola cita pendiente; aviso sin la fecha y no se deja confirmar; B) como A, pero el aviso muestra la fecha; C) varias citas, nunca dos el mismo día; D) sin límite | **A** (la recomendada), pero **con el aviso recortado** a «Este vehículo ya tiene una cita pendiente.» | La descripción del sistema dice que «El taller solo puede recibir cada día un número determinado de coches»: cada plaza del cupo cuenta. Cancelar o modificar citas queda fuera de la spec, así que una cita repetida ocuparía una plaza del cupo que nadie puede liberar: una sola cita pendiente lo evita con la regla más simple (principio I). No se muestra la fecha para que quien escriba una matrícula ajena no sepa cuándo irá ese coche al taller (principio V). **Parte rechazada de la recomendación:** el aviso original añadía «Si necesitas cambiarla, llama al taller.». Se quitó después de la sesión porque la v1 no permite cancelar ni modificar una cita, tampoco a recepción: el mensaje prometería un cambio que el sistema no ofrece (principio III) | Persona A | Persona B (pendiente) |
| Q4 | HU-01 · FR-009, escenarios 3, 8, 9 y 10 de la US1, casos límite | ¿Quién puede ver la lista de citas de un día y qué datos muestra cada cita? | A) recepción y jefe, con sesión; hoy por defecto y otro día a elegir; matrícula, marca, modelo, motivo, nombre y teléfono, en el orden en que se pidieron; «No hay citas para este día»; los demás roles no la ven; B) como A y además el correo; C) también los mecánicos, sin datos personales; D) recepción y jefe, solo matrícula y motivo | **A** (la recomendada) | La descripción del sistema dice que «El día acordado, la persona de recepción recibe el vehículo y abre la orden de reparación»: la lista es para recepción, que necesita esos datos para recibir el coche y llamar al cliente, que es el uso del teléfono acordado en Q2. El correo no se muestra porque solo lo usan los avisos automáticos (principio V). El jefe de taller puede hacer todo lo que hacen los demás roles (FR-002 de la base común). El acceso por rol se comprueba entrando con un usuario de cada rol (principio IV) | Persona A | Persona B (pendiente) |
| Q5 | HU-01 · FR-003, escenario 7 de la US1, casos límite, Assumptions | Si la matrícula ya existe y los datos de contacto han cambiado (otro teléfono o correo, o un nuevo dueño), ¿qué permite hacer el formulario? | A) no se piden ni se pueden cambiar; solo día y motivo; la cita queda asociada al propietario registrado; aviso «Si tus datos de contacto han cambiado, llama al taller»; B) enlace opcional cuyos datos sustituyen a los del propietario; C) como B, pero los datos se guardan solo en la cita; D) pedir siempre los datos y crear un propietario nuevo si no coinciden | **A sin el aviso** de llamar al taller. Además se añade en Assumptions que en la v1 ningún rol puede editar los datos de un cliente ya registrado (trabajo futuro) | Si se pudieran cambiar los datos desde una página pública sabiendo solo la matrícula, cualquiera podría desviar a su correo los avisos de otro cliente. La descripción del sistema dice que el cliente recibe el enlace del presupuesto «desde el que pueda aceptarlo o rechazarlo», así que quien desviara el correo podría aceptar o rechazar el presupuesto de otra persona (spec-004, principio V). Editar clientes no lo pide ninguna HU (principio III). **Parte rechazada de la recomendación:** el aviso «Si tus datos de contacto han cambiado, llama al taller». Se rechazó porque en la v1 ningún rol, tampoco recepción, puede editar los datos de un cliente: el aviso prometería un cambio que nadie puede hacer en la aplicación | Persona A | Persona B (pendiente) |

## Criterios de aceptación resultantes

Cada criterio está escrito igual que el escenario de [spec.md](spec.md) que se indica entre
corchetes; los escenarios marcados como «nuevo» se añadieron a la spec al preparar este registro,
porque hasta entonces la decisión solo figuraba como caso límite. Los casos límite equivalentes
siguen en la spec y no contradicen ningún criterio.

### Q1 · Matrícula válida (FR-002)

- **Dado** el formulario de cita abierto, **cuando** el cliente escribe «12#4», «A» o
  «ABCDE123456» (11 caracteres), **entonces** ve «La matrícula no es válida», no se muestran marca
  ni modelo ni se piden datos de vehículo nuevo, y no puede confirmar; en cambio, «AB» y
  «ABCDE12345» (10 caracteres) se aceptan como matrículas. [US1, escenario 5, nuevo]

### Q2 · Teléfono y correo (FR-004, FR-004a)

- **Dado** una matrícula que no existe y el resto de datos, día y motivo rellenados, **cuando** el
  cliente escribe el teléfono «12345» o el correo «ana@correo» (sin punto en el dominio), o deja
  vacío alguno de los dos, e intenta confirmar, **entonces** ve «El teléfono no es válido» o «El
  correo no es válido» en el campo afectado y no puede confirmar. [US2, escenario 3, nuevo]
- **Dado** una matrícula que no existe y el resto de datos, día y motivo rellenados, **cuando** el
  cliente escribe el teléfono «+34 612 345 678» o «612-345-678» y el correo «ana@correo.es» y
  confirma, **entonces** se crean el vehículo y la cita y ve la confirmación con fecha y motivo.
  [US2, escenario 4, nuevo]

### Q3 · Una sola cita pendiente por matrícula (FR-010)

- **Dado** el vehículo 1234 BCD ya tiene una cita con fecha de hoy o posterior, **cuando** el
  cliente escribe «1234BCD», **entonces** ve exactamente el aviso «Este vehículo ya tiene una cita
  pendiente.», sin la fecha de esa cita ni ninguna invitación a llamar al taller o a cambiarla, y
  no puede confirmar otra. [US1, escenario 4]
- **Dado** el vehículo 1234 BCD tuvo una cita con fecha anterior a hoy y no tiene ninguna otra,
  **cuando** el cliente escribe «1234BCD», elige un día disponible, escribe el motivo y confirma,
  **entonces** ve la confirmación con la fecha y el motivo de la nueva cita. [US1, escenario 6,
  nuevo]

### Q3 (ajuste posterior) · Texto del aviso de FR-010

- **Dado** el vehículo 1234 BCD ya tiene una cita con fecha de hoy o posterior, **cuando** el
  cliente escribe «1234BCD», **entonces** ve exactamente el aviso «Este vehículo ya tiene una cita
  pendiente.», sin la fecha de esa cita ni ninguna invitación a llamar al taller o a cambiarla, y
  no puede confirmar otra. [US1, escenario 4, ampliado con «exactamente» y «ninguna invitación a
  llamar al taller o a cambiarla» para recoger este ajuste]

### Q4 · Lista de citas de recepción (FR-009)

- **Dado** la cita confirmada, **cuando** el personal de recepción consulta las citas de ese día,
  **entonces** la cita aparece con matrícula, marca, modelo, motivo, nombre y teléfono del
  propietario, y sin el correo. [US1, escenario 3]
- **Dado** un usuario con rol recepción o jefe de taller con sesión iniciada, **cuando** abre la
  lista de citas, **entonces** ve las citas de hoy y puede elegir otro día; si dos citas de ese día
  se pidieron primero para 1234 BCD y después para 5678 CDF, aparecen en ese orden. [US1,
  escenario 8, nuevo]
- **Dado** un usuario con rol recepción con sesión iniciada, **cuando** elige en la lista un día
  sin citas, **entonces** ve «No hay citas para este día». [US1, escenario 9, nuevo]
- **Dado** un usuario con rol mecánico o almacén con sesión iniciada, o alguien sin sesión
  iniciada, **cuando** busca la lista de citas en el menú o intenta abrir su página, **entonces**
  no ve la opción en el menú y no puede abrir la lista. [US1, escenario 10, nuevo]

### Q5 · Vehículo conocido con datos de contacto distintos (FR-003)

- **Dado** el vehículo 1234 BCD ya registrado con un propietario, **cuando** el cliente escribe
  «1234BCD», **entonces** el formulario solo pide día y motivo, sin campos de nombre, teléfono o
  correo ni ningún aviso para cambiarlos; y, al confirmar, la lista de recepción muestra la cita
  con el nombre y el teléfono del propietario registrado, sin cambios. [US1, escenario 7, nuevo]

## Comando y versión

- **Comando**: `/speckit-clarify` con este argumento:

  > Revisa las ambigüedades de HU-01 en specs/001-pedir-cita-en-linea/spec.md, especialmente:
  > qué datos son válidos (matrícula con formato inválido — hallazgo I11 de
  > specs/000-base-comun/hallazgos.md —, teléfono y correo), si una misma matrícula puede tener
  > varias citas (el mismo día o futuras), quién puede ver la lista de citas de recepción y qué
  > datos muestra, y qué ocurre si un vehículo conocido trae datos de contacto distintos. Formula
  > las preguntas de una en una para concretar criterios de aceptación verificables. Respeta la
  > constitución (principios III y V).

  Después de la sesión se aplicaron a mano, a petición de la autora, dos ajustes: el punto de
  Assumptions sobre no editar clientes y el recorte del aviso de FR-010. Al preparar este registro
  se añadieron a la spec los escenarios 5 a 10 de la US1 y 3 y 4 de la US2, para que cada decisión
  tenga su criterio «Dado… Cuando… Entonces…».
- **Spec Kit**: plantillas del proyecto inicializadas con la versión 1.0.6
  (`.specify/init-options.json`); CLI `specify` instalada: 1.0.8.dev0.
