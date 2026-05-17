# Hint: Capture Groups

`regex.captures` returns `( list Bool )` where the list holds each parenthesized group (no full-match entry). Use `if` to branch on the Bool: pull list elements out in the success branch, return empty placeholders on failure.

The trick is that the test pops captured strings off the stack from the *bottom*, not the top — read the `test.assert-eq-str` order carefully. For `parse-date`, the test compares against `"15"` first, then `"01"`, then `"2024"`, so the stack must end with year-on-bottom / day-on-top.

## Solution

```seq
: parse-date ( String -- String String String Bool )
    "(\\d{4})-(\\d{2})-(\\d{2})" regex.captures [
        dup 0 list.get drop   # year
        over 1 list.get drop  # month
        rot 2 list.get drop   # day
        true
    ] [
        drop "" "" "" false
    ] if
;

: parse-time ( String -- String String Bool )
    "(\\d{2}):(\\d{2})" regex.captures [
        dup 0 list.get drop   # hours
        swap 1 list.get drop  # minutes
        true
    ] [
        drop "" "" false
    ] if
;
```

## How It Works

`list.get` returns `( element Bool )`; the patterns guarantee the indices exist, so `drop` the Bool. `dup`/`over`/`rot` keep the list on the stack across multiple `list.get` calls — each access needs its own copy of the list reference.
