# Hint: f.+

`f.+` works just like `i.+`, but for floats.

## The Pattern

```seq
a b f.+    # Leaves (a + b) as a float
```

## Solution

```seq
3.5 1.5 f.+
```

## Why Separate Operations?

You might wonder why Seq has `i.+` and `f.+` instead of just `add`.

The answer: **explicit is better than implicit**. When you write `f.+`, you know:
- Both operands must be floats
- The result will be a float
- Float arithmetic rules apply (including precision limits)

This clarity prevents bugs and teaches you to think about types.
