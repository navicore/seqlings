# Hint: PBKDF2 Key Derivation

`crypto.pbkdf2-sha256` takes `( password salt iterations -- key Bool )`. The user-facing `derive-key` hardcodes the iteration count, so just push 100000 and call the builtin.

## Solution

```seq
: derive-key ( String String -- String Bool )
    100000 crypto.pbkdf2-sha256
;
```

## Why So Many Iterations?

PBKDF2's whole job is to be *slow*. 100,000 rounds takes ~100ms on a modern CPU — imperceptible to a legitimate user logging in once, but a brutal multiplier for an attacker trying billions of passwords. The salt prevents pre-computed rainbow-table attacks.
