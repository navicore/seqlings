# TCP Networking

Seq has full-duplex TCP: server (`listen`, `accept`) and client
(`connect`), plus the obvious `read`, `write`, `close`. Every word
ends with a success Bool so the caller decides how to handle
failure.

## TCP Operations

| Word | Stack Effect | Description |
|------|--------------|-------------|
| `net.tcp.listen`  | `( Int -- Socket Bool )`           | Listen on a port |
| `net.tcp.connect` | `( String Int -- Socket Bool )`    | Connect to host:port |
| `net.tcp.accept`  | `( Socket -- Socket Bool )`        | Accept a connection (parks) |
| `net.tcp.read`    | `( Socket -- String Bool )`        | Read from a socket (parks) |
| `net.tcp.write`   | `( String Socket -- Bool )`        | Write to a socket |
| `net.tcp.close`   | `( Socket -- Bool )`               | Close a socket |

Note: `accept`, `read`, and `write` do **not** consume the socket
they operate on — the underlying handle stays open and the integer
stays on the stack via `dup`/`over` so you can use the socket again.

## The loopback testing pattern

This chapter does not ask you to trust that the API works — it asks
you to use it. Every exercise from 02 onward runs a real client and
a real server inside the same program. That works because Seq is
cooperative: `strand.spawn` runs the spawned strand alongside main,
and every `net.tcp.*` parking point gives the scheduler a chance to
switch strands.

The canonical shape:

```seq
: connector ( -- )
    "127.0.0.1" <port> net.tcp.connect
    [ ... do client work ... net.tcp.close drop ]
    [ drop ]
    if
;

: test-... ( -- )
    <port> net.tcp.listen
    [ # ( server )
      [ connector ] strand.spawn drop
      dup net.tcp.accept
      [ # ( server client )
        ... server work ...
        net.tcp.close drop      # close client
        net.tcp.close drop      # close server
      ]
      [ drop net.tcp.close drop ]
      if
    ]
    [ drop ]
    if
;
```

Main listens *before* spawning so the connector cannot find the
listener absent. Main then parks in `accept`, which yields to the
connector. The connector calls `net.tcp.connect`, the kernel
completes the handshake, accept unparks, the protocol proceeds.

## A note on ports

Each exercise uses a different fixed port (18261 through 18265) so
that a re-run does not collide with a still-cooling TIME_WAIT slot
from the previous test. The patch-seq TCP API does not yet return
the OS-assigned port for `listen 0`, so fixed ports are the
practical choice.

## Exercises

1. **01-listen.seq** — Open and close a listener (no spawn needed).
2. **02-accept.seq** — Spawn a connector, accept the connection.
3. **03-read-write.seq** — Connector sends "hello"; server reads
   and asserts.
4. **04-close.seq** — Detect peer close. Three-way handling of
   `net.tcp.read`'s outcome: data, EOF, error.
5. **05-echo.seq** — Capstone. Echo server in a spawned strand,
   client in main, full round-trip asserted.
