// state.rs - controls the state of the game

// Position - A simple struct for the position of an object in PacMan
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// Ghost - Pacmon ghost modeled
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Ghost {
    pub id: u8,
    pub pos: Position,
    pub direction: u32,
    pub is_eaten: bool,
}

// GameState - Controls the gamestate of the game
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub pacman: Position,
    pub ghosts: Vec<Ghost>,
    pub score: i32,
    pub level: i32,
    pub lives: i32,
    pub pellets_remaining: i32,
}

impl GameState {
    // Sets up a new game
    pub fn new() -> Self {
        Self {
            pacman: Position { x: 0, y: 0 },
            ghosts: vec![],
            score: 0,
            level: 1,
            lives: 3,
            pellets_remaining: 244,
        }
    }

    // Have PacMan make a move
    pub fn make_move(&mut self, direction: &str) {
        match direction {
            "UP" => self.pacman.y -= 1,
            "DOWN" => self.pacman.y += 1,
            "LEFT" => self.pacman.x -= 1,
            "RIGHT" => self.pacman.x += 1,
            _ => (),
        }
        
        // Temporarily adds a score of 10, assuming that he eats a pellet everytime he moves
        self.score += 10;
    }

    // Temporarily subtracts score by 10 - WIP (will work on creating a stack of previous moves
    // that were done and then undo the most previous move ASAP)
    pub fn undo_move(&mut self) {
        self.score.saturating_sub(10);
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
