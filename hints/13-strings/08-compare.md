# Hint: Case-Insensitive Comparison

```seq
: equal-ignore-case? ( String String -- Bool )
    string.to-lower swap string.to-lower string.equal?
;
```
