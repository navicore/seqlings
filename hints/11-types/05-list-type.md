# Hint: List Operations

The big idea this exercise plants: **lists are variants under the hood.** That's why list indexing uses `variant.field-at` — there's no separate `list.at`, because a list IS a variant whose fields are its elements.

So to grab an element, the recipe is:

1. Get the list onto the stack (the exercise uses `string.split` to produce one from a string).
2. Push the index you want (0-based).
3. `variant.field-at` — pops both, leaves the element.

That's the entire pattern. The first-element test needs index 0; the last-element test (for the 3-element list "alpha beta gamma") needs index 2 — length minus one.

The list-length and list-empty? tests are even simpler — those words take just the list and return their answer. The exercise prose lists them.

## The variant-list connection

Most languages keep lists and tagged unions as separate concepts. Seq merges them: a list is a variant tagged `:Cons` (with head and tail fields) or `:Empty` (no fields), and the same `variant.*` primitives work on both. You'll see this pattern again in chapter 38 when you build a cons-list from scratch.

For now, the takeaway: when you need to reach into a list, reach for `variant.field-at`.
