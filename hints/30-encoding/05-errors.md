# Hint: Handling Decode Errors

After `encoding.base64-decode` (or `hex-decode`), the stack is `( decoded-or-empty Bool )`. Use `if` with two quotations: the success branch leaves the decoded String on the stack, the failure branch drops the empty placeholder and pushes `"ERROR"`.

## Solution

```seq
: safe-base64-decode ( String -- String )
    encoding.base64-decode [
        # Success - decoded string already on stack
    ] [
        drop "ERROR"
    ] if
;

: safe-hex-decode ( String -- String )
    encoding.hex-decode [
        # Success - decoded string already on stack
    ] [
        drop "ERROR"
    ] if
;
```

## How It Works

`if` consumes a Bool plus two quotations. The success branch is empty because the decoded String is already where we want it. The failure branch sees the empty placeholder String left by the failed decode and replaces it with `"ERROR"`.
