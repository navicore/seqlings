# Hint: Refactoring

"Add 1, then double" is the pattern.

## Solution

```seq
: increment-double ( Int -- Int )
    1 i.+ dup i.+
;
```

Or equivalently:
```seq
: increment-double ( Int -- Int )
    1 i.+ 2 i.*
;
```

## The Refactoring Mindset

When you see a pattern repeated, ask:
1. Can I name this pattern?
2. What would a clear name be?
3. How can I parameterize it if the pattern varies slightly?

This mindset - seeing patterns and abstracting them - is the core skill of programming.
