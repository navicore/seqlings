# Hint: Compression Levels

The `-level` variants take an extra Int on top. The string is already on the stack from the caller, so just push the level and call the builtin.

## Solution

```seq
: fast-gzip ( String -- String Bool )
    1 compress.gzip-level
;

: best-gzip ( String -- String Bool )
    9 compress.gzip-level
;

: fast-zstd ( String -- String Bool )
    1 compress.zstd-level
;
```

## Levels

- Gzip: 1 (fastest) to 9 (best ratio)
- Zstd: 1 (fastest) to 22 (best ratio)
