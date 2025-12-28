# Part 10: Quotations and Higher-Order Programming

**This is one of the most important concepts in all of programming.**

Quotations are Seq's mechanism for **higher-order programming** - the ability to treat functions as data. This section makes the concept explicit so you can apply it deliberately.

## What is Higher-Order Programming?

A **higher-order function** is a function that:
1. Takes a function as an argument, OR
2. Returns a function as a result

This simple idea is extraordinarily powerful. It's the foundation of:
- Callbacks and event handlers
- Map, filter, reduce operations
- Decorators and middleware
- Dependency injection
- Strategy and visitor patterns
- And much more...

## Quotations: Functions as Values

In Seq, a quotation is a block of code that can be passed around:

```seq
[ 1 i.add ]    # A quotation that adds 1
```

This is NOT executed immediately. It's a **value** - a piece of code wrapped up as data.

## The `call` Word

To execute a quotation, use `call`:

```seq
5 [ 1 i.add ] call    # Stack: ( 6 )
```

The quotation `[ 1 i.add ]` is passed to `call`, which executes it with 5 on the stack.

## Why This Matters

Consider `times`, which runs a quotation n times:

```seq
5 [ "Hello!" io.write-line ] times
```

The word `times` takes a quotation as an argument. It doesn't know or care what that quotation does - it just runs it 5 times. This is **abstraction** at a higher level.

## Combinators

Words that operate on quotations are called **combinators**. Key combinators:

| Word    | Effect                                   |
|---------|------------------------------------------|
| `call`  | Execute a quotation                      |
| `times` | Run a quotation n times                  |
| `while` | Run while condition is true              |
| `until` | Run until condition becomes true         |

## The Deep Insight

When you pass `[ 1 i.add ]` to another word, you're saying:
> "Here is some behavior. Do with it what you will."

The receiving word controls **when** and **how often** to run that behavior. This separation of "what to do" from "when to do it" is the core of higher-order programming.

This pattern appears everywhere in modern programming:
- JavaScript: `array.map(x => x + 1)`
- Python: `map(lambda x: x + 1, list)`
- Rust: `iter.map(|x| x + 1)`
- Seq: `list [ 1 i.add ] list.map`

Seq makes this concept explicit and first-class.
