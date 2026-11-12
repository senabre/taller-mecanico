#!/bin/bash
# Base limpia, servidor nuevo y una comprobación en el navegador.
# Uso: ejecutar.sh <script.mjs> <fase>
S=$(cd "$(dirname "$0")" && pwd)
taskkill //IM taller.exe //F > /dev/null 2>&1
sleep 1
bash "$S/base.sh" taller_c1 > /dev/null 2>&1 || { echo "no se pudo crear la base"; exit 1; }
nohup bash "$S/servidor.sh" taller_c1 "$S/servidor-$2.log" > /dev/null 2>&1 &
for i in $(seq 1 20); do curl -s -o /dev/null http://127.0.0.1:3000/entrar && break; sleep 1; done
cd "$S" && node "$1" "$S/salida" "$2" 2>&1 | grep -E "^(OK|FALLA)|correctas|Error"
taskkill //IM taller.exe //F > /dev/null 2>&1
