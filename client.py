import asyncio
import websockets


async def connect():
    uri = "ws://127.0.0.1:10000"
    async with websockets.connect(uri) as websocket:
        await websocket.send(b'0;axious')
        await asyncio.sleep(1)
        await websocket.send(b'1;1')
        
        while True:
            recv = await websocket.recv()
            print(recv)

asyncio.run(connect())