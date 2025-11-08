# Generic coding rules for the project

## Comments

- All the text on the comment must be in english.
- Comments for struct and function must use ///

## Imports

- Only one 'use' or 'mod' per line.
- If a required module is on the parent module use 'use super::'

## Function Separators

- Each function in an implementation must be separated by a line composed of '-' that stop at column 80 maximum. And let one empty line before and after this separation line.

## Struct and impl Separators

- Each struct definition must be separated from other structs or implementations by a line composed of '=' that stop at column 80 maximum. And let one empty line before and after this separation line.
- Each impl block must be separated from other impl blocks or structs by a line composed of '=' that stop at column 80 maximum. And let one empty line before and after this separation line.
- If multiple impl blocks exist for the same struct, they can be grouped together with only the standard function separators ('-') between methods within each impl block.
