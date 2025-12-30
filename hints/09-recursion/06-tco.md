# Hint: Tail Call Optimization

The key is making the recursive call the **last** action.

## Solution

```seq
: count-helper ( Int Int -- Int )
    # Stack: ( target current )
    2dup i.<= if
        # target <= current means we're done
        nip                  # drop target, return current
    else
        1 i.+                # ( target current+1 )
        count-helper         # tail call - MUST be last action!
    then
;

: count-up-to ( Int -- Int )
    0 count-helper
;
```

## Why This Works for 100,000

In non-tail recursion:
```
sum(3) = 3 + sum(2)           # must WAIT for sum(2), then add
       = 3 + (2 + sum(1))     # stack grows with each call
       = 3 + (2 + (1 + sum(0)))
       = 3 + (2 + (1 + 0))    # finally unwind all those frames
```

In tail recursion:
```
count-helper(3, 0)
  -> count-helper(3, 1)       # no waiting - just jump!
  -> count-helper(3, 2)       # same stack frame, new values
  -> count-helper(3, 3)       # done, return 3
```

## The "Aha" Moment

**Tail recursion + TCO = recursion as efficient as a while loop.**

The compiler sees that nothing happens after the recursive call, so it transforms:
```
count-helper(target, current+1)
```
into essentially:
```
current = current + 1
goto start_of_function
```

This is why functional languages love recursion - with TCO, it's just as efficient as imperative loops, but often clearer to reason about.

## CS Concept: Tail Call Optimization

From the glossary: *"Compiler optimization transforming tail-recursive calls into loops, preventing stack overflow while maintaining elegant recursive code structure."*

Many languages support TCO: Scheme (required by spec), Haskell, Erlang, Seq. Some don't: Python, Java (without workarounds).
