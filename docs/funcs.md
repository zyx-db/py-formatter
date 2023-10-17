# Python Function Guide

## g function

```
g = lambda depth, iterable: next(filter(lambda x: x is not None and not (type(x) == tuple and len(x) == 2 and x[0] == ... and type(x[1]) == int and x[1] > depth), iterable), None)
```

- g stands for "Group"
- Combine multiple python expressions into a single (one-line) expression
- Takes in the current loop nesting depth as well as the iterable of expressions, outputs a compound expression
- It will break out early on the expression (..., depth) where depth is specified in the g function, or return the first non None value, defaulting to None

## n function

```
n = lambda x: None
```

- n stands for "None"
- Take in some expression, and simply return None for use with the g function
- Note that the expression is evaluated as it is passed in
