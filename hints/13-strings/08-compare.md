# Hint: Case-Insensitive Comparison

Two strings are equal ignoring case when their **lowercased forms** are equal. The recipe:

1. Convert one string to lowercase.
2. Convert the other string to lowercase.
3. Compare the two lowercased strings with `string.equal?`.

The stack effect is `( String String -- Bool )` — two strings in, one Bool out. The body needs to do step 1 on whichever string is on top, get the other string to the top (a `swap` does that), do step 2, then compare.

Lowercasing the top string first is just one valid order — you could swap before any lowercasing instead. Either way the body is the same four primitives in some order.

## Why this is a common need

Case-insensitive comparison comes up everywhere: usernames, file paths (on Windows/Mac), HTTP headers. The "lowercase both then compare" recipe is the standard solution in most languages — Seq just makes the data flow explicit.
