# Hint: How Many Arguments?

## Solution

```seq
: user-arg-count ( -- Int )
    args.count 1 i.-
;
```

`args.count` always includes `args[0]`, which is the program path
itself. Subtract one to get the number of user-supplied arguments.

## Why the test asserts 0

The seqlings test runner copies your exercise into a temp file and
calls `seqc test` on it with no further arguments. Inside that
invocation `args.count` is 1 (just the temp binary's path), so
`user-arg-count` is 0.

If you built your code into a real binary and ran it as
`./prog hello world`, `args.count` would be 3 and `user-arg-count`
would be 2.
