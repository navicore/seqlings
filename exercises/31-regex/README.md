# Chapter 31: Regular Expressions

Regular expressions are patterns for matching text. Seq's regex module is powered by Rust's `regex` crate - fast, safe, and no catastrophic backtracking.

## Available Operations

| Operation | Stack Effect | Description |
|-----------|-------------|-------------|
| `regex.match?` | `( String String -- Bool )` | Does pattern match anywhere? |
| `regex.find` | `( String String -- String Bool )` | Find first match |
| `regex.find-all` | `( String String -- List )` | Find all matches |
| `regex.replace` | `( String String String -- String )` | Replace first match |
| `regex.replace-all` | `( String String String -- String )` | Replace all matches |
| `regex.captures` | `( String String -- List Bool )` | Extract capture groups |
| `regex.split` | `( String String -- List )` | Split by pattern |
| `regex.valid?` | `( String -- Bool )` | Is pattern valid? |

## Pattern Syntax

Common patterns:
- `.` - Any character
- `\d` - Digit (0-9)
- `\w` - Word character (a-z, A-Z, 0-9, _)
- `\s` - Whitespace
- `[abc]` - Character class
- `[^abc]` - Negated class
- `*` - Zero or more
- `+` - One or more
- `?` - Zero or one
- `{n,m}` - Between n and m
- `^` - Start of string
- `$` - End of string
- `(...)` - Capture group

## Escaping

In Seq strings, backslashes must be escaped:
```seq
"\\d+"        # Pattern for one or more digits
"\\s+"        # Pattern for whitespace
"[a-z]+"      # No escaping needed for character classes
```

## Exercises

1. **01-match** - Basic pattern matching
2. **02-find** - Finding matches in text
3. **03-replace** - Search and replace
4. **04-captures** - Extracting groups
5. **05-split** - Splitting strings
6. **06-validate** - Input validation patterns
