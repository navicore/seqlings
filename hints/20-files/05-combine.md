# Hint: Combining File Operations

## spit then append

`file.spit` truncates and writes; `file.append` adds to the end.
Use one `spit` to create-or-reset, then `append` per record:

```seq
"start\n"  "/tmp/seqlings-05-log.txt" file.spit   drop
"step 1\n" "/tmp/seqlings-05-log.txt" file.append drop
"step 2\n" "/tmp/seqlings-05-log.txt" file.append drop
"done\n"   "/tmp/seqlings-05-log.txt" file.append drop
```

The `drop` after each call discards the success Bool. Production
code would `if` on the flag and surface failures somewhere — we
trust /tmp here.

## Why spit first?

Without the leading `spit ""` (or `spit <header>`), each test run
would append onto whatever the previous run left behind. The first
`spit` resets the file so the test is repeatable.

## Read it back

```seq
"/tmp/seqlings-05-log.txt" file.slurp
[ string.chomp ]
[ drop "" ]
if
```

`string.chomp` strips the single trailing newline, which lets you
compare against a literal that doesn't end in `\n`.
