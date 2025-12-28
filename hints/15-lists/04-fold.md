# Hint: list.fold

fold takes an initial value and combines it with each element.

## Solution

```seq
: sum-list ( List -- Int )
    0 [ i.+ ] list.fold
;
```

## How fold Works

```
[ 10 20 30 40 ] 0 [ i.+ ] list.fold

Step 1: acc=0, elem=10  → 0 + 10 = 10
Step 2: acc=10, elem=20 → 10 + 20 = 30
Step 3: acc=30, elem=30 → 30 + 30 = 60
Step 4: acc=60, elem=40 → 60 + 40 = 100
Result: 100
```

## fold is Fundamental

You can implement map and filter using fold. It's the most general list operation.
