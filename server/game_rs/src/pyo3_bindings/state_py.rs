// pyo3 bindings for state.rs

use pyo3::prelude::*;
use crate::state::GameState;

#[pyclass]
pub struct PyGameState {
    inner: GameState
}

#[pymethods]
impl PyGameState {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: GameState::new()
        }
    }

    pub fn make_move(&mut self, dir: &str) {
        self.inner.make_move(dir)
    }

    pub fn undo_move(&mut self) {
        self.inner.undo_move()
    }

    pub fn get_score(&self) -> i32 {
        self.inner.score
    }

    pub fn __repr__(&self) -> String {
        format!(
            "PyGameState(score={}, pacman's position=({}, {}), lives={})", 
            self.inner.score, 
            self.inner.pacman.x, 
            self.inner.pacman.y, 
            self.inner.lives
        )
    }
}
