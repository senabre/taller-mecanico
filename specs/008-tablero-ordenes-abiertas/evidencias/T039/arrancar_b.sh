#!/bin/bash
# Arranca el servidor con el reloj en el día B del §9 (18/11/2026), con libfaketime.
S=/tmp/claude-0/-home-claude/25f19f03-592c-5f7c-94f0-51da21d8588e/scratchpad/e2e
cd $S/wt004/backend && LD_PRELOAD=/usr/lib/x86_64-linux-gnu/faketime/libfaketimeMT.so.1 FAKETIME='@2026-11-18 08:27:00' setsid nohup ./target/debug/taller > $S/server004.log 2>&1 < /dev/null &
