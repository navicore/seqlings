# TCP Networking

Seq provides built-in TCP networking for building servers and clients.

## TCP Operations

| Word | Stack Effect | Description |
|------|--------------|-------------|
| `tcp.listen` | `( port -- socket )` | Listen on a port, returns socket |
| `tcp.accept` | `( socket -- client )` | Accept a connection, returns client socket |
| `tcp.read` | `( socket -- string )` | Read data from socket |
| `tcp.write` | `( string socket -- )` | Write data to socket |
| `tcp.close` | `( socket -- )` | Close a socket |

## Server Pattern

```seq
: echo-server ( -- )
    8080 tcp.listen   # Start listening on port 8080
    tcp.accept        # Wait for a connection
    dup tcp.read      # Read from client
    over tcp.write    # Echo back
    tcp.close         # Close client connection
    tcp.close         # Close server socket
;
```

## Concurrency Note

TCP operations work naturally with Seq's strand-based concurrency.
Each connection can be handled in its own strand for concurrent servers.

## Exercises

1. **01-listen.seq** - Start a TCP listener
2. **02-accept.seq** - Accept connections
3. **03-read-write.seq** - Read and write data
4. **04-close.seq** - Close connections properly
5. **05-echo.seq** - Build a simple echo server
