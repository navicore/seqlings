# Hint: Accepting Connections

`tcp.accept` blocks until a client connects:

```seq
server-socket tcp.accept   # ( socket -- client-socket )
```

Each accepted connection gives you a new client socket.
