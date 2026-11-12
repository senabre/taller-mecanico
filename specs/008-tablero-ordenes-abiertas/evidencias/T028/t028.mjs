// T028 de spec-008: quickstart común §8 en el navegador, con los datos demo recién cargados (sin §1–§7:
// la carga de partida es «Luis 5 · Marta 3», como prevé el paso 3).
// Dos contextos de navegador: el tablero de paco (que no se toca) y «el otro puesto».
// Uso: node t028.mjs <carpeta de evidencias> <orden para rearrancar el servidor>
import { chromium } from 'playwright';
import fs from 'node:fs';
import { execSync, spawn } from 'node:child_process';

const BASE = 'http://localhost:3000';
const OUT = process.argv[2];
const REARRANCAR = process.argv[3];
fs.mkdirSync(OUT, { recursive: true });
const res = [];
let n = 0;
const nav = await chromium.launch();
// Arranca el servidor como proceso independiente (equivale a volver a ejecutar «cargo run»).
function arrancarServidor() {
  const dir = '/tmp/claude-0/-home-claude/25f19f03-592c-5f7c-94f0-51da21d8588e/scratchpad/e2e/wt004/backend';
  const log = execSync(`echo ${dir}/../../server004.log`).toString().trim();
  const fd = fs.openSync(log, 'a');
  spawn('./target/debug/taller', [], { cwd: dir, detached: true, stdio: ['ignore', fd, fd] }).unref();
}
const GRANDE = { width: 1920, height: 1080 };
const sql = (q) => execSync(`psql -h localhost -U taller -d taller -tAc "${q}"`).toString().trim();
const limpio = (t) => t.replace(/ | /g, ' ');
const espera = (ms) => new Promise((r) => setTimeout(r, ms));

async function captura(p, nombre) {
  n += 1; const f = `${String(n).padStart(2, '0')}-${nombre}.png`;
  await p.screenshot({ path: `${OUT}/${f}`, fullPage: true }); return f;
}
function anota(paso, esperado, observado, ok, img) {
  res.push({ paso, esperado, observado, ok, img }); console.log(`${ok ? 'OK   ' : 'FALLA'} ${paso}: ${observado}`);
}
async function sesion(usuario, viewport = GRANDE) {
  const c = await nav.newContext({ viewport, locale: 'es-ES', timezoneId: 'Europe/Madrid' });
  const p = await c.newPage();
  await p.goto(`${BASE}/entrar`); await p.fill('input[name=login]', usuario);
  await p.fill('input[name=contrasena]', 'demo1234'); await p.click('button[type=submit]');
  await p.waitForTimeout(800);
  return p;
}
async function api(p, metodo, ruta, cuerpo) {
  return p.evaluate(async ([m, r, b]) => {
    const x = await fetch(`/api${r}`, { method: m, headers: { 'content-type': 'application/json' }, body: b === undefined ? undefined : JSON.stringify(b) });
    let j = null; try { j = await x.json(); } catch {} return { status: x.status, json: j };
  }, [metodo, ruta, cuerpo]);
}
// Lee el tablero tal como se ve: columnas (en orden), matrículas de cada una, retrasadas y carga.
async function leer(p) {
  return p.evaluate(() => {
    const cols = [...document.querySelectorAll('main section[aria-label]')].map((s) => ({
      nombre: s.getAttribute('aria-label'),
      tarjetas: [...s.querySelectorAll('li.card')].map((li) => ({
        matricula: li.querySelector('.font-mono')?.textContent.trim(),
        retrasada: !!li.querySelector('.badge') && li.textContent.includes('Retrasada'),
        borde: getComputedStyle(li).borderTopWidth,
        texto: li.innerText
      })),
      vacia: s.innerText.includes('Sin órdenes')
    }));
    const carga = document.querySelector('section[aria-labelledby=carga] ul')?.innerText.replace(/\s+/g, ' ').trim();
    const aviso = document.querySelector('[role=status]')?.innerText ?? '';
    return { cols, carga, aviso };
  });
}
const columnaDe = (t, m) => t.cols.find((c) => c.tarjetas.some((x) => x.matricula === m))?.nombre ?? null;
// Espera (sin tocar la página) a que se cumpla la condición; devuelve los ms o null.
async function hasta(p, cond, max = 8000) {
  const t0 = Date.now();
  while (Date.now() - t0 < max) { if (cond(await leer(p))) return Date.now() - t0; await espera(100); }
  return null;
}
const ordenId = (m) => Number(sql(`select o.id from orden o join cita c on c.id = o.cita_id join vehiculo v on v.id = c.vehiculo_id where v.matricula = '${m}' and o.estado <> 'facturada' order by o.id desc limit 1`));

// §8.1 · Los demás roles no ven el tablero
for (const u of ['ana', 'luis', 'pepe']) {
  const p = await sesion(u);
  const menu = await p.locator('header').getByRole('link', { name: 'Tablero' }).count();
  await p.goto(`${BASE}/tablero`); await p.waitForTimeout(900);
  const t = limpio(await p.locator('main').innerText());
  const columnas = await p.locator('main section[aria-label]').count();
  const status = (await api(p, 'GET', '/tablero')).status;
  anota(`§8.1 ${u} y el tablero`, 'Sin «Tablero» en el menú; /tablero dice «No tienes permiso para hacer esto», sin columnas; la API da 403',
    `menú=${menu}; mensaje=${t.includes('No tienes permiso para hacer esto')}; columnas=${columnas}; API=${status}`,
    menu === 0 && t.includes('No tienes permiso para hacer esto') && columnas === 0 && status === 403, await captura(p, `p1-${u}-sin-permiso`));
  await p.context().close();
}

// §8.2–§8.5 · El tablero de paco
const paco = await sesion('paco');
paco.on('crash', () => console.log('PACO: la página se ha caído'));
paco.on('close', () => console.log('PACO: la página se ha cerrado'));
paco.on('framenavigated', (f) => { if (f === paco.mainFrame()) console.log('PACO: navega a', f.url()); });
const menuPaco = await paco.locator('header').getByRole('link', { name: 'Tablero' }).count();
await paco.goto(`${BASE}/tablero`); await paco.waitForTimeout(1200);
let t = await leer(paco);
{
  const nombres = t.cols.map((c) => c.nombre);
  const esperado = ['Recibida', 'En diagnóstico', 'En reparación', 'En espera de piezas', 'Pendiente de conformidad', 'Terminada'];
  const todas = t.cols.flatMap((c) => c.tarjetas.map((x) => x.matricula));
  const abiertas = Number(sql("select count(*) from orden where estado <> 'facturada'"));
  const fueraDeSu = t.cols.flatMap((c) => c.tarjetas.filter((x) => {
    const e = sql(`select o.estado from orden o join cita ci on ci.id = o.cita_id join vehiculo v on v.id = ci.vehiculo_id where v.matricula = '${x.matricula}' and o.estado <> 'facturada'`);
    return ({ recibida: 'Recibida', en_diagnostico: 'En diagnóstico', en_reparacion: 'En reparación', en_espera_piezas: 'En espera de piezas', pendiente_conformidad: 'Pendiente de conformidad', terminada: 'Terminada' })[e] !== c.nombre;
  }).map((x) => x.matricula));
  anota('§8.2 Columnas (paco)', `«Tablero» en el menú; 6 columnas en orden; sin 9999LLL ni 1313MMM; todas las abiertas, cada una en su columna`,
    `menú=${menuPaco}; columnas=[${nombres.join(', ')}]; tarjetas=${todas.length} de ${abiertas} abiertas; 9999LLL=${todas.includes('9999LLL')}; 1313MMM=${todas.includes('1313MMM')}; fuera de su columna=[${fueraDeSu}]; vacías=${t.cols.filter((c) => c.vacia).length}`,
    menuPaco === 1 && JSON.stringify(nombres) === JSON.stringify(esperado) && todas.length === abiertas && !todas.includes('9999LLL') && !todas.includes('1313MMM') && fueraDeSu.length === 0,
    await captura(paco, 'p2-tablero-1920'));
  const porMec = {};
  for (const c of t.cols) for (const x of c.tarjetas) { const m = x.texto.split('\n').find((l) => /^(Luis|Marta|Pedro)$/.test(l.trim())); porMec[m] = (porMec[m] || 0) + 1; }
  anota('§8.3 Carga por mecánico', '«Luis 5 · Marta 3» (datos recién cargados); coincide con las tarjetas; Pedro no sale',
    `carga=«${t.carga}»; tarjetas por mecánico=${JSON.stringify(porMec)}`,
    t.carga === 'Luis 5 · Marta 3' && porMec.Luis === 5 && porMec.Marta === 3 && !t.carga.includes('Pedro'), null);
  const ret = t.cols.flatMap((c) => c.tarjetas.filter((x) => x.retrasada).map((x) => `${x.matricula} (${c.nombre}, borde ${x.borde})`));
  const t3 = t.cols.flatMap((c) => c.tarjetas).find((x) => x.matricula === '3333DDD');
  anota('§8.4 Retrasadas (lo que hay que encontrar en SC-001)', 'Solo 8888KKK, en «Terminada», con borde destacado y «Retrasada»; 3333DDD (entrega hoy) sin «Retrasada»',
    `retrasadas=[${ret}]; 3333DDD retrasada=${t3?.retrasada}`, ret.length === 1 && ret[0].startsWith('8888KKK (Terminada, borde 2px') && t3 && !t3.retrasada, null);
  const rep = t.cols.find((c) => c.nombre === 'En reparación').tarjetas.map((x) => x.matricula);
  anota('§8.5 Orden por fecha de entrega', '«En reparación»: 3333DDD (hoy) encima de 2222CCC (pasado mañana)', `orden=[${rep}]`,
    rep.indexOf('3333DDD') >= 0 && rep.indexOf('3333DDD') < rep.indexOf('2222CCC'), null);
}

// §8.6–§8.8 · Cambios desde otro puesto, sin tocar el tablero de paco
{
  const luis = await sesion('luis');
  // «Terminar reparación» (spec-005) pide confirmación con confirm() del navegador: se acepta y se anota el texto.
  let dialogo = null; luis.on('dialog', async (d) => { dialogo = d.message(); await d.accept(); });
  await luis.goto(`${BASE}/ordenes/${ordenId('2222CCC')}`); await luis.waitForTimeout(900);
  await luis.getByRole('button', { name: 'Terminar reparación' }).click();
  const ms = await hasta(paco, (x) => columnaDe(x, '2222CCC') === 'Terminada');
  t = await leer(paco);
  anota('§8.6 luis termina 2222CCC', 'En menos de 5 s pasa a «Terminada»; la carga de Luis no cambia', `confirm()=«${dialogo}»; tiempo=${ms} ms; columna=${columnaDe(t, '2222CCC')}; carga=«${t.carga}»`,
    ms !== null && ms < 5000 && t.carga === 'Luis 5 · Marta 3', await captura(paco, 'p6-2222CCC-terminada'));
  await luis.context().close();
}
const ana = await sesion('ana');
// «Facturar» (spec-007) también pide confirmación con confirm(): se acepta y se anota el texto.
let dialogoAna = null; ana.on('dialog', async (d) => { dialogoAna = d.message(); await d.accept(); });
{
  await ana.goto(`${BASE}/citas`); await ana.waitForTimeout(900);
  await ana.locator('tr', { hasText: '8888KKK' }).getByText('Abrir orden').click(); await ana.waitForTimeout(800);
  const manana = sql("select to_char((now() at time zone 'Europe/Madrid')::date + 1, 'YYYY-MM-DD')");
  await ana.fill('input[name=km]', '40000'); await ana.fill('textarea[name=danos]', 'Sin daños visibles');
  await ana.selectOption('select[name=mecanico]', { label: 'Marta' }); await ana.fill('input[name=entrega]', manana);
  await ana.click('button[type=submit]');
  const ms = await hasta(paco, (x) => x.cols.find((c) => c.nombre === 'Recibida').tarjetas.some((y) => y.matricula === '8888KKK'));
  t = await leer(paco);
  anota('§8.7 ana abre una orden de 8888KKK para Marta', 'En menos de 5 s su tarjeta sale en «Recibida» y la carga pasa a «Marta 4»', `tiempo=${ms} ms; carga=«${t.carga}»`,
    ms !== null && ms < 5000 && t.carga === 'Luis 5 · Marta 4', await captura(paco, 'p7-8888KKK-recibida'));
}
{
  await ana.goto(`${BASE}/ordenes/${ordenId('2222CCC')}`); await ana.waitForTimeout(900);
  await ana.getByRole('button', { name: 'Facturar' }).click();
  const ms = await hasta(paco, (x) => columnaDe(x, '2222CCC') === null);
  t = await leer(paco);
  anota('§8.8 ana factura 2222CCC', 'En menos de 5 s la tarjeta desaparece y la carga pasa a «Luis 4»', `confirm()=«${dialogoAna}»; tiempo=${ms} ms; carga=«${t.carga}»`,
    ms !== null && ms < 5000 && t.carga === 'Luis 4 · Marta 4', await captura(paco, 'p8-2222CCC-facturada'));
}

// §8.9 · Servidor parado y vuelta a arrancar; después, red cortada en el navegador de paco
{
  execSync('/tmp/claude-0/-home-claude/25f19f03-592c-5f7c-94f0-51da21d8588e/scratchpad/e2e/tester/parar.sh');
  const msAviso = await hasta(paco, (x) => x.aviso.includes('Sin conexión, reintentando…'), 10000);
  const img1 = await captura(paco, 'p9-sin-conexion');
  arrancarServidor();
  const msVuelta = await hasta(paco, (x) => x.aviso === '' && x.cols.length === 6, 20000);
  anota('§8.9 Parar y volver a arrancar el servidor', '«Sin conexión, reintentando…» y, al volver, el aviso desaparece solo, sin recargar',
    `aviso tras ${msAviso} ms; vuelta tras ${msVuelta} ms de arrancar`, msAviso !== null && msVuelta !== null, img1);
  await captura(paco, 'p9-reconectado');
  await paco.context().setOffline(true);
  const msOff = await hasta(paco, (x) => x.aviso.includes('Sin conexión'), 10000);
  const luis = await sesion('luis');
  const r = await api(luis, 'POST', `/ordenes/${ordenId('3333DDD')}/terminar`);
  await espera(1500);
  const antes = columnaDe(await leer(paco), '3333DDD');
  await paco.context().setOffline(false);
  const msOn = await hasta(paco, (x) => x.aviso === '' && columnaDe(x, '3333DDD') === 'Terminada', 20000);
  anota('§8.9 Red cortada en el navegador de paco (opcional)', 'Con la red cortada, luis termina 3333DDD; al volver la red, el aviso se va y 3333DDD ya está en «Terminada», sin recargar',
    `aviso=${msOff !== null}; terminar=${r.status}; columna sin red=${antes}; al volver: ${msOn} ms`, msOff !== null && r.status === 200 && antes === 'En reparación' && msOn !== null,
    await captura(paco, 'p9-tras-red-cortada'));
  await luis.context().close();
}

// §8.10 · 390 px y pantalla grande
{
  const m = await sesion('paco', { width: 390, height: 844 });
  await m.goto(`${BASE}/tablero`); await m.waitForTimeout(1200);
  const d = await m.evaluate(() => {
    const tira = document.querySelector('main section[aria-label]').parentElement;
    const cols = [...document.querySelectorAll('main section[aria-label]')].map((s) => s.getBoundingClientRect());
    const carga = document.querySelector('section[aria-labelledby=carga] ul').getBoundingClientRect();
    return { pagina: document.documentElement.scrollWidth, tira: tira.scrollWidth > tira.clientWidth, primera: cols[0].right <= 390, siguiente: cols[1].left < 390, cargaAncho: carga.right <= 390 };
  });
  anota('§8.10 Móvil a 390 px', 'Una columna entera y el borde de la siguiente; las columnas se desplazan dentro del tablero; la página no se mueve de lado; la carga cabe',
    JSON.stringify(d), d.pagina <= 390 && d.tira && d.primera && d.siguiente && d.cargaAncho, await captura(m, 'p10-movil-390'));
  await m.locator('main section[aria-label]').first().evaluate((s) => s.parentElement.scrollBy({ left: 340 }));
  await m.waitForTimeout(400);
  await captura(m, 'p10-movil-390-desplazado');
  await m.context().close();
  const g = await paco.evaluate(() => {
    const cols = [...document.querySelectorAll('main section[aria-label]')].map((s) => s.getBoundingClientRect());
    return { fila: cols.every((c) => Math.abs(c.top - cols[0].top) < 1), dentro: cols[5].right <= window.innerWidth, ancho: Math.round(cols[5].right - cols[0].left), pagina: document.documentElement.scrollWidth };
  });
  anota('§8.10 Pantalla grande (1920 px)', 'Las 6 columnas una al lado de otra, ocupando todo el ancho, sin desplazamiento lateral', JSON.stringify(g),
    g.fila && g.dentro && g.ancho > 1600 && g.pagina <= 1920, await captura(paco, 'p10-pantalla-grande'));
}
fs.writeFileSync(`${OUT}/resultados.json`, JSON.stringify(res, null, 2));
await nav.close();
console.log(`\n${res.filter((x) => x.ok).length}/${res.length} comprobaciones superadas`);
