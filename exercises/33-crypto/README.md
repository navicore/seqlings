# Chapter 33: Cryptography

Seq provides cryptographic primitives for secure applications. These are built on industry-standard algorithms.

## Hashing

```seq
crypto.sha256 ( String -- String )   # SHA-256 hash (hex output)
```

## Message Authentication (HMAC)

```seq
crypto.hmac-sha256 ( String String -- String )  # ( message key -- signature )
crypto.constant-time-eq ( String String -- Bool ) # Timing-safe comparison
```

## Random Generation

```seq
crypto.random-bytes ( Int -- String )  # Generate N random bytes (hex)
crypto.uuid4 ( -- String )             # Generate UUID v4
```

## Encryption (AES-256-GCM)

```seq
crypto.aes-gcm-encrypt ( String String -- String Bool )  # ( message key -- ciphertext ok )
crypto.aes-gcm-decrypt ( String String -- String Bool )  # ( ciphertext key -- plaintext ok )
```

Keys must be 32 bytes (64 hex characters).

## Key Derivation (PBKDF2)

```seq
crypto.pbkdf2-sha256 ( String String Int -- String Bool )  # ( password salt iterations -- key ok )
```

## Digital Signatures (Ed25519)

```seq
crypto.ed25519-keypair ( -- String String )              # ( -- public private )
crypto.ed25519-sign ( String String -- String Bool )     # ( message private -- signature ok )
crypto.ed25519-verify ( String String String -- Bool )   # ( message signature public -- valid )
```

## Security Notes

- **Never** hardcode keys in source code
- Use `crypto.constant-time-eq` to compare secrets (prevents timing attacks)
- Use at least 100,000 iterations for PBKDF2
- AES-GCM provides authenticated encryption (integrity + confidentiality)

## Exercises

1. **01-sha256** - Hashing data with SHA-256
2. **02-hmac** - Message authentication codes
3. **03-random** - Secure random generation
4. **04-encrypt** - AES-GCM encryption/decryption
5. **05-pbkdf2** - Password-based key derivation
6. **06-signatures** - Ed25519 digital signatures
