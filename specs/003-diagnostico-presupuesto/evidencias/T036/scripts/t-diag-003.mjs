// T038 y T039 de spec-003 (converge-1): la sección «Diagnóstico» de la ficha.
//  (a) T038: guardar el diagnóstico vacío → el mensaje de la aplicación «Escribe el diagnóstico», sin
//      mensaje del navegador (orden demo 1, recibida, como luis).
//  (b) T039: con un presupuesto ya emitido, el diagnóstico se ve sin formulario y con el motivo (orden
//      demo 5, presupuesto emitido, como paco, que puede editar todas).
//  (c) Sin presupuesto emitido sigue pudiéndose cambiar (orden demo 4, como luis).
// Uso (desde una carpeta con playwright 1.48 instalado): node t-diag-003.mjs <carpeta-salida> <antes|despues>
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
const MOTIVO = 'Ya se ha emitido un presupuesto para esta orden; el diagnóstico no se puede cambiar';

const nav = await chromium.launch();
async function sesion(usuario, ancho = 1280) {
  const c = await nav.newContext({ viewport: { width: ancho, height: 900 }, locale: 'es-ES', timezoneId: 'Europe/Madrid' });
  const p = await c.newPage();
  p.on('dialog', (d) => d.dismiss());
  await p.goto(`${BASE}/entrar`);
  await p.fill('input[name=login]', usuario);
  await p.fill('input[name=contrasena]', 'demo1234');
  await p.click('button[type=submit]');
  await p.waitForURL((u) => !u.pathname.startsWith('/entrar'));
  return p;
}
async function abrirOrden(p, numero) {
  const lista = await (await p.request.get(`${BASE}/api/ordenes`)).json();
  const o = (Array.isArray(lista) ? lista : lista.ordenes).find((x) => x.numero === numero);
  await p.goto(`${BASE}/ordenes/${o.id}`);
  await p.getByRole('heading', { name: 'Diagnóstico' }).waitFor();
  return o.id;
}
const seccion = (p) => p.locator('section', { has: p.getByRole('heading', { name: 'Diagnóstico' }) });

// (a) diagnóstico vacío
const luis = await sesion('luis');
await abrirOrden(luis, 1);
const sa = seccion(luis);
await sa.locator('textarea').fill('');
const nativo = await sa.locator('form').evaluate((f) =>
  f.noValidate ? '' : [...f.elements].map((e) => e.validationMessage).filter(Boolean).join(' | '));
await sa.getByRole('button', { name: 'Guardar diagnóstico' }).click();
const textoA = await sa.getByRole('alert').textContent({ timeout: 1500 }).catch(() => '');
await luis.screenshot({ path: `${OUT}/${FASE}-diag-a-vacio.png`, fullPage: true });
anota('(a) Diagnóstico vacío', '«Escribe el diagnóstico», sin mensaje del navegador',
  `aplicación=«${textoA}»; navegador=«${nativo}»`, textoA.includes('Escribe el diagnóstico') && nativo === '');

// (b) con presupuesto emitido
const paco = await sesion('paco');
const id5 = await abrirOrden(paco, 5);
const sb = seccion(paco);
const cuadro = await sb.locator('textarea').count();
const boton = await sb.getByRole('button', { name: 'Guardar diagnóstico' }).count();
const motivo = await sb.getByText(MOTIVO).count();
const guardado = (await (await paco.request.get(`${BASE}/api/ordenes/${id5}`)).json()).diagnostico;
const visible = await sb.getByText(guardado, { exact: true }).count();
await paco.screenshot({ path: `${OUT}/${FASE}-diag-b-emitido.png`, fullPage: true });
anota('(b) Con un presupuesto emitido', 'el diagnóstico guardado, el motivo y ni cuadro ni «Guardar diagnóstico»',
  `cuadro=${cuadro}; botón=${boton}; motivo=${motivo}; diagnóstico visible=${visible}`,
  cuadro === 0 && boton === 0 && motivo > 0 && visible > 0);

// (c) sin presupuesto emitido sigue editable
await abrirOrden(luis, 4);
const sc = seccion(luis);
await sc.locator('textarea').fill('Sensor de oxígeno y sonda lambda defectuosos');
await sc.getByRole('button', { name: 'Guardar diagnóstico' }).click();
await luis.waitForTimeout(800);
const errorC = await sc.getByRole('alert').count();
const valor = await sc.locator('textarea').inputValue().catch(() => '');
anota('(c) Sin presupuesto emitido', 'se guarda el cambio, sin error',
  `error=${errorC}; cuadro=«${valor}»`, errorC === 0 && valor === 'Sensor de oxígeno y sonda lambda defectuosos');

// (b) a 390 px
if (FASE === 'despues') {
  const movil = await sesion('paco', 390);
  await abrirOrden(movil, 5);
  const ancho = await movil.evaluate(() => document.documentElement.scrollWidth);
  await movil.screenshot({ path: `${OUT}/${FASE}-diag-b-movil.png`, fullPage: true });
  anota('(b) A 390 px', 'cabe sin desplazamiento horizontal', `ancho del documento=${ancho}px`, ancho <= 390);
}

fs.writeFileSync(`${OUT}/resultados-diag-${FASE}.json`, JSON.stringify(res, null, 2));
await nav.close();
const fallos = res.filter((r) => !r.ok).length;
console.log(`${res.length - fallos} correctas y ${fallos} fallos`);
