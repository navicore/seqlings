# Hint: URL-Safe Base64

Same shape as the standard base64 wrappers — just call the `-url` variants. URL-safe base64 uses `-` and `_` instead of `+` and `/`, and omits the `=` padding.

## Solution

```seq
: make-token ( String -- String )
    encoding.base64url-encode
;

: parse-token ( String -- String Bool )
    encoding.base64url-decode
;
```
