// Recorrido de T032 (quickstart común §1 + pasos a–e) y parte de T034, con capturas.
import { chromium } from 'playwright';
import fs from 'node:fs';

const BASE = 'http://localhost:3000';
const OUT = process.argv[2];
const MODO = process.argv[3] || 'normal'; // 'normal' | 'cupo0'
fs.mkdirSync(OUT, { recursive: true });
const res = [];
let n = 0;
const navegador = await chromium.launch();

async function captura(page, nombre) {
  n += 1;
  const f = `${String(n).padStart(2, '0')}-${nombre}.png`;
  await page.screenshot({ path: `${OUT}/${f}`, fullPage: true });
  return f;
}
function anota(paso, esperado, observado, ok, img) {
  res.push({ paso, esperado, observado, ok, img });
  console.log(`${ok ? 'OK  ' : 'FALLA'} ${paso}: ${observado}`);
}
const fechaEs = (d) => d.split('-').reverse().join('/');

async function nuevaPagina(viewport = { width: 1280, height: 900 }) {
  const ctx = await navegador.newContext({ viewport, locale: 'es-ES', timezoneId: 'Europe/Madrid' });
  return ctx.newPage();
}
async function abrirCita(page) {
  await page.goto(`${BASE}/cita`);
  await page.locator('select[name=fecha]').or(page.getByText('No quedan fechas libres')).first().waitFor();
}
async function escribirMatricula(page, m) {
  await page.fill('input[name=matricula]', m);
  await page.locator('input[name=matricula]').blur();
  await page.waitForTimeout(400);
}
async function rellenarNuevo(page, d) {
  for (const [k, v] of Object.entries(d)) await page.fill(`input[name=${k}]`, v);
}
async function opcionesDia(page) {
  return page.$$eval('select[name=fecha] option', (os) => os.map((o) => o.value).filter(Boolean));
}
async function confirmar(page) {
  await page.click('button[type=submit]');
  await page.waitForTimeout(600);
}
const texto = (page) => page.locator('main').innerText();
const botonActivo = (page) => page.locator('button[type=submit]').isEnabled();

async function entrar(page, login) {
  await page.goto(`${BASE}/entrar`);
  await page.fill('input[name=login]', login);
  await page.fill('input[name=contrasena]', 'demo1234');
  await page.click('button[type=submit]');
  await page.waitForTimeout(800);
}
async function verDia(page, dia) {
  await page.fill('input[name=dia]', dia);
  await page.waitForTimeout(600);
}
async function filas(page) {
  return page.$$eval('table tbody tr', (trs) => trs.map((tr) => [...tr.cells].map((c) => c.innerText.trim())));
}

if (MODO === 'cupo0') {
  // (c) No quedan fechas libres, con TALLER_CUPO_DIARIO=0
  const p = await nuevaPagina();
  let visto = false;
  await p.route('**/api/publico/dias-disponibles', async (r) => {
    await new Promise((s) => setTimeout(s, 1500));
    await r.continue();
  });
  await p.goto(`${BASE}/cita`);
  await p.waitForTimeout(500);
  visto = (await texto(p)).includes('No quedan fechas libres');
  const imgCarga = await captura(p, 'c-cargando-dias');
  await p.waitForSelector('text=No quedan fechas libres');
  const t = await texto(p);
  const hay = t.includes('No quedan fechas libres en los próximos 30 días. Llama al taller para pedir cita.');
  const sel = await p.locator('select[name=fecha]').count();
  const img = await captura(p, 'c-no-quedan-fechas');
  anota('(c) Sin fechas libres (cupo 0)', 'Mensaje exacto, sin selector, Confirmar desactivado y sin parpadeo al cargar',
    `mensaje=${hay}, selector=${sel}, confirmar activo=${await botonActivo(p)}, mensaje durante la carga=${visto}`,
    hay && sel === 0 && !(await botonActivo(p)) && !visto, `${imgCarga}, ${img}`);
  fs.writeFileSync(`${OUT}/resultados.json`, JSON.stringify(res, null, 2));
  await navegador.close();
  process.exit(0);
}

const p = await nuevaPagina();
await abrirCita(p);
const dias = await opcionesDia(p);
const [D1, D2, D3, D4, D5, D6] = dias; // primeros días laborables disponibles
console.log('Días:', dias.slice(0, 6).join(' '));

// Paso 1
for (const m of ['12#4', 'A', 'ABCDE123456']) {
  await escribirMatricula(p, m);
  const t = await texto(p);
  const img = await captura(p, `p1-matricula-${m.replace('#', '')}`);
  anota(`§1.1 Matrícula «${m}»`, '«La matrícula no es válida» y no deja confirmar',
    `error=${t.includes('La matrícula no es válida')}, confirmar activo=${await botonActivo(p)}`,
    t.includes('La matrícula no es válida') && !(await botonActivo(p)), img);
}
await escribirMatricula(p, 'AB');
{
  const t = await texto(p); const img = await captura(p, 'p1-matricula-AB');
  const campos = await p.locator('input[name=marca]').count();
  anota('§1.1 Matrícula «AB»', 'Se acepta y pide los datos del vehículo nuevo', `sin error=${!t.includes('no es válida')}, pide datos=${campos === 1}`,
    !t.includes('no es válida') && campos === 1, img);
}

// Paso 2
await escribirMatricula(p, '1234-bcd');
{
  const c = await Promise.all(['marca', 'modelo', 'nombre', 'telefono', 'email'].map((k) => p.locator(`input[name=${k}]`).count()));
  const img = await captura(p, 'p2-vehiculo-nuevo');
  anota('§1.2 «1234-bcd» (US2-1)', 'Pide marca, modelo, nombre, teléfono y correo', `campos=${c.join(',')}`, c.every((x) => x === 1), img);
}

// Paso 3
await rellenarNuevo(p, { marca: 'Fiat', modelo: 'Tipo', nombre: 'Ana Ruiz', telefono: '12345', email: 'ana@correo.es' });
await p.selectOption('select[name=fecha]', D1);
await p.fill('textarea[name=motivo]', 'Ruido al frenar');
await confirmar(p);
{
  const t = await texto(p); const img = await captura(p, 'p3-telefono-no-valido');
  anota('§1.3 Teléfono «12345» (US2-3)', '«El teléfono no es válido» y no se confirma', `error=${t.includes('El teléfono no es válido')}`,
    t.includes('El teléfono no es válido') && !t.includes('Cita confirmada'), img);
}
await rellenarNuevo(p, { telefono: '612-345-678', email: 'ana@correo' });
await confirmar(p);
{
  const t = await texto(p); const img = await captura(p, 'p3-correo-no-valido');
  anota('§1.3 Correo «ana@correo» (US2-3)', '«El correo no es válido» y no se confirma', `error=${t.includes('El correo no es válido')}`,
    t.includes('El correo no es válido') && !t.includes('Cita confirmada'), img);
}
await rellenarNuevo(p, { email: 'ana@correo.es' });
await confirmar(p);
{
  const t = await texto(p); const img = await captura(p, 'p3-cita-confirmada');
  const ok = t.includes('Cita confirmada') && t.includes(fechaEs(D1)) && t.includes('Ruido al frenar');
  anota('§1.3 Alta y confirmación (US2-2, US1-2)', `Confirmación con ${fechaEs(D1)} y el motivo`, ok ? 'confirmada con fecha dd/mm/aaaa y motivo' : t.slice(0, 120), ok, img);
}

// Paso 4
await abrirCita(p);
await escribirMatricula(p, '1234BCD');
{
  const t = await texto(p); const img = await captura(p, 'p4-cita-pendiente');
  const ok = t.includes('Fiat Tipo') && t.includes('Este vehículo ya tiene una cita pendiente.') && (await p.getByRole('alert').filter({ hasText: 'cita pendiente' }).innerText()) === 'Este vehículo ya tiene una cita pendiente.' && !t.includes('Ana Ruiz') && !(await botonActivo(p));
  anota('§1.4 «1234BCD» con cita pendiente (US1-1, US1-4)', 'Marca y modelo, aviso sin fecha, sin datos del cliente, no deja confirmar',
    `marca/modelo=${t.includes('Fiat Tipo')}, aviso=${t.includes('Este vehículo ya tiene una cita pendiente.')}, confirmar activo=${await botonActivo(p)}`, ok, img);
}

// Paso 5 (+ (a) US3-1)
await abrirCita(p);
await escribirMatricula(p, '5678CDF');
await rellenarNuevo(p, { marca: 'Renault', modelo: 'Laguna', nombre: 'Jorge Gil', telefono: '+34 612 345 678', email: 'jorge@correo.es' });
await p.selectOption('select[name=fecha]', D1);
await p.fill('textarea[name=motivo]', 'Ruedas desgastadas');
await confirmar(p);
{
  const t = await texto(p); const img = await captura(p, 'p5-telefono-internacional');
  anota('§1.5 «+34 612 345 678» (US2-4)', 'Se confirma', t.includes('Cita confirmada') ? 'confirmada' : t.slice(0, 120), t.includes('Cita confirmada'), img);
}
await abrirCita(p);
{
  const ops = await opcionesDia(p); await p.click('select[name=fecha]').catch(() => {});
  const img = await captura(p, 'p5-a-dia-lleno-no-aparece');
  const ok = !ops.includes(D1) && ops.includes(D2);
  anota('§1.5 / (a) Día lleno (US3-1, SC-003)', `${fechaEs(D1)} ya no aparece y ${fechaEs(D2)} sí`, `primer día ofrecido=${fechaEs(ops[0])}`, ok, img);
}

// Paso 6
await escribirMatricula(p, '1111BBB');
{
  const t = await texto(p);
  const contacto = await p.locator('input[name=nombre]').count();
  const img = await captura(p, 'p6-vehiculo-conocido');
  anota('§1.6 «1111BBB» (US1-1, US1-7)', '«Seat Ibiza», solo día y motivo, sin avisos',
    `Seat Ibiza=${t.includes('Seat Ibiza')}, campos de contacto=${contacto}, aviso=${t.includes('cita pendiente')}`,
    t.includes('Seat Ibiza') && contacto === 0 && !t.includes('cita pendiente'), img);
}
await p.selectOption('select[name=fecha]', D2);
await p.fill('textarea[name=motivo]', 'Cambio de aceite');
await confirmar(p);
{
  const t = await texto(p); const img = await captura(p, 'p6-confirmada');
  anota('§1.6 Cita de 1111BBB (US1-2, US1-6)', 'Se confirma', t.includes('Cita confirmada') ? `confirmada para ${fechaEs(D2)}` : t.slice(0, 120), t.includes('Cita confirmada') && t.includes('Cambio de aceite'), img);
}

// Vista móvil de /cita
{
  const m = await nuevaPagina({ width: 390, height: 844 });
  await abrirCita(m);
  await escribirMatricula(m, '1234-bcd');
  const ancho = await m.evaluate(() => document.documentElement.scrollWidth);
  const img = await captura(m, 'movil-cita');
  anota('/cita en móvil (390 px)', 'Usable, sin desplazamiento horizontal', `ancho del documento=${ancho}px`, ancho <= 390, img);
  await m.context().close();
}

// Paso 7, 8 (ana)
const r = await nuevaPagina();
await entrar(r, 'ana');
await r.goto(`${BASE}/citas`); await r.waitForTimeout(800);
{
  const t = await texto(r); const v = await r.inputValue('input[name=dia]');
  const img = await captura(r, 'p7-ana-hoy');
  anota('§1.7 «Citas» abre con hoy (US1-8)', 'Título «Citas de hoy» y selector con la fecha de hoy', `título=${t.split('\n')[0]}, selector=${v}`, t.startsWith('Citas de hoy') && v !== '', img);
}
await verDia(r, D1);
{
  const f = await filas(r); const cab = await r.$$eval('table thead th', (ths) => ths.map((t) => t.innerText.trim()));
  const img = await captura(r, 'p7-ana-dia-1');
  const ok = f.length === 2 && f[0][0] === '1234BCD' && f[1][0] === '5678CDF' && !cab.some((c) => /correo/i.test(c)) && f[1].includes('+34 612 345 678');
  anota('§1.7 Lista del día (US1-3, US1-8, FR-009)', '1234BCD y después 5678CDF, con nombre y teléfono, sin correo',
    `filas=${f.map((x) => x[0]).join(', ')}; columnas=${cab.join(' | ')}`, ok, img);
}
await verDia(r, D2);
{
  const f = await filas(r); const img = await captura(r, 'p7-ana-dia-2');
  const fila = f.find((x) => x[0] === '1111BBB');
  anota('§1.7 1111BBB con sus datos sin cambiar (US1-7, FR-003)', '«María López» y 600111222', fila ? fila.join(' · ') : 'no aparece',
    !!fila && fila.includes('María López') && fila.includes('600111222'), img);
}
await verDia(r, D5);
{
  const t = await texto(r); const img = await captura(r, 'p8-dia-sin-citas');
  anota('§1.8 Día sin citas (US1-9)', '«No hay citas para este día»', t.includes('No hay citas para este día') ? 'mensaje visible' : 'no aparece', t.includes('No hay citas para este día'), img);
}

// Paso 9
for (const login of ['luis', 'pepe']) {
  const u = await nuevaPagina();
  await entrar(u, login);
  const menu = await u.locator('header, nav').first().innerText().catch(() => '');
  await u.goto(`${BASE}/citas`); await u.waitForTimeout(800);
  const t = await texto(u); const tabla = await u.locator('table').count();
  const img = await captura(u, `p9-${login}`);
  anota(`§1.9 ${login} (US1-10)`, 'Sin «Citas» en el menú; en /citas solo «No tienes permiso para hacer esto»',
    `menú con Citas=${/\bCitas\b/.test(menu)}, mensaje=${t.includes('No tienes permiso para hacer esto')}, tabla=${tabla}`,
    !/\bCitas\b/.test(menu) && t.includes('No tienes permiso para hacer esto') && tabla === 0, img);
  await u.context().close();
}
{
  const u = await nuevaPagina();
  await u.goto(`${BASE}/citas`); await u.waitForTimeout(800);
  const img = await captura(u, 'p9-sin-sesion');
  anota('§1.9 Sin sesión (US1-10)', 'Lleva a /entrar', `url=${new URL(u.url()).pathname}`, u.url().endsWith('/entrar'), img);
  await u.context().close();
}
{
  const u = await nuevaPagina();
  await entrar(u, 'paco');
  await u.goto(`${BASE}/citas`); await u.waitForTimeout(600);
  await verDia(u, D1);
  const f = await filas(u); const img = await captura(u, 'p9-paco');
  anota('§1.9 paco (jefe)', 'Ve la misma lista que ana', `filas=${f.map((x) => x[0]).join(', ')}`, f.length === 2 && f[0][0] === '1234BCD', img);
  await u.context().close();
}

// Paso 10: recorridos de SC-001 y SC-002 (tiempo de máquina, no de persona)
{
  const t0 = Date.now();
  await abrirCita(p); await escribirMatricula(p, '2222CCC');
  await p.selectOption('select[name=fecha]', D3); await p.fill('textarea[name=motivo]', 'Revisión');
  await confirmar(p);
  const ok = (await texto(p)).includes('Cita confirmada');
  const img = await captura(p, 'p10-sc001-conocido');
  anota('§1.10 SC-001 vehículo conocido', 'Recorrido con solo matrícula, día y motivo (3 datos)', `${ok ? 'confirmada' : 'no confirmada'}; 3 datos escritos; ${((Date.now() - t0) / 1000).toFixed(1)} s de máquina`, ok, img);
}
{
  const t0 = Date.now();
  await abrirCita(p); await escribirMatricula(p, '9012DFG');
  await rellenarNuevo(p, { marca: 'Hyundai', modelo: 'i30', nombre: 'Marta Sanz', telefono: '622 111 333', email: 'marta@correo.es' });
  await p.selectOption('select[name=fecha]', D3); await p.fill('textarea[name=motivo]', 'Pitido al arrancar');
  await confirmar(p);
  const ok = (await texto(p)).includes('Cita confirmada');
  const img = await captura(p, 'p10-sc002-nuevo');
  anota('§1.10 SC-002 vehículo nuevo', 'Recorrido con 8 datos', `${ok ? 'confirmada' : 'no confirmada'}; 8 datos escritos; ${((Date.now() - t0) / 1000).toFixed(1)} s de máquina`, ok, img);
}

// (b) US3-2: dos pestañas por la última plaza de D2 (ya tiene 1111BBB)
{
  const A = await nuevaPagina(); const B = await nuevaPagina();
  for (const [pg, m] of [[A, '3333DDD'], [B, '4444FFF']]) {
    await abrirCita(pg); await escribirMatricula(pg, m);
    await pg.selectOption('select[name=fecha]', D2); await pg.fill('textarea[name=motivo]', 'Última plaza');
  }
  await confirmar(A);
  const okA = (await texto(A)).includes('Cita confirmada');
  const imgA = await captura(A, 'b-pestana-A-confirmada');
  await confirmar(B);
  const tB = await texto(B); const ops = await opcionesDia(B); const val = await B.inputValue('select[name=fecha]');
  const imgB = await captura(B, 'b-pestana-B-dia-completado');
  const okB = tB.includes('Ese día se acaba de completar; elige otro') && val === '' && !ops.includes(D2) && !(await botonActivo(B));
  anota('(b) US3-2 última plaza en dos pestañas', 'A se confirma; B ve «Ese día se acaba de completar; elige otro», selector vacío, día retirado y Confirmar desactivado',
    `A=${okA}, aviso B=${tB.includes('Ese día se acaba de completar')}, selector=${val || 'Elige un día'}, día sigue=${ops.includes(D2)}, confirmar activo=${await botonActivo(B)}`, okA && okB, `${imgA}, ${imgB}`);
  await B.selectOption('select[name=fecha]', D4);
  await confirmar(B);
  const okB2 = (await texto(B)).includes('Cita confirmada');
  const img2 = await captura(B, 'b-pestana-B-otro-dia');
  anota('(b) US3-2 B elige otro día', 'Se confirma', okB2 ? `confirmada para ${fechaEs(D4)}` : 'no confirmada', okB2, img2);
  await A.context().close(); await B.context().close();
}

// (d) T035: misma matrícula nueva en dos pestañas
{
  const A = await nuevaPagina(); const B = await nuevaPagina();
  for (const [pg, d] of [[A, { marca: 'Mazda', modelo: '3', nombre: 'Pestaña A', telefono: '611000111', email: 'a@correo.es' }],
                          [B, { marca: 'Skoda', modelo: 'Octavia', nombre: 'Pestaña B', telefono: '611000222', email: 'b@correo.es' }]]) {
    await abrirCita(pg); await escribirMatricula(pg, '7777XYZ'); await rellenarNuevo(pg, d);
    await pg.fill('textarea[name=motivo]', 'Prueba T035');
  }
  await B.selectOption('select[name=fecha]', D5); await A.selectOption('select[name=fecha]', D6);
  await confirmar(B);
  await confirmar(A); await A.waitForTimeout(600);
  const t = await texto(A);
  const campos = await A.locator('input[name=marca]').count();
  const img = await captura(A, 'd-t035-pestana-A');
  const ok = t.includes('Skoda Octavia') && t.includes('Este vehículo ya tiene una cita pendiente.') && campos === 0 && !(await botonActivo(A));
  anota('(d) T035 misma matrícula nueva en dos pestañas (FR-010)', 'En A: marca y modelo de B, aviso, campos ocultos y Confirmar desactivado',
    `Skoda Octavia=${t.includes('Skoda Octavia')}, aviso=${t.includes('cita pendiente')}, campos nuevos=${campos}, confirmar activo=${await botonActivo(A)}`, ok, img);
  await r.reload(); await r.waitForTimeout(600); await verDia(r, D5);
  const f = (await filas(r)).filter((x) => x[0] === '7777XYZ');
  const img2 = await captura(r, 'd-t035-lista');
  anota('(d) T035 en /citas', 'Una sola cita de 7777XYZ, con el nombre de B', f.map((x) => x.join(' · ')).join(' / ') || 'ninguna',
    f.length === 1 && f[0].includes('Pestaña B'), img2);
  await A.context().close(); await B.context().close();
}

// (e) T037: respuesta tardía en /citas
{
  let retrasar = true;
  await r.route('**/api/citas?fecha=' + D2, async (rt) => {
    if (retrasar) await new Promise((s) => setTimeout(s, 1500));
    await rt.continue();
  });
  await r.fill('input[name=dia]', D2); // respuesta lenta
  await r.waitForTimeout(100);
  await r.fill('input[name=dia]', D1); // respuesta rápida
  await r.waitForTimeout(2200); // llega la de D2 tarde
  const titulo = (await texto(r)).split('\n')[0]; const f = await filas(r);
  const img = await captura(r, 'e-t037-respuesta-tardia');
  const ok = titulo.includes(fechaEs(D1)) && f.length === 2 && f[0][0] === '1234BCD';
  anota('(e) T037 respuesta tardía en /citas (FR-009)', `Con la respuesta de ${fechaEs(D2)} retrasada 1,5 s, la tabla sigue siendo la de ${fechaEs(D1)}`,
    `título=${titulo}; filas=${f.map((x) => x[0]).join(', ')}`, ok, img);
  retrasar = false;
  for (const d of [D3, D2, D1, D4]) await r.fill('input[name=dia]', d);
  await r.click('text=Hoy'); await r.waitForTimeout(800);
  const t2 = await texto(r);
  const img2 = await captura(r, 'e-t037-cambios-rapidos-y-hoy');
  anota('(e) T037 cambios rápidos y «Hoy»', 'La tabla corresponde al día del título', `título=${t2.split('\n')[0]}; sin citas=${t2.includes('No hay citas para este día')}`,
    t2.startsWith('Citas de hoy') && t2.includes('No hay citas para este día'), img2);
}

// T034 (parte de spec-001): 9999LLL
{
  await abrirCita(p); await escribirMatricula(p, '9999LLL');
  const t = await texto(p); const img = await captura(p, 't034-9999LLL');
  const ok1 = t.includes('Opel Corsa') && !t.includes('cita pendiente');
  await p.selectOption('select[name=fecha]', D6); await p.fill('textarea[name=motivo]', 'Revisión de kilometraje');
  await confirmar(p);
  const ok2 = (await texto(p)).includes('Cita confirmada');
  const img2 = await captura(p, 't034-9999LLL-confirmada');
  await verDia(r, D6);
  const f = (await filas(r)).filter((x) => x[0] === '9999LLL');
  const img3 = await captura(r, 't034-9999LLL-en-citas');
  anota('T034 «9999LLL» (§2.3, parte de spec-001)', '«Opel Corsa», sin aviso, se confirma y aparece en /citas', `Opel Corsa=${t.includes('Opel Corsa')}, aviso=${t.includes('cita pendiente')}, confirmada=${ok2}, en /citas=${f.length === 1}`,
    ok1 && ok2 && f.length === 1, `${img}, ${img2}, ${img3}`);
}

fs.writeFileSync(`${OUT}/resultados.json`, JSON.stringify(res, null, 2));
await navegador.close();
console.log(`\n${res.filter((x) => x.ok).length}/${res.length} comprobaciones superadas`);
