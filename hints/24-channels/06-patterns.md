# Hint: Channel Patterns

Three receives, three flags to drop, three values to sum.

After each receive, get a fresh copy of the channel reference on
top of the stack:

- `dup` — copy from one below the top
- `over` — copy from two below the top
- `rot` — bring three-from-top to the top (and consume it)

`chan.receive` returns ( value flag ); for this exercise the flag
is always `true` (we know three values were sent), so a plain
`drop` after each receive is fine.

## Solution

```seq
: sum-from-channel ( Channel -- Int )
    dup chan.receive drop
    over chan.receive drop
    rot chan.receive drop
    i.+ i.+
;
```

## Why no `while` loop?

`while` was removed from the stdlib (see exercise 05). For
streaming receives of *unknown* length you'd write a recursive
helper that branches on `chan.receive`'s success flag — but in
this test the channel never fully closes (the test strand still
holds a sender reference), so the recursion would block on a
fourth receive. Hardcoding the count is the right move here.
