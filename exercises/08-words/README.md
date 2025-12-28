# Part 8: Defining Words

Words are Seq's functions. Unlike traditional functions, words operate on the stack - they consume inputs from it and push outputs to it.

## The Anatomy of a Word

```seq
: word-name ( inputs -- outputs )
    body
;
```

- `:` starts the definition
- `word-name` is what you'll call it
- `( inputs -- outputs )` is the **stack effect** comment
- `body` is the code
- `;` ends the definition

## Stack Effect Comments

These comments document what a word does to the stack:

```seq
: double ( Int -- Int )      # Takes one Int, leaves one Int
    dup i.add
;

: add3 ( Int Int Int -- Int ) # Takes three Ints, leaves one
    i.add i.add
;

: greet ( -- )               # Takes nothing, leaves nothing
    "Hello!" io.write-line
;
```

## Why Words Matter

Words are the primary abstraction mechanism in Seq:

1. **Naming**: Give meaningful names to operations
2. **Reuse**: Call the same code from multiple places
3. **Composition**: Build complex operations from simple ones
4. **Documentation**: Stack effects make behavior explicit

## Composition Over Nesting

In most languages, you nest function calls: `f(g(h(x)))`.

In Seq, you compose: `x h g f`.

This reads left-to-right, matching how data flows through the stack. Each word transforms the stack, passing its output to the next.

## Building Vocabulary

Expert Seq programmers build up a vocabulary of words that make their code read almost like English:

```seq
user validate
if-valid -> process-request
else     -> show-error
cond
```

This is **domain-specific language** - using words to express intent at a high level.
