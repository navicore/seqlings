# Hint: Combining Math Operations

## Manhattan Distance

`rot ( a b c -- b c a )` brings the third-from-top to the top.
With `( x1 y1 x2 y2 )`, the first `rot` produces `( x1 x2 y2 y1 )`,
and `i.- abs` then computes `|y2 - y1|`. The second pair of
`rot`s lines up x1 and x2 the same way.

```seq
: manhattan-distance ( Int Int Int Int -- Int )
    rot i.- abs
    rot rot i.- abs
    i.+
;
```

## Bounded Power

The aux stack (`>aux` / `aux>`) is word-local temporary
storage — push values out of the way, do work on the main
stack, pop them back. With `( base exp min max )` on top, stash
`max` then `min` to aux, leaving `( base exp )` for `i.pow`. Drop
the success Bool, restore min and max in the right order, then
`clamp` does the rest.

```seq
: bounded-power ( Int Int Int Int -- Int )
    >aux >aux       # stash min and max
    i.pow drop      # base^exp (drop the success Bool)
    aux> aux>       # restore min, max
    clamp
;
```
