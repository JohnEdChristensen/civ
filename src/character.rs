use crate::{map::Map, tile::TileType};

pub struct Character {
    pub tile: TileType,
    pub x: usize,
    pub y: usize,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Character {
    pub fn try_move_to(&mut self, direction: Direction, map: &Map) {
        let offset = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let x = self.x as i32 + offset.0;
        let y = self.y as i32 + offset.1;

        if x >= 0 && x < map.width as i32 && y >= 0 && y < map.height as i32 {
            let x = x as usize;
            let y = y as usize;
            let previous_tile = map.tile_map.tiles.get(&map.tiles[y][x]).unwrap();
            match previous_tile.navigation {
                crate::tile::Navigation::Wall => (),
                crate::tile::Navigation::Floor => {
                    self.x = x;
                    self.y = y;
                }
            }
        };
    }
}
