# Hint: Writing Higher-Order Words

You're creating a word that takes a quotation as input!

## Solution

```seq
: apply-twice ( ..a quot -- ..b )
    dup call call
;
```

Or using times:
```seq
: apply-twice ( ..a quot -- ..b )
    2 swap times
;
```

Wait, `times` expects `n quot`, so:
```seq
: apply-twice ( ..a quot -- ..b )
    2 swap times
;
```

Actually simpler:
```seq
: apply-twice ( ..a quot -- ..b )
    dup call call
;
```

## What You've Built

`apply-twice` is an abstraction over behavior. It doesn't know what the quotation does - it just knows to do it twice.

This is the same pattern as:
- JavaScript's `array.map(fn)`
- Python's `functools.reduce(fn, ...)`
- Rust's `iter.filter(predicate)`

Writing words that accept quotations opens up powerful ways to structure code.

## The Stack Effect

Note `( ..a quot -- ..b )`. The `..a` and `..b` are stack polymorphism - the quotation can have any stack effect, and `apply-twice` works with it.
