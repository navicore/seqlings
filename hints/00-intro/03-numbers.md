# Hint: Numbers

This exercise introduces numbers and the test framework.

## Literals in Seq

When you write a number, it gets pushed onto the stack:

```seq
42      # Pushes the integer 42
3.14    # Pushes the float 3.14
```

## The Answer

You need to return 42 (the answer to life, the universe, and everything - from The Hitchhiker's Guide to the Galaxy).

Replace `0` with `42`.

## How test.assert-eq Works

`test.assert-eq` takes two values from the stack and checks if they're equal:

```seq
answer 42 test.assert-eq
```

This means: "call `answer`, push `42`, then assert they're equal."
