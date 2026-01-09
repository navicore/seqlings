# Hint: Channel Operation Results

`chan.receive` returns both the value and a success status.

## Solution

```seq
chan.make
dup 99 swap chan.send drop   # send returns Bool, we drop it
chan.receive                  # returns (value Bool)
# Stack now has: ( 99 true )
```

## Why Return Status?

When a channel is closed:
- `chan.send` returns `false`
- `chan.receive` returns `(dummy false)` where dummy is a placeholder value

This lets you detect when communication has ended without crashing.
