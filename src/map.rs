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
        Map {
            width,
            height,
            tiles: Self::map_gen(width, height, 0., 0.),
            tile_map,
        }
    }
    pub fn map_gen(width: usize, height: usize, x: f64, y: f64) -> Vec<Vec<TileType>> {
        let noise_width = 0.6;
        let noise_height = noise_width; //noise_width * (height / width) as f64;
                                        //let fbm = Fbm::<Perlin>::new(0);

        let fbm = Fbm::<Perlin>::default();

        let noise = PlaneMapBuilder::new(fbm)
            .set_size(width, height)
            .set_x_bounds(-noise_width + x, noise_width + x)
            .set_y_bounds(-noise_height + y, noise_height + y)
            .build()
            .into_iter()
            .chunks(width);

        noise
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
            .collect()
    }
    pub fn layer(&self, tile: TileType, x: usize, y: usize) -> Vec<Vec<TileType>> {
        let mut out = self.tiles.clone();
        out[y][x] = tile;
        out
    }
    pub fn shift_to_point(&mut self, x: i32, y: i32) {
        let x = (1.2 / self.width as f64) * x as f64;
        let y = (1.2 / self.height as f64) * y as f64;
        self.tiles = Self::map_gen(self.width, self.height, x, y)
    }
}
