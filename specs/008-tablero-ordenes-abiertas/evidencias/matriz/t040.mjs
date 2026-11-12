// Matriz de trazabilidad, HU-08: dos criterios que solo tenían cobertura parcial en el navegador.
//  US2-2: un mecánico de baja con órdenes abiertas sale en la carga marcado «(de baja)». Se prepara con SQL
//         (Pedro, de baja, pasa a ser el mecánico de la orden demo 1) y se restaura al terminar.
//  US4-5: lo que cambia mientras el tablero está sin conexión aparece al volver (servidor parado y cambio
//         hecho directamente en la base mientras tanto).
// Uso: node t040.mjs <carpeta>
import { chromium } from 'playwright';
import fs from 'node:fs';
import { execSync, spawn } from 'node:child_process';
const BASE = 'http://localhost:3000';
const OUT = process.argv[2]; fs.mkdirSync(OUT, { recursive: true });
const res = []; const sql = (q) => execSync(`psql -h localhost -U taller -d taller -tAc "${q}"`).toString().trim();
const anota = (paso, esperado, observado, ok, img) => { res.push({ paso, esperado, observado, ok, img }); console.log(`${ok ? 'OK   ' : 'FALLA'} ${paso}: ${observado}`); };
const nav = await chromium.launch();
async function sesion(u) { const c = await nav.newContext({ viewport: { width: 1920, height: 1080 }, locale: 'es-ES', timezoneId: 'Europe/Madrid' }); const p = await c.newPage();
  await p.goto(`${BASE}/entrar`); await p.fill('input[name=login]', u); await p.fill('input[name=contrasena]', 'demo1234'); await p.click('button[type=submit]'); await p.waitForTimeout(800); return p; }
const carga = (p) => p.locator('section[aria-labelledby=carga] ul').innerText().then((t) => t.replace(/\s+/g, ' ').trim());
const columnaDe = (p, m) => p.evaluate((m) => [...document.querySelectorAll('main section[aria-label]')].find((s) => [...s.querySelectorAll('li.card .font-mono')].some((x) => x.textContent.trim() === m))?.getAttribute('aria-label') ?? null, m);

// US2-2
{
  const antes = sql('select mecanico_id from orden where numero = 1');
  sql("update orden set mecanico_id = (select id from usuario where login = 'pedro') where numero = 1");
  const paco = await sesion('paco'); await paco.goto(`${BASE}/tablero`); await paco.waitForTimeout(1200);
  const c = await carga(paco);
  const tarjeta = await paco.locator('li.card', { hasText: '1111BBB' }).innerText();
  await paco.screenshot({ path: `${OUT}/c01-us2-2-de-baja.png`, fullPage: true });
  anota('HU-08 US2-2 Mecánico de baja con órdenes (preparado por SQL)', 'Pedro sale en la carga marcado «(de baja)» con 1; la carga suma lo mismo que las tarjetas',
    `carga=«${c}»; tarjeta de 1111BBB=«${tarjeta.replace(/\n/g, ' / ')}»`, /Pedro \(de baja\) 1/.test(c) && tarjeta.includes('Pedro'), 'c01-us2-2-de-baja.png');
  sql(`update orden set mecanico_id = ${antes} where numero = 1`);
  await paco.context().close();
}
// US4-5 · Se para el servidor (corte real de la conexión de eventos), y mientras está parado la orden de
// 3333DDD pasa a «terminada» directamente en la base (nadie avisa al tablero). Al volver a arrancar el
// servidor, el tablero reconecta y, como pide entero el tablero al reconectar, la tarjeta ya sale en «Terminada».
{
  const paco = await sesion('paco'); await paco.goto(`${BASE}/tablero`); await paco.waitForTimeout(1500);
  const inicio = await columnaDe(paco, '3333DDD');
  execSync(`${process.env.PARAR}`);
  const t0 = Date.now(); let aviso = '';
  while (Date.now() - t0 < 10000 && !aviso.includes('Sin conexión')) { await paco.waitForTimeout(200); aviso = await paco.locator('[role=status]').innerText().catch(() => ''); }
  sql("update orden set estado = 'terminada' where id = (select o.id from orden o join cita c on c.id = o.cita_id join vehiculo v on v.id = c.vehiculo_id where v.matricula = '3333DDD')");
  await paco.waitForTimeout(1000);
  const durante = await columnaDe(paco, '3333DDD');
  await paco.screenshot({ path: `${OUT}/c02-us4-5-sin-conexion.png`, fullPage: true });
  { // vuelve a arrancar el servidor como proceso independiente (equivale a volver a ejecutar «cargo run»)
    const dir = '/tmp/claude-0/-home-claude/25f19f03-592c-5f7c-94f0-51da21d8588e/scratchpad/e2e/wt004/backend';
    const fd = fs.openSync(`${dir}/../../server004.log`, 'a');
    spawn('./target/debug/taller', [], { cwd: dir, detached: true, stdio: ['ignore', fd, fd] }).unref();
  }
  const t1 = Date.now(); let despues = durante;
  while (Date.now() - t1 < 20000 && despues !== 'Terminada') { await paco.waitForTimeout(250); despues = await columnaDe(paco, '3333DDD'); }
  const ms = Date.now() - t1;
  const avisoFin = await paco.locator('[role=status]').innerText().catch(() => '');
  await paco.screenshot({ path: `${OUT}/c03-us4-5-al-volver.png`, fullPage: true });
  anota('HU-08 US4-5 Lo que cambió sin conexión aparece al volver', 'Con el servidor parado sale «Sin conexión, reintentando…» y el tablero no cambia; al volver a arrancar, 3333DDD ya está en «Terminada», sin recargar, y el aviso desaparece',
    `antes=${inicio}; aviso=«${aviso}»; con el servidor parado=${durante}; al volver: ${despues} tras ${ms} ms; aviso al final=«${avisoFin}»`,
    inicio === 'En reparación' && aviso.includes('Sin conexión') && durante === 'En reparación' && despues === 'Terminada' && avisoFin === '', 'c02-us4-5-sin-conexion.png, c03-us4-5-al-volver.png');
}
fs.writeFileSync(`${OUT}/resultados.json`, JSON.stringify(res, null, 2));
await nav.close();
