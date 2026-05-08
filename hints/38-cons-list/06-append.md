# Hint: Append

```seq
: append ( List List -- List )
    over empty? [
        nip
    ] [
        over head
        rot tail rot
        rot
        [ append ] dip
        swap prepend
    ] if
;
```

## Trace through append on (1 2) ++ (3 4)

```
( (1 2) (3 4) )                       ← entry
( (1 2) (3 4) (1 2) )                 over           — copy A
( (1 2) (3 4) 1 )                     head           — head of A
( (3 4) 1 (1 2) )                     rot            — bring A back
( (3 4) 1 (2) )                       tail           — tail of A
( 1 (2) (3 4) )                       rot            — get B back to top
( (2) (3 4) 1 )                       rot            — head goes to top
( (2 3 4) 1 )                         [ append ] dip — recurse on (2)++(3 4),
                                                     keep head saved
( 1 (2 3 4) )                         swap
( (1 2 3 4) )                         prepend        — done
```

## What `[ append ] dip` is doing

`dip`'s contract is "hide the top, run the quotation, restore the
top." That's exactly what we need: the recursive `append` call has
to operate on the two lists below the saved head, but the head has
to survive the call so we can prepend it onto the result.

Without `dip` you'd need three `swap`s and a save-stash, or you'd
have to carry the head through a different stack slot — `dip`
makes the intent ("save this for after the recursion") explicit.

## Why no accumulator?

`length` and `reverse` accumulated because their operation has a
natural "running total." Append doesn't — the answer is built by
prepending heads onto the recursive result, *after* the recursion
returns. That's what makes append's recursion right-to-left
shaped, while length's and reverse's are left-to-right.
