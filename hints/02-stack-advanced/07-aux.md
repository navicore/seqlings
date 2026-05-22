# Hint: aux stack

The aux stack is a second stack, private to the current word, useful for parking values out of the main stack's way.

## The Operations

| Word   | Effect on main stack | Effect on aux stack |
|--------|----------------------|---------------------|
| `>aux` | pop top              | push that value     |
| `aux>` | push the value       | pop top             |

A handy mnemonic: the `>` points from where the value *was* toward where it's *going*. `>aux` moves it to aux; `aux>` moves it from aux back.

## Walking the Solution

Start: `( 10 20 30 )` — call these `a b c`. You want `( a+c b )` = `( 40 20 )`.

```seq
10 20 30        ( 10 20 30 )           aux: []
swap            ( 10 30 20 )           aux: []     bring b to top
>aux            ( 10 30 )              aux: [20]   stash b
i.+             ( 40 )                 aux: [20]   sum a + c
aux>            ( 40 20 )              aux: []     restore b
```

Five tokens, no `pick` or `roll`. The narrative reads as "set b aside, do the work, get b back" — which is the pattern aux is meant to capture.

## When to reach for aux

If you only need to reorder a few values, `swap`, `over`, `rot`, `pick`, and `roll` are usually clearer. Reach for aux when:

- A library call needs the main stack a specific way and you have leftover values that would get in its way.
- You're accumulating an intermediate result and don't want to keep computing depths as the stack changes shape.
- You'd otherwise need `roll` with a constant that's painful to read.

Aux is a tool, not the tool. The exercises that come later will lean on it where it pays off.
