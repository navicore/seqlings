# Hint: Reading Files

## file.slurp returns *two* values

`file.slurp` pushes the file's contents *and* a Bool status flag:

```
( path -- contents status )
```

The compiler will warn (`unchecked-file-slurp`) if you forget the
status flag is there. Branch on it with `if`:

```seq
"data.txt" file.slurp
[ ...success: contents on top of stack... ]
[ drop ...failure: drop the empty string... ]
if
```

## Why you also need to strip a newline

The fixture `exercises/20-files/data/greeting.txt` contains:

```
Hello, Seq!\n
```

So slurping gives `"Hello, Seq!\n"`, which is *not* equal to
`"Hello, Seq!"`. Strip the trailing newline before comparing:

- `string.chomp` removes a single trailing newline (the right tool here)
- `string.trim` removes leading and trailing whitespace (also works)

## Solution

```seq
: read-greeting ( -- String )
    "exercises/20-files/data/greeting.txt" file.slurp
    [ string.chomp ]
    [ drop "" ]
    if
;
```

## Path resolution

The path is relative to wherever you launched `seqlings`, so run it
from the repo root.
