#!/bin/bash
S=/tmp/claude-0/-home-claude/25f19f03-592c-5f7c-94f0-51da21d8588e/scratchpad/e2e
cd $S/wt004/backend && setsid nohup ./target/debug/taller > $S/server004.log 2>&1 < /dev/null &
echo $! > $S/servidor.pid
