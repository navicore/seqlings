# Hint: Current Working Directory

Same shape as `getenv-or` from exercise 01: the lookup returns
`( path status )`, the fallback sits underneath, branch on the
flag, return whichever string ends up on top.

## Solution

```seq
: cwd-or ( fallback -- String )
    os.current-dir
    [ nip ]
    [ drop ]
    if
;
```
