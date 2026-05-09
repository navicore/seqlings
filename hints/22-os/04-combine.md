# Hint: Building a Path Under Home

After `os.home-dir` your stack is `( filename home status )`.
The `if` runs over `( filename home )`:

- present: leave the stack alone — `home` is what you want
- missing: replace `home` with `"/tmp"`

Then swap so the filename is on top, and `os.path-join` does
the rest.

## Solution

```seq
: home-file ( filename -- String )
    os.home-dir
    [ ]
    [ drop "/tmp" ]
    if
    swap os.path-join
;
```
