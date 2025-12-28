# Hint: Safe Operations

`receive-safe` returns both value and status.

## Solution

```seq
chan.make
dup 99 chan.send
chan.receive-safe
# Stack now has: ( 99 true )
```
