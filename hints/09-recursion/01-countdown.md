# Hint: Countdown

Sum the numbers from n down to 1.

## Solution

```seq
: countdown ( Int -- Int )
    dup 0 <= if
        drop 0
    else
        dup 1 i.- countdown i.+
    then
;
```

## How It Works

1. Check if n <= 0 (base case)
2. If yes, return 0
3. If no, compute n + countdown(n-1)

The key insight: `dup` before the recursive call keeps n available for the final addition.
