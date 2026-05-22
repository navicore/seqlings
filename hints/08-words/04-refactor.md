# Hint: Refactoring

The pattern "add 1, then double" decomposes into two simple steps you already know:

1. **Add 1** — push 1, then add. Two tokens.
2. **Double** — either multiply by 2, or copy the value and add it to itself. Both work; pick whichever reads more naturally to you.

The body is those two pieces in sequence inside the word definition. The test calls `5 increment-double` and expects 12 — `(5+1)*2`.

## The Refactoring Mindset

When you see a pattern repeated, ask:

1. Can I name this pattern?
2. What would a clear name be?
3. How can I parameterize it if the pattern varies slightly?

This mindset — seeing patterns and abstracting them — is the core skill of programming. The actual arithmetic in `increment-double` is trivial; the lesson is recognizing the pattern and giving it a name so the next time you see it, you don't have to rewrite the steps.
