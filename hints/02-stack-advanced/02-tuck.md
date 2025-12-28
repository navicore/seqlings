# Hint: tuck

`tuck` copies the top element and inserts it below the second element.

## Understanding tuck

```seq
5 10 tuck    # Stack: ( 10 5 10 )
```

The top (10) gets "tucked" below where it was, with a copy remaining on top.

## Composition Insight

`tuck` is equivalent to `swap over`:

```seq
5 10          # Stack: ( 5 10 )
swap          # Stack: ( 10 5 )
over          # Stack: ( 10 5 10 )
```

## When tuck is Useful

`tuck` is handy when you need to use the top value but also save it for later:

```seq
: save-and-process ( a b -- b result b )
    tuck          # Keep a copy of b below
    some-operation
;
```

## The Solution

Add `tuck` after the two values:

```seq
5 10
tuck
```
