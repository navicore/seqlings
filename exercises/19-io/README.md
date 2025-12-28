# Part 19: Input/Output

I/O (Input/Output) is how programs interact with the outside world - reading user input, writing output, communicating with files and networks.

## Standard I/O

Every program has three standard streams:
- **stdin**: Standard input (keyboard by default)
- **stdout**: Standard output (terminal by default)
- **stderr**: Standard error (terminal by default)

## Writing Output

```seq
"Hello!" io.write-line    # Print with newline
"Hello" io.write          # Print without newline
```

## Reading Input

```seq
io.read-line              # Read one line from stdin
io.read-line+             # Read line, returning (line true) or (false) on EOF
io.read-n                 # Read exactly n bytes
```

## The EOF Challenge

When reading input, you need to handle end-of-file (EOF):

```seq
io.read-line+    # Returns ( line true ) or ( "" false )
if
    # Process line
else
    # Handle EOF
then
```

## Interactive Programs

Combining read and write creates interactive programs:

```seq
: prompt ( -- String )
    "> " io.write
    io.read-line
;
```

## I/O and Effects

I/O operations are **side effects** - they affect the world outside the program. Unlike pure computation:
- They can't be reordered freely
- They depend on external state (what the user types)
- They're not referentially transparent

Understanding this distinction helps you structure programs well: pure computation at the core, I/O at the edges.
