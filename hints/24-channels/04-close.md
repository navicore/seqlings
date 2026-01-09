# Hint: Closing Channels

Closing doesn't discard pending values - they can still be received.

## Solution

```seq
chan.make
dup 42 swap chan.send drop   # send returns Bool, drop it
dup chan.close
chan.receive drop            # receive returns (value Bool), drop the Bool
# Stack now has: ( 42 )
```

## After Closing

- Pending values are still received successfully (Bool = true)
- Once empty, `chan.receive` returns `(dummy false)`
- `chan.send` returns `false` on a closed channel
