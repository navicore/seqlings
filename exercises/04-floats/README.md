# Part 4: Floating-Point Numbers

Seq distinguishes between integers and floating-point numbers at the type level. This section teaches you to work with floats and understand why the distinction matters.

## Why Separate Types?

Many languages silently convert between int and float. This convenience hides important differences:

- **Precision**: Floats can't represent all decimal values exactly
- **Performance**: Float operations are different from int operations
- **Semantics**: Division behaves differently (5/2 = 2 vs 5.0/2.0 = 2.5)

Seq makes these distinctions explicit, teaching you to think precisely about numeric types.

## Float Syntax

Float literals contain a decimal point:

```seq
3.14      # Float
42.0      # Float (note the .0)
42        # Integer (no decimal point)
```

## Float Operations

Float operations have an `f.` prefix:

| Word         | Effect                    | Description        |
|--------------|---------------------------|--------------------|
| `f.add`      | `( a b -- sum )`          | a + b              |
| `f.subtract` | `( a b -- diff )`         | a - b              |
| `f.multiply` | `( a b -- product )`      | a × b              |
| `f.divide`   | `( a b -- quotient )`     | a / b (real div)   |

## Type Conversions

To mix ints and floats, you must explicitly convert:

```seq
42 int->float 3.14 f.add    # Convert 42 to 42.0, then add
```

This explicitness prevents subtle bugs and teaches you to think about type boundaries.

## Floating-Point Gotchas

Be aware that floats have limited precision:

```seq
0.1 0.2 f.add    # Might not equal exactly 0.3!
```

This isn't a Seq quirk - it's how all IEEE 754 floating-point works. Understanding this is essential for any programmer.
