const SLOTS: usize = 7; // there are 6 playable slots and one goal slot
const STARTING_STONES: u8 = 4;
const BOARD_LENGTH: usize = SLOTS * 2;

#[derive(Debug, Clone)]
pub struct GameState {
    player_one: String,
    player_two: String,
    game_board: [u8; SLOTS * 2],
    player_one_goal_slot: usize,
    player_two_goal_slot: usize,
    player_one_turn: bool,
    active: bool,
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
            active: true,
        }
    }

    fn collect_remaining_stones(&mut self) {
        self.game_board[self.player_one_goal_slot] +=
            &self.game_board[1..self.player_one_goal_slot].iter().sum();
        self.game_board[self.player_two_goal_slot] += &self.game_board
            [self.player_one_goal_slot + 1..]
            .iter()
            .sum();
    }

    fn is_game_over(&mut self) -> bool {
        if self.player_one_turn {
            return self.game_board[1..self.player_one_goal_slot]
                .iter()
                .all(|&x| x == 0);
        }
        self.game_board[self.player_one_goal_slot + 1..]
            .iter()
            .all(|&x| x == 0)
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
        let goal_slots: (usize, usize) = self.get_players_goal_slots();
        self.game_board[slot_to_move] = 0;
        let mut cur_slot: usize = (slot_to_move + 1) % BOARD_LENGTH;
        loop {
            if cur_slot == goal_slots.1 {
                // skip opponent's goal
                cur_slot = (cur_slot + 1) % BOARD_LENGTH;
                continue;
            }
            self.game_board[cur_slot] += 1;
            num_of_stones -= 1;
            if num_of_stones == 0 {
                break;
            }
            cur_slot = (cur_slot + 1) % BOARD_LENGTH;
        }
        // only change turns if current player didn't score
        // TODO - decide if we should change turns on capture
        if cur_slot != goal_slots.0 && !self.capture(cur_slot) {
            self.player_one_turn = !self.player_one_turn;
        }
        if self.is_game_over() {
            self.collect_remaining_stones();
            self.active = false;
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
    assert!(gs.active);
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

#[test]
fn test_collect_remaining() {
    let mut gs: GameState = GameState::new("asdf".to_string(), "asdf2".to_string());
    let mut gs2: GameState = GameState::new("asdf".to_string(), "asdf2".to_string());
    gs.collect_remaining_stones();
    gs2.make_move(6);
    gs2.collect_remaining_stones();
    assert_eq!(gs.game_board[gs.player_one_goal_slot], 24);
    assert_eq!(gs.game_board[gs.player_two_goal_slot], 24);
    assert_eq!(gs2.game_board[gs2.player_one_goal_slot], 21);
    assert_eq!(gs2.game_board[gs2.player_two_goal_slot], 27);
}

// ---------------------------------------------------------------------------------------------- //

#[derive(PartialEq, Debug)]
enum MsgType {
    Read,
    Action,
    Response,
}

#[derive(PartialEq, Debug)]
enum MsgStatus {
    Ok,
    Error,
}

#[derive(PartialEq, Debug)]
enum Header {
    Pregame,
    Ingame,
}

#[derive(PartialEq, Debug)]
enum Actions {
    Join,
    MakeMove,
    SendChat, // todo - decide to do this or not
    SetName,
}

struct GameData {
    player_one: String,
    player_two: String,
    game_board: [u8; SLOTS * 2],
    player_one_turn: bool,
    active: bool, // TODO - other stuff? Time, date, etc?
}

impl GameData {
    pub fn new(game_state: &GameState) -> GameData {
        GameData {
            player_one: game_state.player_one.clone(),
            player_two: game_state.player_two.clone(),
            game_board: game_state.game_board,
            player_one_turn: game_state.player_one_turn,
            active: game_state.active,
        }
    }
}

struct MsgBody {
    game_data: GameData,
    move_to_make: usize,
}

impl MsgBody {
    pub fn new() -> MsgBody {
        // TODO - this is dumb, need to find a way to "default" or nullable init values
        MsgBody {
            game_data: GameData {
                player_one: "".to_string(),
                player_two: "".to_string(),
                game_board: [0; SLOTS * 2],
                player_one_turn: false,
                active: false,
            },
            move_to_make: 0,
        }
    }

    pub fn set_game_state(&mut self, game_state: GameState) {
        self.game_data.active = game_state.active;
        self.game_data.player_one = game_state.player_one;
        self.game_data.player_two = game_state.player_two;
        self.game_data.player_one_turn = game_state.player_one_turn;
        self.game_data.game_board = game_state.game_board;
    }

    pub fn set_move_to_make(&mut self, move_to_make: usize) {
        self.move_to_make = move_to_make;
    }
}

struct Msg {
    msg_type: MsgType,
    header: Header,
    status: MsgStatus,
    errors: Vec<String>,
    body: MsgBody,
}

impl Msg {
    pub fn new(msg_type: MsgType, header: Header, body: MsgBody) -> Msg {
        Msg {
            msg_type,
            header,
            status: MsgStatus::Ok,
            errors: Vec::<String>::new(),
            body,
        }
    }
}

// ---------------------------------------------------------------------------------------------- //

struct ClientMessenger {}

impl ClientMessenger {
    pub fn new() -> ClientMessenger {
        ClientMessenger {}
    }

    pub fn make_nickname_msg(&self, name: &'static str) -> Option<Msg> {
        if name.is_empty() {
            return None;
        }
        let msg_t: MsgType = MsgType::Action;
        let msg_h: Header = Header::Pregame;
        let msg_b: MsgBody = MsgBody::new();
        let msg_to_server: Msg = Msg::new(msg_t, msg_h, msg_b);
        Some(msg_to_server)
    }

    // TODO - update this with check for player one/twos slots
    fn is_move_legal(&self, move_to_make: usize) -> bool {
        move_to_make > 0 && move_to_make < (SLOTS * 2)
    }

    pub fn make_move_msg(&self, move_to_make: usize) -> Option<Msg> {
        if !self.is_move_legal(move_to_make) {
            return None;
        }
        let msg_t: MsgType = MsgType::Action;
        let msg_h: Header = Header::Ingame;
        let mut msg_b: MsgBody = MsgBody::new();
        msg_b.move_to_make = move_to_make;
        let msg_to_server: Msg = Msg::new(msg_t, msg_h, msg_b);
        Some(msg_to_server)
    }
}

#[test]
fn test_client_msg_make_nickname() {
    let c: ClientMessenger = ClientMessenger::new();
    let m: Option<Msg> = c.make_nickname_msg("asdf");
    match &m {
        Some(msg) => assert_eq!(true, true),
        None => assert_eq!(true, false),
    };
    let msg: Msg = m.unwrap();
    assert_eq!(msg.msg_type, MsgType::Action);
    assert_eq!(msg.errors.len(), 0);
    assert_eq!(msg.header, Header::Pregame);
    assert_eq!(msg.status, MsgStatus::Ok);
}

#[test]
fn test_client_msg_make_move() {
    let c: ClientMessenger = ClientMessenger::new();
    let m: Option<Msg> = c.make_move_msg(1);
    let m1: Option<Msg> = c.make_move_msg(0);
    let m2: Option<Msg> = c.make_move_msg(SLOTS * 2);
    match &m {
        Some(msg) => assert!(true),
        None => assert!(false),
    };
    match &m1 {
        Some(msg) => assert!(false),
        None => assert!(true),
    };
    match &m2 {
        Some(msg) => assert!(false),
        None => assert!(true),
    };
    let msg: Msg = m.unwrap();
    assert_eq!(msg.msg_type, MsgType::Action);
    assert_eq!(msg.errors.len(), 0);
    assert_eq!(msg.header, Header::Ingame);
    assert_eq!(msg.status, MsgStatus::Ok);
    assert_eq!(msg.body.move_to_make, 1);
}

// ---------------------------------------------------------------------------------------------- //

struct ServerMessenger<'a> {
    client_connection_name: &'a str, // TODO - different identifier per client?
}

impl <'a> ServerMessenger<'a> {
    pub fn new(client_connection_name: &'a str) -> ServerMessenger <'a> {
        ServerMessenger {
            client_connection_name
        }
    }

    pub fn make_gamestate_message(&self, game_state: &GameState) -> Option<Msg> {
        let msg_t: MsgType = MsgType::Response;
        let msg_h: Header = Header::Ingame;
        let mut msg_b: MsgBody = MsgBody::new();
        msg_b.game_data = GameData::new(game_state);
        let msg_to_server: Msg = Msg::new(msg_t, msg_h, msg_b);
        Some(msg_to_server)
    }
}

#[test]
fn test_make_server_messenger_gamestate_msg() {
    let name = "client_1";
    let s_msgr = ServerMessenger::new(name);
    assert_eq!(s_msgr.client_connection_name, name);
    let gs: GameState = GameState::new("asdf".to_string(), "asdf2".to_string());
    let gs_msg: Option<Msg> = s_msgr.make_gamestate_message(&gs);
    match &gs_msg {
        Some(Msg)   => assert!(true),
        None => assert!(false)
    }
    let msg: Msg = gs_msg.unwrap();
    assert_eq!(msg.msg_type, MsgType::Response);
    assert_eq!(msg.errors.len(), 0);
    assert_eq!(msg.header, Header::Ingame);
    assert_eq!(msg.status, MsgStatus::Ok);
    assert_eq!(msg.body.game_data.player_one, gs.player_one);
    assert_eq!(msg.body.game_data.player_two, gs.player_two);
    assert_eq!(msg.body.game_data.player_one_turn, gs.player_one_turn);
    assert_eq!(msg.body.game_data.game_board, gs.game_board);
    assert_eq!(msg.body.game_data.active, gs.active);
    assert_eq!(msg.body.move_to_make, 0);
}