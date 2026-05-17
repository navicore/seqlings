# Hint: HMAC Authentication

`sign-message` is a direct wrap of `crypto.hmac-sha256`. The interesting word is `verify-signature`: with `( message key expected )` on the stack, you need to compute HMAC(message, key) and compare it to `expected` *in constant time* to avoid timing leaks.

## Solution

```seq
: sign-message ( String String -- String )
    crypto.hmac-sha256
;

: verify-signature ( String String String -- Bool )
    # ( message key expected -- valid? )
    rot rot           # ( expected message key )
    crypto.hmac-sha256  # ( expected computed )
    crypto.constant-time-eq
;
```

## Why Constant Time?

A naive `=` short-circuits on the first byte mismatch — an attacker can measure timing to learn how many leading bytes they guessed correctly. `crypto.constant-time-eq` always compares every byte.
