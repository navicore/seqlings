# Hint: Ping-Pong

Receive from the pong channel. Remember `chan.receive` returns `(value Bool)` and you have two channels on stack:
```seq
chan.receive drop   # ( ping value ) - receive from pong, drop Bool
nip                 # ( value ) - remove ping channel
```
