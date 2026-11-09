# T028 · Cronometraje humano de SC-001

- **Fecha**: 09/11/2026 · **Quién**: Persona A, como cliente.
- **Navegador**: Chrome, en español.
- **Criterio** (SC-001): el cliente acepta o rechaza un presupuesto en **menos de 1 minuto** desde que
  abre el enlace.
- **Resultado**: **13 segundos**. ✅ Se cumple SC-001.

## Qué se midió

El paso 2 del quickstart común §4: desde el clic en el enlace del correo de Mailpit, en la primera
pestaña, hasta ver el mensaje con la fecha y la hora. Por en medio están «Aceptar», la confirmación
dentro de la página y «Sí, aceptar». La persona usó un cronómetro; no vale el tiempo de un navegador
automático.

| Dato | Valor |
|---|---|
| Orden | Nº 4, `4444FFF` (Volkswagen Golf) |
| Presupuesto | Inicial, 133,10 € (110,00 € + 23,10 € de IVA), válido hasta el 24/11/2026 |
| Correo | Mailpit, 09/11/2026 a las 11:55, para el correo ficticio del cliente del seed |
| Mensaje al terminar | «Aceptado el 09/11/2026 a las 12:04» |
| Tiempo | **13 s** |

## Capturas

- [correo-enlace.png](correo-enlace.png): el correo en Mailpit, con el enlace del presupuesto.
- [aceptado-13s.png](aceptado-13s.png): la página pública con «Aceptado el 09/11/2026 a las 12:04».

## Desviaciones

- **Otra orden**: se usó la orden Nº 4 (`4444FFF`) y no la de 1313MMM del paso 2. La página y el recorrido
  son los mismos y el total también es 133,10 €. Así no hizo falta recargar la base de datos ni repetir
  el §2 y el §3, como pedía T028 una vez hecho T027. Decisión de la Persona A.
- **Un intento descartado**: un intento anterior dio **1 minuto exacto**. Se descarta porque no midió
  solo ese tramo: probablemente incluyó abrir el correo, que queda fuera de SC-001. Se deja constancia
  aquí para que no se pierda.
