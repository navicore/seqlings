# Hint: Mutual Recursion

Two functions that call each other.

## Solution

```seq
: is-even? ( Int -- Bool )
    dup 0 = if
        drop true
    else
        1 i.- is-odd?
    then
;

: is-odd? ( Int -- Bool )
    dup 0 = if
        drop false
    else
        1 i.- is-even?
    then
;
```

## How It Works

- is-even?(4) → is-odd?(3) → is-even?(2) → is-odd?(1) → is-even?(0) → true
- is-odd?(4) → is-even?(3) → is-odd?(2) → is-even?(1) → is-odd?(0) → false

## Forward Declarations

In Seq, you may need to declare a word before it's fully defined if another word references it. The exercise shows this pattern.

## Real-World Mutual Recursion

Mutual recursion appears in:
- Parsers (expressions contain terms, terms contain factors, factors contain expressions)
- State machines (state A transitions to B, B transitions to A)
- Tree traversals with different node types
