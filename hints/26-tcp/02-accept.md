# Hint: Accepting Connections

The full pattern:

```seq
: connector ( -- )
    "127.0.0.1" 18262 net.tcp.connect
    [ net.tcp.close drop ]
    [ drop ]
    if
;

: test-accept ( -- )
    18262 net.tcp.listen
    [ # success: ( server )
      [ connector ] strand.spawn drop
      dup net.tcp.accept
      [ # success: ( server client )
        net.tcp.close drop      # close client
        net.tcp.close drop      # close server
        true
      ]
      [ # accept failed: ( server client-junk )
        drop                    # drop junk client
        net.tcp.close drop      # close server anyway
        false
      ]
      if
    ]
    [ drop false ]              # listen failed
    if
    test.assert
;
```

## Why this works

- `net.tcp.listen` runs on the main strand and is non-blocking; it
  registers the socket with the kernel and immediately returns.
- `strand.spawn` queues the connector but doesn't run it yet. We
  drop the returned strand-id; we don't need it.
- `dup net.tcp.accept` parks the main strand. The runtime now has
  no runnable strand on the main carrier — the connector gets to
  run, calls `net.tcp.connect("127.0.0.1", 18262)`, and the kernel
  completes the handshake. accept unparks with the client socket.
- Each side closes what it owns: the connector closes its end, the
  main strand closes both the accepted client and the listener.

## Why two closes in the success branch

`net.tcp.accept` does **not** consume the server socket — that's
why we `dup`'d it. After accept the stack is `( server client )`.
`net.tcp.close drop` closes the top (client). Another
`net.tcp.close drop` closes the next (server).
