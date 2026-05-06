# File Operations

Seq exposes file I/O through the `file.*` namespace. Every
operation that *reads from disk* returns a status Bool alongside
the result, so you branch on the flag with `if` and handle the
missing-or-unreadable case explicitly.

## Reading a whole file

```seq
"path/to/file.txt" file.slurp
[ ...success: contents on top of stack... ]
[ drop ...failure: drop the empty string... ]
if
```

The compiler emits a `unchecked-file-slurp` warning if you forget
the Bool is there.

`file.slurp` returns the file *exactly* as it sits on disk,
trailing newline included. `string.chomp` strips a single trailing
newline; `string.trim` strips any leading and trailing whitespace.

## Writing a whole file

```seq
"hello, world\n" "/tmp/out.txt" file.spit drop
```

Argument order: **content under, path on top**. `file.spit`
truncates first; if you want to extend an existing file, use
`file.append` (same shape).

## Existence check

```seq
"config.toml" file.exists?
[ "config.toml" file.slurp ... ]
[ ...use defaults... ]
if
```

`file.slurp` already reports its own success, so a pre-check with
`file.exists?` is **not** required for safety. Use it when
present-vs-absent is part of your control flow.

## Iterating lines

```seq
"log.txt" [ io.write-line ] file.for-each-line+
drop drop
```

Three things to remember about `file.for-each-line+`:

1. The quotation must have effect `( ..a String -- ..a )` —
   polymorphic in the rest of the stack. You can't carry an
   accumulator through; aggregate via side effects (append to a
   file, write to a channel, mutate a map).
2. Each line is delivered with its trailing `\n` still attached.
3. The word leaves a sentinel `( "" true )` on the stack at EOF,
   so callers tail it with `drop drop`.

## Other file ops

| Word | Stack Effect | Notes |
|------|--------------|-------|
| `file.delete` | `( path -- status )` | Bool indicates success |
| `file.size`   | `( path -- size status )` | size in bytes when status is true |

## Stack Effects (cheat sheet)

| Word | Stack Effect |
|------|--------------|
| `file.slurp`           | `( path -- contents status )` |
| `file.spit`            | `( content path -- status )` |
| `file.append`          | `( content path -- status )` |
| `file.exists?`         | `( path -- Bool )` |
| `file.delete`          | `( path -- status )` |
| `file.size`            | `( path -- size status )` |
| `file.for-each-line+`  | `( path quot -- "" true )` |

## Path resolution

Paths are resolved relative to wherever you launched the seqlings
runner. Run from the project root so the bundled fixtures under
`exercises/20-files/data/` resolve.

## Exercises in This Section

1. **01-read** — read a fixture file (`file.slurp` + `string.chomp`)
2. **02-write** — round-trip a string through disk (`file.spit` + `file.slurp`)
3. **03-exists** — branch on file presence (`file.exists?`)
4. **04-lines** — process a file line by line (`file.for-each-line+`)
5. **05-combine** — build a small log with `spit` + repeated `append`
