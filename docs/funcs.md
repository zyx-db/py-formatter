# Python Function Guide

## g function
- g stands for "Group"
- Combine multiple python expressions into a single (one-line) expression
- Takes in the current loop nesting depth as well as the iterable of expressions, outputs a compound expression
- It will break out early on the expression (..., depth) where depth is specified in the g function, or return the first non None value, defaulting to None

## n function
- n stands for "None"
- Take in some expression, and simply return None for use with the g function
- Note that the expression is evaluated as it is passed in