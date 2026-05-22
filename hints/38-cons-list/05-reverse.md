# Hint: Reverse

## reverse-acc

Same shape as `length-acc`, with one extra wrinkle: the cons branch
moves a value (the head) from the input list onto the accumulator
list before recursing.

```seq
: reverse-acc ( Variant Variant -- Variant )
    swap dup empty?
    [ drop ]
    [ dup head rot prepend swap tail swap reverse-acc ]
    if
;
```

## Trace through reverse-acc on (1 2 3) with acc = ()

```
( () (1 2 3) )           ← entry, after swap+dup empty? → false branch
( () (1 2 3) (1 2 3) )   dup
( () (1 2 3) 1 )         head             — got the head out
( (1 2 3) 1 () )         rot              — move acc to top
( (1 2 3) (1) )          prepend          — built new acc
( (1) (1 2 3) )          swap             — input on top to take its tail
( (1) (2 3) )            tail
( (2 3) (1) )            swap             — back to (input acc) order
                         reverse-acc      — recurse
```

The dance is: each iteration moves *one* element from the input
list to the front of the accumulator. After three iterations the
input is empty and the accumulator has been built up as `(3 2 1)`.

## reverse

Kick off with an empty accumulator:

```seq
: reverse ( Variant -- Variant )
    empty reverse-acc
;
```

## Why reverse "for free"?

You might think reverse would need two passes — one to find the
end, one to flip. The accumulator trick collapses both into one:
prepending to a fresh list naturally builds the result in reverse
order, so when the input runs out, the accumulator is already
backwards.
