# Hint: FizzBuzz

FizzBuzz is a classic interview question. The key insight: check divisibility by 15 first!

## Solution

```seq
: fizzbuzz ( Int -- String )
    dup 15 i.% 0 i.= if
        drop "FizzBuzz"
    else
        dup 3 i.% 0 i.= if
            drop "Fizz"
        else
            dup 5 i.% 0 i.= if
                drop "Buzz"
            else
                int->string
            then
        then
    then
;
```

## Why Check 15 First?

If you check 3 first, you'll never reach the "both" case:
- 15 is divisible by 3, so "Fizz" would be returned
- The check for "both 3 and 5" would never happen

By checking 15 (which is 3 × 5) first, we catch the "both" case.

## The Modulo Trick

`a b i.% 0 i.=` checks if a is divisible by b:
- `15 3 i.%` → 0 (divisible)
- `14 3 i.%` → 2 (not divisible)
