# Part 9: Recursion

Recursion is a function calling itself. It's one of the fundamental techniques in programming, and understanding it deeply will change how you think about problem-solving.

## The Structure of Recursion

Every recursive solution has two parts:

1. **Base case**: When to stop (prevents infinite recursion)
2. **Recursive case**: How to break down the problem and call yourself

```seq
: countdown ( Int -- )
    dup 0 <= if
        drop                    # Base case: stop at 0
    else
        dup io.write-line       # Do something
        1 i.subtract countdown  # Recursive case: call self with smaller input
    then
;
```

## Why Recursion Matters

Recursion naturally expresses problems that have recursive structure:
- Trees (a tree is a node with subtrees)
- Lists (a list is an element followed by a list)
- Mathematical sequences (Fibonacci, factorial)
- Divide-and-conquer algorithms

## Recursion and the Stack

Each recursive call adds a frame to the call stack. This is why:
- Deep recursion can cause stack overflow
- Tail recursion can be optimized (next topic)

In Seq, you're already comfortable with stacks - recursion is just another way the stack is used.

## Thinking Recursively

The key insight: **assume the recursive call works correctly**, then figure out how to use that result.

For factorial:
- Assume `(n-1)!` gives the right answer
- Then `n!` is just `n × (n-1)!`

This "leap of faith" is how recursive thinking works.

## Tail Recursion

A tail-recursive function calls itself as its **last action**. These can be optimized to use constant stack space:

```seq
# Not tail-recursive (multiply happens AFTER recursive call returns)
: factorial ( n -- result )
    dup 1 <= if drop 1 else dup 1 - factorial i.multiply then
;

# Tail-recursive (recursive call is the last thing)
: factorial-tail ( n acc -- result )
    over 1 <= if nip else swap dup 1 - swap rot i.multiply factorial-tail then
;
```

Tail recursion connects recursion to iteration - they're two ways of expressing the same thing.
