# Hint: Input Validation

Validators are anchored matchers — wrap the pattern in `^...$` so it must match the *entire* string, not just a substring. Then it's just `regex.match?`.

## Solution

```seq
: valid-email? ( String -- Bool )
    "^[^@]+@[^@]+\\.[^@]+$" regex.match?
;

: valid-phone? ( String -- Bool )
    "^\\d{10}$" regex.match?
;

: valid-username? ( String -- Bool )
    "^[a-zA-Z][a-zA-Z0-9_]{2,15}$" regex.match?
;
```

## Pattern Breakdown

- Email: `^[^@]+@[^@]+\.[^@]+$` — at least one non-`@` char, `@`, more non-`@`, literal `.`, more non-`@`. Naive but useful as a sanity check.
- Phone: `^\d{10}$` — exactly ten digits, nothing else.
- Username: `^[a-zA-Z][a-zA-Z0-9_]{2,15}$` — starts with a letter, then 2–15 more chars (so total length 3–16). The leading-letter anchor is what makes `"123user"` fail.
