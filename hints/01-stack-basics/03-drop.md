# Hint: drop

The `drop` word removes the top value from the stack.

## How drop Works

```seq
1 2 3 drop    # Stack: ( 1 2 )  - the 3 is gone
```

Stack effect: `( a -- )`

The value is simply discarded.

## The Solution

The stack has `( 42 99 )` and we only want `42`. Add `drop` to remove the 99:

```seq
42 99
drop
42 test.assert-eq
```

## When to Use drop

Use `drop` when a function returns a value you don't need:

```seq
: do-something ( -- Int )
    42  # Returns a result
;

do-something drop  # Call but ignore the result
```
