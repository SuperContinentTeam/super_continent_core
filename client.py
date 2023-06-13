import json
import os
import socket
import random

client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect(("127.0.0.1", 55555))

name = os.urandom(6).hex()

data = json.dumps({"op": "join", "body": {"name": name, "room": "localhost"}})

client.send(f"{data}\n".encode())

while recv := client.recv(65535):
    print(recv.decode())
