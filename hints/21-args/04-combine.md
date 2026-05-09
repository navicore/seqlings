# Hint: A Tiny CLI Summary

## Solution

```seq
: summary ( -- String )
    args.count 1 i.- dup
    0 i.= [
        drop "no arguments provided"
    ] [
        dup 1 i.= [
            drop "got: " 1 args.at string.concat
        ] [
            "got " swap int->string string.concat
            " arguments starting with " string.concat
            1 args.at string.concat
        ] if
    ] if
;
```

## How the dispatch works

Compute `args.count - 1` (the number of *user* args) and `dup` it
so we can compare without losing it. Then a two-level `if`:

- `0 i.=` → empty case, drop the saved count and return the
  literal "no arguments provided".
- otherwise, `dup 1 i.=` → single-arg case, drop the saved count
  and concatenate "got: " with `1 args.at`.
- otherwise → multi-arg case, the saved count is still on the
  stack; turn it into a string and weave it into the message.

## The test only checks your word runs

To see all three branches in action, drop your solution into the
shebang script in the exercise header and run it with zero, one,
and two args.
