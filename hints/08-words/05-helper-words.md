# Hint: Predicates and `i.modulo`

A number is even when its remainder mod 2 is zero. The trap: `i.modulo` doesn't just return the remainder — it returns `( remainder Bool )`, where the Bool is a success flag (false on divide-by-zero, which can't happen here, but the type system insists). You need to discard that Bool before comparing the remainder to 0.

So the body is three short pieces in sequence: compute the modulo, drop the flag, compare. Each piece is one or two tokens you already know.

## Naming Conventions

The `?` at the end of `is-even?` is the convention for predicates — words that return a Bool. Other examples you'll meet later:

- `string.empty?`
- `list.contains?`
- `file.exists?`

The trailing `?` is purely a naming convention; the compiler doesn't care. But human readers do — `is-even?` reads as a question that gets a yes/no answer, which is exactly what a predicate is.
