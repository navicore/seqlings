# Part 0: Introduction to Seq

Welcome to Seqlings! You're about to learn **stack-based programming** - a paradigm that predates and underpins much of modern computing.

## Why Stack-Based Programming Matters

Beneath every function call, every expression evaluation, and every local variable lies a **stack**. Your CPU has one. The JVM has one. Python's bytecode interpreter has one.

By learning Seq, you're not just learning a language - you're learning how computation actually works at its core.

## What You'll Learn in This Section

1. **Hello, Seq!** - Your first program, introducing `io.write-line`
2. **Comments** - Documenting your code
3. **Numbers** - How literal values become stack operations

## The Stack Mental Model

Imagine a stack of plates:
- You can only add plates to the **top** (push)
- You can only remove plates from the **top** (pop)
- You can look at the top plate, but not plates below without removing the ones above

In Seq, every value you write gets pushed onto this stack. Every operation pops its inputs and pushes its results.

```seq
5       # Push 5         Stack: ( 5 )
3       # Push 3         Stack: ( 5 3 )
i.add   # Pop 3 and 5, push 8   Stack: ( 8 )
```

This is **postfix notation** (also called Reverse Polish Notation). The operator comes after the operands. No parentheses needed, no operator precedence to remember.

## A Note on Learning

These exercises are designed to teach you programming concepts, not just Seq syntax. Pay attention to the **why**, not just the **how**. The patterns you learn here - stack manipulation, composition, higher-order programming - are universal.
