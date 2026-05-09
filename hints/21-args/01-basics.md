# Hint: How Many Arguments?

## Solution

```seq
: user-arg-count ( -- Int )
    args.count 1 i.-
;
```

`args.count` always includes `args[0]`, which is the program path
itself. Subtract one to get the number of user-supplied arguments.

## The test only checks your word runs

`seqc test` invokes your program with no extra arguments, so a
real per-arg assertion can't exercise more than the zero-args
case. The test in this exercise just verifies your word
type-checks and doesn't crash — to actually see the count change,
build the shebang script in the exercise header and run it with
varying numbers of arguments.
