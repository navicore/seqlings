# Hint: Writing Files

## file.spit

`file.spit` overwrites a file with a String:

```
( content path -- status )
```

The argument order trips people up — content sits *under* path on
the stack, just like `file.append`, `tcp.write`, and other "write
something to somewhere" words. Mnemonic: "what" goes down, "where"
goes on top.

If the file doesn't exist yet, `file.spit` creates it. If it already
exists, the previous contents are gone — there's no "open in append
mode" flag, that's what `file.append` is for.

## Round-trip

```seq
"Hello, file!" "/tmp/seqlings-write.txt" file.spit drop
"/tmp/seqlings-write.txt" file.slurp
[ string.chomp ]
[ drop "" ]
if
```

The `drop` after `file.spit` discards the success flag. We trust
/tmp to be writable here — production code would branch on the flag
the same way the slurp side does.

## Why chomp on the way back?

Strictly, `file.spit "Hello, file!"` writes exactly 12 bytes with no
trailing newline, so chomp is a no-op. But leaving chomp in place
makes the same word safe if you ever swap the literal for something
that *does* end in `\n`.
