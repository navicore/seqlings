# Hint: Tail Call Optimization

The exercise stub already lays out the shape — your job is to fill in the body so the recursive call is the very last action.

For `count-helper ( target current )`:

- **Base case**: `current` has reached `target` → return `current`, drop `target`.
- **Recursive case**: increment `current` by 1, recurse with the same `target` and the new `current`. Nothing after the recurse.

That "nothing after the recurse" rule is THE point of the exercise. If you `dup` before the call and `i.+` after, the compiler can't optimize, and 100,000 frames overflow the stack. Re-read the stub's commented hint: "with no further work after it."

Two primitives that pair naturally with this shape: `2dup` to compare both values without consuming either, and `nip` to discard one of two stack values cleanly (the base case wants to drop `target` while keeping `current`).

## Why this works for 100,000

In non-tail recursion:

```
sum(3) = 3 + sum(2)           # must WAIT for sum(2), then add
       = 3 + (2 + sum(1))     # stack grows with each call
       = 3 + (2 + (1 + 0))    # finally unwind all those frames
```

In tail recursion:

```
count-helper(3, 0)
  -> count-helper(3, 1)       # no pending work — just jump
  -> count-helper(3, 2)       # same stack frame, new values
  -> count-helper(3, 3)       # done, return 3
```

The compiler sees nothing happens after the recursive call and transforms:

```
count-helper(target, current+1)
```

into essentially:

```
current = current + 1
goto start_of_function
```

## CS concept: Tail Call Optimization

From the glossary: *"Compiler optimization transforming tail-recursive calls into loops, preventing stack overflow while maintaining elegant recursive code structure."*

Many languages support TCO: Scheme (required by spec), Haskell, Erlang, Seq. Some don't: Python, Java (without workarounds).
