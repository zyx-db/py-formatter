# Goal

Parse our file for indentation in order to properly "group" code.

For example the following code should be grouped accordingly.

```
Line 1: def test():
Line 2:     x = 0
Line 3:     while True:
Line 4:         if x > 5: 
Line 5:             break
Line 6:         x += 1
Line 7:     print(x)
```

Should evaluate to the following in terms of what lines are within what groups.

```
(1 (2, 3 (4 ( 5), 6), 7))
```

From here we have a tree-like structure, where we can merge groups, starting from leaf, making our way up to the root.

# Approach

We will have a monotonically increasing stack, with each object containing the indentation and expression. When we encounter an expression with a lower level of indentation than the top of the stack, we remove that object, and any others with the same indentation. From there we can group and merge it with its parent.

Running through our previous example, line by line:
(The right is the top of the stack)
(Within each object is the indent, and the expression it maps to)

```
Line 1: [(0, 1)]
Line 2: [(0, 1), (1, 2)]
Line 3: [(0, 1), (1, 2), (1, 3)]
Line 4: [(0, 1), (1, 2), (1, 3), (2, 4)]
Line 5: [(0, 1), (1, 2), (1, 3), (2, 4), (3, 5)]
Line 6: [(0, 1), (1, 2), (1, 3), (2, 4 (5)), (2, 6)]
Line 6: [(0, 1), (1, 2), (1, 3), (2, 4 (5)), (2, 6)]
Line 7: [(0, 1), (1, 2), (1, 3 (4 (5) 6)), (1, 7)]
```

From here all that is left is to iterate through in reverse order and group accordingly. We could simply push an element with 0 indentation, compressing the stack, making our full output the base of the stack.
