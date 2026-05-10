# Hint: Ping-Pong

Receive from pong (top of stack), drop the flag, and `nip` to
discard the leftover ping reference so the pong value is alone
on top.

```seq
chan.receive drop   # ( ping value )
nip                 # ( value )
```
