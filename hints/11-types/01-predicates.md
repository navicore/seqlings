# Hint: Type-Specific Operators

Seq doesn't overload `+`. Each type has its own family of operators, named with a prefix that identifies the type they work on:

| Prefix       | Type    | Add operator   |
|--------------|---------|----------------|
| `i.`         | Int     | `i.+`          |
| `f.`         | Float   | `f.+`          |
| `string.`    | String  | `string.concat` (different concept, same idea) |

Notice that strings don't use a `+` because concatenation is conceptually different from addition — Seq keeps the naming honest. But the *convention* is the same: the prefix tells you which type the operator belongs to.

For each stub in this exercise, pick the prefix that matches the stack effect, then the operation name that means "combine these two." That's the entire lesson — the bodies are one token each.

## Why no overloading?

Languages with `+`-as-everything-overloaded need machinery (typeclasses, traits, ad-hoc polymorphism) to figure out which `+` to dispatch to. Seq sidesteps that by naming each one. Less magical, easier to read, and the compiler's type errors point straight at the prefix mismatch when you reach for the wrong one.
