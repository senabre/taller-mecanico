#!/bin/bash
# Lanza 200 peticiones a /api/tablero que se cortan a los 0,5–4,5 ms y cuenta las conexiones que quedan dentro de una transacción.
cd /tmp/claude-0/-home-claude/25f19f03-592c-5f7c-94f0-51da21d8588e/scratchpad/e2e
curl -s -c cj -H 'content-type: application/json' -d '{"login":"paco","contrasena":"demo1234"}' localhost:3000/api/sesion -o /dev/null
for r in 1 2 3 4 5 6 7 8; do
  for i in $(seq 1 25); do t=$(awk -v s=$RANDOM 'BEGIN{srand(s); printf "%.4f", 0.0005+rand()*0.004}'); (curl -s -b cj --max-time $t localhost:3000/api/tablero -o /dev/null 2>&1 &) ; done
  sleep 0.5
done
sleep 2
echo "conexiones: $(psql -h localhost -U taller -d taller -tAc "select string_agg(state || '=' || n, ', ') from (select state, count(*) n from pg_stat_activity where datname='taller' and pid <> pg_backend_pid() group by state) x")"
echo -n "10 inicios de sesión: "; for i in $(seq 1 10); do curl -s -H 'content-type: application/json' -d '{"login":"ana","contrasena":"demo1234"}' localhost:3000/api/sesion -o /dev/null -w "%{http_code} "; done; echo
echo "errores en el servidor: $(sed 's/\x1b\[[0-9;]*m//g' server004.log | grep ERROR | grep -vc on_failure)"
