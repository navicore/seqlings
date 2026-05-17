# Hint: Pattern Matching

`regex.match?` takes a String to search and a String pattern, and returns a Bool. The pattern goes on top of the stack. Each word here is a one-liner: push the right pattern and call `regex.match?`.

Backslashes in patterns need escaping — `\d` is written `"\\d"`.

## Solution

```seq
: has-digits? ( String -- Bool )
    "\\d" regex.match?
;

: starts-with-letter? ( String -- Bool )
    "^[a-zA-Z]" regex.match?
;

: is-all-lowercase? ( String -- Bool )
    "^[a-z]+$" regex.match?
;
```

## Pattern Notes

- `\d` — any digit (no anchors, so matches anywhere in the string)
- `^[a-zA-Z]` — letter at the start
- `^[a-z]+$` — entire string is one-or-more lowercase letters (both anchors)
