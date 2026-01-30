# Hint: Pattern Matching

## Stack flow

Before match: `( default Option )`
After swap:   `( Option default )` - wait, we need Option on top

Actually: `swap` gives us `( default Option )` with Option on top for match.

## Solution

```seq
: unwrap-or ( Option Int -- Int )
    swap match                    # ( default Option ) -> match consumes Option
        Some { >value } -> nip    # ( default value ) -> nip drops default
        None ->                   # ( default ) -> just return it
    end
;
```

- In the `Some` case: match extracts value, stack is `( default value )`, use `nip` to keep value
- In the `None` case: stack is just `( default )`, return it as-is
