# 加入房间 (b'01;A;axious')
```json
{
    "op": "join",
    "room": "A",
    "name": "p1"
}
```

# 查询房间 (02;rooms)
```json
{
    "op": "query",
    "payload": "rooms"
}
```


# 开始状态 (03;A;1)
```json
{
    "op": "update",
    "room": "A",
    "status": 1
}
```

