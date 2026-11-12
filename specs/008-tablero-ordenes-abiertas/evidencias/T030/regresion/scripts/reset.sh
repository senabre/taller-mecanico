#!/bin/bash
S=/tmp/claude-0/-home-claude/25f19f03-592c-5f7c-94f0-51da21d8588e/scratchpad/e2e
pg_isready -h /tmp -q || { rm -f /tmp/pgdata/postmaster.pid; su postgres -c "/usr/lib/postgresql/16/bin/pg_ctl -D /tmp/pgdata -l /tmp/pgdata/log -o '-p 5432 -k /tmp' start" >/dev/null; sleep 2; }
/tmp/claude-0/-home-claude/25f19f03-592c-5f7c-94f0-51da21d8588e/scratchpad/e2e/tester/parar.sh; sleep 1
cd $S/wt004/backend
psql -h /tmp -U postgres -q -c "DROP DATABASE IF EXISTS taller" -c "CREATE DATABASE taller OWNER taller"
for f in migrations/*.sql; do psql -h localhost -U taller -d taller -q -v ON_ERROR_STOP=1 -f $f; done
psql -h localhost -U taller -d taller -q -v ON_ERROR_STOP=1 -f seed/demo.sql
rm -f $S/correo/*
/tmp/claude-0/-home-claude/25f19f03-592c-5f7c-94f0-51da21d8588e/scratchpad/e2e/tester/arrancar.sh
sleep 4
