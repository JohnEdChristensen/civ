use std::path::PathBuf;

use image::{GenericImageView, ImageBuffer, Rgba};

#[derive(Clone)]
pub enum Tile {
    Empty,
    Character,
    Grass,
}

pub struct TileMap {
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl TileMap {
    pub fn new(path: PathBuf) -> Self {
        TileMap {
            image: image::open(path).unwrap().into_rgba8(),
        }
    }
    pub fn sprite(&self, tile: &Tile) -> Vec<u8> {
        match tile {
            Tile::Empty => vec![15; 64],
            Tile::Character => self
                .image
                .view(328, 32, 8, 8)
                .pixels()
                .map(|(_, _, i)| if i.0[3] == 0 { 15 } else { 10 })
                .collect(),
            Tile::Grass => self
                .image
                .view(64, 216, 8, 8)
                .pixels()
                .map(|(_, _, i)| if i.0[3] == 0 { 15 } else { 2 })
                .collect(),
        }
    }
}

pub fn pattern_1(width: usize, height: usize) -> Vec<Tile> {
    let mut tiles = vec![Tile::Empty; width * height];

    tiles[width / 2 + (width * height / 2)] = Tile::Character;
    tiles[10 + 10 * width] = Tile::Grass;
    tiles[13 + 11 * width] = Tile::Grass;
    tiles[21 + 22 * width] = Tile::Grass;
    tiles[20 + 23 * width] = Tile::Grass;
    tiles
    //tiles
    //    .iter_mut()
    //    .enumerate()
    //    .map(|(i, _)| {
    //        let x = i % width;
    //        let y = i / height;
    //
    //        if (x + y) % 2 == 0 {
    //            Tile::Character
    //        } else {
    //            Tile::Grass
    //        }
    //    })
    //    .collect()
}
