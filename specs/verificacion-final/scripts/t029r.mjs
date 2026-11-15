// T029 de spec-004: regresión del quickstart común, secciones §0, §3, §5, §6 y §7 en pantalla.
// (§1, §2 y §4 los recorren t032.mjs, t029.mjs/t029b.mjs y t027.mjs.)
// Uso: node t029r.mjs <carpeta> <sección: 0 | 3 | 5 | 6 | 7> <carpeta de correos>
import { chromium } from 'playwright';
import fs from 'node:fs';
import { execSync } from 'node:child_process';

const BASE = 'http://localhost:3000';
const OUT = process.argv[2];
const SEC = process.argv[3];
const CORREO = process.argv[4];
fs.mkdirSync(OUT, { recursive: true });
const res = [];
let n = 0;
const nav = await chromium.launch();
const sql = (q) => execSync(`psql -h localhost -U taller -d taller -tAc "${q}"`).toString().trim();
const limpio = (t) => t.replace(/ | /g, ' ');
const ddmm = (ymd) => ymd.split('-').reverse().join('/');
async function captura(p, nombre) {
  n += 1; const f = `s${SEC}-${String(n).padStart(2, '0')}-${nombre}.png`;
  await p.screenshot({ path: `${OUT}/${f}`, fullPage: true }); return f;
}
function anota(paso, esperado, observado, ok, img) {
  res.push({ paso, esperado, observado, ok, img });
  console.log(`${ok ? 'OK   ' : 'FALLA'} ${paso}: ${observado}`);
}
async function sesion(usuario, contrasena = 'demo1234') {
  const c = await nav.newContext({ viewport: { width: 1280, height: 900 }, locale: 'es-ES', timezoneId: 'Europe/Madrid' });
  const p = await c.newPage();
  p.on('dialog', (d) => d.accept());
  if (usuario) {
    await p.goto(`${BASE}/entrar`); await p.fill('input[name=login]', usuario);
    await p.fill('input[name=contrasena]', contrasena); await p.click('button[type=submit]');
    await p.waitForTimeout(900);
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
const todo = async (p) => limpio(await p.locator('body').innerText());
async function abrir(p, ruta) { await p.goto(`${BASE}${ruta}`); await p.waitForTimeout(1000); }
const idOrden = (num) => Number(sql(`select id from orden where numero = ${num}`));
const num1313 = () => Number(sql("select o.numero from orden o join cita c on c.id = o.cita_id join vehiculo v on v.id = c.vehiculo_id where v.matricula = '1313MMM' order by o.numero desc limit 1"));
const nav_ = async (p) => limpio(await p.locator('header, nav').first().innerText().catch(() => ''));
async function lineaMano(p, concepto, horas, boton) {
  const f = p.locator('form', { hasText: boton }).last();
  await f.locator('select').selectOption('mano_obra');
  await f.locator('input[name=concepto]').fill(concepto); await f.locator('input[name=horas]').fill(horas);
  await f.getByRole('button', { name: boton }).click(); await p.waitForTimeout(900);
  return limpio(await f.locator('[role=alert]').innerText().catch(() => ''));
}
async function lineaPieza(p, ref, uds, boton) {
  const f = p.locator('form', { hasText: boton }).last();
  await f.locator('select').selectOption('pieza'); await p.waitForTimeout(200);
  await f.locator('#pieza').fill(ref); await p.waitForTimeout(700);
  await f.locator('ul button').first().click(); await p.waitForTimeout(200);
  await f.locator('input[name=unidades]').fill(String(uds));
  await f.getByRole('button', { name: boton }).click(); await p.waitForTimeout(900);
  return limpio(await f.locator('[role=alert]').innerText().catch(() => ''));
}
function correoCon(cadena) {
  for (const f of fs.readdirSync(CORREO).sort().reverse()) {
    const t = fs.readFileSync(`${CORREO}/${f}`, 'utf8').replace(/\?=\r?\n[ \t]+/g, '?= ').replace(/\r?\n[ \t]+/g, ' ').replace(/=\r?\n/g, '')
      .replace(/(=[0-9A-F]{2})+/g, (m) => Buffer.from(m.split('=').slice(1).map((h) => parseInt(h, 16))).toString('utf8'))
      .replace(/=\?utf-8\?b\?([^?]*)\?=/gi, (_, b) => Buffer.from(b, 'base64').toString('utf8'));
    if (t.includes(cadena)) return t;
  }
  return null;
}

// ---------------------------------------------------------------------------------------------
if (SEC === '0') {
  const menu = async (u) => { const p = await sesion(u); const t = await nav_(p); return { p, t }; };
  const tiene = (t, xs) => xs.filter((x) => t.includes(x));
  {
    const { p, t } = await menu('luis');
    const si = tiene(t, ['Mis órdenes', 'Historial']); const no = tiene(t, ['Citas', 'Almacén', 'Pedidos', 'Tablero']);
    anota('§0.1 Menú de luis', 'Mis órdenes e Historial; sin Citas, Almacén, Pedidos ni Tablero', `ve=[${si}]; no debería ver=[${no}]`,
      si.length === 2 && no.length === 0, await captura(p, 'menu-luis'));
  }
  {
    const p = await sesion('luis', 'equivocada');
    const t = await todo(p);
    anota('§0.2 Contraseña errónea', '«Usuario o contraseña incorrectos»', t.includes('Usuario o contraseña incorrectos') ? 'mensaje visible' : t.slice(0, 80),
      t.includes('Usuario o contraseña incorrectos'), await captura(p, 'contrasena-erronea'));
  }
  {
    const { p, t } = await menu('paco');
    const si = tiene(t, ['Citas', 'Órdenes', 'Historial', 'Almacén', 'Pedidos', 'Tablero']);
    anota('§0.3 Menú de paco', 'Todo el menú, incluido el Tablero', `ve=[${si}]`, si.includes('Tablero') && si.length === 6, await captura(p, 'menu-paco'));
  }
  for (const [u, debe] of [['ana', ['Citas', 'Órdenes', 'Historial']], ['pepe', ['Almacén', 'Pedidos', 'Historial']]]) {
    const { p, t } = await menu(u);
    const si = tiene(t, debe);
    anota(`§0.4 Menú de ${u}`, `${debe.join(', ')}; sin Tablero`, `ve=[${si}]; Tablero=${t.includes('Tablero')}`,
      si.length === 3 && !t.includes('Tablero'), await captura(p, `menu-${u}`));
  }
}

// ---------------------------------------------------------------------------------------------
if (SEC === '3') {
  const id = idOrden(num1313());
  const luis = await sesion('luis');
  await abrir(luis, `/ordenes/${id}`);
  await luis.fill('textarea[name=diagnostico]', 'Pastillas delanteras desgastadas');
  await luis.getByRole('button', { name: 'Guardar diagnóstico' }).click(); await luis.waitForTimeout(900);
  let t = await texto(luis);
  const e1 = t.includes('En diagnóstico');
  await luis.fill('textarea[name=diagnostico]', 'Pastillas y discos delanteros desgastados');
  await luis.getByRole('button', { name: 'Guardar diagnóstico' }).click(); await luis.waitForTimeout(900);
  await abrir(luis, `/ordenes/${id}`);
  t = await texto(luis);
  const guardado = (await luis.inputValue('textarea[name=diagnostico]')) === 'Pastillas y discos delanteros desgastados';
  anota('§3.1 Diagnóstico y cambio', '«En diagnóstico»; el cambio se guarda y sigue «En diagnóstico»', `en diagnóstico=${e1}/${t.includes('En diagnóstico')}; guardado=${guardado}`,
    e1 && t.includes('En diagnóstico') && guardado, await captura(luis, 'diagnostico'));

  await luis.getByRole('button', { name: 'Nuevo presupuesto' }).click(); await luis.waitForTimeout(900);
  await lineaMano(luis, 'Cambio de pastillas', '1,5', 'Añadir línea');
  await lineaPieza(luis, 'PF-123', 2, 'Añadir línea');
  t = await texto(luis);
  const l2 = ['Cambio de pastillas', '60,00 €', 'PF-123 Pastillas freno', '50,00 €'].filter((x) => !t.includes(x));
  anota('§3.2 Líneas del presupuesto', '«Cambio de pastillas» 60,00 € y «PF-123 Pastillas freno» 50,00 €', `faltan=[${l2}]`, l2.length === 0, await captura(luis, 'lineas'));
  const tot = ['110,00 €', '23,10 €', '133,10 €'].filter((x) => !t.includes(x));
  anota('§3.3 Totales', 'Base 110,00 €, IVA 23,10 €, total 133,10 €', `faltan=[${tot}]`, tot.length === 0, null);

  const err13 = await lineaMano(luis, 'Prueba', '1,3', 'Añadir línea');
  const m13 = err13;
  const errPieza = await lineaPieza(luis, 'PF-123', 2.5, 'Añadir línea').catch((e) => `error del navegador: ${e.message}`);
  const valid = await luis.locator('form', { hasText: 'Añadir línea' }).last().locator('input[name=unidades]').evaluate((i) => i.validationMessage).catch(() => '');
  const r25 = await api(luis, 'POST', `/presupuestos/${sql(`select id from presupuesto where orden_id = ${id}`)}/lineas`, { tipo: 'pieza', pieza_id: Number(sql("select id from pieza where referencia = 'PF-123'")), cantidad: '2.5' });
  anota('§3.4 1,3 h y 2,5 unidades', 'Se rechaza explicando el motivo', `1,3 h=«${m13}»; 2,5 uds: navegador=«${valid}», API=${r25.status} «${r25.json?.error}»`,
    !!m13 && r25.status === 422, await captura(luis, 'rechazos'));
  await abrir(luis, `/ordenes/${id}`);
  await lineaMano(luis, 'Prueba', '1', 'Añadir línea');
  const conPrueba = (await texto(luis)).includes('Prueba');
  await luis.locator('tr', { hasText: 'Prueba' }).getByRole('button').click(); await luis.waitForTimeout(900);
  t = await texto(luis);
  anota('§3.5 Añadir y quitar «Prueba»', 'Los totales vuelven a 110,00 / 23,10 / 133,10', `añadida=${conPrueba}; quitada=${!t.includes('Prueba')}; totales=${['110,00 €', '23,10 €', '133,10 €'].every((x) => t.includes(x))}`,
    conPrueba && !t.includes('Prueba') && ['110,00 €', '23,10 €', '133,10 €'].every((x) => t.includes(x)), await captura(luis, 'prueba-quitada'));

  await luis.getByRole('button', { name: 'Emitir presupuesto' }).click(); await luis.getByRole('button', { name: 'Sí, emitir' }).click(); /* confirmación dentro de la página (V2) */ await luis.waitForTimeout(1500);
  t = await texto(luis);
  const caduca = sql("select to_char((now() at time zone 'Europe/Madrid')::date + 15, 'YYYY-MM-DD')");
  const formLineas = await luis.locator('form', { hasText: 'Añadir línea' }).count();
  const quitar = await luis.locator('section', { hasText: 'Presupuesto' }).locator('tbody button').count();
  const enlace = await luis.locator('input[aria-label="Enlace del presupuesto"]').inputValue().catch(() => '');
  const correo = enlace ? correoCon(enlace) : null;
  anota('§3.6 Emitir', `«Válido hasta ${ddmm(caduca)}»; sin añadir ni quitar líneas; correo con el enlace`, `válido=${t.includes(`Válido hasta ${ddmm(caduca)}`)}; formulario=${formLineas}; botones quitar=${quitar}; correo=${!!correo && /\/presupuesto\/\S+/.test(correo)}`,
    t.includes(`Válido hasta ${ddmm(caduca)}`) && formLineas === 0 && quitar === 0 && !!correo, await captura(luis, 'emitido'));

  // Desde el converge de spec-003 (V2 de spec-004), tras emitir el diagnóstico se ve sin «Guardar
  // diagnóstico» y con el aviso (quickstart §3.7 actualizado). Se comprueba también que la API lo rechaza.
  const msg = 'Ya se ha emitido un presupuesto para esta orden; el diagnóstico no se puede cambiar';
  const t7 = await texto(luis);
  const cuadro = await luis.locator('textarea[name=diagnostico]').count();
  const guardar = await luis.getByRole('button', { name: 'Guardar diagnóstico' }).count();
  const rApi = await luis.evaluate(async (id) => { const r = await fetch(`/api/ordenes/${id}/diagnostico`, { method: 'PUT', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ diagnostico: 'Otro diagnóstico' }) }); let j = null; try { j = await r.json(); } catch {} return { status: r.status, error: j?.error }; }, id);
  const enBase = sql(`select diagnostico from orden where id = ${id}`);
  anota('§3.7 El diagnóstico ya no se puede cambiar tras emitir', `Sin «Guardar diagnóstico», con «${msg}»; la API lo rechaza y no se guarda`,
    `cuadro editable=${cuadro}; botón=${guardar}; aviso=${t7.includes(msg)}; API=${rApi.status} «${rApi.error}»; en la base=«${enBase}»`,
    guardar === 0 && t7.includes(msg) && rApi.status === 409 && enBase === 'Pastillas y discos delanteros desgastados', await captura(luis, 'diagnostico-bloqueado'));
}

// ---------------------------------------------------------------------------------------------
if (SEC === '5') {
  const num = num1313(); const id = idOrden(num);
  const luis = await sesion('luis');
  await abrir(luis, `/ordenes/${id}`);
  const boton = await luis.getByRole('button', { name: 'Terminar reparación' }).count();
  const r = await api(luis, 'POST', `/ordenes/${id}/terminar`);
  anota('§5.1 Terminar sin imputar', '«Primero hay que imputar el trabajo realizado»', `botón en pantalla=${boton}; API=${r.status} «${r.json?.error}»`,
    r.json?.error === 'Primero hay que imputar el trabajo realizado', await captura(luis, 'terminar-sin-imputar'));

  await lineaPieza(luis, 'PF-123', 2, 'Imputar');
  let t = await texto(luis);
  const pepe = await sesion('pepe');
  await abrir(pepe, '/almacen');
  const stock = (await pepe.locator('tr', { hasText: 'PF-123' }).innerText()).trim().split(/\s+/).pop();
  const f3 = await lineaPieza(luis, 'PF-123', 3, 'Imputar');
  const m3 = f3;
  anota('§5.2 Imputar PF-123 × 2 y luego × 3', '«PF-123 Pastillas freno»; stock 4 → 2; «Solo hay 2 unidades en stock de PF-123…»', `concepto=${t.includes('PF-123 Pastillas freno')}; stock=${stock}; error=«${m3}»`,
    t.includes('PF-123 Pastillas freno') && stock === '2' && m3.startsWith('Solo hay 2 unidades en stock de PF-123'), await captura(luis, 'imputar-pieza'));
  await captura(pepe, 'almacen-stock');
  await abrir(luis, `/ordenes/${id}`);
  await lineaMano(luis, 'Cambio de pastillas', '1', 'Imputar');
  t = await texto(luis);
  anota('§5.3 Imputar 1 h', '«Acumulado 90,00 € de 110,00 €»', `${t.match(/Acumulado [^\n]*/)?.[0]}`, t.includes('Acumulado 90,00 € de 110,00 €'), await captura(luis, 'acumulado-90'));
  await lineaMano(luis, 'Cambio de pastillas', '1', 'Imputar');
  await abrir(luis, `/ordenes/${id}`);
  t = await texto(luis);
  const sinImputar = await luis.getByRole('button', { name: 'Imputar' }).count();
  const sinTerminar = await luis.getByRole('button', { name: 'Terminar reparación' }).count();
  anota('§5.4 Imputar 1 h más', '130,00 € > 121,00 € → «Pendiente de conformidad»; no deja imputar ni terminar', `${t.match(/Acumulado [^\n]*/)?.[0]}; pendiente=${t.includes('Pendiente de conformidad')}; imputar=${sinImputar}; terminar=${sinTerminar}`,
    t.includes('Acumulado 130,00 €') && t.includes('Pendiente de conformidad') && sinImputar === 0 && sinTerminar === 0, await captura(luis, 'pendiente-conformidad'));

  await luis.getByRole('button', { name: 'Nuevo presupuesto' }).click(); await luis.waitForTimeout(900);
  await lineaMano(luis, 'Sustitución de discos', '1', 'Añadir línea');
  await luis.getByRole('button', { name: 'Emitir presupuesto' }).click(); await luis.getByRole('button', { name: 'Sí, emitir' }).click(); /* confirmación dentro de la página (V2) */ await luis.waitForTimeout(1500);
  const enlace = await luis.locator('input[aria-label="Enlace del presupuesto"]').inputValue();
  const correo = correoCon(enlace);
  const cli = await sesion(null);
  await abrir(cli, new URL(enlace).pathname);
  await cli.getByRole('button', { name: 'Aceptar', exact: true }).click(); await cli.waitForTimeout(300);
  await cli.getByRole('button', { name: 'Sí, aceptar' }).click(); await cli.waitForTimeout(1000);
  const ok = (await texto(cli)).includes('Aceptado el');
  await captura(cli, 'ampliacion-aceptada');
  await abrir(luis, `/ordenes/${id}`);
  t = await texto(luis);
  anota('§5.5 Ampliación de 1 h aceptada desde el correo', '«En reparación» con 150,00 € presupuestados', `correo=${!!correo}; aceptada=${ok}; en reparación=${t.includes('En reparación')}; ${t.match(/Acumulado [^\n]*/)?.[0]}`,
    !!correo && ok && t.includes('En reparación') && t.includes('de 150,00 € presupuestados'), await captura(luis, 'en-reparacion-150'));

  await luis.getByRole('button', { name: 'Terminar reparación' }).click(); await luis.getByRole('button', { name: 'Sí, terminar' }).click(); /* confirmación dentro de la página (V2) */ await luis.waitForTimeout(1200);
  await abrir(luis, `/ordenes/${id}`);
  t = await texto(luis);
  anota('§5.6 Terminar', '«Terminada» y ya no deja imputar', `terminada=${t.includes('Terminada')}; imputar=${await luis.getByRole('button', { name: 'Imputar' }).count()}`,
    t.includes('Terminada') && (await luis.getByRole('button', { name: 'Imputar' }).count()) === 0, await captura(luis, 'terminada'));
}

// ---------------------------------------------------------------------------------------------
if (SEC === '6') {
  const R = 3; const idR = idOrden(R);
  const pepe = await sesion('pepe');
  await abrir(pepe, '/almacen');
  await pepe.fill('input[name=referencia]', 'FA-9'); await pepe.fill('input[name=descripcion]', 'Filtro aceite');
  await pepe.fill('input[name=precio]', '12,00'); await pepe.getByRole('button', { name: 'Dar de alta' }).click(); await pepe.waitForTimeout(900);
  const fila = limpio(await pepe.locator('tr', { hasText: 'FA-9' }).innerText().catch(() => ''));
  await pepe.fill('input[name=referencia]', 'FA-9'); await pepe.fill('input[name=descripcion]', 'Filtro aceite');
  await pepe.fill('input[name=precio]', '12,00'); await pepe.getByRole('button', { name: 'Dar de alta' }).click(); await pepe.waitForTimeout(900);
  const t = await texto(pepe);
  anota('§6.1 Alta de FA-9 sin stock y repetida', 'Stock 0; «Ya existe la referencia FA-9»', `fila=«${fila.replace(/\s+/g, ' ')}»; repetida=${t.includes('Ya existe la referencia FA-9')}`,
    /FA-9\s+Filtro aceite\s+12,00 €\s+0/.test(fila) && t.includes('Ya existe la referencia FA-9'), await captura(pepe, 'alta-fa9'));

  await abrir(pepe, '/pedidos');
  const opciones = await pepe.locator('select[name=proveedor] option').allInnerTexts();
  anota('§6.2 Proveedores', '«Recambios Sur» precargado, sin opción de crear', `opciones=[${opciones}]`, opciones.includes('Recambios Sur') && !opciones.some((o) => /nuevo|crear/i.test(o)), null);
  async function pedido(ref, cant, orden) {
    await pepe.selectOption('select[name=proveedor]', { label: 'Recambios Sur' });
    await pepe.fill('#referencia', ref); await pepe.waitForTimeout(700);
    await pepe.locator('form ul button', { hasText: ref }).first().click(); await pepe.waitForTimeout(200);
    await pepe.fill('input[name=cantidad]', String(cant)); await pepe.fill('input[name=orden]', String(orden));
    await pepe.getByRole('button', { name: 'Registrar pedido' }).click(); await pepe.waitForTimeout(1000);
    return limpio(await pepe.locator('form').innerText());
  }
  await pedido('FA-9', 1, R);
  let lista = limpio(await pepe.locator('tr', { hasText: 'FA-9' }).innerText().catch(() => ''));
  const hoy = sql("select to_char((now() at time zone 'Europe/Madrid')::date, 'DD/MM/YYYY')");
  let estR = sql(`select estado from orden where id = ${idR}`);
  anota(`§6.3 Pedido FA-9 para la orden ${R}`, `«Pendiente» con fecha ${hoy}; la orden pasa a «En espera de piezas»`, `fila=«${lista.replace(/\s+/g, ' ')}»; orden=${estR}`,
    lista.includes(hoy) && estR === 'en_espera_piezas', await captura(pepe, 'pedido-fa9'));
  const f4 = await pedido('FA-9', 1, R);
  anota('§6.4 FA-9 otra vez', `«Ya hay un pedido pendiente de FA-9 para la orden ${R}»`, f4.split('\n').find((x) => x.includes('Ya hay')) || f4.slice(-120),
    f4.includes(`Ya hay un pedido pendiente de FA-9 para la orden ${R}`), null);
  const f5 = await pedido('PF-123', 1, R);
  estR = sql(`select estado from orden where id = ${idR}`);
  anota('§6.5 PF-123 para la misma orden', 'Se admite y la orden sigue en espera', `pendientes de ${R}=${sql(`select count(*) from pedido where orden_id = ${idR} and recibido_en is null`)}; orden=${estR}`,
    sql(`select count(*) from pedido where orden_id = ${idR} and recibido_en is null`) === '2' && estR === 'en_espera_piezas', null);
  const f6 = await pedido('PF-123', 1, 4);
  anota('§6.6 Pedido para una orden en diagnóstico', 'Se rechaza explicando el motivo', f6.split('\n').filter((x) => /orden/i.test(x)).pop() || f6.slice(-120),
    sql(`select count(*) from pedido where orden_id = ${idOrden(4)}`) === '0' && /diagn|reparaci/i.test(f6), await captura(pepe, 'pedido-orden-diagnostico'));

  const luis = await sesion('luis');
  await abrir(luis, `/ordenes/${idR}`);
  const tl = await texto(luis);
  anota(`§6.7 Orden ${R} como luis`, 'Ve los dos pedidos pendientes; no puede imputar ni terminar', `FA-9=${tl.includes('FA-9')}; PF-123=${tl.includes('PF-123')}; imputar=${await luis.getByRole('button', { name: 'Imputar' }).count()}; terminar=${await luis.getByRole('button', { name: 'Terminar reparación' }).count()}`,
    tl.includes('FA-9') && tl.includes('PF-123') && (await luis.getByRole('button', { name: 'Imputar' }).count()) === 0 && (await luis.getByRole('button', { name: 'Terminar reparación' }).count()) === 0,
    await captura(luis, 'orden-en-espera'));

  await abrir(pepe, '/pedidos');
  await pepe.locator('tr', { hasText: 'FA-9' }).getByRole('button', { name: 'Marcar recibido' }).click(); await pepe.waitForTimeout(1000);
  const stockFA = sql("select stock from pieza where referencia = 'FA-9'");
  estR = sql(`select estado from orden where id = ${idR}`);
  const sigue = await pepe.locator('tr', { hasText: 'FA-9' }).getByRole('button', { name: 'Marcar recibido' }).count();
  anota('§6.8 Recibir FA-9', 'Stock de FA-9 = 1; la orden sigue en espera; sin «Marcar recibido» en ese pedido', `stock=${stockFA}; orden=${estR}; botón=${sigue}`,
    stockFA === '1' && estR === 'en_espera_piezas' && sigue === 0, await captura(pepe, 'recibido-fa9'));
  await pepe.locator('tr', { hasText: 'PF-123' }).getByRole('button', { name: 'Marcar recibido' }).click(); await pepe.waitForTimeout(1000);
  estR = sql(`select estado from orden where id = ${idR}`);
  await abrir(luis, `/ordenes/${idR}`);
  anota('§6.9 Recibir PF-123', 'La orden vuelve sola a «En reparación» (el tablero queda para spec-008)', `orden=${estR}; ficha=${(await texto(luis)).includes('En reparación')}`,
    estR === 'en_reparacion' && (await texto(luis)).includes('En reparación'), await captura(luis, 'vuelve-en-reparacion'));
}

// ---------------------------------------------------------------------------------------------
if (SEC === '7') {
  const num = num1313(); const id = idOrden(num);
  const ana = await sesion('ana');
  await abrir(ana, `/ordenes/${idOrden(2)}`);
  anota('§7.1 Orden en reparación (Nº 2) como ana', 'No aparece «Facturar»', `Facturar=${await ana.getByRole('button', { name: 'Facturar' }).count()}`,
    (await ana.getByRole('button', { name: 'Facturar' }).count()) === 0, await captura(ana, 'sin-facturar'));
  await abrir(ana, `/ordenes/${id}`);
  await ana.getByRole('button', { name: 'Facturar' }).click(); await ana.getByRole('button', { name: 'Sí, facturar' }).click(); /* confirmación dentro de la página (V2) */ await ana.waitForTimeout(1500);
  let t = await texto(ana);
  const fnum = sql(`select numero from factura where orden_id = ${id}`);
  const previa = sql(`select coalesce(max(numero), 0) from factura where orden_id <> ${id}`);
  anota('§7.2 Facturar la orden de 1313MMM', '«Factura nº N» correlativo, total que cuadra', `${t.match(/Factura nº [^\n]*/)?.[0]}; anterior=${previa}`,
    t.includes(`Factura nº ${fnum}`) && Number(fnum) === Number(previa) + 1 && t.includes('157,30 €'), await captura(ana, 'facturada'));
  await ana.getByRole('link', { name: 'Ver e imprimir' }).click(); await ana.waitForTimeout(1200);
  t = await todo(ana);
  const lineas = ['PF-123 Pastillas freno', 'Cambio de pastillas', '130,00 €', '27,30 €', '157,30 €'].filter((x) => !t.includes(x));
  await ana.emulateMedia({ media: 'print' });
  // El menú de la app es el que lleva «Salir»; la cabecera propia de la factura no cuenta.
  const menuImpr = await ana.getByRole('button', { name: 'Salir' }).isVisible();
  const img = await captura(ana, 'factura-impresion');
  await ana.emulateMedia({ media: 'screen' });
  anota('§7.2 y §7.3 Ver e imprimir', 'Líneas imputadas; base 130,00 €, IVA 27,30 €, total 157,30 €; al imprimir, sin menú; sin logo sin TALLER_LOGO', `faltan=[${lineas}]; menú al imprimir=${menuImpr}; imágenes=${await ana.locator('main img').count()}`,
    lineas.length === 0 && !menuImpr && (await ana.locator('main img').count()) === 0, img);
  const correo = correoCon('157,30');
  anota('§7.4 Correo al cliente', '«Su vehículo 1313MMM está listo para recoger», con el total', `asunto=${!!correo && correo.includes('Su vehículo 1313MMM está listo para recoger')}; total=${!!correo && correo.includes('157,30')}`,
    !!correo && correo.includes('Su vehículo 1313MMM está listo para recoger') && correo.includes('157,30'), null);
  await abrir(ana, '/historial');
  async function buscar(m) { await ana.fill('input[name=matricula]', m); await ana.getByRole('button', { name: 'Buscar' }).click(); await ana.waitForTimeout(1000); return texto(ana); }
  t = await buscar('1313 mmm');
  const h1 = ['1313MMM', `Orden ${num}`, `Factura nº ${fnum}`, '85.000 km', 'Pastillas y discos delanteros desgastados', 'PF-123 Pastillas freno'].filter((x) => !t.includes(x));
  const i1 = await captura(ana, 'historial-1313');
  t = await buscar('9999LLL'); const h2 = t.includes('9999LLL') && t.includes('Factura nº'); await captura(ana, 'historial-9999');
  t = await buscar('0000ZZZ'); const h3 = t.includes('No hay historial para esa matrícula');
  anota('§7.5 Historial', '1313MMM con fecha, km, diagnóstico y líneas; 9999LLL facturada; matrícula inexistente → «No hay historial para esa matrícula»', `faltan en 1313=[${h1}]; 9999LLL=${h2}; inexistente=${h3}`,
    h1.length === 0 && h2 && h3, i1);
}

fs.writeFileSync(`${OUT}/resultados-s${SEC}.json`, JSON.stringify(res, null, 2));
await nav.close();
console.log(`\n${res.filter((x) => x.ok).length}/${res.length} comprobaciones superadas`);
