# Hint: Splitting Strings

`regex.split` returns `( list Bool )`. The pattern is the *delimiter* — what gets cut out — and what's left between matches becomes the list elements.

## Solution

```seq
: split-csv ( String -- Variant )
    "\\s*,\\s*" regex.split drop
;

: split-sentences ( String -- Variant )
    "[.!?]+\\s*" regex.split drop
;
```

## Patterns

- `\s*,\s*` — comma with any amount of surrounding whitespace (handles `"a,b"`, `"a, b"`, `"a ,  b"` uniformly)
- `[.!?]+\s*` — one or more sentence terminators followed by optional whitespace

## Gotcha

If the input *ends* with a delimiter, `regex.split` leaves a trailing empty string in the list. The test inputs here are crafted to avoid that, but keep it in mind for real-world use.
