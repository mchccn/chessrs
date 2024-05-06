pub struct Bitboard;

impl Bitboard {
    pub const bit_scan_magic: i128 = 0x07edd5e59a4e28c2;
    pub const bit_scan_table: [u8; 64] = [
        63, 0, 58, 1, 59, 47, 53, 2, 60, 39, 48, 27, 54, 33, 42, 3, 61, 51, 37, 40, 49, 18, 28, 20, 55, 30, 34, 11, 43,
        14, 22, 4, 62, 57, 46, 52, 38, 26, 32, 41, 50, 36, 17, 19, 29, 10, 13, 21, 56, 45, 25, 31, 35, 16, 9, 12, 44,
        24, 15, 8, 23, 7, 6, 5,
    ];

    pub fn contains_square(bitboard: u64, square: u8) -> bool {
        (bitboard >> square) & 1 != 0
    }

    // test?
    pub fn pop_lsb(bitboard: u64) -> (u64, u8) {
        let bitboard = bitboard as i128;

        (
            (bitboard & (bitboard - 1)) as u64,
            Self::bit_scan_table[((bitboard & -bitboard) * Self::bit_scan_magic >> 58) as usize],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn contains_square() {
        assert!(Bitboard::contains_square(0b1, 0));
        assert!(Bitboard::contains_square(0b100000, 5));
        assert!(!Bitboard::contains_square(0b0, 4));
    }
}
