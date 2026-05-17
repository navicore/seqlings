# Hint: Zstd Compression

Same shape as the gzip exercise — wrap `compress.zstd` and `compress.unzstd` directly.

## Solution

```seq
: zstd-compress ( String -- String Bool )
    compress.zstd
;

: zstd-decompress ( String -- String Bool )
    compress.unzstd
;
```
