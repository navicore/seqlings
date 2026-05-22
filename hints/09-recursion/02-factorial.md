# Hint: Factorial

Same recursion shape as `countdown` from the previous exercise — fill in the base and recursive case for `n!`:

- **Base case**: `0! = 1` and `1! = 1`. Easiest check covers both: "n <= 1 returns 1".
- **Recursive step**: `n × factorial(n-1)`.

The pattern transfers wholesale. If countdown worked, factorial is the same body shape with two changes: the base-case constant is 1 instead of 0, and the combining operator is `i.*` instead of `i.+`.

That's by design — this chapter teaches the *shape* of recursion, and the next exercises (fibonacci, accumulator) start varying it in interesting ways. Building the shape into muscle memory pays off there.
