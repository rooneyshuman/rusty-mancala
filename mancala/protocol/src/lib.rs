//The idea behind these objects is that we should be able to represent a typical mancala game in a
//concise and easily serializiable manner.  These objects should be ale to be passed through a TCP
//socket opened between the client and the server without much overhead.
//
//The game state can be easily represented by modeling the "board", which consists of two sides and
//two goals.  The two sides will have several "SLOTS", which in turn will be occuped by zero or more
//"stones".  The goals each belong to a certain player and are only reachable by that player.  The
//game moves forwards by a player deciding to move the stones in one of their SLOTS one at a time
//around the board, depositing them in the following SLOTS one at a time in a counter-clockwise
//motion.
//
//Any time a stone is passed over a player's goal (the slot at the right hand side of either player)
//that stone is deposited there and a point is scored.  Any time a turn ends in a point scored, the
//player is allowed to play again.  Players also score points if they end their turn in an otherwise
//empty slot ON THEIR SIDE that lies directly across from a non-empty opponent slot.  In that case, the
//last stone placed as well as all of the stones from the opposing slot are scored for the player who
//made the move.

//Possible workflow for moves:
//
// - Poll for "is it my turn"
// - Once true, Player sends a slot they'd like to move
// - Slot is checked against possible moves and accepted or rejected
//    - checked by client or server?
//        - basic bounds checked by client (does it exist on my side of the board)
//        - probably server for any check dependent on game state (was the slot i picked empty)

// remove magic numbers
const SLOTS: usize = 7; // there are 6 playable slots and one goal slot
const STARTING_STONES: u8 = 4;

#[derive(Debug, Clone)]
struct GameState {
    player_one: String,
    player_two: String,
    game_board: [u8; SLOTS * 2],
    player_one_goal_slot: usize,
    player_two_goal_slot: usize,
    player_one_turn: bool,
}

impl GameState {
    fn new(p_one: String, p_two: String) -> GameState {
        let mut init_game_board = [STARTING_STONES; SLOTS * 2];
        init_game_board[SLOTS + 1] = 0;
        init_game_board[0] = 0;
        GameState {
            player_one: p_one,
            player_two: p_two,
            game_board: init_game_board,
            player_one_goal_slot: SLOTS,
            player_two_goal_slot: 0,
            player_one_turn: true,
        }
    }

    fn cur_players_goal_slot(&mut self) -> usize {
        if self.player_one_turn {
            self.player_one_goal_slot
        } else {
            self.player_two_goal_slot
        }
    }

    fn add_points(&mut self, points_to_add: u8) {
        if self.player_one_turn {
            self.game_board[self.player_one_goal_slot] += points_to_add;
        } else {
            self.game_board[self.player_two_goal_slot] += points_to_add;
        }
    }

    pub fn make_move(&mut self, slot_to_move: usize) {
        let num_of_stones: u8 = self.game_board[slot_to_move];
        let board_length: usize = self.game_board.len();
        let goal_slot: usize = self.cur_players_goal_slot();
        self.game_board[slot_to_move] = 0;
        for cur_slot in (slot_to_move + 1)..=slot_to_move + num_of_stones as usize {
            let slot_to_add_to: usize = cur_slot % board_length;
            if slot_to_add_to == goal_slot {
                self.add_points(1);
                continue;
            }
            self.game_board[slot_to_add_to] += 1;
        }
        self.player_one_turn = !self.player_one_turn;
    }
}

#[test]
fn test_game_state_can_be_initialized() {
    let gs: GameState = GameState::new("asdf".to_string(), "asdf2".to_string());
}

#[test]
fn test_game_state_init_values_are_correct() {
    let gs: GameState = GameState::new("asdf".to_string(), "asdf2".to_string());
    let mut init_game_board = [STARTING_STONES; SLOTS * 2];
    init_game_board[SLOTS + 1] = 0;
    init_game_board[0] = 0;
    assert_eq!(gs.player_one, "asdf".to_string());
    assert_eq!(gs.player_two, "asdf2".to_string());
    assert_eq!(gs.game_board, init_game_board);
    assert!(gs.player_one_turn);
}

#[test]
fn test_game_state_updates_after_one_move() {
    let mut gs: GameState = GameState::new("asdf".to_string(), "asdf2".to_string());
    let mut init_game_board = [STARTING_STONES; SLOTS * 2];
    init_game_board[SLOTS + 1] = 0;
    init_game_board[0] = 0;
    gs.make_move(1);
    assert_ne!(gs.game_board, init_game_board);
}

#[test]
fn test_turn_changes_after_making_move() {
    let mut gs: GameState = GameState::new("asdf".to_string(), "asdf2".to_string());
    let turn1: bool = gs.player_one_turn;
    gs.make_move(1);
    let turn2: bool = gs.player_one_turn;
    gs.make_move(2);
    let turn3: bool = gs.player_one_turn;
    gs.make_move(3);
    let turn4: bool = gs.player_one_turn;
    assert_eq!(turn1, turn3);
    assert_eq!(turn2, turn4);
    assert_ne!(turn1, turn2);
    assert_ne!(turn3, turn4);
}
