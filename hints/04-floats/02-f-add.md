# Hint: f.add

`f.add` works just like `i.add`, but for floats.

## The Pattern

```seq
a b f.add    # Leaves (a + b) as a float
```

## Solution

```seq
3.5 1.5 f.add
```

## Why Separate Operations?

You might wonder why Seq has `i.add` and `f.add` instead of just `add`.

The answer: **explicit is better than implicit**. When you write `f.add`, you know:
- Both operands must be floats
- The result will be a float
- Float arithmetic rules apply (including precision limits)

This clarity prevents bugs and teaches you to think about types.
