# Hint: Splitting Strings

The body for this exercise has just two operations in sequence: push the source string and the delimiter, then call `string.split`. That's it — the assertion does the length check itself.

The delimiter for this exercise is `":"` — a single colon as a string literal, not a character. Seq doesn't have a separate `Char` type, so delimiters are always strings even when they're one character long.

`string.split` produces a list (in this case a 3-element list: `"red"`, `"green"`, `"blue"`). The assertion line — which you don't edit — calls `list.length` on it and compares against 3, so your body only needs to leave the list on the stack.

We'll work with lists themselves in chapter 15; for now, treating `string.split`'s output as an opaque "thing of length 3" is enough.
