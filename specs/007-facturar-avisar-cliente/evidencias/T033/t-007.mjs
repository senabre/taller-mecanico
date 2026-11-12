// spec-007, analyze-1 C1 (principio II): la confirmación de «Facturar» y el mensaje del historial los
// pone la aplicación, en español, y no el navegador. Como ana (recepción).
//  (a) Orden demo 8 (terminada): «Facturar» pregunta en la página, sin diálogo del navegador; «Volver» no
//      factura y «Sí, facturar» emite la factura (a 390 px, sin desplazamiento horizontal).
//  (b) /historial sin matrícula → «Escribe la matrícula», sin mensaje del navegador.
//  (c) /historial con «9999 lll» → la reparación de la orden 9 (camino normal).
// Uso (desde una carpeta con playwright 1.48 instalado): node t-007.mjs <carpeta-salida> <antes|despues>
import { chromium } from 'playwright';
import fs from 'node:fs';

const BASE = 'http://127.0.0.1:3000';
const OUT = process.argv[2];
const FASE = process.argv[3];
fs.mkdirSync(OUT, { recursive: true });
const res = [];
const anota = (paso, esperado, observado, ok) => {
  res.push({ fase: FASE, paso, esperado, observado, ok });
  console.log(`${ok ? 'OK   ' : 'FALLA'} ${paso}: ${observado}`);
};

const nav = await chromium.launch();
const ctx = await nav.newContext({ viewport: { width: 1280, height: 900 }, locale: 'es-ES', timezoneId: 'Europe/Madrid' });
const p = await ctx.newPage();
const dialogos = [];
p.on('dialog', async (d) => {
  dialogos.push(d.message());
  await d.dismiss();
});
await p.goto(`${BASE}/entrar`);
await p.fill('input[name=login]', 'ana');
await p.fill('input[name=contrasena]', 'demo1234');
await p.click('button[type=submit]');
await p.waitForURL((u) => !u.pathname.startsWith('/entrar'));

// (a) Facturar la orden 8
const lista = await (await p.request.get(`${BASE}/api/ordenes`)).json();
const orden = (Array.isArray(lista) ? lista : lista.ordenes).find((o) => o.numero === 8);
await p.goto(`${BASE}/ordenes/${orden.id}`);
const seccion = p.locator('section', { has: p.getByRole('heading', { name: 'Factura', exact: true }) });
await seccion.getByRole('button', { name: 'Facturar' }).click();
await p.waitForTimeout(600);
const dialogo = dialogos.at(-1) ?? '';
const pregunta = await seccion.getByText('¿Emitir la factura?').count();
const si = await seccion.getByRole('button', { name: 'Sí, facturar' }).count();
const volver = await seccion.getByRole('button', { name: 'Volver' }).count();
await p.screenshot({ path: `${OUT}/${FASE}-007-a1-confirmacion.png`, fullPage: true });
let sigue = false;
let facturada = '';
let ancho = 0;
if (volver) {
  await seccion.getByRole('button', { name: 'Volver' }).click();
  await p.waitForTimeout(400);
  sigue = (await seccion.getByRole('button', { name: 'Facturar' }).count()) > 0;
  await p.setViewportSize({ width: 390, height: 844 });
  await seccion.getByRole('button', { name: 'Facturar' }).click();
  await seccion.getByRole('button', { name: 'Sí, facturar' }).waitFor();
  ancho = await p.evaluate(() => document.documentElement.scrollWidth);
  await p.screenshot({ path: `${OUT}/${FASE}-007-a2-movil.png`, fullPage: true });
  await seccion.getByRole('button', { name: 'Sí, facturar' }).click();
  await seccion.getByText(/Factura nº \d+/).waitFor({ timeout: 5000 }).catch(() => {});
  facturada = ((await seccion.getByText(/Factura nº \d+/).textContent().catch(() => '')) || '').replace(/\s+/g, ' ').trim();
  await p.screenshot({ path: `${OUT}/${FASE}-007-a3-facturada.png`, fullPage: true });
  await p.setViewportSize({ width: 1280, height: 900 });
}
anota('(a) Facturar la orden 8', 'pregunta en la página con «Sí, facturar» y «Volver», sin diálogo del navegador; Volver no factura; Sí, facturar emite la factura',
  `diálogo del navegador=«${dialogo}»; pregunta=${pregunta}; Sí, facturar=${si}; Volver=${volver}; tras Volver sigue=${sigue}; resultado=«${facturada}»`,
  dialogo === '' && pregunta > 0 && si > 0 && volver > 0 && sigue && /Factura nº \d+/.test(facturada));
anota('(a) Confirmación a 390 px', 'cabe sin desplazamiento horizontal', `ancho del documento=${ancho}px`, ancho > 0 && ancho <= 390);

// (b) historial sin matrícula
await p.goto(`${BASE}/historial`);
const fh = p.locator('form');
const nativo = await fh.evaluate((f) =>
  f.noValidate ? '' : [...f.elements].map((e) => e.validationMessage).filter(Boolean).join(' | '));
await fh.getByRole('button', { name: 'Buscar' }).click();
const textoB = await p.getByRole('alert').textContent({ timeout: 1500 }).catch(() => '');
await p.screenshot({ path: `${OUT}/${FASE}-007-b-vacia.png`, fullPage: true });
anota('(b) Historial sin matrícula', '«Escribe la matrícula», sin mensaje del navegador',
  `aplicación=«${textoB}»; navegador=«${nativo}»`, textoB.includes('Escribe la matrícula') && nativo === '');

// (c) historial de 9999 lll
await fh.getByLabel('Matrícula').fill('9999 lll');
await fh.getByRole('button', { name: 'Buscar' }).click();
await p.getByText('9999LLL').first().waitFor({ timeout: 5000 }).catch(() => {});
const reparaciones = await p.getByText(/Orden \d+ · Factura nº \d+/).count();
await p.screenshot({ path: `${OUT}/${FASE}-007-c-historial.png`, fullPage: true });
anota('(c) Historial de «9999 lll»', 'aparece la reparación facturada', `reparaciones=${reparaciones}`, reparaciones > 0);

fs.writeFileSync(`${OUT}/resultados-007-${FASE}.json`, JSON.stringify(res, null, 2));
await nav.close();
const fallos = res.filter((r) => !r.ok).length;
console.log(`${res.length - fallos} correctas y ${fallos} fallos`);
