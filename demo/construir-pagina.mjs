// Construye demo/index.html a partir de plantilla.html y capitulos.json (que deja grabar-demo.mjs).
// Los capítulos van dentro de la página para que funcione abriéndola desde el zip, sin servidor.
// Uso: node demo/construir-pagina.mjs
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const aqui = path.dirname(fileURLToPath(import.meta.url));
const capitulos = fs.readFileSync(path.join(aqui, 'capitulos.json'), 'utf8');
const plantilla = fs.readFileSync(path.join(aqui, 'plantilla.html'), 'utf8');
const seguro = JSON.stringify(JSON.parse(capitulos)).replace(/</g, '\\u003c');
fs.writeFileSync(path.join(aqui, 'index.html'), plantilla.replace('/*CAPITULOS*/', seguro));
console.log('demo/index.html construida');
