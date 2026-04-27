# Hint: bxor

`bxor` returns 1 for bits that differ, 0 for bits that match. So XOR-ing a value with itself gives 0; XOR-ing with the same key twice cancels out.

For the inverse test: apply `bxor` twice with the same key in a row. The second application reverses the first.
