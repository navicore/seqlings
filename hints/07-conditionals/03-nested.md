# Hint: Nested Conditionals

You can put one `if` inside another's branches. The brackets help: each `[ ... ]` is one branch, and you can see exactly where each conditional ends.

Pattern for a three-way choice: outer `if` for the first condition, inner `if` for the remaining two cases inside the outer's else-branch.

Each `dup`-then-compare-then-`if` pattern lets you check a value without losing it for the next check — `dup` makes the copy, the comparison consumes it, the original stays.
