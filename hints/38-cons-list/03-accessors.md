# Hint: Accessors

## head

Field 0 holds the head:

```seq
: head ( List -- T )
    0 variant.field-at
;
```

## tail

Field 1 holds the tail:

```seq
: tail ( List -- List )
    1 variant.field-at
;
```

## Why these are this short

The work was done in `prepend`. Once `:Cons variant.make-2`
captured the two stack values into fields 0 and 1, getting them
back is trivial. This is the payoff for picking a clean
constructor: every operation downstream gets simpler.

## What about Empty?

`variant.field-at` on an `Empty` (zero-field) variant is a runtime
error. `head` and `tail` therefore have a precondition: "the list
is non-empty." Real production code would either pre-check with
`empty?` or return a Result; for these exercises we trust callers
to have already checked.
