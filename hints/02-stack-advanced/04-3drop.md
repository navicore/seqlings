# Hint: 3drop

`3drop` removes the top three elements from the stack.

## Understanding 3drop

```seq
42 99 88 77 3drop    # Stack: ( 42 )
```

## Cleanup Operations

In real programs, operations often produce intermediate values you don't need. `3drop` (and `drop`, `2drop`, etc.) clean up the stack.

This is explicit **resource management**. Unlike garbage-collected languages where cleanup is hidden, stack-based programming makes you think about data lifecycle.

```seq
: complex-operation ( a b c -- result )
    # ... do stuff, leaving junk on stack ...
    # Keep only the result
    swap 3drop
;
```

## The Solution

The stack has `42 99 88 77`. You want to keep `42` and remove the top three:

```seq
42 99 88 77
3drop
```
