// C1 de analyze-1 de spec-003 (principio II): lo que ve el mecánico al preparar y emitir un presupuesto
// lo pone la aplicación, en español, y no el navegador.
//  (a) 2,5 unidades de pieza → el mensaje del servidor «Las unidades de pieza deben ser un número entero».
//  (b) Mano de obra sin concepto → «Escribe el concepto de la mano de obra».
//  (c) «Emitir presupuesto» pide confirmación dentro de la página (sin diálogo del navegador); «Volver»
//      no emite y «Sí, emitir» emite.
// Uso (desde una carpeta con playwright 1.48 instalado): node t-c1-003.mjs <carpeta-salida> <antes|despues>
// Datos: base aparte recién migrada y con backend/seed/demo.sql de hoy; orden demo 4 (luis, en diagnóstico,
// sin presupuesto).
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

const lista = await (await p.request.get(`${BASE}/api/ordenes`)).json();
const orden = (Array.isArray(lista) ? lista : lista.ordenes).find((o) => o.numero === 4);
await p.goto(`${BASE}/ordenes/${orden.id}`);
await p.getByRole('button', { name: 'Nuevo presupuesto' }).click();
await p.getByRole('heading', { name: 'Añadir línea' }).waitFor();

const formulario = p.locator('form', { has: p.getByRole('heading', { name: 'Añadir línea' }) });
const alerta = () => formulario.getByRole('alert').textContent({ timeout: 1500 }).catch(() => '');
// Mensaje que el navegador pone por su cuenta al enviar (validación nativa), si lo hay. Con `novalidate`
// el navegador no valida ni enseña nada, y el formulario llega a la aplicación.
const mensajeNavegador = () =>
  formulario.evaluate((f) =>
    f.noValidate ? '' : [...f.elements].map((e) => e.validationMessage).filter(Boolean).join(' | '));

// (a) 2,5 unidades de PF-123
await formulario.locator('select').selectOption('pieza');
await formulario.getByPlaceholder('Busca por referencia o descripción').fill('PF-123');
await formulario.getByRole('button', { name: /PF-123/ }).click();
// Un campo `type="number"` no admite la coma al escribir desde Playwright: ahí se escribe «2.5».
const campoUnidades = formulario.getByLabel('Unidades');
const esNumero = (await campoUnidades.getAttribute('type')) === 'number';
await campoUnidades.fill(esNumero ? '2.5' : '2,5');
const nativoA = await mensajeNavegador();
await formulario.getByRole('button', { name: 'Añadir línea' }).click();
const textoA = await alerta();
const lineasA = await p.locator('table tbody tr').count();
anota('(a) 2,5 unidades de PF-123', '«Las unidades de pieza deben ser un número entero», sin mensaje del navegador',
  `aplicación=«${textoA}»; navegador=«${nativoA}»; líneas=${lineasA}`,
  textoA.includes('Las unidades de pieza deben ser un número entero') && nativoA === '');
await p.screenshot({ path: `${OUT}/${FASE}-a-unidades.png`, fullPage: true });

// (b) mano de obra sin concepto
await formulario.locator('select').selectOption('mano_obra');
await formulario.getByLabel('Horas').fill('1');
const nativoB = await mensajeNavegador();
await formulario.getByRole('button', { name: 'Añadir línea' }).click();
const textoB = await alerta();
anota('(b) Mano de obra sin concepto', '«Escribe el concepto de la mano de obra», sin mensaje del navegador',
  `aplicación=«${textoB}»; navegador=«${nativoB}»`,
  textoB.includes('Escribe el concepto de la mano de obra') && nativoB === '');
await p.screenshot({ path: `${OUT}/${FASE}-b-concepto.png`, fullPage: true });

// Una línea válida para poder emitir
await formulario.getByLabel('Concepto').fill('Cambio de pastillas');
await formulario.getByLabel('Horas').fill('1,5');
await formulario.getByRole('button', { name: 'Añadir línea' }).click();
await p.getByRole('cell', { name: 'Cambio de pastillas' }).waitFor();

// (c) Emitir: confirmación dentro de la página
const antes = dialogos.length;
await p.getByRole('button', { name: 'Emitir presupuesto' }).click();
await p.waitForTimeout(600);
const dialogoNavegador = dialogos.length > antes ? dialogos.at(-1) : '';
const pregunta = await p.getByText('Una vez emitido no se podrá modificar').count();
const botonSi = await p.getByRole('button', { name: 'Sí, emitir' }).count();
const botonCancelar = await p.getByRole('button', { name: 'Volver' }).count();
await p.screenshot({ path: `${OUT}/${FASE}-c1-confirmacion.png`, fullPage: true });
let sigueBorrador = false;
let emitido = false;
if (botonCancelar) {
  await p.getByRole('button', { name: 'Volver' }).click();
  await p.waitForTimeout(400);
  sigueBorrador = (await p.getByText('Inicial · Borrador').count()) > 0 &&
    (await p.getByRole('button', { name: 'Emitir presupuesto' }).count()) > 0;
  await p.getByRole('button', { name: 'Emitir presupuesto' }).click();
  await p.getByRole('button', { name: 'Sí, emitir' }).click();
  await p.getByText('Válido hasta').waitFor({ timeout: 5000 }).catch(() => {});
  emitido = (await p.getByText('Válido hasta').count()) > 0;
  await p.screenshot({ path: `${OUT}/${FASE}-c2-emitido.png`, fullPage: true });
}
anota('(c) Emitir presupuesto', 'pregunta en la página con «Sí, emitir» y «Volver», sin diálogo del navegador; Volver no emite; Sí, emitir emite',
  `diálogo del navegador=«${dialogoNavegador}»; pregunta=${pregunta}; Sí, emitir=${botonSi}; Volver=${botonCancelar}; tras Volver sigue en borrador=${sigueBorrador}; emitido=${emitido}`,
  dialogoNavegador === '' && pregunta > 0 && botonSi > 0 && botonCancelar > 0 && sigueBorrador && emitido);

// La confirmación a 390 px: se repite en la orden demo 1 tras guardar su diagnóstico (otra orden de luis).
if (FASE === 'despues') {
  await p.setViewportSize({ width: 390, height: 844 });
  const o1 = (Array.isArray(lista) ? lista : lista.ordenes).find((o) => o.numero === 1);
  await p.goto(`${BASE}/ordenes/${o1.id}`);
  await p.locator('textarea').fill('Ruido en la suspensión delantera');
  await p.getByRole('button', { name: 'Guardar diagnóstico' }).click();
  await p.getByRole('button', { name: 'Nuevo presupuesto' }).click();
  const f = p.locator('form', { has: p.getByRole('heading', { name: 'Añadir línea' }) });
  await f.getByLabel('Concepto').fill('Revisión de la suspensión');
  await f.getByLabel('Horas').fill('1');
  await f.getByRole('button', { name: 'Añadir línea' }).click();
  await p.getByRole('cell', { name: 'Revisión de la suspensión' }).waitFor();
  await p.getByRole('button', { name: 'Emitir presupuesto' }).click();
  await p.getByRole('button', { name: 'Sí, emitir' }).waitFor();
  const ancho = await p.evaluate(() => document.documentElement.scrollWidth);
  await p.screenshot({ path: `${OUT}/${FASE}-movil-confirmacion.png`, fullPage: true });
  anota('(c) Confirmación a 390 px', 'cabe sin desplazamiento horizontal', `ancho del documento=${ancho}px`, ancho <= 390);
}

fs.writeFileSync(`${OUT}/resultados-${FASE}.json`, JSON.stringify(res, null, 2));
await nav.close();
const fallos = res.filter((r) => !r.ok).length;
console.log(`${res.length - fallos} correctas y ${fallos} fallos`);
