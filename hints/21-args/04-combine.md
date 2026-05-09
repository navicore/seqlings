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

## What you'd see on a real run

Under `seqc test` the count is always 1, so only the first branch
fires and that's what the test asserts. If you built the program
into a real binary and ran it:

```
$ ./prog                     → "no arguments provided"
$ ./prog hello               → "got: hello"
$ ./prog hello world         → "got 2 arguments starting with hello"
```

Three reachable behaviours from one `summary` word.
