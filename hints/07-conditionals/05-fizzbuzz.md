# Hint: FizzBuzz

FizzBuzz is a classic interview question. The key insight: check divisibility by 15 first!

## Solution

```seq
: fizzbuzz ( Int -- String )
    cond
        dup 15 i.mod 0 = -> drop "FizzBuzz"
        dup 3 i.mod 0 = -> drop "Fizz"
        dup 5 i.mod 0 = -> drop "Buzz"
        otherwise -> int->string
    end
;
```

## Why Check 15 First?

If you check 3 first, you'll never reach the "both" case:
- 15 is divisible by 3, so "Fizz" would be returned
- The check for "both 3 and 5" would never happen

By checking 15 (which is 3 × 5) first, we catch the "both" case.

## The Modulo Trick

`a b i.mod 0 =` checks if a is divisible by b:
- `15 3 i.mod` → 0 (divisible)
- `14 3 i.mod` → 2 (not divisible)
