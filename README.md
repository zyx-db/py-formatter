## Make Python Beautiful and Concise

Python linter to remove unnecessary lines of code.

## COMPLETE:

- Cleaning and grouping expressions at the same tab level
- Handling of basic assignment, as well as indexed assignment
- Reading and writing to Python files with EXACTLY ONE function declaration

## TODO:

- Handling return statements
- Handling the "+=" operator for basic assignment (need to break into x := x + ...)
- Handling the "+=" operator for indexed assignment (can modify and call existing fix_assignment)
- Handling the other operations, like [-=, /=, *=, <<=] etc (honestly low priority)
- Detecting (nested) if blocks using tab levels, and combining to one expression with implicit else: None
- Detecting if else blocks, and handling appropriately
- Detecting and combining for loops
- Allow breaking out of for loops, and implement while loops (new g-like function)
- Allow defining functions (Detect block, combine, and turn into lambda)
- replace numbers through binary representation using bitwise ops and True/False addition

