# Hint: Yielding

`chan.yield` is the body of the loop. The loop itself is
self-recursion: the word calls itself with N-1, then throws away
the recursive result so the original N stays on top of the stack.

The base case is N <= 0 — nothing to do, leave N alone.

## Solution

```seq
: count-with-yields ( Int -- Int )
    dup 0 i.>
    [
        chan.yield
        dup 1 i.- count-with-yields drop
    ]
    [ ]
    if
;
```
