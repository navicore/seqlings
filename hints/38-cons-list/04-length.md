# Hint: Length

## length-acc

The accumulator pattern from chapter 09 in one shape:

```seq
: length-acc ( List Int -- Int )
    swap dup empty?
    [ drop ]                          # empty: drop the list, return acc
    [ tail swap 1 i.+ length-acc ]    # cons: walk to tail, bump acc
    if
;
```

Trace through `length-acc` on `(1 2 3)` with acc = 0:

```
( (1 2 3) 0 )      ← entry
( 0 (1 2 3) )      swap
( 0 (1 2 3) (1 2 3) )  dup
( 0 (1 2 3) false )    empty?
                       false branch:
( 0 (1 2 3) )          if pops the Bool
( 0 (2 3) )            tail
( (2 3) 0 )            swap
( (2 3) 1 )            1 i.+
                       recurse → eventually returns 3
```

## length

The public word just kicks off with acc = 0:

```seq
: length ( List -- Int )
    0 length-acc
;
```

## Why a separate `-acc` helper?

It keeps the public signature clean (`( List -- Int )`) while the
recursion needs an extra slot for the accumulator. Same trick you
used in `factorial` and `countdown` in chapter 09.
