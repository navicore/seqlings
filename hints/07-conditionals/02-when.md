# Hint: when

When you only need an action in one case, reach for `when` instead of two-armed `if` with an empty branch.

The body of `when` must leave the stack the same shape it found — same number of items, same types. Replacing one Int with another counts as preserving shape; pushing or popping an extra item does not.

Don't forget the `include std:control` at the top of the file (already there for you in this exercise) — `when` and `unless` live in that library, not in the core builtins.
