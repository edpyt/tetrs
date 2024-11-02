const BLOCK_CHAR: char = 'X';

pub trait BlockMove {
    /// Moves the brick on dx, dy steps
    fn r#move(&mut self, dx: i32, dy: i32);
    /// Check if can move block in provided game
    fn can_move(&mut self, game: &Game, dx: i32, dy: i32) -> bool;
}

#[derive(Debug)]
pub struct Block {
    x: i32,
    y: i32,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Self {
        Block { x, y }
    }
}

impl BlockMove for Block {
    fn r#move(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn can_move(&mut self, game: &Game, dx: i32, dy: i32) -> bool {
        let x = self.x + dx;
        let y = self.y + dy;

        true
    }
}

#[derive(Debug)]
pub struct Game {
    win: Option<bool>,
    current_block: Option<Block>,
    border_size: i32,
}

impl Game {
    pub fn new() -> Self {
        Game { ..Game::default() }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            win: None,
            current_block: None,
            border_size: 1,
        }
    }
}

pub fn main() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{Block, BlockMove, Game};

    #[test]
    fn test_tetris_game() {
        let game = Game::new();

        // NOTE: Create game with default values.
        assert!(game.win.is_none());
        assert!(game.current_block.is_none());
        assert_eq!(game.border_size, 1)
    }

    #[test]
    fn test_tetra_block() {
        let mut block = Block::new(1, 2);

        assert_eq!(block.x, 1);
        assert_eq!(block.y, 2);

        block.r#move(3, 4);

        assert_eq!(block.x, 1 + 3);
        assert_eq!(block.y, 2 + 4);

        let game = Game::new();
        // TODO:
        // let can_move_block: bool = block.can_move(&game, , dy)
    }
}
