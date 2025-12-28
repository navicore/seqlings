# Hint: Channel Patterns

Use receive-safe in a loop until it returns false.

## Solution

```seq
: sum-from-channel ( Chan -- Int )
    0 swap   # ( acc chan )
    [ dup chan.receive-safe ]
    [
        # ( acc chan value true )
        drop rot i.add swap
    ]
    while
    # receive-safe returned false
    drop drop   # drop false and chan
;
```
