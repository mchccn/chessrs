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
    fn is_playing(&self) -> bool {
        use GameState::*;
        matches!(self, Playing)
    }

    fn is_win_lose(&self) -> bool {
        use GameState::*;
        matches!(self, WhiteCheckmatedBlack | BlackCheckmatedWhite)
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_playing() {
        assert!(GameState::Playing.is_playing());
        assert!(!GameState::InsufficientMaterial.is_playing());
    }

    #[test]
    fn is_win_lose() {
        assert!(GameState::WhiteCheckmatedBlack.is_win_lose());
        assert!(GameState::BlackCheckmatedWhite.is_win_lose());
        assert!(!GameState::Playing.is_win_lose());
    }

    #[test]
    fn is_draw() {
        assert!(GameState::Stalemate.is_draw());
        assert!(GameState::ThreeFoldRepetition.is_draw());
        assert!(GameState::InsufficientMaterial.is_draw());
        assert!(GameState::Timeout.is_draw());
        assert!(!GameState::Playing.is_draw());
        assert!(!GameState::WhiteCheckmatedBlack.is_draw());
        assert!(!GameState::BlackCheckmatedWhite.is_draw());
    }
}
