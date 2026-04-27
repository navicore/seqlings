# Hint: band and bor

`band` keeps a bit only when both inputs have it set; `bor` keeps a bit when either has it.

For the masking case: AND with the mask. Bits where the mask is 0 always become 0; bits where the mask is 1 keep their original value.

For the set-bit case: OR with a value that has only that bit set. The bit you OR with becomes 1; everything else stays the same.
