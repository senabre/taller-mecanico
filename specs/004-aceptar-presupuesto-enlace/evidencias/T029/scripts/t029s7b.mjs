// Repite solo las comprobaciones §7.3 (menú al imprimir) y §7.4 (asunto del correo) sobre el mismo estado,
// tras corregir dos fallos del script (no de la app). Uso: node t029s7b.mjs <carpeta> <carpeta de correos>
import { chromium } from 'playwright';
import fs from 'node:fs';
import { execSync } from 'node:child_process';
const [OUT, CORREO] = process.argv.slice(2);
const sql = (q) => execSync(`psql -h localhost -U taller -d taller -tAc "${q}"`).toString().trim();
const res = []; const anota = (paso, esperado, observado, ok, img) => { res.push({ paso, esperado, observado, ok, img }); console.log(`${ok ? 'OK   ' : 'FALLA'} ${paso}: ${observado}`); };
const nav = await chromium.launch();
const p = await (await nav.newContext({ viewport: { width: 1280, height: 900 }, locale: 'es-ES', timezoneId: 'Europe/Madrid' })).newPage();
await p.goto('http://localhost:3000/entrar'); await p.fill('input[name=login]', 'ana'); await p.fill('input[name=contrasena]', 'demo1234'); await p.click('button[type=submit]'); await p.waitForTimeout(800);
const fid = sql("select f.id from factura f join orden o on o.id = f.orden_id join cita c on c.id = o.cita_id join vehiculo v on v.id = c.vehiculo_id where v.matricula = '1313MMM'");
await p.goto(`http://localhost:3000/facturas/${fid}`); await p.waitForTimeout(1000);
const enPantalla = await p.getByRole('button', { name: 'Salir' }).isVisible();
await p.emulateMedia({ media: 'print' });
const alImprimir = await p.getByRole('button', { name: 'Salir' }).isVisible();
await p.screenshot({ path: `${OUT}/s7b-factura-impresion.png`, fullPage: true });
anota('§7.3 Al imprimir, sin menú (repetido)', 'El menú de la app se ve en pantalla y no al imprimir', `menú en pantalla=${enPantalla}; al imprimir=${alImprimir}; imágenes=${await p.locator('main img').count()}`, enPantalla && !alImprimir, 's7b-factura-impresion.png');
let txt = null;
for (const f of fs.readdirSync(CORREO).sort().reverse()) {
  const t = fs.readFileSync(`${CORREO}/${f}`, 'utf8').replace(/\?=\r?\n[ \t]+/g, '?= ').replace(/\r?\n[ \t]+/g, ' ').replace(/=\r?\n/g, '')
    .replace(/=\?utf-8\?b\?([^?]*)\?=/gi, (_, b) => Buffer.from(b, 'base64').toString('utf8'))
    .replace(/(=[0-9A-F]{2})+/g, (m) => Buffer.from(m.split('=').slice(1).map((h) => parseInt(h, 16))).toString('utf8'));
  if (t.includes('157,30')) { txt = t; break; }
}
const asunto = txt?.match(/^Subject: (.*)$/m)?.[1];
anota('§7.4 Correo al cliente (repetido)', '«Su vehículo 1313MMM está listo para recoger», con el total', `asunto=«${asunto}»; total=${!!txt && txt.includes('157,30')}`,
  asunto === 'Su vehículo 1313MMM está listo para recoger' && txt.includes('157,30'), null);
fs.writeFileSync(`${OUT}/resultados-s7b.json`, JSON.stringify(res, null, 2));
await nav.close();
