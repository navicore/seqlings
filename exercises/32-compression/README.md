# Chapter 32: Compression

Seq provides two compression algorithms: gzip (widely compatible) and zstd (modern, faster).

## Available Operations

| Operation | Stack Effect | Description |
|-----------|-------------|-------------|
| `compress.gzip` | `( String -- String Bool )` | Gzip compress (default level) |
| `compress.gzip-level` | `( String Int -- String Bool )` | Gzip with level 1-9 |
| `compress.gunzip` | `( String -- String Bool )` | Gzip decompress |
| `compress.zstd` | `( String -- String Bool )` | Zstd compress (default level) |
| `compress.zstd-level` | `( String Int -- String Bool )` | Zstd with level 1-22 |
| `compress.unzstd` | `( String -- String Bool )` | Zstd decompress |

## Output Format

Compressed data is returned as **base64-encoded** strings. This makes it safe to store in text formats like JSON or transmit over text protocols.

## Compression Levels

**Gzip** (1-9):
- Level 1: Fastest, least compression
- Level 6: Default balance
- Level 9: Best compression, slowest

**Zstd** (1-22):
- Level 1: Fastest
- Level 3: Default
- Level 22: Maximum compression

## When to Use Each

| Algorithm | Best For |
|-----------|----------|
| **Gzip** | Web (HTTP Accept-Encoding), broad compatibility |
| **Zstd** | Storage, internal data, when speed matters |

## Example

```seq
"Hello, World!" compress.gzip
if
  # compressed is base64-encoded
  compress.gunzip
  if
    io.write-line  # "Hello, World!"
  else
    drop "Decompress failed" io.write-line
  then
else
  drop "Compress failed" io.write-line
then
```

## Exercises

1. **01-gzip** - Basic gzip compression
2. **02-zstd** - Zstd compression
3. **03-levels** - Compression levels
4. **04-roundtrip** - Compress-decompress patterns
