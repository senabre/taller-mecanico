// T027 de spec-004: quickstart común §4 en el navegador (escritorio 1280 px y móvil 390 px).
// La preparación (órdenes 10 y 11 de §2, diagnóstico y presupuesto de §3, diagnóstico previo del
// §4.5 por I5(b) y la ampliación del §4.7) se hace por la API con las sesiones de ana y luis:
// esas pantallas son de spec-002, 003 y 005 y ya tienen su propia verificación.
// Uso: node t027.mjs <carpeta de evidencias> <carpeta de correos>
import { chromium } from 'playwright';
import fs from 'node:fs';
import { execSync } from 'node:child_process';

const BASE = 'http://localhost:3000';
const OUT = process.argv[2];
const CORREO = process.argv[3];
fs.mkdirSync(OUT, { recursive: true });
const res = [];
let n = 0;
const nav = await chromium.launch();
const ESCRITORIO = { width: 1280, height: 900 };
const MOVIL = { width: 390, height: 844 };

const sql = (q) => execSync(`psql -h localhost -U taller -d taller -tAc "${q}"`).toString().trim();
const madrid = (iso, op) => new Intl.DateTimeFormat('es-ES', { timeZone: 'Europe/Madrid', ...op }).format(new Date(/^\d+$/.test(String(iso)) ? Number(iso) : iso));
const fechaDe = (iso) => madrid(iso, { day: '2-digit', month: '2-digit', year: 'numeric' });
const horaDe = (iso) => madrid(iso, { hour: '2-digit', minute: '2-digit', hour12: false });
const ddmm = (ymd) => ymd.split('-').reverse().join('/');
const limpio = (t) => t.replace(/ | /g, ' ');

async function captura(p, nombre) {
  n += 1; const f = `${String(n).padStart(2, '0')}-${nombre}.png`;
  await p.screenshot({ path: `${OUT}/${f}`, fullPage: true }); return f;
}
function anota(paso, esperado, observado, ok, img) {
  res.push({ paso, esperado, observado, ok, img });
  console.log(`${ok ? 'OK   ' : 'FALLA'} ${paso}: ${observado}`);
}
async function sesion(usuario, viewport = ESCRITORIO, permisos = []) {
  const c = await nav.newContext({ viewport, locale: 'es-ES', timezoneId: 'Europe/Madrid' });
  if (permisos.length) await c.grantPermissions(permisos, { origin: BASE });
  const p = await c.newPage();
  if (usuario) {
    await p.goto(`${BASE}/entrar`); await p.fill('input[name=login]', usuario);
    await p.fill('input[name=contrasena]', 'demo1234'); await p.click('button[type=submit]');
    await p.waitForTimeout(800);
  }
  return p;
}
async function api(p, metodo, ruta, cuerpo) {
  return p.evaluate(async ([m, r, b]) => {
    const resp = await fetch(`/api${r}`, { method: m, headers: { 'content-type': 'application/json' },
      body: b === undefined ? undefined : JSON.stringify(b) });
    const t = await resp.text(); let j = null; try { j = JSON.parse(t); } catch {}
    return { status: resp.status, json: j };
  }, [metodo, ruta, cuerpo]);
}
const texto = async (p) => limpio(await p.locator('main').innerText());
const anchoOk = (p) => p.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth);
async function abrir(p, ruta) { await p.goto(`${BASE}${ruta}`); await p.waitForTimeout(900); }
function correoCon(enlace) {
  for (const f of fs.readdirSync(CORREO).sort().reverse()) {
    const crudo = fs.readFileSync(`${CORREO}/${f}`, 'utf8');
    const t = crudo.replace(/=\r?\n/g, '').replace(/=([0-9A-F]{2})/g, (_, h) => String.fromCharCode(parseInt(h, 16)));
    if (t.includes(enlace)) return { fichero: f, para: (crudo.match(/^To: (.*)$/m) || [])[1] };
  }
  return null;
}
async function presupuestoEmitido(luis, ordenId, lineas) {
  const nuevo = await api(luis, 'POST', `/ordenes/${ordenId}/presupuestos`);
  for (const l of lineas) await api(luis, 'POST', `/presupuestos/${nuevo.json.id}/lineas`, l);
  const emitido = await api(luis, 'POST', `/presupuestos/${nuevo.json.id}/emitir`);
  await new Promise((r) => setTimeout(r, 1500));
  return emitido.json;
}

// ---------------------------------------------------------------------------------------------
// Preparación (§2.4, §2.5 y §3) por la API
// ---------------------------------------------------------------------------------------------
const ana = await sesion('ana');
const luis = await sesion('luis');
const hoy = sql("select to_char((now() at time zone 'Europe/Madrid')::date, 'YYYY-MM-DD')");
const entrega = sql("select to_char((now() at time zone 'Europe/Madrid')::date + 3, 'YYYY-MM-DD')");
const idLuis = Number(sql("select id from usuario where login = 'luis'"));
const cita = (m) => Number(sql(`select c.id from cita c join vehiculo v on v.id = c.vehiculo_id where v.matricula = '${m}' and c.fecha = '${hoy}'`));
const o10 = await api(ana, 'POST', '/ordenes', { cita_id: cita('1313MMM'), km: 85000, danos: 'Rayón puerta trasera izquierda', mecanico_id: idLuis, fecha_entrega_prometida: entrega });
const o11 = await api(ana, 'POST', '/ordenes', { cita_id: cita('1414NNN'), km: 23500, danos: 'Sin daños visibles', mecanico_id: idLuis, fecha_entrega_prometida: entrega });
const pf123 = (await api(luis, 'GET', '/piezas?q=PF-123')).json[0].id;
await api(luis, 'PUT', `/ordenes/${o10.json.id}/diagnostico`, { diagnostico: 'Pastillas y discos delanteros desgastados' });
const p10 = await presupuestoEmitido(luis, o10.json.id, [
  { tipo: 'mano_obra', concepto: 'Cambio de pastillas', cantidad: '1.5' },
  { tipo: 'pieza', pieza_id: pf123, cantidad: '2' }]);
const prep = `orden ${o10.json.numero} (${o10.status}) y ${o11.json.numero} (${o11.status}); presupuesto ${p10.id} emitido, total ${p10.total}, aviso_enviado=${p10.aviso_enviado}`;
anota('Preparación §2.4, §2.5 y §3 (API)', 'Órdenes Nº 10 y 11 y presupuesto de 1313MMM emitido por 133,10 €', prep,
  o10.json.numero === 10 && o11.json.numero === 11 && p10.total === '133.10' && p10.aviso_enviado === true, null);

// ---------------------------------------------------------------------------------------------
// §4.1 · Abrir el enlace del correo
// ---------------------------------------------------------------------------------------------
const c10 = correoCon(p10.enlace);
anota('§4.1 Correo con el enlace (Mailpit)', 'Llega al cliente un correo con el enlace del presupuesto',
  c10 ? `${c10.fichero} para ${c10.para}; enlace ${p10.enlace}` : 'sin correo', !!c10, null);
const cliente = sql("select c.nombre || '|' || c.telefono || '|' || c.email from cliente c join vehiculo v on v.cliente_id = c.id where v.matricula = '1313MMM'").split('|');
for (const [vp, nombre] of [[ESCRITORIO, 'escritorio'], [MOVIL, 'movil']]) {
  const p = await sesion(null, vp);
  await abrir(p, new URL(p10.enlace).pathname);
  const t = await texto(p); const img = await captura(p, `p1-enlace-${nombre}`);
  const faltan = ['Seat', 'León', '1313MMM', 'Pastillas y discos delanteros desgastados', 'Cambio de pastillas',
    'PF-123 Pastillas freno', '110,00 €', '23,10 €', '133,10 €', `Válido hasta ${ddmm(p10.caduca_el)}`].filter((x) => !t.includes(x));
  const datosCliente = cliente.filter((x) => t.includes(x));
  const ancho = await anchoOk(p);
  anota(`§4.1 Página pública (${nombre})`, 'Vehículo, diagnóstico, líneas, base, IVA, total y caducidad; sin datos del cliente; sin desplazamiento lateral',
    `faltan=[${faltan}]; datos del cliente=[${datosCliente}]; sin scroll lateral=${ancho}`,
    faltan.length === 0 && datosCliente.length === 0 && ancho, img);
  await p.context().close();
}

// ---------------------------------------------------------------------------------------------
// §4.2 · Aceptar con confirmación (móvil, que es como lo abrirá el cliente)
// ---------------------------------------------------------------------------------------------
{
  const p = await sesion(null, MOVIL);
  let dialogo = false; p.on('dialog', async (d) => { dialogo = true; await d.dismiss(); });
  await abrir(p, new URL(p10.enlace).pathname);
  await p.getByRole('button', { name: 'Aceptar', exact: true }).click(); await p.waitForTimeout(300);
  const pregunta = '¿Acepta este presupuesto por 133,10 €? Una vez confirmado, no se podrá cambiar.';
  const t1 = await texto(p); const img1 = await captura(p, 'p2-confirmacion-movil');
  anota('§4.2 Pulsar «Aceptar»', `«${pregunta}» con «Sí, aceptar» y «Volver», dentro de la página`,
    `pregunta=${t1.includes(pregunta)}; botones=${await p.getByRole('button', { name: 'Sí, aceptar' }).isVisible()}/${await p.getByRole('button', { name: 'Volver' }).isVisible()}; confirm() del navegador=${dialogo}`,
    t1.includes(pregunta) && !dialogo, img1);
  await p.getByRole('button', { name: 'Volver' }).click(); await p.waitForTimeout(300);
  const volver = await p.getByRole('button', { name: 'Aceptar', exact: true }).isVisible() && !(await texto(p)).includes(pregunta);
  const estado1 = sql(`select estado from presupuesto where id = ${p10.id}`);
  anota('§4.2 «Volver»', 'La página queda como estaba y no se guarda nada', `botones de nuevo=${volver}; estado en la base=${estado1}`,
    volver && estado1 === 'emitido', await captura(p, 'p2-volver-movil'));
  await p.getByRole('button', { name: 'Aceptar', exact: true }).click(); await p.waitForTimeout(200);
  await p.getByRole('button', { name: 'Sí, aceptar' }).click(); await p.waitForTimeout(1000);
  const resp = sql(`select estado || '|' || (extract(epoch from respondido_en)*1000)::bigint from presupuesto where id = ${p10.id}`).split('|');
  const msg = `Aceptado el ${fechaDe(resp[1])} a las ${horaDe(resp[1])}`;
  const t2 = await texto(p); const img2 = await captura(p, 'p2-aceptado-movil');
  anota('§4.2 «Sí, aceptar»', `Se ve «${msg}» (hora de Madrid) y se guarda en la base`, `mensaje=${t2.includes(msg)}; base=${resp[0]} ${resp[1]}`,
    t2.includes(msg) && resp[0] === 'aceptado', img2);
  await abrir(p, new URL(p10.enlace).pathname);
  const botones = await p.getByRole('button').count(); const t3 = await texto(p);
  anota('§4.2 Volver a abrir el enlace', 'Se ve la decisión con su fecha y hora, sin botones', `botones=${botones}; mensaje=${t3.includes(msg)}`,
    botones === 0 && t3.includes(msg), await captura(p, 'p2-reabierto-movil'));
  await p.context().close();
}

// ---------------------------------------------------------------------------------------------
// §4.3 · La orden de 1313MMM como luis
// ---------------------------------------------------------------------------------------------
{
  await abrir(luis, `/ordenes/${o10.json.id}`);
  const r = sql(`select (extract(epoch from respondido_en)*1000)::bigint from presupuesto where id = ${p10.id}`);
  const linea = `Inicial · Aceptado el ${fechaDe(r)} a las ${horaDe(r)}`;
  const t = await texto(luis); const enlace = await luis.locator('input[aria-label="Enlace del presupuesto"]').count();
  anota('§4.3 Orden de 1313MMM (luis)', `«En reparación», «${linea}» y sin enlace`,
    `en reparación=${t.includes('En reparación')}; línea=${t.includes(linea)}; enlace visible=${enlace > 0}; estado en la base=${sql(`select estado from orden where id = ${o10.json.id}`)}`,
    t.includes('En reparación') && t.includes(linea) && enlace === 0, await captura(luis, 'p3-orden-1313MMM'));
}

// ---------------------------------------------------------------------------------------------
// §4.4 · Orden demo 5: enlace y «Copiar enlace» (ana) y sin imputar (marta)
// ---------------------------------------------------------------------------------------------
const id5 = Number(sql('select id from orden where numero = 5'));
for (const [vp, nombre] of [[ESCRITORIO, 'escritorio'], [MOVIL, 'movil']]) {
  const p = await sesion('ana', vp, ['clipboard-read', 'clipboard-write']);
  await abrir(p, `/ordenes/${id5}`);
  const campo = p.locator('input[aria-label="Enlace del presupuesto"]');
  const valor = (await campo.count()) ? await campo.inputValue() : '';
  const solo = (await campo.count()) ? await campo.getAttribute('readonly') !== null : false;
  await p.getByRole('button', { name: 'Copiar enlace' }).click();
  await p.waitForTimeout(300);
  const copiado = await p.getByRole('button', { name: 'Copiado' }).isVisible().catch(() => false);
  const portapapeles = await p.evaluate(() => navigator.clipboard.readText());
  const ancho = await anchoOk(p);
  anota(`§4.4 Orden demo 5, «Copiar enlace» (ana, ${nombre})`, 'Enlace visible en un campo de solo lectura; «Copiar enlace» lo copia y pasa a «Copiado»',
    `enlace=${valor}; solo lectura=${solo}; «Copiado»=${copiado}; portapapeles igual=${portapapeles === valor}; sin scroll lateral=${ancho}`,
    valor === `${BASE}/presupuesto/demo-presupuesto-emitido` && solo && copiado && portapapeles === valor && ancho,
    await captura(p, `p4-copiar-enlace-${nombre}`));
  await p.context().close();
}
{
  const marta = await sesion('marta');
  await abrir(marta, `/ordenes/${id5}`);
  const imputar = await marta.getByRole('button', { name: 'Imputar' }).count();
  const t = await texto(marta);
  const mensaje = 'La orden está «En diagnóstico»: solo se imputa en órdenes en reparación.';
  const directa = await api(marta, 'POST', `/ordenes/${id5}/imputaciones`, { tipo: 'mano_obra', concepto: 'Prueba', cantidad: '1' });
  anota('§4.4 Orden demo 5 como marta, su mecánica', 'No aparece la opción de imputar (y el servidor también lo impide)',
    `botón «Imputar»=${imputar}; aviso=${t.includes(mensaje)}; POST directo=${directa.status} «${directa.json?.error}»`,
    imputar === 0 && t.includes(mensaje) && directa.status === 409, await captura(marta, 'p4-sin-imputar-marta'));
  await marta.context().close();
}

// ---------------------------------------------------------------------------------------------
// §4.5 · Rechazo del inicial en la orden de 1414NNN
// ---------------------------------------------------------------------------------------------
await api(luis, 'PUT', `/ordenes/${o11.json.id}/diagnostico`, { diagnostico: 'Aceite degradado' });
const p11 = await presupuestoEmitido(luis, o11.json.id, [{ tipo: 'mano_obra', concepto: 'Cambio de aceite', cantidad: '1' }]);
{
  const c = correoCon(p11.enlace);
  const p = await sesion(null, ESCRITORIO);
  await abrir(p, new URL(p11.enlace).pathname);
  await p.getByRole('button', { name: 'Rechazar' }).click(); await p.waitForTimeout(200);
  const pregunta = '¿Rechaza este presupuesto por 48,40 €? Una vez confirmado, no se podrá cambiar.';
  const conf = (await texto(p)).includes(pregunta);
  await captura(p, 'p5-confirmar-rechazo');
  await p.getByRole('button', { name: 'Sí, rechazar' }).click(); await p.waitForTimeout(1000);
  const r = sql(`select (extract(epoch from respondido_en)*1000)::bigint from presupuesto where id = ${p11.id}`);
  const msg = `Rechazado el ${fechaDe(r)} a las ${horaDe(r)}`;
  const tp = await texto(p); const img = await captura(p, 'p5-rechazado');
  await abrir(luis, `/ordenes/${o11.json.id}`);
  const t = await texto(luis);
  anota('§4.5 Rechazar el presupuesto de 1414NNN', `Confirmación «${pregunta}»; «${msg}» en la página; la orden sigue «En diagnóstico» con «Inicial · ${msg}»`,
    `correo=${!!c}; confirmación=${conf}; página=${tp.includes(msg)}; ficha en diagnóstico=${t.includes('En diagnóstico')}; ficha línea=${t.includes(`Inicial · ${msg}`)}; estado en la base=${sql(`select estado from orden where id = ${o11.json.id}`)}`,
    !!c && conf && tp.includes(msg) && t.includes('En diagnóstico') && t.includes(`Inicial · ${msg}`), img);
  await captura(luis, 'p5-orden-1414NNN');
  await p.context().close();
}

// ---------------------------------------------------------------------------------------------
// §4.6 · Orden demo 7: la ampliación caducada no enseña enlace
// ---------------------------------------------------------------------------------------------
const id7 = Number(sql('select id from orden where numero = 7'));
const caducaAmpl = sql("select to_char(caduca_el, 'YYYY-MM-DD') from presupuesto where token = 'demo-presupuesto-caducado'");
{
  await abrir(luis, `/ordenes/${id7}`);
  const t = await texto(luis);
  const enlace = await luis.locator('input[aria-label="Enlace del presupuesto"]').count();
  const copiar = await luis.getByRole('button', { name: 'Copiar enlace' }).count();
  anota('§4.6 Orden demo 7 (luis)', `«Pendiente de conformidad»; la ampliación sale «Caducado el ${ddmm(caducaAmpl)}», sin enlace ni «Copiar enlace»`,
    `pendiente=${t.includes('Pendiente de conformidad')}; caducado=${t.includes(`Caducado el ${ddmm(caducaAmpl)}`)}; enlace=${enlace}; copiar=${copiar}`,
    t.includes('Pendiente de conformidad') && t.includes(`Caducado el ${ddmm(caducaAmpl)}`) && enlace === 0 && copiar === 0,
    await captura(luis, 'p6-orden7-caducada'));
}

// ---------------------------------------------------------------------------------------------
// §4.7 · Ampliación rechazada en una pestaña y aceptada en otra
// ---------------------------------------------------------------------------------------------
const p7 = await presupuestoEmitido(luis, id7, [{ tipo: 'mano_obra', concepto: 'Sustitución de discos', cantidad: '1' }]);
{
  const c = correoCon(p7.enlace);
  const ctx = await nav.newContext({ viewport: ESCRITORIO, locale: 'es-ES', timezoneId: 'Europe/Madrid' });
  const t1 = await ctx.newPage(); const t2 = await ctx.newPage();
  const ruta = new URL(p7.enlace).pathname;
  await abrir(t1, ruta); await abrir(t2, ruta);
  const titulo = (await texto(t1)).includes('Ampliación de presupuesto');
  await t1.getByRole('button', { name: 'Rechazar' }).click(); await t1.waitForTimeout(200);
  await t1.getByRole('button', { name: 'Sí, rechazar' }).click(); await t1.waitForTimeout(1000);
  await captura(t1, 'p7-pestana1-rechazada');
  const inicial = sql(`select (extract(epoch from respondido_en)*1000)::bigint from presupuesto where orden_id = ${id7} and tipo = 'inicial'`);
  const ampl = sql(`select (extract(epoch from respondido_en)*1000)::bigint from presupuesto where id = ${p7.id}`);
  const lIni = `Inicial · Aceptado el ${fechaDe(inicial)} a las ${horaDe(inicial)}`;
  const lAmp = `Ampliación · Rechazado el ${fechaDe(ampl)} a las ${horaDe(ampl)}`;
  await abrir(luis, `/ordenes/${id7}`);
  const tf = await texto(luis);
  anota('§4.7 Rechazar la ampliación en la primera pestaña', `La orden sigue «Pendiente de conformidad» con «${lIni}» y «${lAmp}»`,
    `correo=${!!c}; título ampliación=${titulo}; pendiente=${tf.includes('Pendiente de conformidad')}; inicial=${tf.includes(lIni)}; ampliación=${tf.includes(lAmp)}`,
    !!c && titulo && tf.includes('Pendiente de conformidad') && tf.includes(lIni) && tf.includes(lAmp), await captura(luis, 'p7-orden7-dos-respuestas'));
  await t2.getByRole('button', { name: 'Aceptar', exact: true }).click(); await t2.waitForTimeout(200);
  await t2.getByRole('button', { name: 'Sí, aceptar' }).click(); await t2.waitForTimeout(1000);
  const aviso = 'Este presupuesto ya fue respondido';
  const ta = await texto(t2); const img = await captura(t2, 'p7-pestana2-ya-respondido');
  const estado = sql(`select o.estado || '|' || p.estado from orden o join presupuesto p on p.orden_id = o.id where p.id = ${p7.id}`);
  anota('§4.7 Aceptar en la segunda pestaña', `«${aviso}» y la orden no cambia`, `aviso=${ta.includes(aviso)}; orden|ampliación en la base=${estado}`,
    ta.includes(aviso) && estado === 'pendiente_conformidad|rechazado', img);
  await ctx.close();
}

// ---------------------------------------------------------------------------------------------
// §4.8 y §4.9 · Caducado y enlace inventado (escritorio y móvil)
// ---------------------------------------------------------------------------------------------
for (const [vp, nombre] of [[ESCRITORIO, 'escritorio'], [MOVIL, 'movil']]) {
  const p = await sesion(null, vp);
  await abrir(p, '/presupuesto/demo-presupuesto-caducado');
  const msg = `Este presupuesto caducó el ${ddmm(caducaAmpl)}. Contacte con el taller`;
  const t = await texto(p); const botones = await p.getByRole('button').count();
  anota(`§4.8 Presupuesto caducado (${nombre})`, `«${msg}», sin botones`, `mensaje=${t.includes(msg)}; botones=${botones}; sin scroll lateral=${await anchoOk(p)}`,
    t.includes(msg) && botones === 0 && (await anchoOk(p)), await captura(p, `p8-caducado-${nombre}`));
  await abrir(p, '/presupuesto/enlace-inventado');
  const t9 = (await texto(p)).trim();
  anota(`§4.9 Enlace inventado (${nombre})`, 'Solo «Este enlace no es válido»', `texto=«${t9}»`,
    t9 === 'Este enlace no es válido', await captura(p, `p9-enlace-invalido-${nombre}`));
  await p.context().close();
}

fs.writeFileSync(`${OUT}/resultados.json`, JSON.stringify(res, null, 2));
await nav.close();
console.log(`\n${res.filter((x) => x.ok).length}/${res.length} comprobaciones superadas`);
