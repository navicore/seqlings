# Hint: Processing File Lines

## file.for-each-line

```
( path quot -- Bool )
```

The quotation runs once per line and must have effect
`( ..a String -- ..a )` — String popped, no other changes. Because
of that polymorphism constraint, **you can't accumulate on the
stack** across iterations: the type system rejects any quot whose
shape depends on a specific ..a. Aggregate via side effects.

After the call, the word leaves a single success `Bool` on top of
the stack — `drop` it (or branch on it to handle a missing file).

## The line carries its own newline

Each line is delivered with its trailing `\n` already attached, so
re-emitting the line as-is preserves layout. If you want to compare
or parse a line, `string.chomp` it first.

## Solution

```seq
: quote-lines ( -- String )
    "" "/tmp/seqlings-04-out.txt" file.spit drop
    "exercises/20-files/data/lines.txt"
    [
        "> " swap string.concat
        "/tmp/seqlings-04-out.txt" file.append drop
    ] file.for-each-line
    drop
    "/tmp/seqlings-04-out.txt" file.slurp
    [ string.chomp ]
    [ drop "" ]
    if
;
```

The `"" file.spit` at the top resets the output to empty so the
test is repeatable — without it, every run would tack three more
lines onto the previous run's output.
