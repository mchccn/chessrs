pub struct BoardRepresentation;

impl BoardRepresentation {
    pub const RANK_NAMES: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
    pub const FILE_NAMES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    pub const a1: u8 = 0;
    pub const b1: u8 = 1;
    pub const c1: u8 = 2;
    pub const d1: u8 = 3;
    pub const e1: u8 = 4;
    pub const f1: u8 = 5;
    pub const g1: u8 = 6;
    pub const h1: u8 = 7;

    pub const a8: u8 = 56;
    pub const b8: u8 = 57;
    pub const c8: u8 = 58;
    pub const d8: u8 = 59;
    pub const e8: u8 = 60;
    pub const f8: u8 = 61;
    pub const g8: u8 = 62;
    pub const h8: u8 = 63;

    pub fn file_index(index: u8) -> u8 {
        index & 0b111
    }

    pub fn rank_index(index: u8) -> u8 {
        index >> 3
    }

    pub fn index_from_coord(file: u8, rank: u8) -> u8 {
        rank * 8 + file
    }

    pub fn is_index_light_square(index: u8) -> bool {
        (Self::file_index(index) + Self::rank_index(index)) % 2 != 0
    }

    pub fn is_coord_light_square(file: u8, rank: u8) -> bool {
        (file + rank) % 2 != 0
    }

    pub fn square_str_index(index: u8) -> String {
        let file = Self::FILE_NAMES[Self::file_index(index) as usize];
        let rank = Self::RANK_NAMES[Self::rank_index(index) as usize];

        format!("{}{}", file, rank)
    }

    pub fn square_str_coord(file: u8, rank: u8) -> String {
        let file = Self::FILE_NAMES[file as usize];
        let rank = Self::RANK_NAMES[rank as usize];

        format!("{}{}", file, rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_index() {
        assert_eq!(BoardRepresentation::file_index(43), 3);
        assert_eq!(BoardRepresentation::file_index(0), 0);
        assert_eq!(BoardRepresentation::file_index(32), 0);
        assert_eq!(BoardRepresentation::file_index(31), 7);
    }

    #[test]
    fn rank_index() {
        assert_eq!(BoardRepresentation::rank_index(43), 5);
        assert_eq!(BoardRepresentation::rank_index(0), 0);
        assert_eq!(BoardRepresentation::rank_index(32), 4);
        assert_eq!(BoardRepresentation::rank_index(31), 3);
    }

    #[test]
    fn index_from_coord() {
        assert_eq!(BoardRepresentation::index_from_coord(3, 5), 43);
        assert_eq!(BoardRepresentation::index_from_coord(0, 0), 0);
        assert_eq!(BoardRepresentation::index_from_coord(0, 4), 32);
        assert_eq!(BoardRepresentation::index_from_coord(7, 3), 31);
    }

    #[test]
    fn is_light_square() {
        assert!(!BoardRepresentation::is_index_light_square(43));
        assert!(!BoardRepresentation::is_index_light_square(0));
        assert!(!BoardRepresentation::is_index_light_square(32));
        assert!(!BoardRepresentation::is_index_light_square(31));
        assert!(!BoardRepresentation::is_index_light_square(
            BoardRepresentation::index_from_coord(3, 5)
        ));
        assert!(!BoardRepresentation::is_index_light_square(
            BoardRepresentation::index_from_coord(0, 0)
        ));
        assert!(!BoardRepresentation::is_index_light_square(
            BoardRepresentation::index_from_coord(0, 4)
        ));
        assert!(!BoardRepresentation::is_index_light_square(
            BoardRepresentation::index_from_coord(7, 3)
        ));
    }
}
