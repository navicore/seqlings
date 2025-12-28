# Hint: Building a Map Workflow

## Solution

```seq
: make-profile ( String Int String -- Map )
    # Stack: name age email
    # We need to build a map with all three
    map.make
    rot rot rot         # Bring email to accessible position
    "email" swap map.set
    swap                # Get age accessible
    "age" swap map.set
    swap                # Get name accessible
    "name" swap map.set
;

: update-age ( Map Int -- Map )
    "age" swap map.set
;
```

The tricky part is juggling the stack. An alternative approach:

```seq
: make-profile ( String Int String -- Map )
    # email on top, then age, then name at bottom
    >r >r               # Stash name and age on return stack
    map.make
    "email" swap map.set
    r> "age" swap map.set
    r> "name" swap map.set
;
```

If return stack isn't available, the rot approach works.
