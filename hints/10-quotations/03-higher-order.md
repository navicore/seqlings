# Hint: Writing Higher-Order Words

You're creating a word that takes a quotation as input.

The challenge: `call` consumes the quotation it executes. To call it a second time, the quotation has to still be on the stack at the right place — so set up your stack before the first `call` with that in mind.

## What You've Built

`apply-twice` is an abstraction over behavior. It doesn't know what the quotation does — it just knows to do it twice.

This is the same pattern as:
- JavaScript's `array.map(fn)`
- Python's `functools.reduce(fn, ...)`
- Rust's `iter.filter(predicate)`

Writing words that accept quotations opens up powerful ways to structure code.
