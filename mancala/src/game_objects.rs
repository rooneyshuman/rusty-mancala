//The idea behind these objects is that we should be able to represent a typical mancala game in a
//concise and easily serializiable manner.  These objects should be ale to be passed through a TCP
//socket opened between the client and the server without much overhead.
//
//The game state can be easily represented by modeling the "board", which consists of two sides and
//two goals.  The two sides will have several "slots", which in turn will be occuped by zero or more
//"stones".  The goals each belong to a certain player and are only reachable by that player.  The
//game moves forwards by a player deciding to move the stones in one of their slots one at a time
//around the board, depositing them in the following slots one at a time in a counter-clockwise
//motion.
//
//Any time a stone is passed over a player's goal (the slot at the right hand side of either player)
//that stone is deposited there and a point is scored.  Any time a turn ends in a point scored, the
//player is allowed to play again.  Players also score points if they end their turn in an otherwise
//empty slot ON THEIR SIDE that lies directly across from a non-empty opponent slot.  In that case, the
//last stone placed as well as all of the stones from the opposing slot are scored for the player who
//made the move.

//Ideas for modeling the board:
//
// - a single array of uInts that has length 2 * number of slots
//        - each array slot represents a slot on the board
//        - each int in the array represents the amount of stones in that slot
//        - player 1 is slots 0 - (n/2 - 1)
//        - player 2 is slots n/2 -> n - 1
//        - opposing slots are calculated by adding or subtracting distance to n/2 or n/2 - 1
//              - n = 10 (2 sides with 5 slots each)
//              - 0 <--> 9; 1 <--> 8, etc
//              - 0 is 4 away from slot 4, 9 is 4 away from slot 5
//              - 1 is 3 away from slot 4, 8 is 3 away from slot 5, etc
//        - goal slots are also uInts with their own field
// - two arrays of uInts that have length n
//        - player one is array 1
//        - player two is array 2
//        - opposing slots are calculated by just mapping 1 <--> the slots
//              - slot 1 (player 1) <--> slot 1 (player 2)
//        - goal slots are also uInts with their own field

//Possible workflow for moves:
//
// - Poll for "is it my turn"
// - Once true, Player sends a slot they'd like to move
// - Slot is checked against possible moves and accepted or rejected
//    - checked by client or server?
//        - basic bounds checked by client (does it exist on my side of the board)
//        - probably server for any check dependent on game state (was the slot i picked empty)