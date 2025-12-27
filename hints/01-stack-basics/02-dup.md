# Hint: dup

The `dup` word duplicates the top value on the stack.

## How dup Works

```seq
5 dup     # Stack: ( 5 5 )
10 dup    # Stack: ( 10 10 )
```

Stack effect: `( a -- a a )`

## The Solution

The test expects two 7s on the stack. You need to:
1. Push 7
2. Duplicate it with `dup`

Replace `0 0` with `7 dup`.

## Why dup is Useful

`dup` is essential when you need to use a value more than once:

```seq
: square ( Int -- Int )
    dup i.multiply
;
# 5 square leaves 25 on the stack
```
