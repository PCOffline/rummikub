1. Draw -> Immediately end turn
2. Any other move -> Record for undo/clear?
    a. Undo isn't just "popping" a move in the cache, it's also literally undoing the move
    b. Save table instance before moves for clear?
3.

## Problems
Splitting and counting what is and isn't a set shouldn't be by trial and error, but if the tiles are CONNECTED.
i.e. in the UI, there shall be a tile sticking mechanism.

Moves are merely a tool to record and UNDO/CLEAR MOVES, not part of the execution flow.
They are also used for validation (e.g. meld)
