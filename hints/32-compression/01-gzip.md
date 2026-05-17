# Hint: Gzip Compression

One-line wrappers over the `compress.*` builtins. Both return `( String Bool )` — the result (base64-encoded for compress, raw text for decompress) plus a success Bool.

## Solution

```seq
: gzip-compress ( String -- String Bool )
    compress.gzip
;

: gzip-decompress ( String -- String Bool )
    compress.gunzip
;
```
