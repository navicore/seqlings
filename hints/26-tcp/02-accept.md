# Hint: Accepting Connections

`net.tcp.accept` blocks until a client connects:

```seq
server-socket net.tcp.accept   # ( socket -- client-socket )
```

Each accepted connection gives you a new client socket.
