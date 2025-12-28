# Hint: Quotation Basics

A quotation is code as data - it doesn't execute until you tell it to.

## Solution

```seq
5
[ dup i.add ] call
```

## The Paradigm Shift

This may seem simple, but you've just done something profound:
1. You wrote code (`dup i.add`)
2. You wrapped it as data (`[ ... ]`)
3. You passed it around (put it on the stack)
4. You executed it later (`call`)

This is the essence of higher-order programming. Code is just another kind of data.

## Why This Matters

Every callback you've ever used, every `.map()` or `.filter()`, every event handler - they all rely on this principle. Functions as values.
