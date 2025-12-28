# Hint: Defining Words

To multiply by 3, you can either:
- `3 i.*`
- `dup dup i.+ i.+` (add to itself twice)

## Solution

```seq
: triple ( Int -- Int )
    3 i.*
;
```

## Why This Matters

You've just created an abstraction. Instead of thinking "multiply by 3", you can now think "triple". This is how programmers manage complexity - by building up a vocabulary of meaningful operations.
