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

## You Are Now a Higher-Order Programmer

By writing `apply-twice`, you've created an abstraction over behavior. Your word doesn't know what the quotation does - it just knows to do it twice.

This is the same pattern as:
- JavaScript's `array.map(fn)`
- Python's `functools.reduce(fn, ...)`
- Rust's `iter.filter(predicate)`

You're not just using higher-order programming - you're creating higher-order abstractions.

## The Stack Effect

Note `( ..a quot -- ..b )`. The `..a` and `..b` are stack polymorphism - the quotation can have any stack effect, and `apply-twice` works with it.
