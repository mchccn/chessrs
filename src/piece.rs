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
