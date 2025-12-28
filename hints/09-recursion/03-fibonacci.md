# Hint: Fibonacci

fib(n) = fib(n-1) + fib(n-2)

## Solution

```seq
: fib ( Int -- Int )
    dup 1 <= if
        # Base cases: fib(0)=0, fib(1)=1
        # Conveniently, just return n itself!
    else
        dup 1 i.subtract fib
        swap 2 i.subtract fib
        i.add
    then
;
```

## Two Recursive Calls

This is the first time we've had two recursive calls. The stack management:
1. `dup 1 i.subtract fib` - compute fib(n-1), result stays on stack
2. `swap` - bring original n back to top
3. `2 i.subtract fib` - compute fib(n-2)
4. `i.add` - sum the two results

## A Note on Efficiency

This implementation is exponentially slow - it recalculates the same values over and over. The accumulator pattern or memoization can fix this.
