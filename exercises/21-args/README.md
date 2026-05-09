# Command-Line Arguments

Seq exposes command-line arguments through two words. That's the
whole API.

```
args.count   ( -- Int )         total argument count
args.at      ( Int -- String )  argument at the given index
```

## The program path is `args[0]`

`args.count` includes the program path itself. For
`./myprog hello world` you'd see:

```
args.count    → 3
0 args.at     → "./myprog"
1 args.at     → "hello"
2 args.at     → "world"
```

Subtract 1 from `args.count` when you want "the number of
*user-supplied* arguments."

## Out-of-bounds is forgiving

`args.at` on an index past the end returns the empty string `""`,
not a panic. That makes quick scripts pleasant to write but
collapses the distinction between "argument missing" and
"argument was an explicit empty string." Bounds-check with
`args.count` when the difference matters.

## Bounds-check pattern

```seq
: arg-or ( Int String -- String )
    over args.count i.<
    [ drop args.at ]
    [ nip ]
    if
;
```

This is the building block for proper CLI parsing — fetch with a
fallback, never read past the end, no need to think about empty
strings being a sentinel.

## Under `seqc test`

Your program is run with no extra arguments, so `args.count` is
always `1` (just the test binary's path). The assertions in this
chapter are written against that fact. If you build your code
into a real binary with `seqc build` and run it directly, you'll
see the full behaviour in all branches.

## Stack effects

| Word        | Stack Effect          | Notes |
|-------------|-----------------------|-------|
| `args.count` | `( -- Int )`        | total count, includes program path |
| `args.at`    | `( Int -- String )` | "" when out of bounds |

## Exercises in This Section

1. **01-basics** — count *user* arguments with `args.count - 1`
2. **02-at** — fetch by index with `args.at`, plus the
   out-of-bounds-returns-empty rule
3. **03-safe-access** — the bounds-check pattern (`arg-or`)
4. **04-combine** — small CLI summary putting count + at + branch
   together
