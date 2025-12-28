# Hint: Building Vocabulary

A number is even if `n mod 2 = 0`.

## Solution

```seq
: is-even? ( Int -- Bool )
    2 i.mod 0 =
;
```

Step by step:
1. `2 i.mod` - compute n mod 2 (0 if even, 1 if odd)
2. `0 =` - compare to 0

## Naming Conventions

Notice the `?` at the end of `is-even?`. This is a common convention for predicate words (words that return booleans). Other examples:
- `string.empty?`
- `list.contains?`
- `file.exists?`

Good names communicate intent.
