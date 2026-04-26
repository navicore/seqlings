# Hint: Mutual Recursion

Two functions that call each other.

## Solution

```seq
: is-even? ( Int -- Bool )
    dup 0 i.= [
        drop true
    ] [
        1 i.- is-odd?
    ] if
;

: is-odd? ( Int -- Bool )
    dup 0 i.= [
        drop false
    ] [
        1 i.- is-even?
    ] if
;
```

## How It Works

- is-even?(4) → is-odd?(3) → is-even?(2) → is-odd?(1) → is-even?(0) → true
- is-odd?(4) → is-even?(3) → is-odd?(2) → is-even?(1) → is-odd?(0) → false

## Real-World Mutual Recursion

Mutual recursion appears in:
- Parsers (expressions contain terms, terms contain factors, factors contain expressions)
- State machines (state A transitions to B, B transitions to A)
- Tree traversals with different node types
