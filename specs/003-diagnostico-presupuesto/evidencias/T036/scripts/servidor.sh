#!/bin/bash
# Arranca el servidor compilado contra la base aparte. Lee el resto de la configuración del .env.
# Uso: servidor.sh <nombre-de-la-base> <fichero-de-log>
REPO=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
URL=$(grep '^DATABASE_URL=' "$REPO/.env" | cut -d= -f2-)
export DATABASE_URL="${URL%/*}/$1"
cd "$REPO/backend" && exec ./target/debug/taller.exe > "$2" 2>&1
