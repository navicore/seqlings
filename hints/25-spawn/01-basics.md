# Hint: Spawning

Receive from the channel. Remember `chan.receive` returns `(value Bool)` and you have an extra channel on stack:
```seq
chan.receive drop   # ( ch ch ) -> ( ch value )
nip                 # ( value )
```
