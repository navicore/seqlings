# Hint: Encoding Roundtrips

A roundtrip is just encode-then-decode. The decode's success Bool flows through as the word's Bool — no extra plumbing needed, because the encoded string sits on top of the stack exactly where decode expects it.

## Solution

```seq
: base64-roundtrip ( String -- String Bool )
    encoding.base64-encode
    encoding.base64-decode
;

: hex-roundtrip ( String -- String Bool )
    encoding.hex-encode
    encoding.hex-decode
;
```
