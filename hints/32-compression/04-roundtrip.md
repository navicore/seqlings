# Hint: Compression Roundtrips

The pattern matches `30-encoding/05-errors.md`: call compress, branch on its Bool with `if`. The success quotation runs decompress (which produces its own `String Bool`); the failure quotation drops the empty placeholder and returns `"" false`.

## Solution

```seq
: safe-gzip-roundtrip ( String -- String Bool )
    compress.gzip [
        compress.gunzip
    ] [
        drop "" false
    ] if
;

: safe-zstd-roundtrip ( String -- String Bool )
    compress.zstd [
        compress.unzstd
    ] [
        drop "" false
    ] if
;
```

## Why if?

A naive `compress.gzip compress.gunzip` would leave two Bools on the stack and read the wrong String into `gunzip` if compression failed. The `if` collapses to a single `( String Bool )` either way.
