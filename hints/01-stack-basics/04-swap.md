# Hint: swap

The `swap` word exchanges the top two values on the stack.

## How swap Works

```seq
1 2 swap    # Stack: ( 2 1 )
```

Stack effect: `( a b -- b a )`

The top two elements switch places.

## The Solution

The stack starts as `( 20 10 )` (with 10 on top), but we need `( 10 20 )` (with 20 on top).

Add `swap` after pushing the values:

```seq
20 10
swap
20 test.assert-eq
10 test.assert-eq
```

## When to Use swap

`swap` is useful when values are in the wrong order for an operation:

```seq
: subtract-from-10 ( Int -- Int )
    10 swap i.-
;
# 3 subtract-from-10 leaves 7 on the stack (10 - 3)
```
