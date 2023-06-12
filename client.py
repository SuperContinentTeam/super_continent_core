import json
import os
import socket

client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect(("127.0.0.1", 55555))

while (i := input("> ")):
    client.send(f"{i}\n".encode())
    recv = client.recv(65535)
    print("接收消息: ", recv.decode())
