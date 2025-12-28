# Hint: Closing Channels

Closing doesn't discard pending values.

## Solution

```seq
chan.make
dup 42 chan.send
dup chan.close
chan.receive   # Still gets 42
```
