# Hint: Implementing `keep`

The stack going in is `( ..a A quot )`. The stack going out is `( ..b A )` — whatever the quotation produced, with the original `A` restored on top. To pull that off you need a copy of `A` that survives the call.

The dance, in plain English:

1. The original `A` is under the quotation right now. Make a copy of it on top — `over` is built exactly for this.
2. That copy is now blocking the quotation, which needs to be on top to `call` it. Swap them.
3. Run the quotation. It consumes one copy of `A` (and whatever else of `..a` it touches), leaving its result on top.
4. The other copy of `A` — the one you stashed in step 1 — is now below the result. Swap them to put the original on top, as the signature demands.

Four primitives, in that order. No `dup`, no `dip`, no aux stack — just `over`, `swap`, `call`, `swap`.

## The row polymorphism insight

Look at the type signature:

```
( ..a A [ ..a A -- ..b ] -- ..b A )
```

- `A` is a **type variable** — any single type.
- `..a` and `..b` are **row variables** — any stack tail.

This is what lets one definition of `keep` work for Int, String, Bool, custom variants, anything. The quotation specifies what it needs (`..a A` on top), and the surrounding stack `..a` passes through untouched. The compiler still type-checks every call site — it just does so with these variables instantiated to the actual types.

That's *row polymorphism*: the function works on any *row* of values below the parts it actually touches.

## CS concept

From the glossary: *"A type system feature allowing functions to work with stacks of any depth, as long as required types appear on top, providing flexibility without sacrificing type safety."*

Most languages handle this with generics (explicit type parameters) or dynamic typing (runtime checks). Row polymorphism is the third path — implicit type variables, but still checked statically.
