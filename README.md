# Spellcaster
This is my first Rust project. It is a program that attempts to optimally solve the Discord Spellcast game, which is a 5 by 5 grid of letters.
- The player attempts to find the highest scoring word within that grid.
- Letters can be diagonal or adjacent to each other.
- Some letters are worth more than others.
- There are tiles that are worth double or triple the usual value.
- There are tiles that double the score of the word overall.
- There is a length bonus if your word is six letters or longer that factors in after doubles.
- There are *swaps* which allow you to alter any letter on the board to any other.

I thought these constraints made it a reasonably challenging program to learn a new language. This project is also an investigation into pushing more "atomic commits" rather than multiple giant refactors in one commit, as well as a way to learn more low-level optimization tricks like bitmasks and converting from chars to ASCII to an even smaller subset of just the lowercase English letters.