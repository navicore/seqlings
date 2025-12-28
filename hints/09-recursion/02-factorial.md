# Hint: Factorial

n! = n × (n-1)!

## Solution

```seq
: factorial ( Int -- Int )
    dup 1 <= if
        drop 1
    else
        dup 1 i.subtract factorial i.multiply
    then
;
```

## The Pattern

Same structure as countdown:
1. `dup` to preserve n
2. Check base case
3. Recursive call with n-1
4. Combine result with n
