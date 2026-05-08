# Cons Lists — Putting ADTs to Good Use

Chapter 14 taught the `variant.*` primitives. This chapter is where
they earn their keep: we use them to *design* a real algebraic data
type from scratch — a recursive list — and write its operations.

The cons-list is just one example. The pattern you learn here
generalizes to any ADT: a **constructor** word per variant, **tag
predicates** for branching, **field accessors** with
`variant.field-at`, and **recursive operations** that pattern-match
via the predicates. The same recipe builds binary trees, option /
result types, AST nodes — anything you'd reach for `enum` for in
Rust or a tagged union for in C.

## What is a cons list?

A linked list built from two variants:

- **Empty** — the empty list, holds no payload
- **Cons (head, tail)** — a pair of an element and the rest of the list

The list `1 2 3` is `Cons(1, Cons(2, Cons(3, Empty)))`. Every
operation either bottoms out at `Empty` or recurses through `Cons`.

## A note on names

You will see this exact data structure in Lisp, Scheme, OCaml,
Haskell, and Erlang under different names:

| This chapter | Lisp / Scheme | Haskell / OCaml |
|--------------|---------------|-----------------|
| `empty`      | `nil` / `'()` | `[]` / `Nil`    |
| `prepend`    | `cons`        | `:` / `Cons`    |
| `head`       | `car`         | `head` / `hd`   |
| `tail`       | `cdr`         | `tail` / `tl`   |
| `empty?`     | `null?`       | `null` / `is_empty` |

We use the descriptive names because they're easier to read at
first contact, but the Lisp names (`nil`, `cons`, `car`, `cdr`) are
older and more universal — you'll meet them in any FP code. They
mean exactly the same thing as ours.

## Building lists

Because `prepend` has stack effect `( T List -- List )`, building a
list literal looks like this:

```seq
empty             # ( () )
3 swap prepend    # ( (3) )
2 swap prepend    # ( (2 3) )
1 swap prepend    # ( (1 2 3) )
```

The `swap` is there because we naturally write the element first,
but `prepend` wants the list on top.

## What you build

Six exercises. Each one's prelude carries forward the words you
defined in earlier exercises so you don't keep retyping them, but
the *new* word in each file is yours to write.

1. **constructors** — `empty` and `prepend` from variant primitives
2. **predicates** — `empty?` via `variant.tag` and `symbol.=`
3. **accessors** — `head` and `tail` via `variant.field-at`
4. **length** — your first recursive op, using an accumulator
5. **reverse** — recursion + stack juggling
6. **append** — recursive list construction, using `dip`

When you finish, you'll have implemented a recursive data type by
hand, which is the same skill you'd use to build a parse tree, an
AST, or any other shape your code actually needs.

## Stack effects (cheat sheet)

| Word        | Stack Effect          |
|-------------|-----------------------|
| `empty`     | `( -- List )`         |
| `prepend`   | `( T List -- List )`  |
| `empty?`    | `( List -- Bool )`    |
| `head`      | `( List -- T )`       |
| `tail`      | `( List -- List )`    |
| `length`    | `( List -- Int )`     |
| `reverse`   | `( List -- List )`    |
| `append`    | `( List List -- List )` |
