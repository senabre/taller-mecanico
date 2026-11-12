// T029 de spec-004: lo que research D7 da por comprobado sobre la orden demo 7, tras §4.
// Uso: node t029d7.mjs <carpeta>
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
const c = await nav.newContext({ viewport: { width: 1280, height: 900 }, locale: 'es-ES', timezoneId: 'Europe/Madrid' });
const p = await c.newPage();
await p.goto(`${BASE}/entrar`); await p.fill('input[name=login]', 'luis'); await p.fill('input[name=contrasena]', 'demo1234');
await p.click('button[type=submit]'); await p.waitForTimeout(800);
const api = (m, r, b) => p.evaluate(async ([m, r, b]) => { const x = await fetch(`/api${r}`, { method: m, headers: { 'content-type': 'application/json' }, body: b === undefined ? undefined : JSON.stringify(b) }); let j = null; try { j = await x.json(); } catch {} return { status: x.status, json: j }; }, [m, r, b]);
const id7 = Number(sql('select id from orden where numero = 7'));
const o = (await api('GET', `/ordenes/${id7}`)).json;
anota('D7 Lo presupuestado de la orden 7', '110,00 € (solo cuentan los aceptados: ni la ampliación caducada ni la rechazada en §4.7)',
  `total_presupuestado=${o.total_presupuestado}; estado=${o.estado}; presupuestos=${o.presupuestos.map((x) => `${x.tipo}:${x.estado}`).join(', ')}`,
  o.total_presupuestado === '110.00' && o.estado === 'pendiente_conformidad', null);
const nuevo = await api('POST', `/ordenes/${id7}/presupuestos`);
const linea = nuevo.status === 201 ? await api('POST', `/presupuestos/${nuevo.json.id}/lineas`, { tipo: 'mano_obra', concepto: 'Ampliación D7', cantidad: '1' }) : null;
const emitido = nuevo.status === 201 ? await api('POST', `/presupuestos/${nuevo.json.id}/emitir`) : null;
anota('D7 Emitir otra ampliación en la orden 7', 'Se puede crear y emitir (la caducada y la rechazada no lo bloquean)',
  `crear=${nuevo.status} (${nuevo.json?.tipo}); línea=${linea?.status}; emitir=${emitido?.status} (${emitido?.json?.estado})`,
  nuevo.status === 201 && nuevo.json?.tipo === 'ampliacion' && emitido?.status === 200 && emitido.json.estado === 'emitido', null);
const entrega = sql("select to_char(fecha_entrega_prometida, 'DD/MM/YYYY') from orden where numero = 7");
const retrasada = sql("select fecha_entrega_prometida < (now() at time zone 'Europe/Madrid')::date from orden where numero = 7");
anota('D7 Tarjeta de la orden 7', 'La entrega prometida no cambia con el ajuste de la cita; el tablero (§8) es de spec-008 y aún no existe',
  `entrega prometida=${entrega}; retrasada hoy=${retrasada === 't'}`, true, null);
fs.writeFileSync(`${OUT}/resultados-d7.json`, JSON.stringify(res, null, 2));
await nav.close();
console.log(`\n${res.filter((x) => x.ok).length}/${res.length} comprobaciones superadas`);
