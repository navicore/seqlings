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

Consider `list.map`, which applies a quotation to every element of a list:

```seq
"a b c" " " string.split [ string.to-upper ] list.map
# Result: list of "A" "B" "C"
```

The word `list.map` takes a quotation as an argument. It doesn't know or care what that quotation does — it just applies it to each element. This is **abstraction** at a higher level.

## Combinators

Words that operate on quotations are called **combinators**. The most fundamental one is built into the language:

| Word   | Effect              |
|--------|---------------------|
| `call` | Execute a quotation |

Higher-order combinators like `list.map`, `list.filter`, `list.fold`, and the loop combinators in `std:loops` (`times`, `each-integer`, `integer-fold`) all build on the same idea: take a quotation as data, decide when and how to run it. You'll meet them in later chapters.

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

## Concepts You'll Practice

| Concept | What You'll Learn |
|---------|-------------------|
| **Quotation** | Deferred code blocks that act as values |
| **Closure** | Functions that capture their surrounding context |
| **Higher-Order Functions** | Functions that take or return other functions |
| **Combinators** | Words that compose and manipulate quotations |

Quotations are Seq's implementation of **first-class functions** - the ability to treat code as data. This is foundational to functional programming and enables powerful abstractions like map, filter, and reduce.

The final exercise has you write your own higher-order function, crossing the threshold from *using* abstractions to *creating* them.

*For deeper exploration, see the [Seq Glossary](https://github.com/navicore/patch-seq/blob/main/docs/GLOSSARY.md).*
