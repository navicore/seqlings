# Hint: Row Polymorphism

The key is to save the original value before running the quotation.

## Solution

```seq
: keep ( A Quotation -- B A )
    over      # ( x quot x ) - copy x to top
    swap      # ( x x quot ) - get quot on top
    call      # ( x result ) - run quot, consumes one x
    swap      # ( result x ) - put original on top
;
```

## How It Works

```
Stack trace for: 5 [ dup i.* ] keep

5 [ dup i.* ]     # ( 5 quot )
over              # ( 5 quot 5 )
swap              # ( 5 5 quot )
call              # quot runs: 5 dup i.* = 25
                  # ( 5 25 )
swap              # ( 25 5 )
```

## The Row Polymorphism Insight

Notice the type signature:
```seq
: keep ( A Quotation -- B A )
```

- `A` is a type variable - it can be ANY type
- The same definition works for Int, String, Bool, or custom types
- The compiler verifies type safety for each use

This is different from most languages where you'd need:
- Generics with explicit type parameters
- Or dynamic typing with runtime checks

In Seq, row polymorphism makes generic programming natural.

## CS Concept: Row Polymorphism

From the glossary: *"A type system feature allowing functions to work with stacks of any depth, as long as required types appear on top, providing flexibility without sacrificing type safety."*

The "row" refers to the stack - it can have any "row" of values below the elements your function touches. Your function only specifies what it needs on top.
