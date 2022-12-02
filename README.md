# Advent of Code

This is my repository for my solutions to [Advent of
Code](https://adventofcode.com/) puzzles.

Each year's puzzles are in a separate Cargo workspace. To run a particular
solution, use the `-p` switch to `cargo run` from within that year's workspace,
e.g. `cargo run -p day`. Solutions assume that the puzzle's input is within
their directory, in a file named "input.txt".

## A note on Part 1/Part 2

Often the solutions I end up committing solve only Part 2 of the puzzle, as I
often choose not to preserve the logic for the Part 1 solution and instead
rewrite the relevant code to solve Part 2. Other times, however, the final
solution I commit and upload does solve both parts in its output.

Whether a given solution solves both parts or just Part 2 is entirely
hit-or-miss, and other than checking it out there's no indication on it.

(The above only applies to puzzles where I have, in fact, solved both parts.
It's entirely possible I may upload a partial solution that only solves Part 1,
having not yet figured out Part 2 yet.)

