# Hint: Constructors

## empty

`variant.make-0` builds a zero-field variant. The tag is the symbol
just before it on the stack:

```seq
: empty ( -- List )
    :Empty variant.make-0
;
```

That's the whole word. The symbol `:Empty` is the runtime label
that `variant.tag` (next exercise) will pull back out.

## prepend

`variant.make-2` builds a two-field variant from the two values
already on the stack. With effect `( T List -- List )`, the head
sits below the tail when prepend is called, which is exactly the
order we want:

```seq
: prepend ( T List -- List )
    :Cons variant.make-2
;
```

No stack juggling. The two arguments are already where they belong.

## Why two constructors?

Every operation on this list — predicates, accessors, length,
reverse, append — will branch on which constructor a value came
from. That branching is what makes ADTs useful: the data carries
its own dispatch tag, and you can't accidentally treat an `Empty`
as a `Cons` because the runtime knows which it is.
