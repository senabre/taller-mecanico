// T028 (complemento) de spec-008: lo que T011, T012 y T025 piden ver y el recorrido principal no cubría.
// Algunos estados se preparan con SQL (columna vacía, columna llena) o interceptando la red (fallo de
// carga); se dice en cada comprobación. Uso: node t028b.mjs <carpeta>
import { chromium } from 'playwright';
import fs from 'node:fs';
import { execSync } from 'node:child_process';

const BASE = 'http://localhost:3000';
const OUT = process.argv[2];
fs.mkdirSync(OUT, { recursive: true });
const res = [];
let n = 0;
const sql = (q) => execSync(`psql -h localhost -U taller -d taller -tAc "${q}"`).toString().trim();
const anota = (paso, esperado, observado, ok, img) => { res.push({ paso, esperado, observado, ok, img }); console.log(`${ok ? 'OK   ' : 'FALLA'} ${paso}: ${observado}`); };
async function captura(p, nombre) { n += 1; const f = `b${String(n).padStart(2, '0')}-${nombre}.png`; await p.screenshot({ path: `${OUT}/${f}`, fullPage: true }); return f; }
const nav = await chromium.launch();
async function sesion(u, viewport = { width: 1920, height: 1080 }) {
  const c = await nav.newContext({ viewport, locale: 'es-ES', timezoneId: 'Europe/Madrid' });
  const p = await c.newPage();
  await p.goto(`${BASE}/entrar`); await p.fill('input[name=login]', u); await p.fill('input[name=contrasena]', 'demo1234');
  await p.click('button[type=submit]'); await p.waitForTimeout(800); return p;
}
const anchoMain = (p) => p.evaluate(() => Math.round(document.querySelector('main').getBoundingClientRect().width));

// T011 · Las demás pantallas conservan su ancho (max-w-6xl = 1152 px), el menú (header nav) y el contenido
// (main); solo /tablero ocupa todo el ancho. También navegando dentro de la app, sin recargar.
{
  const paco = await sesion('paco');
  const medir = (p) => p.evaluate(() => ({
    menu: Math.round(document.querySelector('header nav').getBoundingClientRect().width),
    main: Math.round(document.querySelector('main').getBoundingClientRect().width)
  }));
  const anchos = {};
  for (const r of ['/citas', '/ordenes', '/almacen', '/pedidos', '/historial', '/tablero']) {
    await paco.goto(`${BASE}${r}`); await paco.waitForTimeout(700); anchos[r] = await medir(paco);
  }
  // Ahora estamos en /tablero: se va a Citas con el menú (sin recargar), se captura Citas y se vuelve.
  await paco.getByRole('link', { name: 'Citas' }).click(); await paco.waitForTimeout(700);
  const citasNav = await medir(paco); const urlCitas = new URL(paco.url()).pathname;
  const imgCitas = await captura(paco, 't011-citas-1920');
  await paco.getByRole('link', { name: 'Tablero' }).click(); await paco.waitForTimeout(700);
  const tableroNav = await medir(paco);
  const imgTablero = await captura(paco, 't011-tablero-1920');
  const otras = Object.entries(anchos).filter(([r]) => r !== '/tablero').map(([, a]) => a);
  const ok = otras.every((a) => a.menu === 1152 && a.main === 1152) && anchos['/tablero'].menu === 1920 && anchos['/tablero'].main === 1920
    && urlCitas === '/citas' && citasNav.menu === 1152 && citasNav.main === 1152 && tableroNav.menu === 1920 && tableroNav.main === 1920;
  anota('T011 Ancho del menú y del contenido (1920 px)', 'Menú y contenido a 1152 px en las demás pantallas y a todo el ancho en /tablero, también navegando sin recargar',
    `${JSON.stringify(anchos)}; navegando: ${urlCitas}=${JSON.stringify(citasNav)}, /tablero=${JSON.stringify(tableroNav)}`, ok, `${imgCitas}, ${imgTablero}`);
  await paco.context().close();
}

// T025 · Con un 403 no se abre la conexión de eventos (ana)
{
  const ana = await sesion('ana');
  const peticiones = [];
  ana.on('request', (r) => { if (r.url().includes('/api/')) peticiones.push(new URL(r.url()).pathname); });
  await ana.goto(`${BASE}/tablero`); await ana.waitForTimeout(3000);
  const t = await ana.locator('main').innerText();
  anota('T025 Con un 403 no se abre /api/eventos (ana)', 'Pide /api/tablero (403) y no abre /api/eventos', `peticiones=[${[...new Set(peticiones)]}]; mensaje=${t.includes('No tienes permiso para hacer esto')}`,
    peticiones.includes('/api/tablero') && !peticiones.includes('/api/eventos') && t.includes('No tienes permiso para hacer esto'), await captura(ana, 't025-ana-403'));
  await ana.context().close();
}

// T012 · «No se pudo cargar el tablero» (la primera carga falla por la red: se intercepta /api/tablero)
{
  const paco = await sesion('paco');
  await paco.route('**/api/tablero', (r) => r.abort('connectionrefused'));
  await paco.goto(`${BASE}/tablero`); await paco.waitForTimeout(1500);
  const t = await paco.locator('main').innerText();
  anota('T012 Falla la primera carga (red interceptada)', '«No se pudo cargar el tablero»', `mensaje=${t.includes('No se pudo cargar el tablero')}`, t.includes('No se pudo cargar el tablero'), await captura(paco, 't012-no-se-pudo-cargar'));
  await paco.context().close();
}

// T012 · Columna vacía: la orden 7 pasa a «en reparación» por SQL y «Pendiente de conformidad» queda vacía
{
  const antes = sql('select estado from orden where numero = 7');
  sql("update orden set estado = 'en_reparacion' where numero = 7");
  const paco = await sesion('paco');
  await paco.goto(`${BASE}/tablero`); await paco.waitForTimeout(1200);
  const col = paco.locator('main section[aria-label="Pendiente de conformidad"]');
  const t = await col.innerText();
  anota('T012 Columna vacía (preparada por SQL)', 'La columna sigue y dice «Sin órdenes»', `existe=${await col.count()}; texto=«${t.replace(/\n/g, ' / ')}»`,
    (await col.count()) === 1 && t.includes('Sin órdenes'), await captura(paco, 't012-columna-vacia'));
  sql(`update orden set estado = '${antes}' where numero = 7`);
  await paco.context().close();
}

// T012 · Columna llena: todas las abiertas pasan a «en reparación» por SQL; la columna se desplaza por dentro
{
  const antes = sql("select string_agg(numero || ':' || estado, ',') from orden where estado <> 'facturada'");
  sql("update orden set estado = 'en_reparacion' where estado <> 'facturada'");
  for (const [vp, nombre] of [[{ width: 1920, height: 1080 }, '1920'], [{ width: 390, height: 844 }, '390']]) {
    const paco = await sesion('paco', vp);
    await paco.goto(`${BASE}/tablero`); await paco.waitForTimeout(1200);
    const d = await paco.evaluate(() => {
      const ul = document.querySelector('main section[aria-label="En reparación"] ul');
      const antes = ul.scrollTop; ul.scrollTop = ul.scrollHeight; const despues = ul.scrollTop;
      return { tarjetas: ul.querySelectorAll('li').length, desborda: ul.scrollHeight > ul.clientHeight, desplaza: despues > antes, pagina: document.documentElement.scrollWidth <= window.innerWidth };
    });
    anota(`T012 Columna llena (preparada por SQL, ${nombre} px)`, 'Las 8 tarjetas en «En reparación»; la columna se desplaza por dentro; la página no se mueve de lado', JSON.stringify(d),
      d.tarjetas === 8 && d.desborda && d.desplaza && d.pagina, await captura(paco, `t012-columna-llena-${nombre}`));
    await paco.context().close();
  }
  for (const par of antes.split(',')) { const [num, est] = par.split(':'); sql(`update orden set estado = '${est}' where numero = ${num}`); }
}
fs.writeFileSync(`${OUT}/resultados-b.json`, JSON.stringify(res, null, 2));
await nav.close();
console.log(`\n${res.filter((x) => x.ok).length}/${res.length} comprobaciones superadas`);
