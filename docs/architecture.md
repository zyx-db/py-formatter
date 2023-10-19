┌───────┬┐  ┌───────┬┐ ┌───────┬┐ ┌───────────┐
│  MAIN │┼─►│ PARSER│┤►│ STACK │┤►│FORMATTING │
└───────┴┘  └───────┴┘ └───────┴┘ └───────────┘

## Main

Opens and reads input.
Passes input to parser.
Outputs final code as needed.

## Parser

Determines the type of expression and depth of each line.
Maintains code structure using stack structure.

## Stack

Responsible for maintaining monotonically increasing depth.
Will group expressions as needed (dependent on expression type)
Does so making use of the formatting library.

## Formatting

Library with methods to group expression and clean up the formatting.
