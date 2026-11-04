// Recorrido del quickstart común §2 (spec-002, US1) y de la ficha compartida, con capturas.
// Uso: node t029.mjs <carpeta> <día de la cita de 1234BCD, YYYY-MM-DD>
import { chromium } from 'playwright';
import fs from 'node:fs';

const BASE = 'http://localhost:3000';
const OUT = process.argv[2];
const DIA_1234 = process.argv[3];
fs.mkdirSync(OUT, { recursive: true });
const res = [];
let n = 0;
const nav = await chromium.launch();
const madrid = (d) => new Date(d.toLocaleString('en-US', { timeZone: 'Europe/Madrid' }));
const iso = (d) => `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
const hoyD = madrid(new Date());
const masDias = (k) => { const d = new Date(hoyD); d.setDate(d.getDate() + k); return iso(d); };
const HOY = iso(hoyD), AYER = masDias(-1);

async function captura(p, nombre) {
  n += 1; const f = `${String(n).padStart(2, '0')}-${nombre}.png`;
  await p.screenshot({ path: `${OUT}/${f}`, fullPage: true }); return f;
}
function anota(paso, esperado, observado, ok, img) {
  res.push({ paso, esperado, observado, ok, img }); console.log(`${ok ? 'OK  ' : 'FALLA'} ${paso}: ${observado}`);
}
async function pagina() {
  const c = await nav.newContext({ viewport: { width: 1280, height: 900 }, locale: 'es-ES', timezoneId: 'Europe/Madrid' });
  return c.newPage();
}
async function entrar(p, login) {
  await p.goto(`${BASE}/entrar`); await p.fill('input[name=login]', login);
  await p.fill('input[name=contrasena]', 'demo1234'); await p.click('button[type=submit]'); await p.waitForTimeout(800);
}
const texto = (p) => p.locator('main').innerText();
async function filasCitas(p) {
  return p.$$eval('table tbody tr', (trs) => trs.map((tr) => ({ m: tr.cells[0].innerText.trim(), t: tr.innerText })));
}
async function verDia(p, d) { await p.fill('input[name=dia]', d); await p.waitForTimeout(700); }
async function errorDe(p, nombre) {
  return p.locator(`[name=${nombre}]`).locator('xpath=ancestor::label[1]').locator('[role=alert]').innerText().catch(() => '');
}
async function rellenar(p, { km, danos, mecanico, entrega }) {
  await p.fill('input[name=km]', km); await p.fill('textarea[name=danos]', danos);
  if (mecanico) await p.selectOption('select[name=mecanico]', { label: mecanico });
  await p.fill('input[name=entrega]', entrega);
}
async function guardar(p) { await p.click('button[type=submit]'); await p.waitForTimeout(900); }

// Paso 1
const ana = await pagina();
await entrar(ana, 'ana');
await ana.goto(`${BASE}/citas`); await ana.waitForTimeout(800);
{
  const f = await filasCitas(ana); const img = await captura(ana, 'p1-citas-de-hoy');
  const con = ['1313MMM', '1414NNN', '8888KKK'].map((m) => f.find((x) => x.m === m)?.t.includes('Abrir orden'));
  anota('§2.1 Citas de hoy', '1313MMM, 1414NNN y 8888KKK con «Abrir orden»', `con botón=${con.join(',')}`, con.every(Boolean), img);
}
await verDia(ana, DIA_1234);
{
  const f = await filasCitas(ana); const fila = f.find((x) => x.m === '1234BCD'); const img = await captura(ana, 'p1-cita-otro-dia');
  anota('§2.1 Cita de otro día (1234BCD)', 'Sin «Abrir orden»', fila ? (fila.t.includes('Abrir orden') ? 'tiene botón' : 'sin botón') : 'no aparece', !!fila && !fila.t.includes('Abrir orden'), img);
}
await verDia(ana, AYER);
{
  const f = await filasCitas(ana); const pp = f.find((x) => x.m === '1515PPP'); const bb = f.find((x) => x.m === '1111BBB');
  const img = await captura(ana, 'p1-ayer');
  anota('§2.1 Ayer (Q2, Q5)', '1515PPP sin botón; 1111BBB con enlace «Orden Nº 1»',
    `1515PPP botón=${pp?.t.includes('Abrir orden')}; 1111BBB=${bb?.t.match(/Orden Nº \d+/)?.[0]}`,
    !!pp && !pp.t.includes('Abrir orden') && /Orden Nº 1\b/.test(bb?.t || ''), img);
}
{ // intento por URL de abrir la orden de una cita de otro día
  const id = await ana.evaluate(async (d) => (await (await fetch(`/api/citas?fecha=${d}`)).json()).find((c) => c.matricula === '1515PPP').id, AYER);
  await ana.goto(`${BASE}/ordenes/nueva?cita=${id}`); await ana.waitForTimeout(800);
  const t = await texto(ana); const img = await captura(ana, 'p1-ayer-por-url');
  anota('§2.1 Abrir por URL una cita de ayer (US1-5)', '«Solo se puede abrir la orden el día de la cita»', t.includes('Solo se puede abrir la orden el día de la cita') ? 'mensaje visible' : t.slice(0, 120), t.includes('Solo se puede abrir la orden el día de la cita'), img);
}

// Paso 2
await ana.goto(`${BASE}/citas`); await ana.waitForTimeout(800);
const t0 = Date.now();
await ana.locator('tr', { hasText: '1313MMM' }).getByText('Abrir orden').click(); await ana.waitForTimeout(800);
{
  const t = await texto(ana);
  const mec = await ana.$$eval('select[name=mecanico] option', (os) => os.map((o) => o.textContent.trim()).filter((x) => x && x !== 'Elige un mecánico'));
  const img = await captura(ana, 'p2-formulario');
  anota('§2.2 Formulario de 1313MMM (US1-1, US1-7)', 'Matrícula, marca, modelo y motivo; mecánicos Luis y Marta, sin Pedro',
    `1313MMM=${t.includes('1313MMM')}; mecánicos=${mec.join(', ')}`, t.includes('1313MMM') && mec.join(',') === 'Luis,Marta', img);
}
// Paso 3
const casos = [
  ['danos', { km: '85000', danos: '', mecanico: 'Luis', entrega: masDias(3) }, 'Describe los daños visibles o escribe “Sin daños visibles”', 'p3-danos-vacios'],
  ['km', { km: '-5', danos: 'Rayón', mecanico: 'Luis', entrega: masDias(3) }, 'El kilometraje debe ser un número entero igual o mayor que 0', 'p3-km-negativo'],
  ['entrega', { km: '85000', danos: 'Rayón', mecanico: 'Luis', entrega: AYER }, 'La fecha de entrega prometida no puede ser anterior a hoy', 'p3-entrega-ayer'],
];
for (const [campo, datos, msg, nombre] of casos) {
  await rellenar(ana, datos); await guardar(ana);
  const e = await errorDe(ana, campo); const url = new URL(ana.url()).pathname; const img = await captura(ana, nombre);
  anota(`§2.3 Error de ${campo} (US1-8, FR-003, FR-006)`, `«${msg}» bajo su campo y no se guarda`, `mensaje=«${e}»; página=${url}`, e === msg && url === '/ordenes/nueva', img);
}
// Paso 4
await rellenar(ana, { km: '85000', danos: 'Rayón puerta trasera izquierda', mecanico: 'Luis', entrega: masDias(3) });
await guardar(ana);
const seg = ((Date.now() - t0) / 1000).toFixed(1);
{
  const t = await texto(ana); const img = await captura(ana, 'p4-ficha-orden-10');
  const ok = /Orden Nº 10\b/.test(t) && t.includes('Recibida') && t.includes('85.000 km') && t.includes('1313MMM') && t.includes('Luis');
  anota('§2.4 Ficha de la orden (US1-2, US1-3, SC-002)', 'Orden Nº 10, «Recibida», 85.000 km, cita, matrícula y mecánico',
    `${t.match(/Orden Nº \d+/)?.[0]}; Recibida=${t.includes('Recibida')}; 85.000 km=${t.includes('85.000 km')}; url=${new URL(ana.url()).pathname}`, ok, img);
  anota('§2.4 SC-001 (tiempo de máquina)', 'Recorrido desde la lista hasta la ficha', `${seg} s de máquina con 3 errores intermedios; el cronometraje humano queda pendiente`, true, img);
}
await ana.goto(`${BASE}/citas`); await ana.waitForTimeout(800);
{
  const fila = (await filasCitas(ana)).find((x) => x.m === '1313MMM'); const img = await captura(ana, 'p4-citas-orden-10');
  await ana.locator('tr', { hasText: '1313MMM' }).getByText(/Orden Nº 10/).click(); await ana.waitForTimeout(700);
  const ok = /Orden Nº 10\b/.test(fila?.t || '') && !fila.t.includes('Abrir orden') && /\/ordenes\/\d+$/.test(ana.url());
  anota('§2.4 Enlace en Citas (US1-4, Q5)', '1313MMM muestra «Orden Nº 10» y lleva a la ficha', `${fila?.t.match(/Orden Nº \d+/)?.[0]}; tras pulsar=${new URL(ana.url()).pathname}`, ok, img);
}
// Paso 5
{
  const A = await pagina(); const B = await pagina();
  for (const p of [A, B]) { await entrar(p, 'ana'); await p.goto(`${BASE}/citas`); await p.waitForTimeout(700);
    await p.locator('tr', { hasText: '1414NNN' }).getByText('Abrir orden').click(); await p.waitForTimeout(700);
    await rellenar(p, { km: '23500', danos: 'Sin daños visibles', mecanico: 'Luis', entrega: masDias(2) }); }
  await guardar(A);
  const tA = await texto(A); const imgA = await captura(A, 'p5-pestana-1-orden-11');
  anota('§2.5 Primera pestaña (SC-002, US1-9)', 'Orden Nº 11 (consecutiva)', tA.match(/Orden Nº \d+/)?.[0] || 'sin orden', /Orden Nº 11\b/.test(tA), imgA);
  await guardar(B);
  const tB = await texto(B); const enlace = await B.getByText('Ver la orden').getAttribute('href').catch(() => null);
  const imgB = await captura(B, 'p5-pestana-2-ya-tiene-orden');
  anota('§2.5 Segunda pestaña (US1-4, FR-008)', '«Esta cita ya tiene una orden (Nº 11)» con enlace a la orden', `mensaje=${tB.includes('Esta cita ya tiene una orden (Nº 11)')}; enlace=${enlace}`,
    tB.includes('Esta cita ya tiene una orden (Nº 11)') && !!enlace, imgB);
  await A.context().close(); await B.context().close();
}
await ana.goto(`${BASE}/ordenes`); await ana.waitForTimeout(800);
{
  const f = await ana.$$eval('table tbody tr', (trs) => trs.map((tr) => [...tr.cells].map((c) => c.innerText.trim())));
  const de1414 = f.filter((x) => x[1] === '1414NNN').length; const nums = f.map((x) => Number(x[0]));
  const desc = nums[0] === 11 && nums[1] === 10; // orden por fecha de apertura (contrato); las dos nuevas, primero
  const t = await texto(ana); const img = await captura(ana, 'p5-ordenes');
  anota('§2.5 «Órdenes» (FR-008)', 'Una sola orden de 1414NNN; las recién abiertas (11 y 10) primero, por fecha de apertura', `órdenes de 1414NNN=${de1414}; números=${nums.join(',')}`, de1414 === 1 && desc && nums[0] === 11, img);
  const fila8 = f.find((x) => x[0] === '8');
  anota('Lista: orden 8 retrasada', '«Retrasada» en la orden 8', fila8 ? fila8.join(' · ') : 'no aparece', !!fila8 && fila8.join(' ').includes('Retrasada'), img);
}
// Paso 7 y ficha
const luis = await pagina();
await entrar(luis, 'luis');
await luis.goto(`${BASE}/ordenes`); await luis.waitForTimeout(800);
{
  const t = await texto(luis);
  const nums = await luis.$$eval('table tbody tr', (trs) => trs.map((tr) => tr.cells[0].innerText.trim()));
  const mec = await luis.$$eval('table tbody tr', (trs) => [...new Set(trs.map((tr) => tr.cells[3].innerText.trim()))]);
  const img = await captura(luis, 'p7-mis-ordenes-luis');
  anota('§2.7 «Mis órdenes» de luis', 'Título «Mis órdenes», con la 10 y la 11 y sin órdenes de Marta', `título=${t.split('\n')[0]}; números=${nums.join(',')}; mecánicos=${mec.join(',')}`,
    t.startsWith('Mis órdenes') && nums.includes('10') && nums.includes('11') && mec.every((m) => m === 'Luis'), img);
}
await luis.getByRole('link', { name: '10', exact: true }).click(); await luis.waitForTimeout(900);
{
  const secciones = await luis.$$eval('h2', (hs) => hs.map((h) => h.innerText.trim()));
  const editable = await luis.locator('section', { hasText: 'Diagnóstico' }).locator('textarea').count();
  const img = await captura(luis, 'ficha-luis');
  anota('Ficha como luis (T026)', 'Datos, Diagnóstico, Presupuesto, Imputaciones y Pedidos; diagnóstico editable', `secciones=${secciones.join(', ')}; textarea de diagnóstico=${editable}`,
    ['Datos', 'Diagnóstico', 'Presupuesto', 'Imputaciones', 'Pedidos'].every((s) => secciones.some((x) => x.startsWith(s))) && editable === 1, img);
  await luis.locator('section', { hasText: 'Diagnóstico' }).locator('textarea').fill('Pastillas delanteras desgastadas');
  await luis.locator('section', { hasText: 'Diagnóstico' }).getByRole('button').first().click(); await luis.waitForTimeout(900);
  const t = await luis.locator('main').innerText(); const img2 = await captura(luis, 'ficha-luis-diagnostico');
  anota('Ficha: guardar diagnóstico', 'El estado pasa a «En diagnóstico» sin recargar', `En diagnóstico=${t.includes('En diagnóstico')}`, t.includes('En diagnóstico'), img2);
}
{
  const url = luis.url(); await ana.goto(url); await ana.waitForTimeout(900);
  const editable = await ana.locator('section', { hasText: 'Diagnóstico' }).locator('textarea').count();
  const secciones = await ana.$$eval('h2', (hs) => hs.map((h) => h.innerText.trim()));
  const img = await captura(ana, 'ficha-ana');
  anota('Ficha como ana', 'Ve las secciones y no puede editar', `secciones=${secciones.length}; textarea de diagnóstico=${editable}`, secciones.length >= 5 && editable === 0, img);
}
await ana.goto(`${BASE}/ordenes/999`); await ana.waitForTimeout(800);
{ const t = await texto(ana); const img = await captura(ana, 'ficha-999');
  anota('Ficha inexistente', '«No existe la orden»', t.trim().slice(0, 60), t.includes('No existe la orden'), img); }
for (const [num, que, re] of [['2', 'presupuesto e imputaciones', /Presupuesto[\s\S]*(Aceptado|aceptado)[\s\S]*Imputaciones[\s\S]*\d+,\d{2}[\s ]€/], ['6', 'un pedido pendiente', /Pedidos[\s\S]*(BJ-7|Pendiente|pendiente)/], ['8', '«Retrasada»', /Retrasada/]]) {
  const id = await ana.evaluate(async (nm) => (await (await fetch('/api/ordenes')).json()).find((o) => String(o.numero) === nm)?.id, num);
  await ana.goto(`${BASE}/ordenes/${id}`); await ana.waitForTimeout(900);
  const t = await texto(ana); const img = await captura(ana, `ficha-orden-${num}`);
  anota(`Ficha de la orden demo ${num}`, `Muestra ${que}`, re.test(t) ? 'sí' : t.replace(/\s+/g, ' ').slice(0, 160), re.test(t), img);
}

fs.writeFileSync(`${OUT}/resultados.json`, JSON.stringify(res, null, 2));
await nav.close();
console.log(`\n${res.filter((x) => x.ok).length}/${res.length} comprobaciones superadas`);
