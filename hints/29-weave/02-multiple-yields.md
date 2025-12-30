# Hint: Multiple Yields

Each `yield` pauses execution, sends a value, and waits for the next resume.

## Solution

```seq
: three-values ( Ctx Int -- Ctx | Yield Int )
    drop           # ignore initial resume value
    1 yield drop   # yield 1, drop resume response
    2 yield drop   # yield 2, drop resume response
    3 yield drop   # yield 3, drop resume response
    # return - weave completes
;
```

## The Yield Cycle

```
Caller                          Weave
------                          -----
strand.weave                    (waiting)
0 strand.resume     -->         wakes with ( Ctx 0 )
                                drop, 1 yield
(Handle 1 true)     <--         (paused)
0 strand.resume     -->         wakes with ( Ctx 0 )
                                drop, 2 yield
(Handle 2 true)     <--         (paused)
0 strand.resume     -->         wakes with ( Ctx 0 )
                                drop, 3 yield
(Handle 3 true)     <--         (paused)
0 strand.resume     -->         wakes, drops, returns
(Handle 0 false)    <--         (done)
```

## Why Drop After Yield?

`yield` returns the value passed to the next `strand.resume`. Since we're ignoring it (just yielding fixed values), we drop it to keep the stack clean.
