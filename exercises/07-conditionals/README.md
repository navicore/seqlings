# Part 7: Conditionals

Conditionals let programs make decisions. With comparisons and booleans under your belt, you can now control program flow.

## Conditionals are Combinators

In Seq, the conditional `if` is a **word**, not a syntactic construct. It takes a Bool and two pieces of code (called *quotations*, written between `[` and `]`) and runs whichever piece matches the Bool.

```seq
condition [ then-branch ] [ else-branch ] if
```

The Bool comes first; both branches come next as quotations; `if` is the word that ties them together.

## A Concrete Example

```seq
5 3 i.> [ "yes" ] [ "no" ] if
# Stack: ( "yes" ) because 5 > 3 is true
```

The Bool (from `5 3 i.>`) selects the first quotation. `if` runs it and discards the other.

## Stack Effects in Branches

Both branches must leave the stack the SAME shape — the type checker enforces it:

```seq
# WRONG - branches have different effects
x 0 i.> [ 42 ] [ 1 2 ] if
# left branch leaves one value; right leaves two — type error
```

This sounds restrictive, but in practice it means you write the branches' shape into the type, which catches mistakes early.

## One-Armed Shortcuts: when and unless

When you only need to act in one case, the two-armed `if` with an empty branch feels noisy:

```seq
cond [ do-something ] [ ] if    # works, but heavy
```

The `when` and `unless` words are one-armed shortcuts. They live in `std:control`:

```seq
include std:control

cond [ do-something ] when      # only when cond is true
cond [ do-something ] unless    # only when cond is false
```

**Important:** the body of `when` / `unless` must leave the stack shape it found (same number of items, same types). For branches that change shape, use the explicit two-armed `if`.

## Conditionals as Expressions

Unlike many languages, Seq conditionals can return values:

```seq
: abs ( Int -- Int )
    dup 0 i.< [ -1 i.* ] [ ] if
;
```

Both branches end with one Int on the stack — the negation, or the original untouched. Whether the body runs or not, you end up with one Int.

## Why Combinators?

Treating `if` as a word that takes code-as-data is what makes Seq's flow control compose with the rest of the language: branches are just quotations, the same things you'll pass to `times`, `list.map`, and other higher-order combinators in later chapters. There is no special syntax to remember — `if` is just a word.

You'll meet quotations formally in chapter 10. For now, think of `[ ... ]` as "code wrapped up so something else can decide when to run it."
