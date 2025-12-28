# Hint: Time-Based Patterns

## Solution

```seq
: is-recent ( Int Int -- Bool )
    swap time.now swap i.- swap <
;
```
