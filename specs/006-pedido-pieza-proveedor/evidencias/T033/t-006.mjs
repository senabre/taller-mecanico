// spec-006, analyze-1 C1 (principio II): los mensajes de /almacen y /pedidos los pone la aplicación,
// en español, y no el navegador. Como pepe (almacén).
//  /almacen: (a1) referencia vacía; (a2) stock -1; (a3) alta de FA-9 sin stock → stock 0.
//  /pedidos: (b1) cantidad 0; (b2) sin nº de orden; (b3) pedido de FA-9 para la orden 3 → registrado.
//  (c) /pedidos a 390 px sin desplazamiento horizontal.
// Uso (desde una carpeta con playwright 1.48 instalado): node t-006.mjs <carpeta-salida> <antes|despues>
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
p.on('dialog', (d) => d.dismiss());
await p.goto(`${BASE}/entrar`);
await p.fill('input[name=login]', 'pepe');
await p.fill('input[name=contrasena]', 'demo1234');
await p.click('button[type=submit]');
await p.waitForURL((u) => !u.pathname.startsWith('/entrar'));

const nativo = (f) => f.evaluate((form) =>
  form.noValidate ? '' : [...form.elements].map((e) => e.validationMessage).filter(Boolean).join(' | '));
const alerta = (f) => f.getByRole('alert').textContent({ timeout: 1500 }).catch(() => '');
async function prueba(f, boton, paso, esperado, captura) {
  const n = await nativo(f);
  await f.getByRole('button', { name: boton }).click();
  const a = await alerta(f);
  await p.screenshot({ path: `${OUT}/${FASE}-006-${captura}.png`, fullPage: true });
  anota(paso, `«${esperado}», sin mensaje del navegador`, `aplicación=«${a}»; navegador=«${n}»`, a.includes(esperado) && n === '');
}

// /almacen
await p.goto(`${BASE}/almacen`);
const fa = p.locator('form');
await fa.getByLabel('Descripción').fill('Filtro aceite');
await fa.getByLabel('Precio de venta (€)').fill('12,00');
await prueba(fa, 'Dar de alta', '(a1) Referencia vacía', 'Indica la referencia de la pieza', 'a1-referencia');
await fa.getByLabel('Referencia').fill('FA-9');
await fa.getByLabel('Stock inicial (opcional)').fill('-1');
await prueba(fa, 'Dar de alta', '(a2) Stock -1', 'El stock debe ser un número entero de 0 o más', 'a2-stock');
await fa.getByLabel('Stock inicial (opcional)').fill('');
await fa.getByRole('button', { name: 'Dar de alta' }).click();
await p.getByRole('cell', { name: 'FA-9', exact: true }).waitFor({ timeout: 5000 }).catch(() => {});
const fila = await p.locator('tr', { has: p.getByRole('cell', { name: 'FA-9', exact: true }) }).textContent().catch(() => '');
anota('(a3) Alta de FA-9 sin stock', 'aparece con stock 0', `fila=«${(fila || '').replace(/\s+/g, ' ').trim()}»`, /FA-9.*0$/.test((fila || '').replace(/\s+/g, ' ').trim()));

// /pedidos
await p.goto(`${BASE}/pedidos`);
const fp = p.locator('form');
await fp.locator('select').selectOption({ label: 'Recambios Sur' });
await fp.getByPlaceholder('Busca por referencia o descripción').fill('FA-9');
await fp.getByRole('button', { name: /FA-9/ }).click();
await fp.getByLabel('Cantidad').fill('0');
await fp.getByLabel('Nº de orden').fill('3');
await prueba(fp, 'Registrar pedido', '(b1) Cantidad 0', 'La cantidad debe ser un número entero mayor que 0', 'b1-cantidad');
await fp.getByLabel('Cantidad').fill('1');
await fp.getByLabel('Nº de orden').fill('');
await prueba(fp, 'Registrar pedido', '(b2) Sin nº de orden', 'Indica el número de la orden', 'b2-orden');
await fp.getByLabel('Nº de orden').fill('3');
await fp.getByRole('button', { name: 'Registrar pedido' }).click();
const ok = await fp.getByRole('status').textContent({ timeout: 3000 }).catch(() => '');
await p.screenshot({ path: `${OUT}/${FASE}-006-b3-registrado.png`, fullPage: true });
anota('(b3) Pedido de FA-9 para la orden 3', '«Pedido registrado: 1 × FA-9 para la orden 3»', `aviso=«${ok}»`, ok.includes('Pedido registrado: 1 × FA-9 para la orden 3'));

// (c) 390 px
await p.setViewportSize({ width: 390, height: 844 });
await p.goto(`${BASE}/pedidos`);
await p.getByRole('heading', { name: 'Nuevo pedido' }).waitFor();
const ancho = await p.evaluate(() => document.documentElement.scrollWidth);
await p.screenshot({ path: `${OUT}/${FASE}-006-c-movil.png`, fullPage: true });
anota('(c) /pedidos a 390 px', 'sin desplazamiento horizontal', `ancho del documento=${ancho}px`, ancho <= 390);

fs.writeFileSync(`${OUT}/resultados-006-${FASE}.json`, JSON.stringify(res, null, 2));
await nav.close();
const fallos = res.filter((r) => !r.ok).length;
console.log(`${res.length - fallos} correctas y ${fallos} fallos`);
