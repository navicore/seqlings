# Hint: The while Loop

`while` takes TWO quotations: a condition and a body.

## Solution

```seq
80
[ dup 10 >= ] [ 2 i.divide ] while
```

Trace:
- 80 >= 10? Yes, divide: 40
- 40 >= 10? Yes, divide: 20
- 20 >= 10? Yes, divide: 10
- 10 >= 10? Yes, divide: 5
- 5 >= 10? No, stop

## Two Quotations = Two Concerns

The condition quotation answers: "Should we continue?"
The body quotation answers: "What do we do each time?"

Separating these concerns is powerful. You could reuse the same body with different conditions, or vice versa.

## Higher-Order Control Flow

`while` implements looping as a higher-order function. The loop logic is abstracted away - you just provide the pieces.
