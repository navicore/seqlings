# Hint: Character Codes

`string.char-at` returns an Int (the character code), not a String.

```seq
: first-char-code ( String -- Int )
    0 string.char-at
;
```

The first character is at index 0.
