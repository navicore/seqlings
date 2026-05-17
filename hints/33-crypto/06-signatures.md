# Hint: Ed25519 Digital Signatures

All three words are direct wraps of `crypto.ed25519-*` builtins. The stack juggling lives in the *test*, not the wrappers.

## Solution

```seq
: generate-keypair ( -- String String )
    crypto.ed25519-keypair
;

: sign ( String String -- String Bool )
    crypto.ed25519-sign
;

: verify ( String String String -- Bool )
    crypto.ed25519-verify
;
```

## Conventions

`generate-keypair` leaves `( public private )` — public on bottom, private on top. `sign` takes `( message private -- sig Bool )`. `verify` takes `( message sig public -- valid )`. The asymmetry mirrors real-world use: the private key signs (kept secret), the public key verifies (shared freely).
