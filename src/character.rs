use crate::{map::Map, tile::TileType};

pub struct Character {
    pub tile: TileType,
    pub x: i32,
    pub y: i32,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Character {
    pub fn try_move_to(&mut self, direction: Direction, map: &Map) {
        let center_x = map.width / 2;
        let center_y = map.height / 2;
        let offset = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let x = self.x + offset.0;
        let y = self.y + offset.1;

        let new_center_x = center_x as i32 + offset.0;
        let new_center_y = center_y as i32 + offset.1;

        let previous_tile = map
            .tile_map
            .tiles
            .get(&map.tiles[new_center_y as usize][new_center_x as usize])
            .unwrap();
        match previous_tile.navigation {
            crate::tile::Navigation::Wall => (),
            crate::tile::Navigation::Floor => {
                self.x = x;
                self.y = y;
            }
        }
    }
}
