# Hint: Predicates

## empty?

Pull the variant's tag, compare to `:Empty`:

```seq
: empty? ( Variant -- Bool )
    variant.tag :Empty symbol.=
;
```

`variant.tag` consumes the variant and pushes its symbol.
`symbol.=` consumes two symbols and pushes a Bool. The whole thing
typechecks because `:Empty` is a symbol literal, just like the one
`empty` used to construct the variant.

## Why only one predicate?

`cons?` would just be `empty? not`. Once you have one, you have
both. In real code people often define both so call sites read
naturally (e.g. `[ ... ] when` reads better as `cons? when` than
`empty? not when`), but for this exercise one is enough.

## Why this works regardless of the list contents

`variant.tag` doesn't look inside the variant — it only reads the
constructor label. An `empty` list and a million-element `Cons`
list produce the same answer in O(1).
