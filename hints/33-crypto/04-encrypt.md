# Hint: AES-GCM Encryption

The user-facing words are wrappers — the test does the interesting stack work itself (juggling the key alongside ciphertext to demonstrate the round trip and the wrong-key failure case).

## Solution

```seq
: encrypt ( String String -- String Bool )
    crypto.aes-gcm-encrypt
;

: decrypt ( String String -- String Bool )
    crypto.aes-gcm-decrypt
;
```

## Why GCM?

AES-GCM is *authenticated* encryption: the ciphertext includes a tag that lets `decrypt` detect tampering or wrong keys. That's why the wrong-key test produces a `false` Bool rather than returning garbage plaintext.
