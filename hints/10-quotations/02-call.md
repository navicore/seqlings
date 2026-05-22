# Hint: The call Word

`call` consumes a quotation and runs its body against whatever's beneath it on the stack.

The stub already pushes `7` for you, and the assertion expects `49` (7 squared). Build a quotation that squares its input, then `call` it. Squaring takes a single value to its product with itself — recipes you've seen include `dup i.*` (multiply value by a copy of itself) and `2 i.pow` if a power word is available; the first is shorter and works here.

So the user contribution is: open a `[`, write the squaring body, close with `]`, then `call`. Five tokens.

## Why this is "meta"

`call` is a higher-order word — its argument is a function (a quotation), and its effect is "do whatever that function says." You can pass different quotations to `call` and get different behavior. That's the essence of higher-order programming: a function whose behavior is parameterized by another function.

Mentally, `call` is like the `()` you'd see in `f()` in C-family languages — except the function isn't fixed; it's the value sitting on the stack.

```
Before call: ( 7 quot )
After call:  ( 49 )       -- where quot was [ dup i.* ]
```
