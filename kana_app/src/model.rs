use serde::{Deserialize, Serialize};

/// Sets of characters for training.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Set {
    /// Hiragana only.
    Hiragana,
    /// Katakana only.
    Katakana,
    /// Hiragana + Katakana
    All,
    /// Hiragana + Katakana + Rare
    Rare,
}

/// Application model.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    /// Current set selected.
    set: Set,

    /// True if we are at the start menu.
    at_start: bool,
}

impl Model {
    /// Returns a new `Model`.
    pub fn new() -> Model {
        return Model {
            set: Set::Hiragana,
            at_start: true,
        };
    }

    /// Starts a new training session.
    pub fn start(&mut self, set: Set, _size: u32) {
        self.restart();
        self.at_start = false;
        self.set = set;
    }

    /// Restarts the `Model` to the initial state.
    pub fn restart(&mut self) {
        self.at_start = true;
    }
}
