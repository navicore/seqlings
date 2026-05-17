# Hint: Secure Random Generation

Both words are direct passthroughs to the `crypto.*` builtins. `random-bytes` takes a byte count and returns that many bytes encoded as hex (so 16 bytes → 32 hex chars).

## Solution

```seq
: generate-token ( Int -- String )
    crypto.random-bytes
;

: generate-uuid ( -- String )
    crypto.uuid4
;
```
