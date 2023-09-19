import sys

import asyncio
import websockets

name = sys.argv[1]

async def start(ws):
    await asyncio.sleep(5)
    await ws.send(b"3;1")


async def connect():
    uri = "ws://127.0.0.1:10000"
    async with websockets.connect(uri) as websocket:
        await websocket.send(f'0;{name}'.encode())
        await asyncio.sleep(1)
        await websocket.send(b'1;1')

        # if name == "admin":
        asyncio.create_task(start(websocket))
        
        while True:
            try:
                recv = await websocket.recv()
                print(recv.decode())
            except Exception as e:
                _ = e
                break

asyncio.run(connect())