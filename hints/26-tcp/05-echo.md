# Hint: Real Echo Server

The client side that goes in `test-echo`:

```seq
"127.0.0.1" 18265 net.tcp.connect
[ # success: ( client )
  "ping" over net.tcp.write drop
  dup net.tcp.read
  [ # ( client data )
    "ping" test.assert-eq-str
  ]
  [ drop ]
  if
  net.tcp.close drop
]
[ # connect failed
  drop false test.assert
]
if
```

## Why two `dup`/`over` choices

- `"ping" over net.tcp.write`: write takes `( data socket )` and we
  have `( client )`; `over "ping"` would push "ping" on top with
  client now beneath, but `over` after "ping" copies the client up
  to be on top, giving `( client "ping" client )`... actually `over`
  here takes `( client "ping" )` and produces `( client "ping" client )`.
  Then `net.tcp.write` consumes `( "ping" client )` from the top
  and leaves `( client Bool )`. `drop` clears the Bool.
- `dup net.tcp.read`: read takes `( socket )` and produces
  `( data Bool )`, consuming the socket. `dup` first preserves the
  client for the eventual close.

## The full chapter composition

This exercise puts together:

- **Chapter 24 (channels)** — not used directly, but the same
  cooperative scheduling model.
- **Chapter 25 (spawn)** — `strand.spawn` queues the server so it
  runs cooperatively against main.
- **Chapter 26 (TCP)** — every operation from 01-04: listen, accept,
  read, write, close, with proper Bool handling.

A real echo server in 16 lines, no threads, no callbacks.
