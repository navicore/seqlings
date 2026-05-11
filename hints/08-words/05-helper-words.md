# Hint: Building Vocabulary

A number is even if `n mod 2 = 0`. `i.modulo` returns
`( Int Bool )` — the remainder and a success flag — so drop the
flag before comparing.

## Solution

```seq
: is-even? ( Int -- Bool )
    2 i.modulo drop 0 i.=
;
```

Step by step:
1. `2 i.modulo` — compute n mod 2, leaves `(remainder true)`
2. `drop` — drop the success flag
3. `0 i.=` — compare remainder to 0

## Naming Conventions

Notice the `?` at the end of `is-even?`. This is a common convention for predicate words (words that return booleans). Other examples:
- `string.empty?`
- `list.contains?`
- `file.exists?`

Good names communicate intent.
