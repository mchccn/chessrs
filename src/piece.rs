use core::panic;

pub struct Piece;

impl Piece {
    pub const None: u8 = 0b00000;
    pub const King: u8 = 0b00001;
    pub const Pawn: u8 = 0b00010;
    pub const Knight: u8 = 0b00011;
    pub const Bishop: u8 = 0b00101;
    pub const Rook: u8 = 0b00110;
    pub const Queen: u8 = 0b00111;

    pub const White: u8 = 0b01000;
    pub const Black: u8 = 0b10000;

    const kind_mask: u8 = 0b00111;
    const colour_mask: u8 = 0b11000;

    pub fn is_colour(piece: u8, colour: u8) -> bool {
        Self::get_colour(piece) == colour
    }

    pub fn is_kind(piece: u8, kind: u8) -> bool {
        Self::get_kind(piece) == kind
    }

    pub fn get_colour(piece: u8) -> u8 {
        piece & Self::colour_mask
    }

    pub fn get_kind(piece: u8) -> u8 {
        piece & Self::kind_mask
    }

    pub fn is_rook_or_queen(piece: u8) -> bool {
        piece & 0b110 == 0b110
    }

    pub fn is_bishop_or_queen(piece: u8) -> bool {
        piece & 0b101 == 0b101
    }

    pub fn is_sliding_piece(piece: u8) -> bool {
        piece & 0b100 == 0b100
    }
}

pub fn piece_str(piece: u8) -> String {
    let kind = Piece::get_kind(piece);

    let name = match kind {
        Piece::None => " ",
        Piece::King => "k",
        Piece::Pawn => "p",
        Piece::Knight => "n",
        Piece::Bishop => "b",
        Piece::Rook => "r",
        Piece::Queen => "q",
        _ => panic!("unknown piece kind: {}", kind),
    };

    if Piece::is_colour(piece, Piece::White) {
        name.to_uppercase()
    } else {
        name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_colour() {
        assert!(Piece::is_colour(Piece::White | Piece::King, Piece::White));
        assert!(Piece::is_colour(Piece::Black | Piece::Pawn, Piece::Black));
        assert!(!Piece::is_colour(Piece::Black | Piece::Rook, Piece::White));
    }

    #[test]
    fn is_kind() {
        assert!(Piece::is_kind(Piece::White | Piece::King, Piece::King));
        assert!(Piece::is_kind(Piece::Black | Piece::Pawn, Piece::Pawn));
        assert!(!Piece::is_kind(Piece::Black | Piece::Pawn, Piece::Rook));
    }

    #[test]
    fn get_colour() {
        assert!(Piece::get_colour(Piece::White | Piece::King) == Piece::White);
        assert!(Piece::get_colour(Piece::Black | Piece::Pawn) == Piece::Black);
    }

    #[test]
    fn get_kind() {
        assert!(Piece::get_kind(Piece::White | Piece::King) == Piece::King);
        assert!(Piece::get_kind(Piece::Black | Piece::Pawn) == Piece::Pawn);
    }

    #[test]
    fn is_rook_or_queen() {
        assert!(Piece::is_rook_or_queen(Piece::White | Piece::Rook));
        assert!(Piece::is_rook_or_queen(Piece::Black | Piece::Queen));
        assert!(!Piece::is_rook_or_queen(Piece::Black | Piece::Pawn));
    }

    #[test]
    fn is_bishop_or_queen() {
        assert!(Piece::is_bishop_or_queen(Piece::White | Piece::Bishop));
        assert!(Piece::is_bishop_or_queen(Piece::Black | Piece::Queen));
        assert!(!Piece::is_bishop_or_queen(Piece::Black | Piece::Pawn));
    }

    #[test]
    fn is_sliding_piece() {
        assert!(Piece::is_sliding_piece(Piece::White | Piece::Bishop));
        assert!(Piece::is_sliding_piece(Piece::Black | Piece::Queen));
        assert!(Piece::is_sliding_piece(Piece::Black | Piece::Rook));
        assert!(!Piece::is_bishop_or_queen(Piece::Black | Piece::Pawn));
    }
}
