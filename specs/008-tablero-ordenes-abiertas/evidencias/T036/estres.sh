#!/usr/bin/env bash
# Comprobación manual del hallazgo V3 de spec-008 (adaptación de evidencias/T030/estres.sh).
# Entra como paco y lanza 200 peticiones a /api/tablero que se cortan a los 0,5–4,5 ms, como un
# navegador que cambia de pantalla; después cuenta las conexiones de la base que quedan dentro de
# una transacción, entra 10 veces como ana y cuenta los errores nuevos del registro del servidor.
#
# Requisitos: la app en marcha (`cargo run` en backend/, con su salida en un fichero), Docker con
# PostgreSQL (docker compose up -d) y los datos demo cargados.
# Uso, desde cualquier carpeta del repositorio:
#   CONTRASENA_DEMO=<la del quickstart> specs/008-tablero-ordenes-abiertas/evidencias/T036/estres.sh <registro del servidor>
# La contraseña demo no se escribe aquí (constitución, principio V): se lee de CONTRASENA_DEMO.
set -u
REGISTRO=${1:?Indica el fichero con la salida del servidor (cargo run > fichero 2>&1)}
: "${CONTRASENA_DEMO:?Pon en CONTRASENA_DEMO la contraseña demo del quickstart}"
BASE=${BASE:-http://localhost:3000}
RAIZ=$(git rev-parse --show-toplevel)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

psql_demo() {
  docker compose -f "$RAIZ/docker-compose.yml" exec -T postgres \
    sh -c 'psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -tAc "$0"' "$1"
}
errores() { sed 's/\x1b\[[0-9;]*m//g' "$REGISTRO" | grep ERROR | grep -vc on_failure; }
entrar() {
  curl -s -c "$2" -H 'content-type: application/json' \
    -d "{\"login\":\"$1\",\"contrasena\":\"$CONTRASENA_DEMO\"}" "$BASE/api/sesion" -o /dev/null -w '%{http_code}'
}

[ "$(entrar paco "$TMP/paco")" = 204 ] || { echo "No se pudo entrar como paco"; exit 1; }
antes=$(errores)
for r in 1 2 3 4 5 6 7 8; do
  for i in $(seq 1 25); do
    t=$(awk -v s=$RANDOM 'BEGIN{srand(s); printf "%.4f", 0.0005+rand()*0.004}')
    (curl -s -b "$TMP/paco" --max-time "$t" "$BASE/api/tablero" -o /dev/null 2>&1 &)
  done
  sleep 0.5
done
sleep 2
echo "conexiones: $(psql_demo "select string_agg(coalesce(state, '-') || '=' || n, ', ') from (select state, count(*) n from pg_stat_activity where datname = current_database() and pid <> pg_backend_pid() group by state) x")"
echo "en transacción: $(psql_demo "select count(*) from pg_stat_activity where datname = current_database() and state like 'idle in transaction%'")"
echo -n "10 inicios de sesión de ana: "
for i in $(seq 1 10); do echo -n "$(entrar ana "$TMP/ana") "; done; echo
echo "errores nuevos en el servidor: $(( $(errores) - antes ))"
