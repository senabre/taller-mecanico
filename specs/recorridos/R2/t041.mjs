// Segundo recorrido integrado (enunciado 5.4): condición de error, permiso y estado, cruzando historias de
// las dos personas. 1414NNN: ana abre la orden (002, A) → luis diagnostica y presupuesta (003, B) → el
// cliente RECHAZA por el enlace (004, A) → no se puede imputar (005, B) ni facturar (007, B) → el enlace ya
// no admite otra respuesta (004, A) → almacén y mecánico no ven el tablero (008, A) → luis hace un
// presupuesto nuevo, el cliente lo acepta y la orden sigue su camino (003/004) con el tablero al día (008).
// Base recién cargada con demo.sql. Uso: node t041.mjs <carpeta> <carpeta de correos>
import { chromium } from 'playwright';
import fs from 'node:fs';
import { execSync } from 'node:child_process';

const BASE = 'http://localhost:3000';
const [OUT, CORREO] = process.argv.slice(2);
fs.mkdirSync(OUT, { recursive: true });
const res = [];
let n = 0;
const sql = (q) => execSync(`psql -h localhost -U taller -d taller -tAc "${q}"`).toString().trim();
const limpio = (t) => t.replace(/ | /g, ' ');
const espera = (ms) => new Promise((r) => setTimeout(r, ms));
const anota = (paso, esperado, observado, ok, img) => { res.push({ paso, esperado, observado, ok, img }); console.log(`${ok ? 'OK   ' : 'FALLA'} ${paso}: ${observado}`); };
async function captura(p, nombre) { n += 1; const f = `r${String(n).padStart(2, '0')}-${nombre}.png`; await p.screenshot({ path: `${OUT}/${f}`, fullPage: true }); return f; }
const nav = await chromium.launch();
async function sesion(u, viewport = { width: 1280, height: 900 }) {
  const c = await nav.newContext({ viewport, locale: 'es-ES', timezoneId: 'Europe/Madrid' });
  const p = await c.newPage(); p.on('dialog', async (d) => { await d.accept(); });
  if (u) { await p.goto(`${BASE}/entrar`); await p.fill('input[name=login]', u); await p.fill('input[name=contrasena]', 'demo1234'); await p.click('button[type=submit]'); await p.waitForTimeout(900); }
  return p;
}
const texto = async (p) => limpio(await p.locator('main').innerText());
async function api(p, metodo, ruta, cuerpo) {
  return p.evaluate(async ([m, r, b]) => { const x = await fetch(`/api${r}`, { method: m, headers: { 'content-type': 'application/json' }, body: b === undefined ? undefined : JSON.stringify(b) });
    let j = null; try { j = await x.json(); } catch {} return { status: x.status, json: j }; }, [metodo, ruta, cuerpo]);
}
const columnaDe = (p, m) => p.evaluate((m) => [...document.querySelectorAll('main section[aria-label]')].find((s) => [...s.querySelectorAll('li.card .font-mono')].some((x) => x.textContent.trim() === m))?.getAttribute('aria-label') ?? null, m);
async function hastaColumna(p, m, col, max = 8000) { const t0 = Date.now(); while (Date.now() - t0 < max) { if ((await columnaDe(p, m)) === col) return Date.now() - t0; await espera(100); } return null; }
async function lineaMano(p, concepto, horas, boton) {
  const f = p.locator('form', { hasText: boton }).last();
  await f.locator('select').selectOption('mano_obra');
  await f.locator('input[name=concepto]').fill(concepto); await f.locator('input[name=horas]').fill(horas);
  await f.getByRole('button', { name: boton }).click(); await p.waitForTimeout(900);
}
const fechaHora = (iso) => { const f = new Intl.DateTimeFormat('es-ES', { timeZone: 'Europe/Madrid', day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit', hour12: false }).formatToParts(new Date(iso));
  const g = (t) => f.find((x) => x.type === t).value; return `${g('day')}/${g('month')}/${g('year')} a las ${g('hour')}:${g('minute')}`; };

// 0 · Tablero de paco, abierto todo el recorrido
const paco = await sesion('paco', { width: 1920, height: 1080 });
await paco.goto(`${BASE}/tablero`); await paco.waitForTimeout(1500);

// 1 · ana abre la orden de 1414NNN (spec-002, Persona A)
const ana = await sesion('ana');
await ana.goto(`${BASE}/citas`); await ana.waitForTimeout(1200);
await ana.locator('tr', { hasText: '1414NNN' }).getByText('Abrir orden').click(); await ana.waitForTimeout(900);
const entrega = sql("select to_char((now() at time zone 'Europe/Madrid')::date + 3, 'YYYY-MM-DD')");
await ana.fill('input[name=km]', '23500'); await ana.fill('textarea[name=danos]', 'Sin daños visibles');
await ana.selectOption('select[name=mecanico]', { label: 'Luis' }); await ana.fill('input[name=entrega]', entrega);
await ana.click('button[type=submit]');
let ms = await hastaColumna(paco, '1414NNN', 'Recibida');
await ana.waitForTimeout(600);
const num = (await texto(ana)).match(/Orden Nº (\d+)/)?.[1];
const id = Number(sql("select o.id from orden o join cita c on c.id = o.cita_id join vehiculo v on v.id = c.vehiculo_id where v.matricula = '1414NNN'"));
anota('R2.1 ana abre la orden de 1414NNN (002 · A)', 'Orden Nº 10; su tarjeta sale en «Recibida»', `Orden Nº ${num}; tablero en ${ms} ms`, num === '10' && ms !== null, await captura(ana, 'orden-abierta'));

// 2 · luis diagnostica, presupuesta y emite (spec-003, Persona B)
const luis = await sesion('luis');
await luis.goto(`${BASE}/ordenes/${id}`); await luis.waitForTimeout(900);
await luis.fill('textarea[name=diagnostico]', 'Aceite degradado');
await luis.getByRole('button', { name: 'Guardar diagnóstico' }).click();
ms = await hastaColumna(paco, '1414NNN', 'En diagnóstico');
await luis.getByRole('button', { name: 'Nuevo presupuesto' }).click(); await luis.waitForTimeout(900);
await lineaMano(luis, 'Cambio de aceite', '1', 'Añadir línea');
await luis.getByRole('button', { name: 'Emitir presupuesto' }).click(); await luis.waitForTimeout(2000);
const enlace1 = await luis.locator('input[aria-label="Enlace del presupuesto"]').inputValue().catch(() => '');
anota('R2.2 luis diagnostica y emite un presupuesto de 48,40 € (003 · B)', 'La tarjeta pasa a «En diagnóstico»; emitir no la mueve; hay enlace para el cliente',
  `tablero en ${ms} ms; columna tras emitir=${await columnaDe(paco, '1414NNN')}; enlace=${!!enlace1}; 48,40 €=${(await texto(luis)).includes('48,40 €')}`,
  ms !== null && (await columnaDe(paco, '1414NNN')) === 'En diagnóstico' && !!enlace1, await captura(luis, 'presupuesto-emitido'));
const token1 = new URL(enlace1).pathname.split('/').pop();

// 3 · El cliente lo RECHAZA desde el enlace (spec-004, Persona A)
const cli = await sesion(null, { width: 390, height: 844 });
await cli.goto(enlace1); await cli.waitForTimeout(1000);
await cli.getByRole('button', { name: 'Rechazar' }).click(); await cli.waitForTimeout(300);
const pregunta = (await texto(cli)).includes('¿Rechaza este presupuesto por 48,40 €? Una vez confirmado, no se podrá cambiar.');
await captura(cli, 'confirmar-rechazo-movil');
await cli.getByRole('button', { name: 'Sí, rechazar' }).click(); await cli.waitForTimeout(1200);
const r1 = Number(sql(`select (extract(epoch from respondido_en) * 1000)::bigint from presupuesto where token = '${token1}'`));
const msgR = `Rechazado el ${fechaHora(r1)}`;
await paco.waitForTimeout(2000);
anota('R2.3 El cliente rechaza desde el móvil (004 · A)', `Confirmación dentro de la página; «${msgR}»; la orden sigue «En diagnóstico» y la tarjeta no se mueve`,
  `confirmación=${pregunta}; página=${(await texto(cli)).includes(msgR)}; orden=${sql(`select estado from orden where id = ${id}`)}; tablero=${await columnaDe(paco, '1414NNN')}`,
  pregunta && (await texto(cli)).includes(msgR) && sql(`select estado from orden where id = ${id}`) === 'en_diagnostico' && (await columnaDe(paco, '1414NNN')) === 'En diagnóstico',
  await captura(cli, 'rechazado-movil'));
await captura(paco, 'tablero-tras-rechazo');

// 4 · Estado: no se puede imputar (spec-005, Persona B)
await luis.goto(`${BASE}/ordenes/${id}`); await luis.waitForTimeout(900);
let t = await texto(luis);
const botonImputar = await luis.getByRole('button', { name: 'Imputar' }).count();
const rImp = await api(luis, 'POST', `/ordenes/${id}/imputaciones`, { tipo: 'mano_obra', concepto: 'Cambio de aceite', cantidad: '1' });
const rTerm = await api(luis, 'POST', `/ordenes/${id}/terminar`);
anota('R2.4 luis no puede imputar ni terminar (005 · B)', '«Inicial · Rechazado el …»; sin botón «Imputar» y con el aviso; por la API, 409 y nada se guarda',
  `ficha=${t.includes(`Inicial · ${msgR}`)}; aviso=${t.includes('solo se imputa en órdenes en reparación')}; botón=${botonImputar}; imputar API=${rImp.status} «${rImp.json?.error}»; terminar API=${rTerm.status} «${rTerm.json?.error}»; imputaciones en la base=${sql(`select count(*) from imputacion where orden_id = ${id}`)}`,
  t.includes(`Inicial · ${msgR}`) && botonImputar === 0 && rImp.status === 409 && rTerm.status >= 400 && sql(`select count(*) from imputacion where orden_id = ${id}`) === '0',
  await captura(luis, 'sin-imputar'));

// 5 · Estado: no se puede facturar (spec-007, Persona B)
await ana.goto(`${BASE}/ordenes/${id}`); await ana.waitForTimeout(900);
const botonFact = await ana.getByRole('button', { name: 'Facturar' }).count();
const rFac = await api(ana, 'POST', `/ordenes/${id}/factura`);
anota('R2.5 ana no puede facturar (007 · B)', 'Sin botón «Facturar»; por la API, rechazo y ninguna factura', `botón=${botonFact}; API=${rFac.status} «${rFac.json?.error}»; facturas=${sql(`select count(*) from factura where orden_id = ${id}`)}`,
  botonFact === 0 && rFac.status >= 400 && sql(`select count(*) from factura where orden_id = ${id}`) === '0', await captura(ana, 'sin-facturar'));

// 6 · Error: el mismo enlace no admite otra respuesta (spec-004, Persona A)
await cli.goto(enlace1); await cli.waitForTimeout(1000);
const botonesCli = await cli.getByRole('button').count();
const rAcc = await api(cli, 'POST', `/publico/presupuestos/${token1}/respuesta`, { decision: 'aceptar' });
anota('R2.6 El cliente vuelve a abrir el enlace e intenta aceptar (004 · A)', 'Se ve su rechazo, sin botones; aceptar por la API da 409 «Este presupuesto ya fue respondido» y la orden no cambia',
  `botones=${botonesCli}; API=${rAcc.status} «${rAcc.json?.error}»; presupuesto=${sql(`select estado from presupuesto where token = '${token1}'`)}; orden=${sql(`select estado from orden where id = ${id}`)}`,
  botonesCli === 0 && rAcc.status === 409 && rAcc.json?.error === 'Este presupuesto ya fue respondido' && sql(`select estado from orden where id = ${id}`) === 'en_diagnostico',
  await captura(cli, 'enlace-ya-respondido'));

// 7 · Permiso: almacén y mecánico no ven el tablero (spec-008, Persona A)
for (const [quien, p] of [['pepe', await sesion('pepe')], ['luis', luis]]) {
  const menu = await p.locator('header').getByRole('link', { name: 'Tablero' }).count();
  await p.goto(`${BASE}/tablero`); await p.waitForTimeout(900);
  const tt = await texto(p); const rT = await api(p, 'GET', '/tablero');
  anota(`R2.7 ${quien} intenta ver el tablero (008 · A)`, 'Sin «Tablero» en el menú; «No tienes permiso para hacer esto», sin tarjetas; la API da 403',
    `menú=${menu}; mensaje=${tt.includes('No tienes permiso para hacer esto')}; tarjetas=${await p.locator('li.card').count()}; API=${rT.status}`,
    menu === 0 && tt.includes('No tienes permiso para hacer esto') && rT.status === 403, await captura(p, `${quien}-sin-tablero`));
}

// 8 · Salida: presupuesto nuevo, el cliente lo acepta y la orden sigue (003 · B, 004 · A, 008 · A)
await luis.goto(`${BASE}/ordenes/${id}`); await luis.waitForTimeout(900);
await luis.getByRole('button', { name: 'Nuevo presupuesto' }).click(); await luis.waitForTimeout(900);
await lineaMano(luis, 'Cambio de aceite', '1', 'Añadir línea');
await luis.getByRole('button', { name: 'Emitir presupuesto' }).click(); await luis.waitForTimeout(2000);
const enlace2 = await luis.locator('input[aria-label="Enlace del presupuesto"]').inputValue().catch(() => '');
await cli.goto(enlace2); await cli.waitForTimeout(1000);
await cli.getByRole('button', { name: 'Aceptar', exact: true }).click(); await cli.waitForTimeout(200);
await cli.getByRole('button', { name: 'Sí, aceptar' }).click();
ms = await hastaColumna(paco, '1414NNN', 'En reparación');
await luis.goto(`${BASE}/ordenes/${id}`); await luis.waitForTimeout(900);
t = await texto(luis);
const lineas = (t.match(/Inicial · (Rechazado|Aceptado) el \d{2}\/\d{2}\/\d{4} a las \d{2}:\d{2}/g) || []);
anota('R2.8 Presupuesto nuevo aceptado: la orden sale del bloqueo', 'Distinto enlace; al aceptar, la tarjeta pasa a «En reparación»; la ficha conserva las dos respuestas',
  `enlace nuevo=${!!enlace2 && enlace2 !== enlace1}; tablero en ${ms} ms; respuestas en la ficha=[${lineas.join(' | ')}]; imputar ya visible=${await luis.getByRole('button', { name: 'Imputar' }).count()}`,
  !!enlace2 && enlace2 !== enlace1 && ms !== null && ms < 5000 && lineas.length === 2, await captura(luis, 'dos-respuestas'));
await captura(paco, 'tablero-en-reparacion');

fs.writeFileSync(`${OUT}/resultados.json`, JSON.stringify(res, null, 2));
await nav.close();
console.log(`\n${res.filter((x) => x.ok).length}/${res.length} comprobaciones superadas`);
