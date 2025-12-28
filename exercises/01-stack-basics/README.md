# Part 1: Stack Basics

Stack manipulation is the foundation of all Seq programming. These operations may seem primitive, but mastering them teaches you to think about data flow explicitly.

## Why This Matters

In most languages, variables hide the flow of data. You write `x = f(y)` and don't think about where `y` comes from or where `x` goes. This abstraction is useful, but it also hides what's really happening.

Stack-based programming makes data flow **explicit**. Every value has a clear origin and destination. This discipline carries over to any language - you'll write cleaner code because you'll think more clearly about data.

## The Core Operations

| Word   | Effect           | Description                    |
|--------|------------------|--------------------------------|
| `dup`  | `( a -- a a )`   | Duplicate the top value        |
| `drop` | `( a -- )`       | Discard the top value          |
| `swap` | `( a b -- b a )` | Exchange top two values        |
| `over` | `( a b -- a b a )`| Copy second to top            |
| `rot`  | `( a b c -- b c a )`| Rotate third to top          |

## Reading Stack Effects

The notation `( a b -- c d )` means:
- **Before**: `a` is second from top, `b` is on top
- **After**: `c` is second from top, `d` is on top

Letters like `a`, `b`, `c` represent values. Same letter = same value.

## Building Intuition

Think of each operation as a physical manipulation:
- `dup` = photocopy the top plate
- `drop` = throw away the top plate
- `swap` = flip the top two plates
- `over` = peek at the second plate and make a copy on top
- `rot` = pull the third plate up through the other two

With practice, you'll stop thinking about individual operations and start thinking about data flow patterns.
