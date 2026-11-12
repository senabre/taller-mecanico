// Matriz de trazabilidad, filas de la Persona B: los criterios que se quedaron sin evidencia.
//  - Cinco criterios de tiempo (003 SC-001, 005 SC-003, 006 SC-001, 007 SC-001 y SC-004). Se hacen con un
//    navegador automático y se mide el tiempo de máquina; NO es un cronometraje humano. Se anotan también
//    los campos y clics que necesita una persona, para valorar si cabe en el tiempo del criterio.
//  - 007 US1-4: el logo de la factura en pantalla y en la vista de impresión, con y sin logo.
// Uso (desde una carpeta con playwright instalado):
//   node t-matriz-b.mjs <carpeta-salida> sinlogo     servidor con TALLER_LOGO apuntando a un archivo que no existe
//   node t-matriz-b.mjs <carpeta-salida> principal   servidor con TALLER_LOGO apuntando a una imagen que existe
// Datos: base aparte `taller_matriz`, recién migrada y con backend/seed/demo.sql cargado el mismo día.
import { chromium } from 'playwright';
import fs from 'node:fs';

const BASE = 'http://127.0.0.1:3000'; // el servidor escucha en 0.0.0.0 (IPv4); «localhost» puede resolverse a ::1
const OUT = process.argv[2];
const FASE = process.argv[3];
fs.mkdirSync(OUT, { recursive: true });
const ficheroResultados = `${OUT}/resultados.json`;
const res = fs.existsSync(ficheroResultados) ? JSON.parse(fs.readFileSync(ficheroResultados, 'utf8')) : [];
const anota = (criterio, esperado, observado, ok, extra = {}) => {
  res.push({ fase: FASE, criterio, esperado, observado, ok, ...extra });
  console.log(`${ok ? 'OK   ' : 'FALLA'} ${criterio}: ${observado}`);
};
const seg = (ms) => `${(ms / 1000).toFixed(2).replace('.', ',')} s`;

const nav = await chromium.launch();
async function sesion(usuario) {
  const c = await nav.newContext({ viewport: { width: 1440, height: 900 }, locale: 'es-ES', timezoneId: 'Europe/Madrid' });
  const p = await c.newPage();
  p.on('dialog', (d) => d.accept());
  await p.goto(`${BASE}/entrar`);
  await p.fill('input[name=login]', usuario);
  await p.fill('input[name=contrasena]', 'demo1234');
  await p.click('button[type=submit]');
  await p.waitForURL((u) => !u.pathname.startsWith('/entrar'));
  return p;
}
// Id de la orden a partir de su número (en la base aparte los ids no coinciden con los números).
async function idDeOrden(p, numero) {
  const lista = await (await p.request.get(`${BASE}/api/ordenes`)).json();
  const o = (Array.isArray(lista) ? lista : lista.ordenes).find((x) => x.numero === numero);
  return o.id;
}

if (FASE === 'sinlogo') {
  // 007 US1-4 (sin logo): TALLER_LOGO configurado, pero el archivo no existe → la factura sale sin logo.
  const ana = await sesion('ana');
  const orden9 = await (await ana.request.get(`${BASE}/api/ordenes/${await idDeOrden(ana, 9)}`)).json();
  await ana.goto(`${BASE}/facturas/${orden9.factura_id}`);
  await ana.getByText(/Factura nº/).first().waitFor();
  const imgs = await ana.locator('main img').count();
  await ana.screenshot({ path: `${OUT}/b01-factura-sin-logo.png`, fullPage: true });
  await ana.emulateMedia({ media: 'print' });
  await ana.screenshot({ path: `${OUT}/b02-impresion-sin-logo.png`, fullPage: true });
  anota('007 US1-4 · sin logo', 'Con TALLER_LOGO apuntando a un archivo que no existe, la factura sale sin logo y sin hueco',
    `Factura nº 1 (9999LLL): ${imgs} imágenes en la página`, imgs === 0, { capturas: ['b01-factura-sin-logo.png', 'b02-impresion-sin-logo.png'] });
}

if (FASE === 'principal') {
  // 006 SC-001 · registrar un pedido (pepe, almacén). Orden Nº 2, «en reparación».
  {
    const pepe = await sesion('pepe');
    const t0 = Date.now();
    await pepe.goto(`${BASE}/pedidos`);
    await pepe.selectOption('select[name=proveedor]', { label: 'Recambios Sur' });
    await pepe.fill('#referencia', 'PF');
    await pepe.getByRole('button', { name: /PF-123/ }).click();
    await pepe.fill('input[name=cantidad]', '1');
    await pepe.fill('input[name=orden]', '2');
    await pepe.getByRole('button', { name: 'Registrar pedido' }).click();
    const aviso = pepe.locator('[role=status]');
    await aviso.waitFor();
    const ms = Date.now() - t0;
    const texto = (await aviso.innerText()).trim();
    await pepe.screenshot({ path: `${OUT}/c01-006-pedido.png`, fullPage: true });
    anota('006 SC-001 · registrar un pedido en menos de 1 minuto',
      'Pedido registrado; una persona necesita 4 campos (proveedor, referencia, cantidad, nº de orden) y 2 clics (elegir la referencia y «Registrar pedido»)',
      `«${texto}» a los ${seg(ms)} de abrir /pedidos (tiempo de máquina)`, ms < 60_000, { ms, capturas: ['c01-006-pedido.png'] });
  }

  // 005 SC-003 · registrar una imputación (luis). Orden Nº 3, «en reparación», 40,00 de 110,00.
  {
    const luis = await sesion('luis');
    const id = await idDeOrden(luis, 3);
    const t0 = Date.now();
    await luis.goto(`${BASE}/ordenes/${id}`);
    const form = luis.locator('form', { has: luis.locator('h3', { hasText: 'Nueva imputación' }) });
    await form.locator('input[name=concepto]').fill('Revisión de frenos');
    await form.locator('input[name=horas]').fill('0,25');
    await form.getByRole('button', { name: /imput/i }).click();
    await luis.getByText(/Acumulado 50,00/).waitFor();
    const ms = Date.now() - t0;
    await luis.screenshot({ path: `${OUT}/c02-005-imputacion.png`, fullPage: true });
    anota('005 SC-003 · registrar una imputación en menos de 30 segundos',
      'La imputación queda y el acumulado pasa de 40,00 a 50,00 €; una persona necesita 2 campos (concepto y horas) y 1 clic',
      `«Acumulado 50,00 € de 110,00 €» a los ${seg(ms)} de abrir la ficha (tiempo de máquina)`, ms < 30_000, { ms, capturas: ['c02-005-imputacion.png'] });
    await luis.context().close();
  }

  // 003 SC-001 · preparar un presupuesto de 5 líneas (luis). Orden Nº 4, «en diagnóstico», sin presupuesto.
  {
    const luis = await sesion('luis');
    const id = await idDeOrden(luis, 4);
    const t0 = Date.now();
    await luis.goto(`${BASE}/ordenes/${id}`);
    await luis.getByRole('button', { name: 'Nuevo presupuesto' }).click();
    const form = luis.locator('form', { has: luis.locator('h3', { hasText: 'Añadir línea' }) });
    await form.waitFor();
    const manoDeObra = [['Diagnosis con máquina', '0,5'], ['Sustitución del sensor de oxígeno', '1'], ['Borrado de averías', '0,25'], ['Prueba en carretera', '0,5']];
    for (const [concepto, horas] of manoDeObra) {
      await form.locator('select').selectOption('mano_obra');
      await form.locator('input[name=concepto]').fill(concepto);
      await form.locator('input[name=horas]').fill(horas);
      await form.getByRole('button', { name: 'Añadir línea' }).click();
      await luis.getByRole('cell', { name: concepto }).waitFor();
    }
    await form.locator('select').selectOption('pieza');
    await form.locator('#pieza').fill('PF');
    await form.getByRole('button', { name: /PF-123/ }).click();
    await form.locator('input[name=unidades]').fill('2');
    await form.getByRole('button', { name: 'Añadir línea' }).click();
    await luis.getByRole('cell', { name: 'PF-123 Pastillas freno' }).waitFor();
    const ms = Date.now() - t0;
    const totales = (await luis.locator('section', { has: luis.locator('h2', { hasText: 'Presupuesto' }) }).innerText()).replace(/\s+/g, ' ');
    const total = (totales.match(/Total ([\d.,]+ €)/) || [])[1] ?? '?';
    await luis.screenshot({ path: `${OUT}/c03-003-presupuesto-5-lineas.png`, fullPage: true });
    anota('003 SC-001 · preparar un presupuesto de 5 líneas en menos de 5 minutos',
      '5 líneas (4 de mano de obra y 1 de pieza) con sus totales; una persona necesita 1 clic para crearlo y, por línea, 2 campos y 1 clic (la pieza: buscar, elegir y unidades)',
      `5 líneas a los ${seg(ms)} de abrir la ficha, total ${total} (tiempo de máquina)`, ms < 300_000, { ms, capturas: ['c03-003-presupuesto-5-lineas.png'] });
    await luis.context().close();
  }

  // 007 SC-001 · emitir una factura (ana). Orden Nº 8, «terminada». Después, 007 US1-4 con logo.
  {
    const ana = await sesion('ana');
    const id = await idDeOrden(ana, 8);
    const t0 = Date.now();
    await ana.goto(`${BASE}/ordenes/${id}`);
    await ana.getByRole('button', { name: 'Facturar' }).click();
    const linea = ana.getByText(/Factura nº \d+ ·/);
    await linea.waitFor();
    const ms = Date.now() - t0;
    const texto = (await linea.innerText()).replace(/\s+/g, ' ').trim();
    await ana.screenshot({ path: `${OUT}/c04-007-factura.png`, fullPage: true });
    anota('007 SC-001 · emitir una factura en menos de 30 segundos sin escribir nada',
      'Factura emitida; una persona necesita 2 clics («Facturar» y aceptar la confirmación) y ningún campo',
      `«${texto}» a los ${seg(ms)} de abrir la ficha (tiempo de máquina)`, ms < 30_000, { ms, capturas: ['c04-007-factura.png'] });

    await ana.getByRole('link', { name: 'Ver e imprimir' }).click();
    await ana.getByText(/Factura nº/).first().waitFor();
    const logo = ana.locator('main img');
    await logo.first().waitFor();
    const n = await logo.count();
    const src = await logo.first().getAttribute('src');
    const alt = await logo.first().getAttribute('alt');
    const cargada = await logo.first().evaluate((i) => i.complete && i.naturalWidth > 0);
    await ana.screenshot({ path: `${OUT}/b03-factura-con-logo.png`, fullPage: true });
    await ana.emulateMedia({ media: 'print' });
    const visibleAlImprimir = await logo.first().isVisible();
    await ana.screenshot({ path: `${OUT}/b04-impresion-con-logo.png`, fullPage: true });
    anota('007 US1-4 · con logo', 'Con TALLER_LOGO apuntando a una imagen que existe, el logo sale en la cabecera, también al imprimir',
      `${n} imagen, src=«${src}», alt=«${alt}», cargada=${cargada}, visible al imprimir=${visibleAlImprimir}`,
      n === 1 && cargada && visibleAlImprimir, { capturas: ['b03-factura-con-logo.png', 'b04-impresion-con-logo.png'] });
    await ana.emulateMedia({ media: 'screen' });

    // 007 SC-004 · consultar el historial escribiendo la matrícula.
    const t1 = Date.now();
    await ana.goto(`${BASE}/historial`);
    await ana.fill('input[name=matricula]', '9999 lll');
    await ana.getByRole('button', { name: 'Buscar' }).click();
    await ana.getByText(/Factura nº/).first().waitFor();
    const ms2 = Date.now() - t1;
    const cabecera = (await ana.locator('main h2').first().innerText()).replace(/\s+/g, ' ');
    await ana.screenshot({ path: `${OUT}/c05-007-historial.png`, fullPage: true });
    anota('007 SC-004 · consultar el historial en menos de 10 segundos',
      'Sale la reparación facturada de 9999LLL; una persona necesita 1 campo (la matrícula) y 1 clic',
      `«${cabecera}» a los ${seg(ms2)} de abrir /historial (tiempo de máquina)`, ms2 < 10_000, { ms: ms2, capturas: ['c05-007-historial.png'] });
  }
}

fs.writeFileSync(ficheroResultados, JSON.stringify(res, null, 2));
await nav.close();
