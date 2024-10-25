use winit::keyboard::Key;
use winit_input_helper::WinitInputHelper;

use crate::{
    character::{Character, Direction},
    map::Map,
    tile::{TileMap, TileType},
};
pub const WIDTH: usize = 32;
pub const HEIGHT: usize = 32;
pub const TILE_SIZE: usize = 8;

pub const PX_WIDTH: usize = WIDTH * TILE_SIZE;
pub const PX_HEIGHT: usize = HEIGHT * TILE_SIZE;

const PALLETTE: [[u8; 4]; 16] = [
    [220, 98, 80, 255],   //r1
    [255, 223, 91, 255],  //y1
    [163, 208, 118, 255], //g1
    [188, 217, 225, 255], //b1
    [157, 126, 213, 255], //p1
    [150, 31, 31, 255],   //r2
    [242, 140, 58, 255],  //y2
    [126, 151, 95, 255],  //g2
    [90, 139, 222, 255],  //b2
    [116, 62, 134, 255],  //p2
    [123, 166, 181, 255], //n1
    [77, 120, 137, 255],  //n2
    [49, 80, 94, 255],    //n3
    [255, 212, 163, 255], //s1
    [153, 67, 45, 255],   //s2
    [33, 8, 32, 255],     //bg
];

pub struct World {
    pub map: Map,
    pub character: Character,
}
impl World {
    pub fn new() -> Self {
        Self {
            map: Map::new(
                WIDTH,
                HEIGHT,
                TileMap::new("assets/minimal_8_v1.2.png".into()),
            ),
            character: Character {
                tile: TileType::Character,
                x: 0,
                y: 0,
            },
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub fn update(&mut self, input: &WinitInputHelper) -> bool {
        let mut redraw_required = false;

        //// Character update
        if input.key_pressed_os_logical(Key::Character("w")) {
            self.character.try_move_to(Direction::Up, &self.map);
            redraw_required = true;
        }
        if input.key_pressed_os_logical(Key::Character("a")) {
            self.character.try_move_to(Direction::Left, &self.map);
            redraw_required = true;
        }
        if input.key_pressed_os_logical(Key::Character("s")) {
            self.character.try_move_to(Direction::Down, &self.map);
            redraw_required = true;
        }
        if input.key_pressed_os_logical(Key::Character("d")) {
            self.character.try_move_to(Direction::Right, &self.map);
            redraw_required = true;
        }

        ////camera pos update
        if redraw_required {
            self.map.shift_to_point(self.character.x, self.character.y)
        }

        redraw_required
    }

    /// Draw the `World` state to the frame buffer.
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&self, frame: &mut [u8]) {
        let layered_tiles = self
            .map
            //.layer(self.character.tile, self.character.x, self.character.y);
            .layer(self.character.tile, self.map.width / 2, self.map.height / 2);

        for (ty, row) in layered_tiles.iter().enumerate() {
            for (tx, tile) in row.iter().enumerate() {
                let sprite = self.map.tile_map.sprite(tile);
                //println!("{tx},{ty}");
                let px = tx * TILE_SIZE;

                sprite.chunks(TILE_SIZE).enumerate().for_each(|(j, row)| {
                    let row_colors = row
                        .iter()
                        .flat_map(|c| PALLETTE[*c as usize])
                        .collect::<Vec<_>>();
                    let py = (ty * TILE_SIZE) + j;
                    //println!("  {px},{py}");
                    let start = (px + py * (WIDTH * TILE_SIZE)) * 4;
                    let end = start + TILE_SIZE * 4;
                    frame[start..end].copy_from_slice(&row_colors);
                });
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
