# Hint: File Existence

## file.exists?

Pushes a Bool indicating whether the path resolves to a real file:

```
( path -- Bool )
```

Use it as the condition in an `if`, with the two branches pushing
different result strings:

```seq
: present? ( String -- String )
    file.exists?
    [ "yes" ]
    [ "no" ]
    if
;
```

## When to use it (and when not to)

`file.slurp` already returns its own success Bool, so you don't need
to call `file.exists?` first just to read safely. Use `file.exists?`
when *the meaning* of "file is there" or "file is missing" is part
of your control flow — for example, "load this config if present,
otherwise fall back to defaults".
