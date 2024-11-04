const BRICK_CHAR: char = 'X';

pub trait BrickMove {
    /// Moves the brick on dx, dy steps
    fn r#move(&mut self, dx: i32, dy: i32);
    /// Check if can move brick in provided game
    fn can_move(&mut self, game: &Game, dx: i32, dy: i32) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct Brick {
    x: i32,
    y: i32,
}

impl Brick {
    pub fn new(x: i32, y: i32) -> Self {
        Brick { x, y }
    }
}

impl BrickMove for Brick {
    fn r#move(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn can_move(&mut self, game: &Game, dx: i32, dy: i32) -> bool {
        let x = self.x + dx;
        let y = self.y + dy;

        (game.border_size..game.width + 1).contains(&x)
            && (0..game.height + 1).contains(&y)
            && game
                .building
                .iter()
                .any(|building| building.cords == (x, y))
    }
}

#[derive(Debug)]
pub struct Block {
    center_brick_index: usize,
    bricks: Vec<Brick>,
    center: Brick,
}

/// Block is trait that can move and rotate blocks on the game board.
trait BlockMethods<'a> {
    fn new(center_x: i32, center_y: i32) -> Block {
        let center_brick_index = 1;
        let bricks: Vec<Brick> = Self::init_coordinates(center_x, center_y)
            .map(|position| Brick::new(position.0, position.1))
            .collect();
        Block {
            center_brick_index,
            center: bricks[center_brick_index],
            bricks,
        }
    }

    fn init_coordinates(center_x: i32, center_y: i32) -> impl Iterator<Item = (i32, i32)>;
}

/// Main Game object
#[derive(Debug)]
pub struct Game {
    win: Option<bool>,
    current_brick: Option<i32>,
    border_size: i32,
    directions: Option<GameDirection>,
    rotate_directions: Option<GameRotateDirections>,
    blocks: Vec<Brick>,
    building: Vec<Building>,
    width: i32,
    height: i32,
    speed: i32,
}

#[derive(Debug)]
struct Building {
    cords: (i32, i32),
    value: Option<i32>,
}

impl Game {
    pub fn new() -> Self {
        Game::default()
    }
}

/// Enum direction key pressed in game
#[derive(Debug)]
enum GameDirection {
    KeyLeft((i32, i32)),
    KeyRight((i32, i32)),
    Down((i32, i32)),
}

/// Enum rotate directions in game
#[derive(Debug)]
enum GameRotateDirections {
    KeyUp(i32),
    KeyDown(i32),
}

impl Default for Game {
    fn default() -> Self {
        Game {
            win: None,
            current_brick: None,
            border_size: 1,
            directions: None,
            rotate_directions: None,
            blocks: Vec::new(),
            building: Vec::new(),
            width: 20,
            height: 20,
            speed: 200,
        }
    }
}

pub fn main() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{Brick, BrickMove, Game};

    #[test]
    fn test_tetris_game() {
        let game = Game::new();

        // NOTE: Create game with default values.
        assert!(game.win.is_none());
        assert!(game.current_brick.is_none());
        assert_eq!(game.border_size, 1)
    }

    #[test]
    fn test_tetra_brick() {
        let mut brick = Brick::new(1, 2);

        assert_eq!(brick.x, 1);
        assert_eq!(brick.y, 2);

        brick.r#move(3, 4);

        assert_eq!(brick.x, 1 + 3);
        assert_eq!(brick.y, 2 + 4);

        let game = Game::new();
        let can_move_brick: bool = brick.can_move(&game, 2, 3);

        assert!(!can_move_brick);
    }

    #[test]
    fn test_tetra_block() {
        let block = Block::new();
    }
}
