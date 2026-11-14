# Demo en vídeo

Contenido de `DemoSSD_GXX.zip` (apartado 6.2 del enunciado):

- **`index.html`**: la página de la demo. Se abre con doble clic (no necesita servidor). Tiene el
  vídeo y, al lado, las 8 historias con sus escenas: pulsar una escena salta a ella, y «Reproducir»
  en una historia reproduce solo esa historia y se para al terminar.
- **`demo.mp4`**: el vídeo (H.264, 1280 × 800).
- **`capitulos.json`**: el momento del vídeo en que empieza cada historia y cada escena, con sus
  criterios (`USn-m`). La página ya lo lleva dentro; el fichero se deja para consultarlo.

El vídeo sigue el [quickstart común](../specs/000-base-comun/quickstart.md) §1–§8 con los datos de
demostración. Cada escena lleva un subtítulo con su historia y sus criterios de aceptación, y un punto
rojo marca dónde se pulsa. Lo que hace «otra persona» a la vez (otra pestaña, otro navegador) se hace
entre bastidores y el subtítulo lo dice: por ejemplo, abrir la misma cita en dos pestañas, responder
al presupuesto en otra pestaña o los cambios que llegan al tablero de Paco. Lo que no se aprecia en un
vídeo (concurrencia, medianoche, el límite exacto del 10 %…) lo comprueban las pruebas automáticas.

## Volver a grabarla

Hace falta Docker, la interfaz compilada (`cd frontend && npm run build`), el servidor compilado
(`cd backend && cargo build`), Node con `playwright` instalado en alguna carpeta y `ffmpeg`.

1. Una base **nueva**, recién migrada, con los datos demo cargados **el mismo día** de la grabación
   (las citas demo son de hoy). Para no tocar la base de desarrollo, usar otra:
   ```bash
   export DATABASE_URL=postgres://<usuario>:<contraseña>@localhost:5432/taller_demo
   cd backend && cargo sqlx database create && cargo sqlx migrate run && cd ..
   docker compose exec -T postgres sh -c 'psql -q -v ON_ERROR_STOP=1 -U "$POSTGRES_USER" -d taller_demo' < backend/seed/demo.sql
   ```
2. Un Mailpit con la bandeja vacía, para que en el vídeo solo salgan los correos de la demo:
   ```bash
   docker run -d --name mailpit-demo -p 1026:1025 -p 8026:8025 axllent/mailpit:v1.27
   export SMTP_URL=smtp://127.0.0.1:1026 MAILPIT=http://127.0.0.1:8026
   ```
3. Opcional: `export TALLER_LOGO=<ruta absoluta a demo/logo-taller.png>` para que la factura salga con logo
   (un logo genérico hecho para la demo).
4. Grabar (el script arranca y para el servidor él mismo; tarda unos 15 minutos):
   ```bash
   cp demo/grabar-demo.mjs <carpeta con playwright>/
   cd <carpeta con playwright>
   FFMPEG=<ruta a ffmpeg> node grabar-demo.mjs <raíz del repositorio> <raíz del repositorio>/demo
   ```
   Con `RITMO=0.2` se ensaya cinco veces más deprisa.
5. Construir la página: `node demo/construir-pagina.mjs`.

El reloj del vídeo no avanza exactamente igual que el del script; al terminar, `grabar-demo.mjs`
localiza en el vídeo los rótulos de cada historia y ajusta con ellos los tiempos de `capitulos.json`.
