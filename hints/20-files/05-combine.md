# Hint: File Workflow

## Counting Lines

Split on newlines and get the length:

```seq
: count-lines ( String -- Int )
    "\n" string.split list.length
;
```

## Adding a Header

Use `string.concat` to join strings:

```seq
: add-header ( String String -- String )
    # Stack: content header
    "\n" string.concat    # header + newline
    swap string.concat    # (header + newline) + content
;
```
