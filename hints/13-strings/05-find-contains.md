# Hint: Finding Substrings

The two checks in the test have the same shape — only the substring being searched differs.

For each check:

1. Push the haystack: `"stack-based programming"`.
2. Push the needle: `"stack"` for check 1, `"heap"` for check 2.
3. Call `string.contains`, which returns a Bool.

The placeholders (`false` for check 1, `true` for check 2) are there so the file lints clean; replace each placeholder line with the actual search expression that produces the Bool the assertion is checking.

`string.contains` is the YES/NO question. Its neighbor `string.find` returns the INDEX where the match starts (or -1 if missing) — useful when you need the location, not just existence. This exercise only needs the YES/NO answer.
