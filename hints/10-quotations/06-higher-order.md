# Hint: Writing Higher-Order Words

You're creating a word that takes a quotation as input!

## Solution

```seq
: apply-twice ( Int [ Int -- Int ] -- Int )
    2 times
;
```

The quotation is already on the stack, we just push `2` and call `times`.

## What You've Built

`apply-twice` is an abstraction over behavior. It doesn't know what the quotation does - it just knows to do it twice.

This is the same pattern as:
- JavaScript's `array.map(fn)`
- Python's `functools.reduce(fn, ...)`
- Rust's `iter.filter(predicate)`

Writing words that accept quotations opens up powerful ways to structure code.
