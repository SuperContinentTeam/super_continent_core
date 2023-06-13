import json
import os
import socket
import threading

client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect(("127.0.0.1", 55555))

name = os.urandom(6).hex()
data = json.dumps({"op": "join", "body": {"name": name, "room": "localhost"}})

client.send(f"{data}\n".encode())

while recv := client.recv(65535):
    print(recv.decode())


def receiver():
    while recv := client.recv(65535):
        print(recv.decode())


def sender():
    while i := input("> "):
        client.send(f"{i}\n".encode())


t1 = threading.Thread(target=receiver)
t2 = threading.Thread(target=sender)

t1.start()
t2.start()

t1.join()
t2.join()
