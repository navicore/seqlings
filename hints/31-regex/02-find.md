# Hint: Finding Matches

Two regex words with subtly different return shapes:

- `regex.find ( String String -- String Bool )` — input string, pattern, returns the first match string AND a Bool that's `false` if no match was found.
- `regex.find-all ( String String -- List Bool )` — same input shape, but returns a list of all matches AND a Bool that's `false` only if the regex itself was invalid (not if zero matches were found — an empty list is the right answer there).

The two functions in this exercise mirror those signatures:

- `find-first-number` keeps both return values intact — its declared stack effect is `( String -- String Bool )`, matching `regex.find`. So the body is just the pattern + `regex.find`.
- `find-all-words` drops the success Bool, keeping only the list — its declared effect is `( String -- Variant )`. So the body is the pattern + `regex.find-all` + `drop`.

## The patterns themselves

- `\d+` — one or more digits. The `+` is a regex quantifier meaning "one or more of the previous element."
- `\w+` — one or more word characters (letters, digits, underscore).

In Seq string literals, the backslash must be doubled: `"\\d+"` is the string `\d+`, which the regex engine then parses as the pattern. This is the same escaping rule as most languages — the string literal layer and the regex layer each get one bite at the backslash.
