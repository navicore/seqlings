# Hint: Setting Environment Variables

## Solution

```seq
: set-and-check ( String String -- Bool )
    os.env-set
    os.env-has?
;
```
