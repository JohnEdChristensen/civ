use std::{collections::HashMap, path::PathBuf};

use image::{GenericImageView, ImageBuffer, Rgba};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TileType {
    Empty,
    Character,
    Grass,
    TallGrass,
    DeepWater,
    ShallowWater,
    Hill,
    Mountain,
    Beach,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Navigation {
    Wall,
    Floor,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Tile {
    pub kind: TileType,
    sprite: Vec<u8>,
    pub navigation: Navigation,
}

pub struct TileMap {
    pub tiles: HashMap<TileType, Tile>,
}

impl TileMap {
    pub fn new(path: PathBuf) -> Self {
        let image = image::open(path).unwrap().into_rgba8();
        let tiles = HashMap::from([
            (
                TileType::Empty,
                Tile {
                    kind: TileType::Empty,
                    sprite: vec![15; 64],
                    navigation: Navigation::Floor,
                },
            ),
            (
                TileType::Character,
                Tile {
                    kind: TileType::Character,
                    sprite: Self::grab_sprite(&image, 328, 32, 15, 10),
                    navigation: Navigation::Wall,
                },
            ),
            (
                TileType::ShallowWater,
                Tile {
                    kind: TileType::ShallowWater,
                    sprite: Self::grab_sprite(&image, 96, 192, 10, 3),
                    navigation: Navigation::Wall,
                },
            ),
            (
                TileType::DeepWater,
                Tile {
                    kind: TileType::DeepWater,
                    sprite: Self::grab_sprite(&image, 96, 208, 11, 8),
                    navigation: Navigation::Wall,
                },
            ),
            (
                TileType::Hill,
                Tile {
                    kind: TileType::Hill,
                    sprite: Self::grab_sprite(&image, 224, 208, 12, 11),
                    navigation: Navigation::Floor,
                },
            ),
            (
                TileType::Mountain,
                Tile {
                    kind: TileType::Mountain,
                    sprite: Self::grab_sprite(&image, 232, 208, 11, 10),
                    navigation: Navigation::Floor,
                },
            ),
            (
                TileType::Beach,
                Tile {
                    kind: TileType::Beach,
                    sprite: Self::grab_sprite(&image, 176, 208, 13, 6),
                    navigation: Navigation::Floor,
                },
            ),
            (
                TileType::TallGrass,
                Tile {
                    kind: TileType::TallGrass,
                    sprite: Self::grab_sprite(&image, 192, 208, 7, 2),
                    navigation: Navigation::Floor,
                },
            ),
            (
                TileType::Grass,
                Tile {
                    kind: TileType::Grass,
                    sprite: Self::grab_sprite(&image, 64, 216, 15, 2),
                    navigation: Navigation::Floor,
                },
            ),
        ]);

        TileMap { tiles }
    }
    fn grab_sprite(
        image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        x_pos: u32,
        y_pos: u32,
        bg: u8,
        fg: u8,
    ) -> Vec<u8> {
        image
            .view(x_pos, y_pos, 8, 8)
            .pixels()
            .map(|(_, _, i)| if i.0[3] == 0 { bg } else { fg })
            .collect()
    }

    pub fn sprite(&self, tile: &TileType) -> &Vec<u8> {
        &self.tiles.get(tile).unwrap().sprite
    }
}
