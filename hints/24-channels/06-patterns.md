# Hint: Channel Patterns

Use `chan.receive` in a loop - it returns `(value Bool)` where Bool is false when the channel is closed.

## Solution

```seq
: sum-from-channel ( Channel -- Int )
    0 swap   # ( acc chan )
    [ dup chan.receive ]
    [
        # ( acc chan value true )
        drop rot i.+ swap
    ]
    while
    # receive returned false - channel closed
    drop drop   # drop dummy value and chan
;
```

## The While Pattern

```seq
[ condition ] [ body ] while
```

The condition quotation should leave a Bool on top. While it's true, the body runs.

Here, `chan.receive` returns `(value true)` or `(dummy false)`, so we loop while receiving succeeds.
