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
pub struct GameState {
    player_one: String,
    player_two: String,
    game_board: [u8; SLOTS * 2],
    player_one_goal_slot: usize,
    player_two_goal_slot: usize,
    player_one_turn: bool,
}

impl GameState {
    pub fn new(p_one: String, p_two: String) -> GameState {
        let mut init_game_board = [STARTING_STONES; SLOTS * 2];
        init_game_board[SLOTS] = 0;
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

    fn get_players_goal_slots(&mut self) -> (usize, usize) {
        if self.player_one_turn {
            (self.player_one_goal_slot, self.player_two_goal_slot)
        } else {
            (self.player_two_goal_slot, self.player_one_goal_slot)
        }
    }

    fn add_capture_points(&mut self, points_to_add: u8) {
        if self.player_one_turn {
            self.game_board[self.player_one_goal_slot] += points_to_add;
        } else {
            self.game_board[self.player_two_goal_slot] += points_to_add;
        }
    }

    // if your last manacala piece ends up on your side, in an empty slot,
    // you get to capture your opponents' pieces in the opposite slot and
    // add them to your goal
    fn capture(&mut self, cur_slot: usize) -> bool {
        if self.game_board[cur_slot] != 1 {
            return false;
        }
        let mut opposite_slot: usize = 0;
        if self.player_one_turn && cur_slot < SLOTS {
            opposite_slot = SLOTS + (SLOTS - cur_slot);
        } else if !self.player_one_turn && cur_slot > SLOTS {
            opposite_slot = SLOTS - (cur_slot - SLOTS);
        }
        if self.game_board[opposite_slot] == 0 || opposite_slot == 0 {
            return false;
        }
        self.add_capture_points(self.game_board[opposite_slot] + 1);
        self.game_board[cur_slot] = 0;
        self.game_board[opposite_slot] = 0;
        true
    }

    pub fn make_move(&mut self, slot_to_move: usize) {
        let mut num_of_stones: u8 = self.game_board[slot_to_move];
        let board_length: usize = self.game_board.len();
        let goal_slots: (usize, usize) = self.get_players_goal_slots();
        self.game_board[slot_to_move] = 0;
        let mut cur_slot: usize = slot_to_move + 1;
        loop {
            if cur_slot == goal_slots.1 {
                // skip opponent's goal
                cur_slot = (cur_slot + 1) % board_length;
                continue;
            }
            self.game_board[cur_slot] += 1;
            num_of_stones -= 1;
            if num_of_stones == 0 {
                break;
            }
            cur_slot = (cur_slot + 1) % board_length;
        }
        // only change turns if current player didn't score
        if cur_slot != goal_slots.0 && !self.capture(cur_slot) {
            self.player_one_turn = !self.player_one_turn;
        }
    }

    pub fn get_board(&mut self) -> [u8; SLOTS * 2] {
        self.game_board
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
    init_game_board[SLOTS] = 0;
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
    init_game_board[SLOTS] = 0;
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

#[test]
fn test_scoring_turns_dont_change_players() {
    let mut gs: GameState = GameState::new("asdf".to_string(), "asdf2".to_string());
    let turn1: bool = gs.player_one_turn;
    gs.make_move(3);
    let turn2: bool = gs.player_one_turn;
    assert_eq!(turn1, turn2);
}

#[test]
fn test_captures() {
    // this test assumes SLOTS = 7 and starting_stones = 4
    let mut gs: GameState = GameState::new("asdf".to_string(), "asdf2".to_string());
    gs.make_move(6);
    gs.make_move(11);
    let turn1: bool = gs.player_one_turn;
    gs.make_move(2);
    let turn2: bool = gs.player_one_turn;
    assert_eq!(turn1, turn2);
    assert_eq!(gs.game_board[gs.player_one_goal_slot], 7);
    assert_eq!(gs.game_board[gs.player_two_goal_slot], 1);
    assert_eq!(gs.game_board[8], 0);
    assert_eq!(gs.game_board[6], 0);
}
