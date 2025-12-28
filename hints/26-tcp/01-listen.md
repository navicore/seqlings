# Hint: Starting a TCP Listener

`tcp.listen` takes a port number and returns a socket handle:

```seq
8080 tcp.listen   # ( -- socket )
```

The socket is used with `tcp.accept` to get client connections.
