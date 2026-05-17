# Hint: Hex Encoding

Direct wrappers over `encoding.hex-encode` and `encoding.hex-decode`. Encode always succeeds; decode returns a success Bool (false on non-hex characters or odd-length input).

## Solution

```seq
: to-hex ( String -- String )
    encoding.hex-encode
;

: from-hex ( String -- String Bool )
    encoding.hex-decode
;
```
