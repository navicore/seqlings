# Chapter 30: Encoding

Binary-to-text encoding is essential for safely transmitting binary data over text-based protocols like HTTP, JSON, or email.

## Available Encodings

| Encoding | Encode | Decode | Use Case |
|----------|--------|--------|----------|
| **Base64** | `encoding.base64-encode` | `encoding.base64-decode` | Email attachments, data URIs |
| **Base64URL** | `encoding.base64url-encode` | `encoding.base64url-decode` | URLs, JWTs, filenames |
| **Hex** | `encoding.hex-encode` | `encoding.hex-decode` | Debugging, checksums, crypto |

## Stack Effects

```seq
# Encoding (always succeeds)
encoding.base64-encode    ( String -- String )
encoding.base64url-encode ( String -- String )
encoding.hex-encode       ( String -- String )

# Decoding (can fail on invalid input)
encoding.base64-decode    ( String -- String Bool )
encoding.base64url-decode ( String -- String Bool )
encoding.hex-decode       ( String -- String Bool )
```

## Base64 vs Base64URL

**Standard Base64** uses `+`, `/`, and `=` padding:
```
"Hello!" → "SGVsbG8h"
"a>b"    → "YT5i"
```

**URL-safe Base64** uses `-`, `_`, and no padding:
```
"Hello!" → "SGVsbG8h"
"a>b"    → "YT5i"
```

The URL-safe variant is preferred for:
- Query parameters
- JWT tokens
- Filenames
- Anywhere `+`, `/`, or `=` would cause problems

## Error Handling

Decode operations return a Bool indicating success:

```seq
"SGVsbG8h" encoding.base64-decode [
  io.write-line    # Use the decoded string
] [
  drop "Invalid base64" io.write-line
] if
```

## Exercises

1. **01-base64** - Basic Base64 encoding and decoding
2. **02-base64url** - URL-safe Base64 for web applications
3. **03-hex** - Hexadecimal encoding
4. **04-roundtrip** - Encode-decode roundtrip patterns
5. **05-errors** - Handling decode failures
