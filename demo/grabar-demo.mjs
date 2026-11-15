// Demo en vídeo de Talleres Rueda: las 8 historias de usuario y sus escenarios, siguiendo el
// quickstart común (§1–§8). Graba una sola pestaña con Playwright (.webm), apunta el momento de cada
// escena en capitulos.json y, al final, ffmpeg la convierte en demo.mp4.
//
// Requisitos: docker compose up (Postgres y Mailpit), una base recién migrada y con demo.sql cargado
// ese mismo día (DATABASE_URL), la interfaz compilada (frontend/build) y el servidor compilado
// (backend/target/debug/taller.exe). El script arranca el servidor él mismo, porque en HU-08 lo para
// y lo vuelve a arrancar.
//
// Uso (desde una carpeta con playwright instalado):
//   DATABASE_URL=… TALLER_LOGO=… node grabar-demo.mjs <raíz del repositorio> <carpeta de salida>
// Variables opcionales: RITMO (1 = normal; 0.2 para ensayar deprisa), FFMPEG (ruta de ffmpeg).
import { chromium, request } from 'playwright';
import { spawn, spawnSync, execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

const RAIZ = process.argv[2];
const SALIDA = process.argv[3];
const RITMO = Number(process.env.RITMO ?? 1);
const BASE = 'http://127.0.0.1:3000';
const MAILPIT = process.env.MAILPIT ?? 'http://127.0.0.1:8025'; // con SMTP_URL apuntando a su SMTP
const ANCHO = 1280;
const ALTO = 800;
fs.mkdirSync(SALIDA, { recursive: true });

// ---------------------------------------------------------------------------------------------
// Servidor
// ---------------------------------------------------------------------------------------------
let servidor = null;
async function arrancarServidor() {
  servidor = spawn(path.join(RAIZ, 'backend/target/debug/taller.exe'), [], {
    cwd: path.join(RAIZ, 'backend'),
    env: process.env,
    stdio: 'ignore'
  });
  for (let i = 0; i < 100; i++) {
    try {
      const r = await fetch(`${BASE}/entrar`);
      if (r.ok) return;
    } catch {}
    await new Promise((r) => setTimeout(r, 200));
  }
  throw new Error('El servidor no arranca');
}
async function pararServidor() {
  servidor.kill();
  for (let i = 0; i < 50; i++) {
    try {
      await fetch(`${BASE}/entrar`);
    } catch {
      return;
    }
    await new Promise((r) => setTimeout(r, 200));
  }
}

// ---------------------------------------------------------------------------------------------
// Fechas (Madrid)
// ---------------------------------------------------------------------------------------------
const iso = (d) => new Intl.DateTimeFormat('en-CA', { timeZone: 'Europe/Madrid' }).format(d);
const HOY = iso(new Date());
const masDias = (n) => iso(new Date(Date.now() + n * 86_400_000));
const es = (isoTxt) => isoTxt.split('-').reverse().join('/');

// ---------------------------------------------------------------------------------------------
// Peticiones «entre bastidores»: lo que hace otra persona a la vez (otra pestaña, otro navegador)
// ---------------------------------------------------------------------------------------------
const apis = {};
async function api(usuario) {
  if (!apis[usuario]) {
    const c = await request.newContext({ baseURL: BASE });
    const r = await c.post('/api/sesion', { data: { login: usuario, contrasena: 'demo1234' } });
    if (!r.ok()) throw new Error(`No entra ${usuario}: ${r.status()}`);
    apis[usuario] = c;
  }
  return apis[usuario];
}
async function json(r) {
  if (!r.ok()) throw new Error(`${r.url()} → ${r.status()} ${await r.text()}`);
  return r.json();
}
const llamar = {
  get: async (u, ruta) => json(await (await api(u)).get(`/api${ruta}`)),
  post: async (u, ruta, data) => json(await (await api(u)).post(`/api${ruta}`, { data })),
  put: async (u, ruta, data) => json(await (await api(u)).put(`/api${ruta}`, { data }))
};
async function ordenPorNumero(numero) {
  const lista = await llamar.get('paco', '/ordenes');
  return (Array.isArray(lista) ? lista : lista.ordenes).find((o) => o.numero === numero);
}
async function citaDeHoy(matricula) {
  const citas = await llamar.get('ana', `/citas?fecha=${HOY}`);
  return (Array.isArray(citas) ? citas : citas.citas).find((c) => c.matricula === matricula);
}

// Correos (Mailpit): el más reciente cuyo asunto contiene el texto, enviado después de empezar.
const INICIO = new Date();
async function correo(asunto) {
  for (let i = 0; i < 50; i++) {
    const r = await (await fetch(`${MAILPIT}/api/v1/messages?limit=50`)).json();
    const m = r.messages.find((x) => x.Subject.includes(asunto) && new Date(x.Created) >= INICIO);
    if (m) {
      const completo = await (await fetch(`${MAILPIT}/api/v1/message/${m.ID}`)).json();
      const enlace = (completo.Text.match(/https?:\/\/\S+\/presupuesto\/[A-Za-z0-9_-]+/) || [])[0];
      return { id: m.ID, enlace: enlace?.replace(/^https?:\/\/[^/]+/, BASE) };
    }
    await new Promise((r) => setTimeout(r, 200));
  }
  throw new Error(`No llega el correo «${asunto}»`);
}

// ---------------------------------------------------------------------------------------------
// Navegador, rótulos y capítulos
// ---------------------------------------------------------------------------------------------
const nav = await chromium.launch();
await arrancarServidor();
const contexto = await nav.newContext({
  viewport: { width: ANCHO, height: ALTO },
  locale: 'es-ES',
  timezoneId: 'Europe/Madrid',
  recordVideo: { dir: SALIDA, size: { width: ANCHO, height: ALTO } }
});
await contexto.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: BASE });
const page = await contexto.newPage();
page.setDefaultTimeout(10_000);
const T0 = Date.now();
page.on('dialog', async (d) => {
  await new Promise((r) => setTimeout(r, 1200 * RITMO));
  await d.accept();
});

// Capa propia de la demo (no es de la aplicación): subtítulo abajo, punto rojo donde se pulsa y
// rótulos a pantalla completa. Se vuelve a poner en cada carga de página.
await contexto.addInitScript(() => {
  const poner = () => {
    if (document.getElementById('demo-capa')) return;
    // La capa va en un shadow root cerrado: así los textos de los subtítulos no los encuentra
    // Playwright cuando busca textos de la aplicación.
    const anfitrion = document.createElement('div');
    anfitrion.id = 'demo-capa';
    const marca = document.createElement('style');
    marca.textContent = '.demo-marca{outline:4px solid #f59e0b !important;outline-offset:3px;border-radius:6px}';
    document.documentElement.appendChild(marca);
    const capa = anfitrion.attachShadow({ mode: 'closed' });
    capa.innerHTML = `
      <style>
        #demo-sub{position:fixed;left:50%;bottom:18px;transform:translateX(-50%);max-width:92%;
          background:rgba(15,23,42,.92);color:#fff;font:600 17px/1.35 system-ui,sans-serif;
          padding:10px 18px;border-radius:10px;z-index:2147483646;box-shadow:0 6px 24px rgba(0,0,0,.35);
          pointer-events:none;display:none}
        #demo-sub b{color:#fbbf24;font-weight:700;margin-right:8px}
        #demo-punto{position:fixed;width:22px;height:22px;margin:-11px 0 0 -11px;border-radius:50%;
          background:rgba(239,68,68,.55);border:2px solid #ef4444;z-index:2147483647;pointer-events:none;
          transition:transform .15s;left:-50px;top:-50px}
        #demo-rotulo{position:fixed;inset:0;background:linear-gradient(135deg,#0f172a,#1e3a8a);color:#fff;
          z-index:2147483645;display:none;flex-direction:column;align-items:center;justify-content:center;
          font-family:system-ui,sans-serif;text-align:center;padding:40px}
        #demo-rotulo .hu{font-size:28px;color:#fbbf24;font-weight:700;letter-spacing:.06em}
        #demo-rotulo .tit{font-size:52px;font-weight:800;margin:12px 0 18px}
        #demo-rotulo .det{font-size:22px;max-width:900px;line-height:1.45;opacity:.9}
      </style>
      <div id="demo-sub"></div><div id="demo-punto"></div>
      <div id="demo-rotulo"><div class="hu"></div><div class="tit"></div><div class="det"></div></div>`;
    document.documentElement.appendChild(anfitrion);
    const punto = capa.querySelector('#demo-punto');
    addEventListener('mousemove', (e) => { punto.style.left = e.clientX + 'px'; punto.style.top = e.clientY + 'px'; }, true);
    addEventListener('mousedown', () => (punto.style.transform = 'scale(.6)'), true);
    addEventListener('mouseup', () => (punto.style.transform = ''), true);
    window.__demoSub = (html) => {
      const s = capa.querySelector('#demo-sub');
      s.innerHTML = html;
      s.style.display = html ? 'block' : 'none';
    };
    window.__demoRotulo = (hu, tit, det) => {
      const r = capa.querySelector('#demo-rotulo');
      if (!hu) { r.style.display = 'none'; return; }
      r.querySelector('.hu').textContent = hu;
      r.querySelector('.tit').textContent = tit;
      r.querySelector('.det').textContent = det;
      r.style.display = 'flex';
    };
  };
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', poner);
  else poner();
});

let subtitulo = '';
const pintar = async () => {
  try {
    await page.waitForFunction(() => !!window.__demoSub, null, { timeout: 3000 });
    await page.evaluate((s) => window.__demoSub(s), subtitulo);
  } catch {}
};
page.on('load', pintar);

const capitulos = [];
let historiaActual = null;
const ahora = () => Math.max(0, (Date.now() - T0) / 1000);
const pausa = (ms) => page.waitForTimeout(ms * RITMO);

async function rotulo(hu, titulo, detalle) {
  historiaActual = { hu, titulo, detalle, inicio: ahora(), escenas: [] };
  capitulos.push(historiaActual);
  console.log(`\n${hu} · ${titulo}`);
  subtitulo = '';
  await pintar();
  await page.evaluate(([a, b, c]) => window.__demoRotulo?.(a, b, c), [hu, titulo, detalle]).catch(() => {});
  await pausa(4500);
  await page.evaluate(() => window.__demoRotulo?.(null)).catch(() => {});
}
async function escena(criterios, texto) {
  historiaActual.escenas.push({ t: ahora(), criterios, texto });
  console.log(`  ${criterios} · ${texto}`);
  subtitulo = `<b>${historiaActual.hu} · ${criterios}</b>${texto}`;
  await pintar();
  await pausa(1800);
}
async function nota(texto) {
  subtitulo = `<b>${historiaActual.hu}</b>${texto}`;
  await pintar();
  await pausa(2200);
}
async function ir(ruta) {
  await page.goto(ruta.startsWith('http') ? ruta : `${BASE}${ruta}`);
  // El tablero mantiene abierta una conexión de eventos: «networkidle» puede no llegar nunca.
  await page.waitForLoadState('networkidle', { timeout: 2500 }).catch(() => {});
  await pintar();
  await pausa(900);
}
async function pulsar(loc) {
  await loc.scrollIntoViewIfNeeded();
  await loc.hover();
  await pausa(450);
  await loc.click();
  await pausa(700);
}
async function escribir(loc, texto) {
  await loc.scrollIntoViewIfNeeded();
  await loc.hover();
  await loc.click();
  await loc.fill('');
  await loc.pressSequentially(String(texto), { delay: 45 * Math.max(RITMO, 0.3) });
  await pausa(250);
}
async function elegir(loc, valor) {
  await loc.scrollIntoViewIfNeeded();
  await loc.hover();
  await pausa(350);
  await loc.selectOption(valor);
  await pausa(500);
}
async function resaltar(loc, ms = 2600) {
  const l = loc.first();
  await l.scrollIntoViewIfNeeded({ timeout: 3000 }).catch(() => {});
  await l.evaluate((e) => e.classList.add('demo-marca'), null, { timeout: 3000 }).catch(() => {});
  await l.hover({ timeout: 3000 }).catch(() => {});
  await pausa(ms);
  await l.evaluate((e) => e.classList.remove('demo-marca'), null, { timeout: 3000 }).catch(() => {});
}
const ver = async (texto, ms = 2400) => {
  const l = (typeof texto === 'string' ? page.getByText(texto) : texto).first();
  await l.waitFor({ timeout: 15_000 });
  await resaltar(l, ms);
};

let usuarioActual = null;
async function entrar(usuario) {
  if (usuarioActual) {
    const salir = page.getByRole('button', { name: 'Salir' });
    if (!page.url().startsWith(BASE) || !(await salir.isVisible().catch(() => false))) {
      await page.goto(`${BASE}/`);
      await page.waitForLoadState('networkidle', { timeout: 2500 }).catch(() => {});
    }
    if (await salir.isVisible().catch(() => false)) {
      await pulsar(salir);
      await page.waitForURL(/\/entrar/, { timeout: 5000 }).catch(() => {});
    }
    await page.request.delete(`${BASE}/api/sesion`).catch(() => {});
  }
  await ir('/entrar');
  await page.locator('input[name=login]').waitFor();
  await escribir(page.locator('input[name=login]'), usuario);
  await escribir(page.locator('input[name=contrasena]'), 'demo1234');
  await pulsar(page.getByRole('button', { name: 'Entrar' }));
  await page.waitForURL((u) => !u.pathname.startsWith('/entrar'));
  await page.waitForLoadState('networkidle', { timeout: 2500 }).catch(() => {});
  await pintar();
  usuarioActual = usuario;
  await pausa(900);
}
async function ficha(numero) {
  const o = await ordenPorNumero(numero);
  await ir(`/ordenes/${o.id}`);
  return o;
}
async function verCorreo(asunto) {
  const m = await correo(asunto);
  await ir(`${MAILPIT}/view/${m.id}`);
  await pausa(3200);
  return m;
}
const formLinea = (titulo) => page.locator('form', { has: page.locator('h3', { hasText: titulo }) });
async function lineaManoObra(titulo, boton, concepto, horas) {
  const f = formLinea(titulo);
  await elegir(f.locator('select'), 'mano_obra');
  await escribir(f.locator('input[name=concepto]'), concepto);
  await escribir(f.locator('input[name=horas]'), horas);
  await pulsar(f.getByRole('button', { name: boton }));
}
async function lineaPieza(titulo, boton, buscar, referencia, unidades) {
  const f = formLinea(titulo);
  await elegir(f.locator('select'), 'pieza');
  await escribir(f.locator('#pieza'), buscar);
  await pulsar(f.getByRole('button', { name: new RegExp(referencia) }));
  await escribir(f.locator('input[name=unidades]'), unidades);
  await pulsar(f.getByRole('button', { name: boton }));
}

// =============================================================================================
// Portada
// =============================================================================================
await ir('/entrar');
historiaActual = { hu: 'Inicio', titulo: 'Talleres Rueda', detalle: '', inicio: ahora(), escenas: [] };
capitulos.push(historiaActual);
await page.evaluate(() => window.__demoRotulo('SI-26 · DEMO', 'Talleres Rueda',
  'Gestión de un taller mecánico: de la cita a la factura. 8 historias de usuario, recorridas con los datos de demostración.'));
await pausa(5000);
await page.evaluate(() => window.__demoRotulo(null));

// =============================================================================================
// HU-01 · Pedir cita en línea (spec-001)
// =============================================================================================
await rotulo('HU-01', 'Pedir cita en línea', 'El cliente pide cita por su cuenta con la matrícula y el motivo; si el coche ya vino, sus datos aparecen solos. Hay un cupo diario de coches.');
await ir('/cita');
const matricula = page.locator('input[name=matricula]');
await escena('US1-5', 'Una matrícula no válida se rechaza y no deja confirmar');
for (const m of ['12#4', 'A', 'ABCDE123456']) {
  await escribir(matricula, m);
  await page.keyboard.press('Tab');
  await ver(page.getByText('La matrícula no es válida'), 1600);
}
await escena('US1-5', '«AB» sí es válida: pide los datos del vehículo nuevo');
await escribir(matricula, 'AB');
await page.keyboard.press('Tab');
await ver(page.locator('input[name=marca]'), 1800);

await escena('US2-1', 'Matrícula nueva «1234-bcd»: pide marca, modelo, nombre, teléfono y correo');
await escribir(matricula, '1234-bcd');
await page.keyboard.press('Tab');
await pausa(800);
await escribir(page.locator('input[name=marca]'), 'Renault');
await escribir(page.locator('input[name=modelo]'), 'Clio');
await escribir(page.locator('input[name=nombre]'), 'Ana García');
await escena('US2-3', 'Un teléfono incorrecto da su mensaje');
await escribir(page.locator('input[name=telefono]'), '12345');
await escribir(page.locator('input[name=email]'), 'ana@correo.es');
const selDia = page.locator('select[name=fecha]');
const dias = await selDia.locator('option:not([disabled])').evaluateAll((os) => os.map((o) => o.value));
const diaA = dias[0];
const diaB = dias[2] ?? dias[1];
await elegir(selDia, diaA);
await escribir(page.locator('textarea[name=motivo]'), 'Ruido al frenar');
await pulsar(page.getByRole('button', { name: 'Confirmar' }));
await ver('El teléfono no es válido');
await escena('US2-3', 'Con el teléfono bien, un correo incorrecto también da su mensaje');
await escribir(page.locator('input[name=telefono]'), '612-345-678');
await escribir(page.locator('input[name=email]'), 'ana@correo');
await pulsar(page.getByRole('button', { name: 'Confirmar' }));
await ver('El correo no es válido');
await escena('US2-2 · US1-2', 'Con todo bien, se dan de alta cliente, vehículo y cita, y se ve la fecha y el motivo');
await escribir(page.locator('input[name=email]'), 'ana@correo.es');
await pulsar(page.getByRole('button', { name: 'Confirmar' }));
await ver(page.getByRole('status'), 3200);

await escena('US1-4', 'Con una cita pendiente: aviso, sin fecha y sin poder confirmar');
await ir('/cita');
await escribir(matricula, '1234BCD');
await page.keyboard.press('Tab');
await ver('Este vehículo ya tiene una cita pendiente.', 2600);
await resaltar(page.getByRole('button', { name: 'Confirmar' }), 1800);

await escena('US2-4 · US3-1', 'Otro coche nuevo para el mismo día, con teléfono «+34 612 345 678»: completa el cupo (2 coches)');
await ir('/cita');
await escribir(matricula, '5678CDF');
await page.keyboard.press('Tab');
await pausa(700);
await escribir(page.locator('input[name=marca]'), 'Toyota');
await escribir(page.locator('input[name=modelo]'), 'Corolla');
await escribir(page.locator('input[name=nombre]'), 'Pedro Ruiz');
await escribir(page.locator('input[name=telefono]'), '+34 612 345 678');
await escribir(page.locator('input[name=email]'), 'pedro@correo.es');
await elegir(selDia, diaA);
await escribir(page.locator('textarea[name=motivo]'), 'Revisión de los 60.000 km');
await pulsar(page.getByRole('button', { name: 'Confirmar' }));
await ver(page.getByRole('status'), 2200);
await escena('US3-1', `El ${es(diaA)} ya está completo: deja de ofrecerse`);
await ir('/cita');
await escribir(matricula, '4321FGH');
await page.keyboard.press('Tab');
await pausa(600);
const primerDia = await selDia.locator('option:not([disabled])').first().innerText();
await resaltar(selDia, 1200);
await nota(`El primer día que se ofrece ahora es el ${primerDia}; el ${es(diaA)} ha desaparecido`);

await escena('US1-1 · US1-7 · US1-6', 'Un coche que ya vino (1111BBB): salen marca y modelo, sin datos del cliente; solo pide día y motivo');
await ir('/cita');
await escribir(matricula, '1111BBB');
await page.keyboard.press('Tab');
await ver('Seat', 2400);
await elegir(selDia, diaB);
await escribir(page.locator('textarea[name=motivo]'), 'Cambio de aceite');
await pulsar(page.getByRole('button', { name: 'Confirmar' }));
await ver(page.getByRole('status'), 2200);

await escena('US1-8', 'Recepción ve la lista de citas de hoy por defecto');
await entrar('ana');
await ir('/citas');
await pausa(2000);
await escena('US1-3', 'Cada cita con matrícula, marca, modelo, motivo, nombre y teléfono, sin el correo, en el orden en que se pidieron');
await page.fill('input[name=dia]', diaA);
await pausa(1200);
await ver(page.locator('table'), 3200);
await escena('US1-7', 'El propietario de 1111BBB no ha cambiado: María López');
await page.fill('input[name=dia]', diaB);
await pausa(1200);
await ver(page.locator('tr', { hasText: '1111BBB' }), 2800);
await escena('US1-9', 'Un día sin citas');
await page.fill('input[name=dia]', masDias(45));
await pausa(1200);
await ver('No hay citas para este día');
await escena('US1-10', 'Un mecánico no tiene «Citas» en el menú ni ve la lista');
await entrar('luis');
await resaltar(page.locator('header nav ul'), 2000);
await ir('/citas');
await pausa(2200);

// =============================================================================================
// HU-02 · Abrir la orden de reparación (spec-002)
// =============================================================================================
await rotulo('HU-02', 'Recibir el vehículo y abrir la orden', 'El día de la cita, recepción abre la orden con número correlativo, los kilómetros y los daños visibles. El kilometraje no puede bajar.');
await entrar('ana');
await escena('US1-5', 'Solo se abre la orden el día de la cita: las de hoy tienen «Abrir orden»');
await ir('/citas');
await resaltar(page.locator('table'), 2600);
await page.fill('input[name=dia]', diaA);
await pausa(1500);
await nota('Las citas de otro día no tienen «Abrir orden»');
await page.fill('input[name=dia]', masDias(-1));
await pausa(1500);
await nota('Tampoco la de ayer (1515PPP)');
await escena('US1-6', 'Un cliente sin cita no está en la lista: no puede tener orden');
await pulsar(page.getByRole('button', { name: 'Hoy' }));
await pausa(800);
await escena('US1-7', 'Solo se ofrecen los mecánicos activos (Luis y Marta, no Pedro, que está de baja)');
await pulsar(page.locator('tr', { hasText: '1313MMM' }).getByRole('link', { name: 'Abrir orden' }));
await pausa(800);
await resaltar(page.locator('select[name=mecanico]'), 1500);
const opcionesMecanico = await page.locator('select[name=mecanico] option:not([disabled])').allInnerTexts();
await nota(`Mecánicos que se ofrecen: ${opcionesMecanico.join(', ')}`);
await escena('US1-8', 'Sin descripción de daños no se guarda');
await escribir(page.locator('input[name=km]'), '85000');
await elegir(page.locator('select[name=mecanico]'), { label: 'Luis' });
await page.fill('input[name=entrega]', masDias(3));
await pulsar(page.getByRole('button', { name: 'Guardar' }));
await ver(page.getByRole('alert').first());
await escena('US1-9', 'Otros datos incorrectos: kilómetros negativos y entrega en el pasado');
await escribir(page.locator('textarea[name=danos]'), 'Rayón puerta trasera izquierda');
await escribir(page.locator('input[name=km]'), '-5');
await pulsar(page.getByRole('button', { name: 'Guardar' }));
await ver(page.getByRole('alert').first());
await escribir(page.locator('input[name=km]'), '85000');
await page.fill('input[name=entrega]', masDias(-1));
await pulsar(page.getByRole('button', { name: 'Guardar' }));
await ver(page.getByRole('alert').first());
await escena('US1-1 · US1-2 · US1-3', 'Con todo bien: Orden Nº 10 en «Recibida», con su cita, vehículo y mecánico (los errores no gastaron número)');
await page.fill('input[name=entrega]', masDias(3));
await pulsar(page.getByRole('button', { name: 'Guardar' }));
await page.waitForURL(/\/ordenes\/\d+$/);
await pausa(800);
await ver(page.getByRole('heading', { level: 1 }), 3600);
await escena('US1-4', 'En «Citas», 1313MMM ya enlaza con su orden y no deja abrir otra');
await ir('/citas');
await ver(page.locator('tr', { hasText: '1313MMM' }), 2600);
await escena('US1-4', 'Dos pestañas con la misma cita: si otra recepcionista la abre antes, la segunda no crea otra orden');
await pulsar(page.locator('tr', { hasText: '1414NNN' }).getByRole('link', { name: 'Abrir orden' }));
const citaId = Number(new URL(page.url()).searchParams.get('cita'));
const mecanicos = await llamar.get('ana', '/mecanicos');
const idLuis = mecanicos.find((m) => m.nombre === 'Luis').id;
const idMarta = mecanicos.find((m) => m.nombre === 'Marta').id;
await escribir(page.locator('input[name=km]'), '23500');
await escribir(page.locator('textarea[name=danos]'), 'Sin daños visibles');
await elegir(page.locator('select[name=mecanico]'), { label: 'Luis' });
await page.fill('input[name=entrega]', masDias(2));
await nota('Mientras tanto, en otra pestaña, se abre la orden de esta misma cita…');
await llamar.post('ana', '/ordenes', { cita_id: citaId, km: 23500, danos: 'Sin daños visibles', mecanico_id: idLuis, fecha_entrega_prometida: masDias(2) });
await pulsar(page.getByRole('button', { name: 'Guardar' }));
await ver(page.getByText(/Esta cita ya tiene una orden \(Nº 11\)/), 3000);
await escena('US2-1', 'Un kilometraje menor que el de la visita anterior se rechaza, con ese valor');
await ir('/citas');
await pulsar(page.locator('tr', { hasText: '8888KKK' }).getByRole('link', { name: 'Abrir orden' }));
await escribir(page.locator('input[name=km]'), '30000');
await escribir(page.locator('textarea[name=danos]'), 'Sin daños visibles');
await elegir(page.locator('select[name=mecanico]'), { label: 'Marta' });
await page.fill('input[name=entrega]', masDias(1));
await pulsar(page.getByRole('button', { name: 'Guardar' }));
await ver(page.getByRole('alert').first(), 3000);
await nota('Igual o mayor que el anterior sí se acepta, y un coche sin visitas acepta cualquier kilometraje (US2-2, US2-3)');
await escena('Mis órdenes', 'El mecánico ve sus órdenes nuevas (Nº 10 y Nº 11) en «Mis órdenes»');
await entrar('luis');
await ir('/ordenes');
await ver(page.locator('table'), 3000);

// =============================================================================================
// HU-03 · Diagnóstico y presupuesto (spec-003)
// =============================================================================================
await rotulo('HU-03', 'Registrar el diagnóstico y preparar el presupuesto', 'El mecánico asignado escribe el diagnóstico y prepara el presupuesto línea a línea, con mano de obra, piezas e IVA desglosado. Al emitirlo vale 15 días.');
await escena('US1-1', 'Luis guarda el diagnóstico de la Orden Nº 10: pasa a «En diagnóstico»');
await ficha(10);
await escribir(page.locator('textarea[name=diagnostico]'), 'Pastillas delanteras desgastadas');
await pulsar(page.getByRole('button', { name: 'Guardar diagnóstico' }));
await ver('En diagnóstico');
await escena('US1-2', 'Sin presupuesto emitido, el diagnóstico se puede cambiar');
await escribir(page.locator('textarea[name=diagnostico]'), 'Pastillas y discos delanteros desgastados');
await pulsar(page.getByRole('button', { name: 'Guardar diagnóstico' }));
await pausa(1500);
await escena('US2-1', 'Nuevo presupuesto: mano de obra «Cambio de pastillas», 1,5 h a 40,00 € → 60,00 €');
await pulsar(page.getByRole('button', { name: 'Nuevo presupuesto' }));
await lineaManoObra('Añadir línea', 'Añadir línea', 'Cambio de pastillas', '1,5');
await ver(page.getByRole('cell', { name: 'Cambio de pastillas' }));
await escena('US2-4 · US2-2', 'Las piezas se eligen del almacén: PF-123 × 2 → «PF-123 Pastillas freno», 50,00 €');
await lineaPieza('Añadir línea', 'Añadir línea', 'PF', 'PF-123', '2');
await ver(page.getByRole('cell', { name: 'PF-123 Pastillas freno' }));
await escena('US2-3', 'Base imponible 110,00 €, IVA (21 %) 23,10 € y total 133,10 €');
await ver(page.getByText('Base imponible').locator('..').locator('..'), 3200);
await escena('Casos límite', 'Las horas van en cuartos: 1,3 h se rechaza explicando el motivo');
await lineaManoObra('Añadir línea', 'Añadir línea', 'Ajuste', '1,3');
await ver(formLinea('Añadir línea').getByRole('alert').first());
await escena('US2-5', 'En borrador se puede quitar una línea: se añade «Prueba» 1 h y se quita, y los totales vuelven a 110,00 / 23,10 / 133,10');
await lineaManoObra('Añadir línea', 'Añadir línea', 'Prueba', '1');
await ver(page.getByRole('cell', { name: 'Prueba', exact: true }), 1600);
await pulsar(page.locator('tr', { hasText: 'Prueba' }).getByRole('button', { name: 'Quitar' }));
await ver(page.getByText('Base imponible').locator('..').locator('..'), 2200);
await escena('US3-1', 'Emitir pide confirmación en la página; al confirmar: «Válido hasta» = hoy + 15 días, y ya no se pueden añadir ni quitar líneas');
await pulsar(page.getByRole('button', { name: 'Emitir presupuesto' }));
await ver(page.getByText('Una vez emitido no se podrá modificar'), 2000);
await pulsar(page.getByRole('button', { name: 'Sí, emitir' }));
await ver(page.getByText(/Válido hasta/), 3200);
await nota('Un presupuesto sin líneas no se puede emitir: el botón solo aparece con al menos una (US3-2)');
await escena('US1-3', 'Con el presupuesto emitido, el diagnóstico queda fijo: sin «Guardar diagnóstico» y con el motivo');
await ver(page.getByText('Ya se ha emitido un presupuesto'), 3000);
await escena('Emisión', 'El cliente recibe por correo el enlace al presupuesto');
const correoP10 = await verCorreo('Presupuesto de su vehículo 1313MMM');

// =============================================================================================
// HU-04 · Aceptar o rechazar el presupuesto desde un enlace (spec-004)
// =============================================================================================
await rotulo('HU-04', 'Aceptar o rechazar el presupuesto desde un enlace', 'El cliente abre el enlace, ve el presupuesto y lo acepta o lo rechaza; queda guardado el momento exacto. No se toca un coche sin presupuesto aceptado.');
await escena('US1-1', 'El enlace muestra el presupuesto completo, sin datos personales del cliente');
await ir(correoP10.enlace);
await pausa(3000);
await escena('US1-2', 'Aceptar pide confirmación con el total; «Volver» lo deja como estaba');
await pulsar(page.getByRole('button', { name: 'Aceptar' }));
await ver(page.getByText(/¿Acepta este presupuesto/), 2200);
await pulsar(page.getByRole('button', { name: 'Volver' }));
await pulsar(page.getByRole('button', { name: 'Aceptar' }));
await pulsar(page.getByRole('button', { name: 'Sí, aceptar' }));
await ver(page.getByRole('status'), 3000);
await escena('US1-3', 'Al volver a abrir el enlace se ve la decisión, sin botones');
await ir(correoP10.enlace);
await pausa(2600);
await escena('US1-2 · US1-5', 'La orden pasa a «En reparación», con la fecha y hora de la aceptación, y ya no muestra el enlace');
await entrar('luis');
await ficha(10);
await ver(page.getByText(/Aceptado el/), 3200);
await escena('US1-5', 'Mientras el presupuesto está vigente, el personal ve el enlace y puede copiarlo');
await entrar('ana');
await ficha(5);
await pulsar(page.getByRole('button', { name: 'Copiar enlace' }));
await ver(page.getByRole('button', { name: 'Copiado' }), 1800);
await escena('US4-1', 'Sin presupuesto aceptado, la mecánica no puede imputar trabajo');
await entrar('marta');
await ficha(5);
await ver(page.getByText(/solo se imputa en órdenes en reparación/), 3000);

await escena('US2-1', 'Rechazo: Luis emite el presupuesto de 1414NNN y el cliente lo rechaza');
const o11 = await ordenPorNumero(11);
await llamar.put('luis', `/ordenes/${o11.id}/diagnostico`, { diagnostico: 'Amortiguadores traseros con pérdida de aceite' });
const p11 = await llamar.post('luis', `/ordenes/${o11.id}/presupuestos`);
await llamar.post('luis', `/presupuestos/${p11.id}/lineas`, { tipo: 'mano_obra', concepto: 'Cambio de amortiguadores', cantidad: '2' });
await llamar.post('luis', `/presupuestos/${p11.id}/emitir`);
const correoP11 = await verCorreo('Presupuesto de su vehículo 1414NNN');
await ir(correoP11.enlace);
await pulsar(page.getByRole('button', { name: 'Rechazar' }));
await pulsar(page.getByRole('button', { name: 'Sí, rechazar' }));
await ver(page.getByRole('status'), 2600);
await nota('La orden sigue «En diagnóstico», con «Rechazado el … a las …»');
await entrar('luis');
await ficha(11);
await ver(page.getByText(/Rechazado el/), 3000);

await escena('US3-1', 'Un presupuesto caducado: la ampliación de la Orden Nº 7 sale «Caducado el…», sin enlace');
await ficha(7);
await ver(page.getByText(/Caducado el/), 3000);
await ir('/presupuesto/demo-presupuesto-caducado');
await pausa(3000);
await escena('US1-4 · US2-2 · US2-3', 'Dos pestañas con el mismo enlace: solo cuenta la primera respuesta');
await ficha(7);
await pulsar(page.getByRole('button', { name: 'Nuevo presupuesto' }));
await lineaManoObra('Añadir línea', 'Añadir línea', 'Revisión de la pinza de freno', '1');
await pulsar(page.getByRole('button', { name: 'Emitir presupuesto' }));
await pulsar(page.getByRole('button', { name: 'Sí, emitir' }));
await page.getByText(/Válido hasta/).waitFor();
const correoP7 = await verCorreo('Presupuesto de su vehículo 7777JJJ');
await ir(correoP7.enlace);
await nota('En otra pestaña, el cliente rechaza la ampliación…');
await fetch(`${BASE}/api/publico/presupuestos/${correoP7.enlace.split('/').pop()}/respuesta`, {
  method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ decision: 'rechazar' })
});
await nota('…y en esta, que aún muestra los botones, intenta aceptarla');
await pulsar(page.getByRole('button', { name: 'Aceptar' }));
await pulsar(page.getByRole('button', { name: 'Sí, aceptar' }));
await ver(page.getByRole('alert').first(), 3000);
await nota('La orden sigue «Pendiente de conformidad» y guarda las dos respuestas con fecha y hora');
await ficha(7);
await ver(page.getByText(/Ampliación · Rechazado el/), 3200);
await escena('Enlace no válido', 'Un enlace modificado no muestra nada');
await ir('/presupuesto/enlace-inventado');
await pausa(2600);

// =============================================================================================
// HU-05 · Imputar tiempo y piezas (spec-005)
// =============================================================================================
await rotulo('HU-05', 'Imputar tiempo y piezas', 'El mecánico carga el tiempo real y las piezas que monta, que se descuentan del almacén al momento. Si el coste supera en más de un 10 % lo presupuestado, el trabajo se detiene.');
await escena('US4-2', 'Sin imputaciones no se puede terminar: no aparece «Terminar reparación» y se indica el motivo');
await ficha(10);
await ver(page.getByText('Primero hay que imputar el trabajo realizado'), 2600);
await escena('US1-1', 'Luis monta PF-123 × 2: la pieza se descuenta del almacén en ese momento');
await lineaPieza('Nueva imputación', 'Imputar', 'PF', 'PF-123', '2');
await ver(page.getByRole('cell', { name: 'PF-123 Pastillas freno' }).last(), 2000);
await entrar('pepe');
await ir('/almacen');
await nota('En el almacén, el stock de PF-123 ha pasado de 4 a 2');
await ver(page.locator('tr', { hasText: 'PF-123' }), 3000);
await escena('US1-3', 'Sin stock suficiente no se imputa, y se indica que puede pedirse al proveedor');
await entrar('luis');
await ficha(10);
await lineaPieza('Nueva imputación', 'Imputar', 'PF', 'PF-123', '3');
await ver(formLinea('Nueva imputación').getByRole('alert').first(), 3000);
await escena('US1-2', 'Una hora de «Cambio de pastillas»: «Acumulado 90,00 € de 110,00 € presupuestados»');
await lineaManoObra('Nueva imputación', 'Imputar', 'Cambio de pastillas', '1');
await ver(page.getByText(/Acumulado 90,00/), 3000);
await escena('US2-2', 'Otra hora: 130,00 € supera el 110 % (121,00 €) → «Pendiente de conformidad» y no se puede seguir');
await lineaManoObra('Nueva imputación', 'Imputar', 'Rectificado de discos', '1');
await ver(page.getByText(/supera en más de un 10 %/), 3600);
await nota('Hasta 121,00 € exactos se puede seguir; un céntimo más bloquea (US2-1, prueba automática)');
await escena('US3-1', 'Luis prepara y emite una ampliación de 1 h (40,00 €)');
await pulsar(page.getByRole('button', { name: 'Nuevo presupuesto' }));
await lineaManoObra('Añadir línea', 'Añadir línea', 'Rectificado de discos', '1');
await pulsar(page.getByRole('button', { name: 'Emitir presupuesto' }));
await pulsar(page.getByRole('button', { name: 'Sí, emitir' }));
await page.getByText(/Válido hasta/).waitFor();
await nota('Mientras la ampliación no tenga respuesta, no se puede imputar (US3-2)');
const correoAmp = await verCorreo('Presupuesto de su vehículo 1313MMM');
await escena('US3-1', 'El cliente la acepta desde el enlace: la orden vuelve a «En reparación» con 150,00 € presupuestados');
await ir(correoAmp.enlace);
await pulsar(page.getByRole('button', { name: 'Aceptar' }));
await pulsar(page.getByRole('button', { name: 'Sí, aceptar' }));
await page.getByRole('status').waitFor();
await pausa(1500);
await ficha(10);
await ver(page.getByText(/de 150,00/), 3200);
await escena('US4-1', 'Luis marca la reparación como terminada (con confirmación en la página): ya no admite imputaciones');
await pulsar(page.getByRole('button', { name: 'Terminar reparación' }));
await ver(page.getByText('¿Marcar la reparación como terminada?'), 1800);
await pulsar(page.getByRole('button', { name: 'Sí, terminar' }));
await pausa(1500);
await ver(page.getByText('Terminada').first(), 3000);

// =============================================================================================
// HU-06 · Pedido de pieza al proveedor (spec-006)
// =============================================================================================
await rotulo('HU-06', 'Pedir una pieza al proveedor', 'Si una pieza no está, el almacén la pide vinculada a la orden; la orden queda en espera de piezas y vuelve sola a reparación al recibir el pedido.');
await entrar('pepe');
await ir('/almacen');
await escena('US1-2', 'Alta a mano de «FA-9 Filtro aceite», 12,00 €, sin indicar stock → stock 0');
const formAlta = page.locator('form').first();
await escribir(formAlta.locator('input[name=referencia]'), 'FA-9');
await escribir(formAlta.locator('input[name=descripcion]'), 'Filtro aceite');
await escribir(formAlta.locator('input[name=precio]'), '12,00');
await pulsar(page.getByRole('button', { name: 'Dar de alta' }));
await ver(page.locator('tr', { hasText: 'FA-9' }), 2600);
await escena('US1-3', 'La misma referencia otra vez se rechaza');
await escribir(formAlta.locator('input[name=referencia]'), 'FA-9');
await escribir(formAlta.locator('input[name=descripcion]'), 'Filtro aceite');
await escribir(formAlta.locator('input[name=precio]'), '12,00');
await pulsar(page.getByRole('button', { name: 'Dar de alta' }));
await ver(page.getByRole('alert').first(), 2600);
await escena('US1-1', 'Alta con stock inicial: «AC-5 Aceite 5W30», 9,50 €, stock 4');
await escribir(formAlta.locator('input[name=referencia]'), 'AC-5');
await escribir(formAlta.locator('input[name=descripcion]'), 'Aceite 5W30');
await escribir(formAlta.locator('input[name=precio]'), '9,50');
await escribir(formAlta.locator('input[name=stock]'), '4');
await pulsar(page.getByRole('button', { name: 'Dar de alta' }));
await ver(page.locator('tr', { hasText: 'AC-5' }), 2600);

await ir('/pedidos');
const pedido = async (proveedor, buscar, referencia, cantidad, orden) => {
  await elegir(page.locator('select[name=proveedor]'), { label: proveedor });
  await escribir(page.locator('#referencia'), buscar);
  await pulsar(page.getByRole('button', { name: new RegExp(referencia) }));
  await escribir(page.locator('input[name=cantidad]'), cantidad);
  await escribir(page.locator('input[name=orden]'), orden);
  await pulsar(page.getByRole('button', { name: 'Registrar pedido' }));
};
await escena('US2-1', 'Pedido a «Recambios Sur» (proveedores precargados) de FA-9 para la Orden Nº 2, en reparación: queda «pendiente» y la orden pasa a «En espera de piezas»');
await pedido('Recambios Sur', 'FA', 'FA-9', '1', '2');
await ver(page.getByRole('status'), 2600);
await escena('US2-5', 'Otro pedido de FA-9 para la misma orden con el primero pendiente se rechaza');
await pedido('Recambios Sur', 'FA', 'FA-9', '1', '2');
await ver(page.getByRole('alert').first(), 2600);
await escena('US2-2', 'Otra pieza (PF-123) para la misma orden en espera sí se admite');
await pedido('Autopartes Norte', 'PF', 'PF-123', '1', '2');
await ver(page.getByRole('status'), 2400);
await escena('US2-3', 'Un pedido para una orden que no está en reparación (Nº 4, en diagnóstico) se rechaza con el motivo');
await pedido('Recambios Sur', 'FA', 'FA-9', '1', '4');
await ver(page.getByRole('alert').first(), 2800);
await escena('US4-2', 'La lista de pedidos pendientes de todas las órdenes, cada uno con su orden');
await ver(page.getByRole('heading', { name: 'Pedidos pendientes' }).locator('..'), 3000);
await escena('US2-4 · US4-1', 'Luis ve los dos pedidos en la ficha de la Orden Nº 2 y no puede imputar ni terminar');
await entrar('luis');
await ficha(2);
await ver(page.getByRole('heading', { name: 'Pedidos' }).locator('..'), 2600);
await ver(page.getByText(/En espera de piezas/).first(), 2200);
await escena('US3-3', 'Nadie puede sacar la orden de la espera a mano, tampoco el jefe: no hay botón para cambiar el estado');
await entrar('paco');
await ficha(2);
await pausa(2600);
await escena('US3-2', 'El almacén recibe FA-9: el stock sube, pero la orden sigue esperando el otro pedido');
await entrar('pepe');
await ir('/pedidos');
await pulsar(page.locator('tr', { hasText: 'FA-9' }).getByRole('button', { name: 'Marcar recibido' }));
await pausa(1200);
await ir('/almacen');
await ver(page.locator('tr', { hasText: 'FA-9' }), 2200);
await escena('US3-1', 'Al recibir el último (PF-123), la orden vuelve sola a «En reparación»');
await ir('/pedidos');
await pulsar(page.locator('tr', { hasText: 'PF-123' }).getByRole('button', { name: 'Marcar recibido' }));
await pausa(1200);
await entrar('paco');
await ficha(2);
await ver(page.getByText('En reparación').first(), 3000);

// =============================================================================================
// HU-07 · Facturar y avisar al cliente (spec-007)
// =============================================================================================
await rotulo('HU-07', 'Facturar y avisar al cliente', 'Recepción genera la factura con lo imputado, sin teclear nada, y el cliente recibe un aviso para recoger el coche. Cada vehículo guarda su historial.');
await entrar('ana');
await escena('US1-5', 'Una orden que no está terminada (Nº 3, en reparación) no se puede facturar');
await ficha(3);
await pausa(2800);
await escena('US1-1 · US1-2', 'La Orden Nº 10 está terminada: «Facturar» genera la factura con las líneas imputadas');
await ficha(10);
await pulsar(page.getByRole('button', { name: 'Facturar' }));
await ver(page.getByText('¿Emitir la factura?'), 1800);
await pulsar(page.getByRole('button', { name: 'Sí, facturar' }));
await ver(page.getByText(/Factura nº \d+ ·/), 3000);
await escena('US1-2 · US1-4', 'La factura: número correlativo, fecha, taller con su logo, cliente, matrícula, líneas, base, IVA y total');
await pulsar(page.getByRole('link', { name: 'Ver e imprimir' }));
// El subtítulo habla del logo: se espera a que la imagen esté cargada y pintada (si no, falla).
await page.waitForFunction(() => {
  const i = document.querySelector('main img');
  return !!i && i.complete && i.naturalWidth > 0;
}, null, { timeout: 15_000 });
await pausa(3800);
await escena('US1-3', 'Al imprimir sale la factura sola, sin el menú de la aplicación (se puede guardar como PDF)');
await page.emulateMedia({ media: 'print' });
await pausa(3800);
await page.emulateMedia({ media: 'screen' });
await escena('US2-1', 'El cliente recibe «Su vehículo 1313MMM está listo para recoger», con el total');
await verCorreo('Su vehículo 1313MMM está listo para recoger');
await escena('US3-1', 'Historial por matrícula, escrita de cualquier forma («1313 mmm»): fecha, km, diagnóstico y líneas');
await ir('/historial');
await escribir(page.locator('input[name=matricula]'), '1313 mmm');
await pulsar(page.getByRole('button', { name: 'Buscar' }));
await ver(page.locator('main section').first(), 3200);
await escribir(page.locator('input[name=matricula]'), '9999LLL');
await pulsar(page.getByRole('button', { name: 'Buscar' }));
await ver(page.locator('main section').first(), 2600);
await escena('US3-2', 'Una matrícula sin reparaciones facturadas');
await escribir(page.locator('input[name=matricula]'), '0000ZZZ');
await pulsar(page.getByRole('button', { name: 'Buscar' }));
await pausa(2600);

// =============================================================================================
// HU-08 · Tablero de órdenes abiertas (spec-008)
// =============================================================================================
await rotulo('HU-08', 'Tablero de órdenes abiertas', 'Paco jubila su pizarra: ve todas las órdenes abiertas por estado, cuántas lleva cada mecánico y, destacadas, las que pasan de la fecha prometida. Se actualiza solo.');
await escena('US1-3', 'Recepción, mecánicos y almacén no tienen tablero: «No tienes permiso para hacer esto»');
await ir('/tablero');
await pausa(2600);
await escena('US1-1 · US1-2', 'Paco ve 6 columnas, una por estado; las facturadas (1313MMM, 9999LLL) no aparecen');
await entrar('paco');
await ir('/tablero');
await pausa(3000);
await escena('US2-1', 'Arriba, cuántas órdenes abiertas lleva cada mecánico');
await ver(page.locator('section[aria-labelledby=carga]'), 3000);
await escena('US3-1 · US3-2', 'Destacada y con «Retrasada» la que pasó su fecha de entrega (8888KKK, ayer); la de hoy (3333DDD) no');
await ver(page.locator('li.card', { hasText: '8888KKK' }), 3200);
await ver(page.locator('li.card', { hasText: '3333DDD' }), 2400);
await escena('US1-4', 'En cada columna, las tarjetas van por fecha de entrega');
await ver(page.locator('main section[aria-label="En reparación"]'), 3000);
const carga = () => page.locator('section[aria-labelledby=carga]');
await escena('US4-1', 'En otro navegador, Luis termina la Orden Nº 2 (2222CCC): la tarjeta pasa a «Terminada» sin recargar');
await llamar.post('luis', `/ordenes/${(await ordenPorNumero(2)).id}/terminar`);
await pausa(1500);
await ver(page.locator('main section[aria-label="Terminada"]'), 3000);
await escena('US4-2', 'Recepción abre una orden para 8888KKK con Marta: aparece en «Recibida» y Marta suma 1');
const cita8888 = await citaDeHoy('8888KKK');
await llamar.post('ana', '/ordenes', { cita_id: cita8888.id, km: 40000, danos: 'Sin daños visibles', mecanico_id: idMarta, fecha_entrega_prometida: masDias(1) });
await pausa(1500);
await ver(page.locator('main section[aria-label="Recibida"]'), 2200);
await ver(carga(), 2000);
await escena('US4-3', 'Recepción factura la Orden Nº 2: su tarjeta desaparece y Luis resta 1');
await llamar.post('ana', `/ordenes/${(await ordenPorNumero(2)).id}/factura`);
await pausa(1500);
await ver(page.locator('main section[aria-label="Terminada"]'), 2200);
await ver(carga(), 2000);
await escena('US4-5', 'Si se corta la conexión con el servidor, el tablero lo avisa y reintenta…');
await pararServidor();
await page.getByText('Sin conexión, reintentando…').waitFor({ timeout: 20_000 });
await ver(page.getByText('Sin conexión, reintentando…'), 3000);
await nota('…al volver el servidor, se reconecta solo y vuelve a pedir el tablero entero');
await arrancarServidor();
await page.getByText('Sin conexión, reintentando…').waitFor({ state: 'hidden', timeout: 30_000 });
await pausa(2000);
await escena('US4-1', 'Ya reconectado, Luis termina la Orden Nº 3 (3333DDD) y la tarjeta se mueve sola');
for (const u of Object.keys(apis)) delete apis[u]; // las sesiones siguen en la base; se vuelve a entrar por si acaso
await llamar.post('luis', `/ordenes/${(await ordenPorNumero(3)).id}/terminar`);
await pausa(1500);
await ver(page.locator('main section[aria-label="Terminada"]'), 3200);

// =============================================================================================
// Cierre
// =============================================================================================
historiaActual = { hu: 'Cierre', titulo: 'Lo que comprueban las pruebas automáticas', detalle: '', inicio: ahora(), escenas: [] };
capitulos.push(historiaActual);
await page.evaluate(() => window.__demoRotulo('Y ADEMÁS', 'Pruebas automáticas',
  'Lo que no se ve bien en un vídeo lo comprueban las pruebas: dos clientes a la vez por la última plaza, el número de orden y de factura sin huecos con peticiones simultáneas, el límite exacto del 10 %, el redondeo del IVA, el correo que no sale, el cambio de día a medianoche y un mecánico de baja con órdenes en el tablero.'));
await pausa(8000);

// ---------------------------------------------------------------------------------------------
const video = page.video();
await contexto.close();
const webm = await video.path();
const duracion = ahora();
await nav.close();
await pararServidor();
for (const c of Object.values(apis)) await c.dispose().catch(() => {});

const ffmpeg = process.env.FFMPEG ?? 'ffmpeg';
const mp4 = path.join(SALIDA, 'demo.mp4');
execFileSync(ffmpeg, ['-y', '-loglevel', 'error', '-i', webm, '-c:v', 'libx264', '-preset', 'medium', '-crf', '24',
  '-pix_fmt', 'yuv420p', '-movflags', '+faststart', mp4]);
fs.rmSync(webm);

// Calibración: el reloj del vídeo no avanza exactamente igual que el del script (la grabación
// pierde fotogramas cuando el equipo va cargado). Los rótulos a pantalla completa son oscuros, así
// que ffmpeg los encuentra con blackdetect; con sus inicios reales y en el vídeo se ajusta una recta
// t_vídeo = a · t_real + b, y con ella se pasan todos los tiempos al vídeo.
const deteccion = spawnSync(ffmpeg, ['-i', mp4, '-vf', 'blackdetect=d=0.6:pix_th=0.32:pic_th=0.85', '-an', '-f', 'null', '-'], { encoding: 'utf8' });
const oscuros = [...deteccion.stderr.matchAll(/black_start:([\d.]+)/g)].map((m) => Number(m[1]));
const reales = capitulos.map((h) => h.inicio);
let a = 1;
let b = 0;
if (oscuros.length === reales.length) {
  const n = reales.length;
  const mx = reales.reduce((s, x) => s + x, 0) / n;
  const my = oscuros.reduce((s, y) => s + y, 0) / n;
  a = reales.reduce((s, x, i) => s + (x - mx) * (oscuros[i] - my), 0) / reales.reduce((s, x) => s + (x - mx) ** 2, 0);
  b = my - a * mx;
  console.log(`Calibración con ${n} rótulos: t_vídeo = ${a.toFixed(4)} · t_real + ${b.toFixed(2)}`);
} else {
  const dur = Number(spawnSync(ffmpeg.replace(/ffmpeg(\.exe)?$/, 'ffprobe$1'), ['-v', 'error', '-show_entries', 'format=duration', '-of', 'csv=p=0', mp4], { encoding: 'utf8' }).stdout);
  a = dur / duracion;
  console.log(`Rótulos detectados: ${oscuros.length} de ${reales.length}; se escala por la duración (${a.toFixed(4)})`);
}
const alVideo = (t) => Math.max(0, Math.round((a * t + b) * 10) / 10);
for (const h of capitulos) {
  h.inicio = alVideo(h.inicio);
  for (const e of h.escenas) e.t = alVideo(e.t);
}
fs.writeFileSync(path.join(SALIDA, 'capitulos.json'), JSON.stringify({ fecha: HOY, duracion: alVideo(duracion), historias: capitulos }, null, 2));
console.log(`\nVídeo: ${mp4} (${Math.round(alVideo(duracion))} s)`);
