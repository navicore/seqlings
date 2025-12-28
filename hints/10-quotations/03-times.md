# Hint: The times Combinator

`times` runs a quotation n times.

## Solution

```seq
1
3 [ dup i.add ] times
```

Trace:
- Start: 1
- After 1st: 1+1 = 2
- After 2nd: 2+2 = 4
- After 3rd: 4+4 = 8

## Declarative vs Imperative

Compare to a traditional loop:

```javascript
// Imperative (how to loop)
let x = 1;
for (let i = 0; i < 3; i++) {
    x = x * 2;
}

// Higher-order (what to do)
1 3 [ dup i.add ] times
```

The higher-order version says WHAT (double) and HOW MANY (3), not HOW to loop.

## The Combinator Pattern

`times` is a **combinator** - a higher-order function with a standard behavior pattern. Other combinators: `map`, `filter`, `fold`, `while`, `until`. Learning these patterns is learning to think in higher-order programming.
