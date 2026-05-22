# Hint: Quotation Basics

A quotation is code as data — it doesn't execute until something calls it.

The stub already has `5` pushed, and the "Do not edit" section already has `call`. Your job is to place a quotation between them — a bracketed expression that, when called, doubles the value on top.

Doubling a value already on the stack has two well-known recipes: multiply by 2, or `dup i.+` (add the value to a copy of itself). Either works inside `[ ... ]`. Whichever you pick, wrap it in square brackets to make it a quotation — a value, not an instruction. The `call` on the next line consumes it.

## The paradigm shift

This may seem simple, but you've just done something profound:

1. You wrote code (the body inside the brackets).
2. You wrapped it as data (the `[ ... ]`).
3. You passed it around (it sat on the stack like any other value).
4. Something else executed it later (`call`).

This is the essence of higher-order programming. Every callback you've ever used, every `.map()` or `.filter()`, every event handler — they all rely on this principle. Functions as values.
