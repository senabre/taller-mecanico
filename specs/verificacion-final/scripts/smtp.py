import asyncio, sys, time, os
from aiosmtpd.controller import Controller
D = sys.argv[1]
class H:
    async def handle_DATA(self, server, session, envelope):
        n = f"{D}/{time.time():.6f}.eml"
        open(n, 'wb').write(envelope.content)
        return '250 OK'
c = Controller(H(), hostname='127.0.0.1', port=1025)
c.start()
asyncio.get_event_loop().run_forever()
