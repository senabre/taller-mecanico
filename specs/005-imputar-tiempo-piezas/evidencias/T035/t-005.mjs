// spec-005, analyze-1 C1 y U1: la sección «Imputaciones» al terminar la reparación.
//  (a) U1: orden en reparación sin imputaciones → se ve el motivo «Primero hay que imputar el trabajo
//      realizado.» (FR-008, US4-2, quickstart §5.1).
//  (b) C1: «Terminar reparación» pide confirmación en la página (sin diálogo del navegador); «Volver» no
//      termina y «Sí, terminar» deja la orden «Terminada».
//  (c) La confirmación a 390 px, sin desplazamiento horizontal.
// Preparación por la API (como luis): la orden demo 4 (en diagnóstico, sin presupuesto) recibe un
// presupuesto de 1 h, se emite y se acepta desde su enlace → «en reparación» sin imputaciones.
// Uso (desde una carpeta con playwright 1.48 instalado): node t-005.mjs <carpeta-salida> <antes|despues>
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
await p.fill('input[name=login]', 'luis');
await p.fill('input[name=contrasena]', 'demo1234');
await p.click('button[type=submit]');
await p.waitForURL((u) => !u.pathname.startsWith('/entrar'));

// Preparación por la API
const api = async (metodo, ruta, cuerpo) => {
  const r = await p.request.fetch(`${BASE}/api${ruta}`, { method: metodo, data: cuerpo });
  if (!r.ok()) throw new Error(`${metodo} ${ruta} → ${r.status()} ${await r.text()}`);
  return r.json();
};
const lista = await api('GET', '/ordenes');
const orden = (Array.isArray(lista) ? lista : lista.ordenes).find((o) => o.numero === 4);
const pres = await api('POST', `/ordenes/${orden.id}/presupuestos`);
await api('POST', `/presupuestos/${pres.id}/lineas`, { tipo: 'mano_obra', concepto: 'Cambio de sensor', cantidad: '1' });
const emitido = await api('POST', `/presupuestos/${pres.id}/emitir`);
const token = new URL(emitido.enlace).pathname.split('/').pop();
await api('POST', `/publico/presupuestos/${token}/respuesta`, { decision: 'aceptar' });

await p.goto(`${BASE}/ordenes/${orden.id}`);
const seccion = p.locator('section', { has: p.getByRole('heading', { name: 'Imputaciones' }) });
await seccion.waitFor();

// (a) sin imputaciones
const motivo = await seccion.getByText('Primero hay que imputar el trabajo realizado').count();
const botonA = await seccion.getByRole('button', { name: 'Terminar reparación' }).count();
await p.screenshot({ path: `${OUT}/${FASE}-005-a-sin-imputaciones.png`, fullPage: true });
anota('(a) En reparación sin imputaciones', 'el motivo «Primero hay que imputar el trabajo realizado.» y ningún botón «Terminar reparación»',
  `motivo=${motivo}; botón=${botonA}`, motivo > 0 && botonA === 0);

// Una imputación desde el formulario
const f = seccion.locator('form');
await f.getByLabel('Concepto').fill('Cambio de sensor');
await f.getByLabel('Horas').fill('1');
await f.getByRole('button', { name: 'Imputar' }).click();
await seccion.getByRole('cell', { name: 'Cambio de sensor' }).waitFor();

// (b) terminar con confirmación en la página
const antes = dialogos.length;
await seccion.getByRole('button', { name: 'Terminar reparación' }).click();
await p.waitForTimeout(600);
const dialogo = dialogos.length > antes ? dialogos.at(-1) : '';
const pregunta = await seccion.getByText('¿Marcar la reparación como terminada?').count();
const si = await seccion.getByRole('button', { name: 'Sí, terminar' }).count();
const volver = await seccion.getByRole('button', { name: 'Volver' }).count();
await p.screenshot({ path: `${OUT}/${FASE}-005-b1-confirmacion.png`, fullPage: true });
let sigue = false;
let terminada = false;
let ancho = 0;
if (volver) {
  await seccion.getByRole('button', { name: 'Volver' }).click();
  await p.waitForTimeout(400);
  sigue = (await seccion.getByRole('button', { name: 'Terminar reparación' }).count()) > 0;
  // (c) la confirmación a 390 px
  await p.setViewportSize({ width: 390, height: 844 });
  await seccion.getByRole('button', { name: 'Terminar reparación' }).click();
  await seccion.getByRole('button', { name: 'Sí, terminar' }).waitFor();
  ancho = await p.evaluate(() => document.documentElement.scrollWidth);
  await p.screenshot({ path: `${OUT}/${FASE}-005-c-movil.png`, fullPage: true });
  await seccion.getByRole('button', { name: 'Sí, terminar' }).click();
  await p.waitForTimeout(1200);
  const ficha = await (await p.request.get(`${BASE}/api/ordenes/${orden.id}`)).json();
  terminada = ficha.estado === 'terminada';
  await p.screenshot({ path: `${OUT}/${FASE}-005-b2-terminada.png`, fullPage: true });
}
anota('(b) Terminar reparación', 'pregunta en la página con «Sí, terminar» y «Volver», sin diálogo del navegador; Volver no termina; Sí, terminar termina',
  `diálogo del navegador=«${dialogo}»; pregunta=${pregunta}; Sí, terminar=${si}; Volver=${volver}; tras Volver sigue=${sigue}; terminada=${terminada}`,
  dialogo === '' && pregunta > 0 && si > 0 && volver > 0 && sigue && terminada);
anota('(c) Confirmación a 390 px', 'cabe sin desplazamiento horizontal', `ancho del documento=${ancho}px`, ancho > 0 && ancho <= 390);

fs.writeFileSync(`${OUT}/resultados-005-${FASE}.json`, JSON.stringify(res, null, 2));
await nav.close();
const fallos = res.filter((r) => !r.ok).length;
console.log(`${res.length - fallos} correctas y ${fallos} fallos`);
