use itertools::Itertools;

use noise::{utils::*, Fbm, Perlin};

use crate::tile::{TileMap, TileType};

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TileType>>,
    pub tile_map: TileMap,
}

impl Map {
    pub fn new(width: usize, height: usize, tile_map: TileMap) -> Self {
        let noise_width = 0.6;
        let noise_height = noise_width; //noise_width * (height / width) as f64;
                                        //let fbm = Fbm::<Perlin>::new(0);

        let fbm = Fbm::<Perlin>::default();

        let noise = PlaneMapBuilder::new(fbm)
            .set_size(width, height)
            .set_x_bounds(-noise_width, noise_width)
            .set_y_bounds(-noise_height, noise_height)
            .build()
            .into_iter()
            .chunks(width);

        let tiles: Vec<Vec<TileType>> = noise
            .into_iter()
            .map(|row| {
                row.map(|v| match v {
                    x if x < -0.2 => TileType::DeepWater,
                    x if x < -0.1 => TileType::ShallowWater,
                    x if x < 0.0 => TileType::Beach,
                    x if x < 0.3 => TileType::TallGrass,
                    x if x < 0.4 => TileType::Hill,
                    x if x < 1.0 => TileType::Mountain,
                    _ => TileType::Empty,
                })
                .collect()
            })
            .collect();
        //let tiles = flat_tiles
        //    .chunks(width)
        //    .map(|&row| row.iter().copied().collect())
        //    .collect();
        //let tiles: Vec<Vec<Tile>> = (0..height)
        //    .map(|_| {
        //        (0..width)
        //            .map(|_| match rng.sample(range) {
        //                0 => Tile::Grass,
        //                _ => Tile::Empty,
        //            })
        //            .collect()
        //    })
        //    .collect();

        Map {
            width,
            height,
            tiles,
            tile_map,
        }
    }
    pub fn layer(&self, tile: TileType, x: usize, y: usize) -> Vec<Vec<TileType>> {
        let mut out = self.tiles.clone();
        out[y][x] = tile;
        out
    }
}
