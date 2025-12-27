# Hint: Hello, Seq!

This exercise is simple - you just need to change the string that gets printed.

## String Literals

In Seq, strings are written with double quotes:

```seq
"Hello"
"Goodbye"
"Hello, Seq!"
```

## The Solution

Replace `"Hello, World!"` with `"Hello, Seq!"` and remove the `# NOT DONE` marker.

## What's Happening

When Seq runs your code:
1. The string `"Hello, Seq!"` is pushed onto the stack
2. `io.write-line` takes that string and prints it to the screen
3. The program exits
