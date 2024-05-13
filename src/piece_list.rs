pub struct PieceList {
    squares: Vec<u8>,
}

impl PieceList {
    pub const empty: PieceList = PieceList { squares: Vec::new() };

    pub fn add_piece(&mut self, square: u8) {
        #[cfg(test)]
        if self.squares.contains(&square) {
            panic!("PieceList::add_piece called with duplicate square");
        }

        self.squares.push(square);
    }

    pub fn remove_piece(&mut self, square: u8) {
        #[cfg(test)]
        if !self.squares.contains(&square) {
            panic!("PieceList::remove_piece called with untracked square");
        }

        let index = self.squares.iter().position(|x| *x == square).unwrap();
        self.squares.remove(index);
    }

    pub fn move_piece(&mut self, start_square: u8, target_square: u8) {
        #[cfg(test)]
        if !self.squares.contains(&start_square) {
            panic!("PieceList::move_piece called with untracked start_square");
        }

        let index = self.squares.iter().position(|x| *x == start_square).unwrap();
        self.squares[index] = target_square;
    }

    pub fn get_squares(self) -> Vec<u8> {
        self.squares
    }

    pub fn get_count(self) -> usize {
        self.squares.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
