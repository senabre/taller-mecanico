// Quickstart común §2.6 (US2, km frente a la visita anterior) y /ordenes/nueva en ancho de móvil.
// Uso: node t029b.mjs <carpeta> <número esperado para la orden de 8888KKK>
import { chromium } from 'playwright';
import fs from 'node:fs';

const BASE = 'http://localhost:3000';
const OUT = process.argv[2];
const NUM = process.argv[3];
fs.mkdirSync(OUT, { recursive: true });
const res = [];
let n = 0;
const nav = await chromium.launch();
const madrid = new Date(new Date().toLocaleString('en-US', { timeZone: 'Europe/Madrid' }));
const dentro = (k) => { const d = new Date(madrid); d.setDate(d.getDate() + k);
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`; };
async function captura(p, nombre) { n += 1; const f = `b${String(n).padStart(2, '0')}-${nombre}.png`;
  await p.screenshot({ path: `${OUT}/${f}`, fullPage: true }); return f; }
function anota(paso, esperado, observado, ok, img) { res.push({ paso, esperado, observado, ok, img });
  console.log(`${ok ? 'OK  ' : 'FALLA'} ${paso}: ${observado}`); }
async function pagina(viewport = { width: 1280, height: 900 }) {
  const c = await nav.newContext({ viewport, locale: 'es-ES', timezoneId: 'Europe/Madrid' }); const p = await c.newPage();
  await p.goto(`${BASE}/entrar`); await p.fill('input[name=login]', 'ana'); await p.fill('input[name=contrasena]', 'demo1234');
  await p.click('button[type=submit]'); await p.waitForTimeout(800); return p;
}
async function abrir(p, matricula) {
  await p.goto(`${BASE}/citas`); await p.waitForTimeout(800);
  await p.locator('tr', { hasText: matricula }).getByText('Abrir orden').click(); await p.waitForTimeout(800);
}
async function rellenar(p, km) {
  await p.fill('input[name=km]', km); await p.fill('textarea[name=danos]', 'Sin daños visibles');
  await p.selectOption('select[name=mecanico]', { label: 'Luis' }); await p.fill('input[name=entrega]', dentro(3));
  await p.click('button[type=submit]'); await p.waitForTimeout(900);
}
const errorKm = (p) => p.locator('[name=km]').locator('xpath=ancestor::label[1]').locator('[role=alert]').innerText().catch(() => '');

const p = await pagina();
await abrir(p, '8888KKK');
await rellenar(p, '30000');
{
  const e = await errorKm(p); const img = await captura(p, 'p6-km-menor');
  const msg = 'El kilometraje no puede ser menor que el de la visita anterior (39.800 km)';
  anota('§2.6 8888KKK con 30.000 km (US2, FR-004)', `«${msg}» bajo Kilómetros y no se guarda`,
    `mensaje=«${e}»; página=${new URL(p.url()).pathname}`, e === msg && new URL(p.url()).pathname === '/ordenes/nueva', img);
}
if (!process.env.SOLO_RECHAZO) await rellenar(p, '40000');
if (!process.env.SOLO_RECHAZO) {
  const t = await p.locator('main').innerText(); const img = await captura(p, 'p6-km-mayor-se-guarda');
  const ok = new RegExp(`Orden Nº ${NUM}\\b`).test(t) && t.includes('40.000 km');
  anota('§2.6 Corregido a 40.000 km (US2, SC-002)', `Se guarda como Orden Nº ${NUM}: el rechazo no gastó número`,
    `${t.match(/Orden Nº \d+/)?.[0]}; 40.000 km=${t.includes('40.000 km')}`, ok, img);
}
if (!process.env.SIN_MOVIL) {
  const m = await pagina({ width: 390, height: 844 });
  await abrir(m, '1414NNN').catch(() => {});
  if (!new URL(m.url()).pathname.startsWith('/ordenes/nueva')) {
    const id = await m.evaluate(async () => (await (await fetch('/api/citas')).json()).find((c) => !c.orden_id)?.id);
    await m.goto(`${BASE}/ordenes/nueva?cita=${id}`); await m.waitForTimeout(800);
  }
  const ancho = await m.evaluate(() => document.documentElement.scrollWidth);
  const campos = await Promise.all(['km', 'danos', 'mecanico', 'entrega'].map((k) => m.locator(`[name=${k}]`).isVisible()));
  const img = await captura(m, 'movil-ordenes-nueva');
  anota('/ordenes/nueva en móvil (390 px)', 'Todos los campos visibles y sin desplazamiento horizontal', `ancho=${ancho}px; campos visibles=${campos.join(',')}`,
    ancho <= 390 && campos.every(Boolean), img);
}
fs.writeFileSync(`${OUT}/resultados-b.json`, JSON.stringify(res, null, 2));
await nav.close();
console.log(`\n${res.filter((x) => x.ok).length}/${res.length} comprobaciones superadas`);
