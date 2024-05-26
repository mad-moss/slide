use macroquad::prelude::*;
use crate::Direction::*;

const TILE_SIZE: u8 = 50;
const PUZZLE_SIZE: u8 = 4;
type Tiles = Vec<Vec<Option<u8>>>;

#[macroquad::main("Slide Puzzle")]
async fn main() {
    let mut puzzle = Puzzle::new(PUZZLE_SIZE);
    loop {


        for y in 0..puzzle.size {
            for x in 0..puzzle.size {
                if is_key_pressed(KeyCode::W) {
                    puzzle.slide(x, y, Up);
                }
                if is_key_pressed(KeyCode::S) {
                    puzzle.slide(x, y, Down);
                }
                if is_key_pressed(KeyCode::A) {
                    puzzle.slide(x, y, Left);
                }
                if is_key_pressed(KeyCode::D) {
                    puzzle.slide(x, y, Right);
                }
            }
        }

        
        clear_background(BLACK);

        //draw tiles
        for y in 0..puzzle.size {
            let y_offset = (y * TILE_SIZE) as f32;
            for x in 0..puzzle.size {
                let x_offset = (x * TILE_SIZE) as f32;
                if let Some(number) = puzzle.tiles[y as usize][x as usize] {
                    draw_rectangle(x_offset, y_offset, TILE_SIZE as f32, TILE_SIZE as f32, RED);
                    draw_text(number.to_string().as_ref(), x_offset, y_offset + TILE_SIZE as f32 / 2., TILE_SIZE as f32, WHITE);
                }
            }
        }

        next_frame().await
    }
}

struct Puzzle {
    tiles: Tiles,
    size: u8,
}

impl Puzzle {
    fn new(size: u8) -> Self {
        let mut tiles = Tiles::new();
        for y in 0..size {
            let mut row: Vec<Option<u8>> = Vec::new();
            for x in 0..size {
                if y == size - 1 && x == size - 1 {
                    row.push(None);
                }
                row.push(Some(y * size + x));
            }
            tiles.push(row);
        }
        Self {
            tiles,
            size,
        }
    }
    fn is_empty(&self, x: u8, y: u8) -> bool {
        if x >= self.size || y >= self.size {
            return false;
        }
        self.tiles[y as usize][x as usize].is_none()
    }
    fn slide(&mut self, x: u8, y: u8, direction: Direction) {
        let (tx, ty) = match direction {
            Up => (x as i8, y as i8 - 1),
            Left => (x as i8 - 1, y as i8),
            Right => (x as i8 + 1, y as i8),
            Down => (x as i8, y as i8 + 1),
        };
        if tx < 0 || ty < 0 || !self.is_empty(tx as u8, ty as u8) {
            return;
        }
        self.tiles[ty as usize][tx as usize] = self.tiles[y as usize][x as usize];
        self.tiles[y as usize][x as usize] = None;
    }
}

enum Direction {
    Up,
    Left,
    Right,
    Down,
}