// T039 de spec-008, día A del §9: el cliente de 3579KLM pide cita en /cita, sin entrar, para el primer
// día que ofrezca la página. Se hace sobre la base tal como quedó al terminar §0–§8 (T030).
// Uso: node t039a.mjs <carpeta>
import { chromium } from 'playwright';
import fs from 'node:fs';
import { execSync } from 'node:child_process';

const BASE = 'http://localhost:3000';
const OUT = process.argv[2];
fs.mkdirSync(OUT, { recursive: true });
const res = [];
const sql = (q) => execSync(`psql -h localhost -U taller -d taller -tAc "${q}"`).toString().trim();
const anota = (paso, esperado, observado, ok, img) => { res.push({ paso, esperado, observado, ok, img }); console.log(`${ok ? 'OK   ' : 'FALLA'} ${paso}: ${observado}`); };
const nav = await chromium.launch();

// El tablero de paco antes de la cita, para comprobar que la cita no lo cambia.
const paco = await (await nav.newContext({ viewport: { width: 1920, height: 1080 }, locale: 'es-ES', timezoneId: 'Europe/Madrid' })).newPage();
await paco.goto(`${BASE}/entrar`); await paco.fill('input[name=login]', 'paco'); await paco.fill('input[name=contrasena]', 'demo1234');
await paco.click('button[type=submit]'); await paco.waitForTimeout(800);
await paco.goto(`${BASE}/tablero`); await paco.waitForTimeout(1200);
const tableroAntes = await paco.locator('main').innerText();
await paco.screenshot({ path: `${OUT}/a00-tablero-antes.png`, fullPage: true });

const p = await (await nav.newContext({ viewport: { width: 1280, height: 900 }, locale: 'es-ES', timezoneId: 'Europe/Madrid' })).newPage();
await p.goto(`${BASE}/cita`);
await p.locator('select[name=fecha]').waitFor();
await p.fill('input[name=matricula]', '3579KLM'); await p.locator('input[name=matricula]').blur(); await p.waitForTimeout(500);
const pide = await Promise.all(['marca', 'modelo', 'nombre', 'telefono', 'email'].map((k) => p.locator(`input[name=${k}]`).count()));
for (const [k, v] of Object.entries({ marca: 'Seat', modelo: 'Arona', nombre: 'Laura Gómez', telefono: '611 223 344', email: 'laura.gomez@ejemplo.test' })) await p.fill(`input[name=${k}]`, v);
const opciones = await p.$$eval('select[name=fecha] option', (os) => os.filter((o) => o.value).map((o) => ({ valor: o.value, texto: o.textContent.trim() })));
const primero = opciones[0];
await p.selectOption('select[name=fecha]', primero.valor);
await p.fill('textarea[name=motivo]', 'Revisión de frenos');
await p.screenshot({ path: `${OUT}/a01-formulario.png`, fullPage: true });
await p.click('button[type=submit]'); await p.waitForTimeout(900);
const t = (await p.locator('main').innerText()).replace(/ /g, ' ');
await p.screenshot({ path: `${OUT}/a02-cita-confirmada.png`, fullPage: true });
const enBase = sql("select to_char(c.fecha, 'YYYY-MM-DD') || '|' || c.motivo from cita c join vehiculo v on v.id = c.vehiculo_id where v.matricula = '3579KLM'");
anota('§9.1 Pedir cita para 3579KLM (día A)', 'Pide los datos del vehículo y del cliente; con el primer día que ofrece la página se confirma',
  `pide datos=${pide.join(',')}; primer día ofrecido=${primero.valor} («${primero.texto}»); confirmada=${t.includes('Cita confirmada')}; en la base=${enBase}`,
  pide.every((x) => x === 1) && t.includes('Cita confirmada') && enBase.startsWith(primero.valor), 'a01-formulario.png, a02-cita-confirmada.png');

await paco.waitForTimeout(2000);
const tableroDespues = await paco.locator('main').innerText();
await paco.screenshot({ path: `${OUT}/a03-tablero-despues.png`, fullPage: true });
anota('§9.1 El tablero no cambia con la cita', 'Las citas no salen en el tablero', `igual=${tableroAntes === tableroDespues}`, tableroAntes === tableroDespues, 'a00-tablero-antes.png, a03-tablero-despues.png');

fs.writeFileSync(`${OUT}/dia-b.txt`, primero.valor + '\n');
fs.writeFileSync(`${OUT}/resultados-a.json`, JSON.stringify(res, null, 2));
await nav.close();
console.log(`\nDía B: ${primero.valor}\n${res.filter((x) => x.ok).length}/${res.length} comprobaciones superadas`);
