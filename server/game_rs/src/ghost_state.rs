// state.rs - controls the state of the game

use serde::{Deserialize, Serialize};

// Position - A simple struct for the position of an object in PacMan
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LocationState {
    pub x: i32,
    pub y: i32,
}

// Ghost - Pacmon ghost modeled
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GhostState {
    // pub id: u8,
    // pub pos: Position,
    // pub direction: u32,
    // pub is_eaten: bool,
    pub loc: *locationState, // Current location
	pub nextLoc: *locationState, // Planned location (for next update)
	pub scatterTarget: *locationState, // Position of (fixed) scatter target
	pub game: *gameState,     // The game state tied to the ghost
	pub color: uint8,
	pub trappedSteps: uint8,
	pub frightSteps: uint8,
	pub spawning: bool,        // Flag set when spawning
	pub eaten: bool,        // Flag set when eaten and returning to ghost house
	pub muState: sync.RWMutex // Mutex to lock general state parameters
}

// // GameState - Controls the gamestate of the game
// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct GameState {
//     pub pacman: Position,
//     pub ghosts: Vec<Ghost>,
//     pub score: i32,
//     pub level: i32,
//     pub lives: i32,
//     pub pellets_remaining: i32,
// }

impl GhostState {
    // Sets up a new ghost
    pub fn new() -> Self {
        Self {
            loc: LocationState {x:0, y:0},
            nextLoc: LocationState {x:0, y:0},
            scatterTarget : LocationState {x=0, y=0},
            gameState: ....
            color: 0,
            trappedSteps: 0,
            frightSteps: 0,
            spawning: true,
            eaten: false,
            muState: ...
            // pacman: Position { x: 0, y: 0 },
            // ghosts: vec![],
            // score: 0,
            // level: 1,
            // lives: 3,
            // pellets_remaining: 244,
        }
    }

    // // Have PacMan make a move
    // pub fn make_move(&mut self, dir: &str) {
    //     match dir {
    //         "UP" => self.pacman.y -= 1,
    //         "DOWN" => self.pacman.y += 1,
    //         "LEFT" => self.pacman.x -= 1,
    //         "RIGHT" => self.pacman.x += 1,
    //         _ => (),
    //     }
        
    //     // Temporarily adds a score of 10, assuming that he eats a pellet everytime he moves
    //     self.score += 10;
    // }

    // // Temporarily subtracts score by 10 - WIP (will work on creating a stack of previous moves
    // // that were done and then undo the most previous move ASAP)
    // pub fn undo_move(&mut self) {
    //     self.score.saturating_sub(10);
    // }

    // pub fn serialize(&self) -> String {
    //     serde_json::to_string(self).unwrap()
    // }
}
