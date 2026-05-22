# Hint: String Transformations

Each of the two checks needs one transformation applied to the input:

- **Check 1**: turn `"seq"` into `"SEQ"`. Which `string.*` operator does that?
- **Check 2**: turn `"  hello  "` into `"hello"`. Whitespace at both ends needs to go.

The three relevant words are listed in the exercise prose. Match each to its test by what it does. Each body is two tokens: push the input literal, apply the transformer.

The `string.equal?` assertion that follows expects the transformed string on top of the stack. Replace the `""` placeholder with your expression — don't add to it, don't keep it around.
