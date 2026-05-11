# Hint: Starting a TCP Listener

`net.tcp.listen` takes a port number and returns a socket handle:

```seq
8080 net.tcp.listen   # ( -- socket )
```

The socket is used with `net.tcp.accept` to get client connections.
