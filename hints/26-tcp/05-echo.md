# Hint: Real Echo Server

Remember the role split: `echo-server` is the SERVER (already
written for you, runs in the spawned strand). The block you need
to write is the CLIENT side, and it lives on the MAIN strand
inside `test-echo`, between the spawn and the `Do not edit` line.

```seq
"127.0.0.1" 18265 net.tcp.connect
[ # connect ok:  ( listener client )
  "ping" over net.tcp.write drop
  dup net.tcp.read
  [ # read ok:  ( listener client data )
    "ping" test.assert-eq-str
  ]
  [ drop ]
  if
  net.tcp.close drop      # close the connect socket
]
[ # connect failed
  drop false test.assert
]
if
```

Final stack must be `( listener )` so the listener-close below fires.

## Why main closes the listener and the spawned strand does not

Sockets in Seq are Int handles into a shared kernel socket table.
When a strand is spawned, copy-on-spawn duplicates the *integer*
into the new strand's stack — but both strands now point at the
same kernel handle. If both call `net.tcp.close` on it, the second
close fails (or worse, closes a different socket if the fd has been
reassigned).

The cleanest rule: *one owner per kernel handle*. Main listened, so
main owns the listener; main closes it. The spawned strand received
a copy as input but doesn't own it — it uses the listener to accept
exactly one connection, then drops its copy without closing. The
accepted client socket is a new kernel handle owned by the spawned
strand; *that* strand closes it.

## Why one connection and not a loop

This is a test. It asserts exactly one thing — that the echo
round-tripped — so the server only needs to handle one client. A
production echo server would wrap `accept ... close-client` in a
recursive call so the listener keeps accepting until the program
shuts down. The shape is the same; just add the loop.

## The full chapter composition

This exercise puts together:

- **Chapter 24 (channels)** — not used directly, but the same
  cooperative scheduling model.
- **Chapter 25 (spawn)** — `strand.spawn` queues the server so it
  runs cooperatively against main.
- **Chapter 26 (TCP)** — every operation from 01-04: listen, accept,
  read, write, close, with proper Bool handling.

A real echo server in 16 lines, no threads, no callbacks.
