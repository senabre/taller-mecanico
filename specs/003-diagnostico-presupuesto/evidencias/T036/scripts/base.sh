#!/bin/bash
# Base aparte para las comprobaciones en el navegador: recién migrada y con los datos demo de hoy.
# Uso: base.sh <nombre-de-la-base>
set -e
BD=$1
REPO=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
PG=tallermecanico-postgres-1
U=$(docker exec $PG printenv POSTGRES_USER)
docker exec $PG psql -q -U "$U" -d postgres -c "DROP DATABASE IF EXISTS $BD WITH (FORCE)" -c "CREATE DATABASE $BD OWNER \"$U\""
for f in "$REPO"/backend/migrations/*.sql "$REPO"/backend/seed/demo.sql; do
  docker exec -i $PG psql -q -v ON_ERROR_STOP=1 -U "$U" -d $BD < "$f"
done
echo "base $BD lista"
