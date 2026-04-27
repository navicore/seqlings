# Hint: bnot

`bnot` is a unary operation — it takes one Int from the stack and pushes back the bit-flipped result.

For zero, every bit was 0; after the flip every bit is 1, which in two's complement is -1.

For the self-inverse: apply `bnot` twice in succession. Each `bnot` flips the bits, so two flips cancel.
