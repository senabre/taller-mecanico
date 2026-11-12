// T039 de spec-008, día B del §9 (pasos 2 a 8), sin recargar los datos demo.
// El servidor y la base corren con el reloj del día B (libfaketime) y cada navegador con su reloj
// en el mismo día (Playwright clock). El tablero de paco se abre al principio y no se toca.
// Uso: node t039b.mjs <carpeta> <carpeta de correos> <hora UTC del día B, p. ej. 2026-11-18T07:30:00Z>
import { chromium } from 'playwright';
import fs from 'node:fs';
import { execSync } from 'node:child_process';

const BASE = 'http://localhost:3000';
const [OUT, CORREO, AHORA] = process.argv.slice(2);
fs.mkdirSync(OUT, { recursive: true });
const res = [];
let n = 0;
const sql = (q) => execSync(`psql -h localhost -U taller -d taller -tAc "${q}"`).toString().trim();
const limpio = (t) => t.replace(/ | /g, ' ');
const espera = (ms) => new Promise((r) => setTimeout(r, ms));
const anota = (paso, esperado, observado, ok, img) => { res.push({ paso, esperado, observado, ok, img }); console.log(`${ok ? 'OK   ' : 'FALLA'} ${paso}: ${observado}`); };
async function captura(p, nombre) { n += 1; const f = `b${String(n).padStart(2, '0')}-${nombre}.png`; await p.screenshot({ path: `${OUT}/${f}`, fullPage: true }); return f; }
const nav = await chromium.launch();
async function contexto(viewport) {
  const c = await nav.newContext({ viewport, locale: 'es-ES', timezoneId: 'Europe/Madrid' });
  await c.clock.install({ time: new Date(AHORA) });
  return c;
}
async function entrar(p, u) {
  await p.goto(`${BASE}/entrar`); await p.fill('input[name=login]', u); await p.fill('input[name=contrasena]', 'demo1234');
  await p.click('button[type=submit]'); await p.waitForTimeout(900);
}
const texto = async (p) => limpio(await p.locator('main').innerText());
async function leer(p) {
  return p.evaluate(() => ({
    cols: [...document.querySelectorAll('main section[aria-label]')].map((s) => ({ nombre: s.getAttribute('aria-label'), mats: [...s.querySelectorAll('li.card .font-mono')].map((x) => x.textContent.trim()) })),
    carga: document.querySelector('section[aria-labelledby=carga] ul')?.innerText.replace(/\s+/g, ' ').trim()
  }));
}
const columnaDe = (t, m) => t.cols.find((c) => c.mats.includes(m))?.nombre ?? null;
async function hasta(p, cond, max = 8000) { const t0 = Date.now(); while (Date.now() - t0 < max) { if (cond(await leer(p))) return Date.now() - t0; await espera(100); } return null; }
const luisDe = (t) => Number((t.carga.match(/Luis (\d+)/) || [])[1]);
function correoCon(cadena) {
  for (const f of fs.readdirSync(CORREO).sort().reverse()) {
    const t = fs.readFileSync(`${CORREO}/${f}`, 'utf8').replace(/\?=\r?\n[ \t]+/g, '?= ').replace(/\r?\n[ \t]+/g, ' ').replace(/=\r?\n/g, '')
      .replace(/(=[0-9A-F]{2})+/g, (m) => Buffer.from(m.split('=').slice(1).map((h) => parseInt(h, 16))).toString('utf8'))
      .replace(/=\?utf-8\?b\?([^?]*)\?=/gi, (_, b) => Buffer.from(b, 'base64').toString('utf8'));
    if (t.includes(cadena)) return t;
  }
  return null;
}
async function lineaMano(p, concepto, horas, boton) {
  const f = p.locator('form', { hasText: boton }).last();
  await f.locator('select').selectOption('mano_obra');
  await f.locator('input[name=concepto]').fill(concepto); await f.locator('input[name=horas]').fill(horas);
  await f.getByRole('button', { name: boton }).click(); await p.waitForTimeout(900);
}

// Paso 2 · Tablero de paco, abierto todo el día B
const cPaco = await contexto({ width: 1920, height: 1080 });
const paco = await cPaco.newPage();
await entrar(paco, 'paco'); await paco.goto(`${BASE}/tablero`); await paco.waitForTimeout(1500);
let t = await leer(paco);
const abiertasPorMec = sql("select string_agg(u.nombre || ' ' || c, ' · ' order by u.nombre) from (select mecanico_id, count(*) c from orden where estado <> 'facturada' group by mecanico_id) x join usuario u on u.id = x.mecanico_id");
const retrasadas = await paco.locator('li.card', { hasText: 'Retrasada' }).locator('.font-mono').allInnerTexts();
anota('§9.2 Tablero de paco (día B)', 'La carga coincide con las tarjetas de cada mecánico; alguna orden demo sale «Retrasada» (normal: los datos son del día A)',
  `carga=«${t.carga}»; en la base=«${abiertasPorMec}»; retrasadas=[${retrasadas}]`, t.carga === abiertasPorMec, await captura(paco, 'tablero-inicio'));
const luis0 = luisDe(t);

// Paso 3 · ana abre la orden de 3579KLM
const cOtro = await contexto({ width: 1280, height: 900 });
let p = await cOtro.newPage();
p.on('dialog', async (d) => { await d.accept(); });
await entrar(p, 'ana');
await p.goto(`${BASE}/citas`); await p.waitForTimeout(1200);
const fila = p.locator('tr', { hasText: '3579KLM' });
const enCitas = (await fila.count()) === 1 && (await fila.getByText('Abrir orden').count()) === 1;
await captura(p, 'citas-de-hoy');
await fila.getByText('Abrir orden').click(); await p.waitForTimeout(900);
const entrega = sql("select to_char((now() at time zone 'Europe/Madrid')::date + 3, 'YYYY-MM-DD')");
await p.fill('input[name=km]', '15000'); await p.fill('textarea[name=danos]', 'Sin daños visibles');
await p.selectOption('select[name=mecanico]', { label: 'Luis' }); await p.fill('input[name=entrega]', entrega);
await p.click('button[type=submit]');
let ms = await hasta(paco, (x) => x.cols.find((c) => c.nombre === 'Recibida').mats.includes('3579KLM'));
await p.waitForTimeout(600);
const tOrden = await texto(p);
const numero = (tOrden.match(/Orden Nº (\d+)/) || [])[1];
t = await leer(paco);
anota('§9.3 ana abre la orden (15.000 km, Luis, entrega en 3 días)', 'Orden Nº 13; en menos de 5 s sale en «Recibida» y Luis suma 1',
  `en Citas de hoy con «Abrir orden»=${enCitas}; ${tOrden.match(/Orden Nº \d+/)?.[0]}; tablero en ${ms} ms; carga=«${t.carga}»`,
  enCitas && numero === '13' && ms !== null && ms < 5000 && luisDe(t) === luis0 + 1, await captura(paco, 'p3-recibida'));
await captura(p, 'p3-orden-13');
const id = Number(sql("select o.id from orden o join cita c on c.id = o.cita_id join vehiculo v on v.id = c.vehiculo_id where v.matricula = '3579KLM'"));

// Paso 4 · luis: diagnóstico, presupuesto y emitir
// luis en su propio navegador (como otro puesto del taller); ana se queda con el suyo.
const pAna = p;
const cLuis = await contexto({ width: 1280, height: 900 });
p = await cLuis.newPage();
p.on('dialog', async (d) => { await d.accept(); });
await entrar(p, 'luis');
await p.goto(`${BASE}/ordenes/${id}`); await p.waitForTimeout(900);
await p.fill('textarea[name=diagnostico]', 'Pastillas delanteras desgastadas');
await p.getByRole('button', { name: 'Guardar diagnóstico' }).click();
ms = await hasta(paco, (x) => columnaDe(x, '3579KLM') === 'En diagnóstico');
anota('§9.4 luis guarda el diagnóstico', 'La tarjeta pasa a «En diagnóstico» en menos de 5 s', `${ms} ms`, ms !== null && ms < 5000, await captura(paco, 'p4-en-diagnostico'));
await p.getByRole('button', { name: 'Nuevo presupuesto' }).click(); await p.waitForTimeout(900);
await lineaMano(p, 'Cambio de pastillas', '1', 'Añadir línea');
await p.getByRole('button', { name: 'Emitir presupuesto' }).click(); await p.waitForTimeout(2500);
const enlace = await p.locator('input[aria-label="Enlace del presupuesto"]').inputValue().catch(() => '');
const correo = enlace ? correoCon(enlace) : null;
t = await leer(paco);
const tP = await texto(p);
anota('§9.4 luis crea el presupuesto (1 h «Cambio de pastillas») y lo emite', 'La tarjeta no se mueve; llega el correo con el enlace',
  `48,40 €=${tP.includes('48,40 €')}; columna=${columnaDe(t, '3579KLM')}; correo con el enlace=${!!correo}`,
  columnaDe(t, '3579KLM') === 'En diagnóstico' && !!correo && tP.includes('48,40 €'), await captura(p, 'p4-presupuesto-emitido'));
await captura(paco, 'p4-tablero-tras-emitir');

// Paso 5 · el cliente abre el enlace del correo y acepta
const cCliente = await contexto({ width: 390, height: 844 });
const cli = await cCliente.newPage();
await cli.goto(enlace); await cli.waitForTimeout(1000);
await cli.getByRole('button', { name: 'Aceptar', exact: true }).click(); await cli.waitForTimeout(300);
await captura(cli, 'p5-confirmar-movil');
await cli.getByRole('button', { name: 'Sí, aceptar' }).click();
ms = await hasta(paco, (x) => columnaDe(x, '3579KLM') === 'En reparación');
await cli.waitForTimeout(500);
const tCli = await texto(cli);
const aceptado = tCli.match(/Aceptado el \d{2}\/\d{2}\/\d{4} a las \d{2}:\d{2}/)?.[0];
anota('§9.5 El cliente acepta desde el enlace', '«Aceptado el … a las …» y la tarjeta pasa a «En reparación» en menos de 5 s', `«${aceptado}»; tablero en ${ms} ms`,
  !!aceptado && aceptado.startsWith('Aceptado el 18/11/2026') && ms !== null && ms < 5000, await captura(cli, 'p5-aceptado-movil'));
await captura(paco, 'p5-en-reparacion');
await cCliente.close();

// Paso 6 · luis imputa 1 h y termina
await p.goto(`${BASE}/ordenes/${id}`); await p.waitForTimeout(900);
await lineaMano(p, 'Cambio de pastillas', '1', 'Imputar');
const acumulado = (await texto(p)).match(/Acumulado [^\n]*/)?.[0];
await p.getByRole('button', { name: 'Terminar reparación' }).click();
ms = await hasta(paco, (x) => columnaDe(x, '3579KLM') === 'Terminada');
anota('§9.6 luis imputa 1 h y termina', 'Sin pasar del 110 %; la tarjeta pasa a «Terminada» en menos de 5 s', `«${acumulado}»; tablero en ${ms} ms`,
  acumulado?.includes('40,00 € de 40,00 €') && ms !== null && ms < 5000, await captura(paco, 'p6-terminada'));

// Paso 7 · ana factura
p = pAna;
await p.goto(`${BASE}/ordenes/${id}`); await p.waitForTimeout(900);
const antes = luisDe(await leer(paco));
await p.getByRole('button', { name: 'Facturar' }).click();
ms = await hasta(paco, (x) => columnaDe(x, '3579KLM') === null);
await p.waitForTimeout(1500);
const tF = await texto(p);
const factura = tF.match(/Factura nº \d+/)?.[0];
const total = sql(`select total from factura where orden_id = ${id}`);
const aviso = correoCon('3579KLM está listo para recoger');
t = await leer(paco);
anota('§9.7 ana factura', '«Factura nº N» con su total; correo «Su vehículo 3579KLM está listo para recoger»; la tarjeta desaparece y Luis resta 1 en menos de 5 s',
  `«${factura}», total ${total} €; correo=${!!aviso}; tablero en ${ms} ms; Luis ${antes} → ${luisDe(t)}`,
  !!factura && !!aviso && ms !== null && ms < 5000 && luisDe(t) === antes - 1, await captura(p, 'p7-factura'));
await captura(paco, 'p7-tablero-sin-3579KLM');

// Paso 8 · historial
await p.goto(`${BASE}/historial`); await p.waitForTimeout(800);
await p.fill('input', '3579KLM'); await p.keyboard.press('Enter'); await p.waitForTimeout(1200);
const tH = await texto(p);
anota('§9.8 Historial de 3579KLM', 'La reparación con su diagnóstico y sus líneas', `diagnóstico=${tH.includes('Pastillas delanteras desgastadas')}; línea=${tH.includes('Cambio de pastillas')}; 15.000 km=${tH.includes('15.000')}`,
  tH.includes('Pastillas delanteras desgastadas') && tH.includes('Cambio de pastillas'), await captura(p, 'p8-historial'));

fs.writeFileSync(`${OUT}/resultados-b.json`, JSON.stringify(res, null, 2));
await nav.close();
console.log(`\n${res.filter((x) => x.ok).length}/${res.length} comprobaciones superadas`);
