# Hint: FizzBuzz

The classic gotcha: check divisibility by 15 BEFORE checking 3 or 5.

If you check 3 first, you'll never reach the "both" case — 15 is divisible by 3, so "Fizz" would always win for multiples of 15. Checking 15 (= 3 × 5) first catches the "both" case before either smaller divisor.

To check divisibility:
- Use `i.%` to compute the remainder, drop the success Bool
- Compare the remainder to 0

Since you'll need the same number for multiple checks, `dup` it first so each comparison can consume its own copy.
