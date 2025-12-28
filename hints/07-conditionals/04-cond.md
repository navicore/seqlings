# Hint: Multi-way cond

`cond` handles multiple conditions cleanly.

## Solution

```seq
: classify-grade ( Int -- Int )
    dup 90 >= -> drop 4
    dup 80 >= -> drop 3
    dup 70 >= -> drop 2
    otherwise -> drop 1
    cond
;
```

Wait, that's not quite right. Let me show the proper syntax:

```seq
: classify-grade ( Int -- Int )
    cond
        dup 90 >= -> drop 4
        dup 80 >= -> drop 3
        dup 70 >= -> drop 2
        otherwise -> drop 1
    end
;
```

## How cond Works

1. Each condition is evaluated in order
2. First true condition's result is used
3. `otherwise` catches all remaining cases
4. `end` terminates the cond block
