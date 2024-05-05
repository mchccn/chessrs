use std::fmt;

#[derive(Clone, Copy)]
pub enum GameState {
    Playing,
    WhiteCheckmatedBlack,
    BlackCheckmatedWhite,
    Stalemate,
    ThreeFoldRepetition,
    InsufficientMaterial,
    Timeout,
}

impl GameState {
    fn is_draw(&self) -> bool {
        use GameState::*;
        matches!(self, Stalemate | ThreeFoldRepetition | InsufficientMaterial | Timeout)
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            GameState::Playing => "playing",
            GameState::WhiteCheckmatedBlack => "white checkmated black",
            GameState::BlackCheckmatedWhite => "black checkmated white",
            GameState::Stalemate => "stalemte",
            GameState::ThreeFoldRepetition => "three-fold repetition",
            GameState::InsufficientMaterial => "insufficient material",
            GameState::Timeout => "timeout",
        };

        write!(f, "{}", name)
    }
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            GameState::Playing => "Playing",
            GameState::WhiteCheckmatedBlack => "WhiteCheckmatedBlack",
            GameState::BlackCheckmatedWhite => "BlackCheckmatedWhite",
            GameState::Stalemate => "Stalemate",
            GameState::ThreeFoldRepetition => "ThreeFoldRepetition",
            GameState::InsufficientMaterial => "InsufficientMaterial",
            GameState::Timeout => "Timeout",
        };

        write!(f, "{} [{}]", name, *self as i32)
    }
}
