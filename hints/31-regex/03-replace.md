# Hint: Search and Replace

`regex.replace-all` has the shape `( text pattern replacement -- result Bool )`. The text is already on the stack; push the pattern, then the replacement, call the word, drop the success Bool.

## Solution

```seq
: censor-numbers ( String -- String )
    "\\d" "X" regex.replace-all drop
;

: normalize-whitespace ( String -- String )
    "\\s+" " " regex.replace-all drop
;
```

## Patterns

- `\d` with replacement `"X"` — each digit becomes a single `X`
- `\s+` with replacement `" "` — runs of any whitespace (spaces, tabs, newlines) collapse to one space
