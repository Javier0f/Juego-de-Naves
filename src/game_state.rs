pub enum GameState {
    RUN,
    MENU,
    PAUSE,
}

pub struct Game {
    pub state: GameState,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::RUN,
        }
    }

    pub fn switch(&mut self, state: u8) {
        match state {
            1 => self.state = GameState::RUN,
            2 => self.state = GameState::MENU,
            3 => self.state = GameState::PAUSE,
            _ => println!("state not found"),
        }
    }
}
