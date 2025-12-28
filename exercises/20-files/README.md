# File Operations

Seq provides built-in operations for reading files using the `file.*` namespace.

## Reading Files

```seq
"path/to/file.txt" file.slurp    # Returns file contents as String
```

## Safe Reading with Status

```seq
"path/to/file.txt" file.slurp-safe    # Returns (contents status)
if
    # Got contents successfully
else
    drop  # Drop empty string
    # Handle file not found
then
```

## Checking File Existence

```seq
"myfile.txt" file.exists?    # Returns 1 (true) or 0 (false)
```

## Processing Files Line by Line

```seq
"data.txt" [
    if   # if status is true (not EOF)
        io.write-line   # Process line
        true            # Continue
    else
        drop false      # Stop at EOF
    then
] file.for-each-line+
```

## Stack Effects

| Word | Stack Effect | Description |
|------|--------------|-------------|
| `file.slurp` | `( path -- contents )` | Read entire file as String |
| `file.slurp-safe` | `( path -- contents status )` | Read file with success flag |
| `file.exists?` | `( path -- status )` | Check if file exists (1=yes, 0=no) |
| `file.for-each-line+` | `( path quot -- )` | Process file line by line |

## Exercises in This Section

1. **01-read** - Reading file contents with `file.slurp`
2. **02-write** - Safe reading with `file.slurp-safe`
3. **03-exists** - Checking if files exist
4. **04-lines** - Processing files line by line
5. **05-combine** - Complete file handling patterns
