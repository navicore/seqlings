# Hint: Safe Argument Access

## Solution

```seq
: arg-or ( Int String -- String )
    over args.count i.<
    [ drop args.at ]
    [ nip ]
    if
;
```

## Trace

For `0 "FALLBACK" arg-or`:

```
( 0 "FALLBACK" )                ← entry
( 0 "FALLBACK" 0 )              over          — copy idx
( 0 "FALLBACK" 0 1 )            args.count    — count is 1 under seqc test
( 0 "FALLBACK" true )           i.<           — 0 < 1
                                if pops bool, runs success branch
( 0 )                           drop          — drop default
( "/tmp/..." )                  args.at       — fetch arg 0
```

For `99 "FALLBACK" arg-or`, `99 < 1` is false, so the failure
branch runs `nip` to drop the index and leave the default on top.

## Why `over` instead of `dup swap`

`over` is exactly the "copy the second value to the top" idiom we
want here — it gives us a copy of the index to feed to `args.count`
without disturbing the `default` we'll need in the failure branch.
