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
io.read-line              # ( -- String Bool ) — line + success flag
io.read-n                 # ( Int -- String Int ) — N bytes + status
```

Both include any trailing newline in the returned String. Use `string.chomp` to strip it when you don't want it.

## The EOF Challenge

When reading input you need to handle end-of-file (EOF). `io.read-line` returns a Bool, which feeds directly into `if`:

```seq
io.read-line [
    # Got a line — it's on the stack, process it
] [
    drop  # Drop the empty line
    # Handle EOF
] if
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

## A Note on the Tests in This Chapter

The unit tests in this chapter mostly just check that your word *runs* — they don't actually inspect what gets written to stdout or feed bytes into stdin. That means a placeholder solution can pass the test even though it's not really doing I/O.

The friendliest way to see your code do real I/O is to write a tiny **executable Seq script**. `seqc` honors the shebang, so you can `chmod +x` a `.seq` file with a `main` word and run it like any other script.

A "hello world" looks like this:

```seq
#!/usr/bin/env seqc
: main ( -- ) "hello" io.write-line ;
```

```
$ chmod +x hello.seq
$ ./hello.seq
hello
```

The same approach works for input — just pipe stdin in. For example, a greeter that reads a name:

```seq
#!/usr/bin/env seqc

: main ( -- )
    "hello "
    io.read-line [ string.concat io.write-line ] [ drop drop ] if
;
```

```
$ chmod +x greet.seq
$ echo "world" | ./greet.seq
hello world
```

Note that `io.read-line` returns `( String Bool )` — the line on success (with its trailing newline; use `string.chomp` if you want to strip it) and a Bool indicating whether anything was read. On EOF you get `( "" false )`, which is why the `[ ... ] [ drop drop ] if` pattern handles both branches.

The REPL is great for trying out the **output** side (`"hi" io.write-line` works directly), but it owns stdin itself, so it's a poor place to experiment with `io.read-line` — your word will usually just hang. Use scripts for anything that reads.
