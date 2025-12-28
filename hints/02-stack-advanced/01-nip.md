# Hint: nip

`nip` removes the second element, keeping only the top.

## Understanding nip

Think of `nip` as "keep the top, remove what's below it":

```seq
100 42 nip    # Stack: ( 42 )
```

## Composition Insight

`nip` is equivalent to `swap drop`:

```seq
100 42        # Stack: ( 100 42 )
swap          # Stack: ( 42 100 )
drop          # Stack: ( 42 )
```

Understanding these equivalences is important. In stack-based programming, complex operations decompose into simple primitives. This is **compositional thinking** - the same principle that powers functional programming.

## The Solution

Just add `nip` after the two values are pushed:

```seq
100 42
nip
```
