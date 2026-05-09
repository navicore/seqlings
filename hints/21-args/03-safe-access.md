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

## Why `over`

`over` copies the second value (the index) to the top so we can
feed it to `args.count` for the comparison without disturbing the
default we'll need in the failure branch.

## Trace

For `2 "fb" arg-or` against an `args` of length 4:

```
( 2 "fb" )                ← entry
( 2 "fb" 2 )              over          — copy idx
( 2 "fb" 2 4 )            args.count
( 2 "fb" true )           i.<           — 2 < 4
( 2 )                     drop          — drop default
( <arg 2> )               args.at
```

For `99 "fb" arg-or`, `99 < 4` is false, so the failure branch
runs `nip` to drop the index and leave the default on top.

## The test only checks your word runs

To see in-bounds vs out-of-bounds vs explicit-empty in action,
build the shebang script in the exercise header and try it with
varying arguments.
