# Command Line Arguments

Seq programs can access command line arguments passed when the program runs.

## Getting Arguments

```seq
args.get-all    # Returns a List of all arguments
args.count      # Returns the number of arguments
```

## Accessing Individual Arguments

```seq
0 args.get      # Get first argument (program name)
1 args.get      # Get second argument
```

## Example Usage

If you run: `seq myprogram.seq hello world`

```seq
args.get-all    # [ "myprogram.seq" "hello" "world" ]
args.count      # 3
0 args.get      # "myprogram.seq"
1 args.get      # "hello"
```

## Safe Access

```seq
1 "default" args.get-or    # Get arg 1 or "default" if missing
```

## Typical Pattern

```seq
: main ( -- )
    args.count 2 <
    [ "Usage: program <name>" io.write-line ]
    [ 1 args.get process-name ]
    if-else
;
```

## Exercises in This Section

1. **basics** - Getting command line arguments
2. **count** - Counting arguments
3. **safe-access** - Handling missing arguments
4. **combine** - Building argument parsers
