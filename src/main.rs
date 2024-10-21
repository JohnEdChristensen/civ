#![deny(clippy::all)]
#![forbid(unsafe_code)]

use civ::character::{Character, Direction};
use civ::map::Map;
use civ::tile::{TileMap, TileType};
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: usize = 32 * 2;
const HEIGHT: usize = 24 * 2;

const TILE_SIZE: usize = 8;

const PX_WIDTH: usize = WIDTH * TILE_SIZE;
const PX_HEIGHT: usize = HEIGHT * TILE_SIZE;
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

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    map: Map,
    character: Character,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(PX_WIDTH as f64, PX_HEIGHT as f64);
        let scaled = LogicalSize::new(size.width * 2.4, size.height * 2.4);
        WindowBuilder::new()
            .with_title(":)")
            .with_inner_size(scaled)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(PX_WIDTH as u32, PX_HEIGHT as u32, surface_texture)?
    };
    let mut world = World::new();

    let res = event_loop.run(|event, elwt| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            world.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                elwt.exit();
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    elwt.exit();
                    return;
                }
            }
            if input.key_held(KeyCode::KeyW) {
                world.character.try_move_to(Direction::Up, &world.map);
            }
            if input.key_held(KeyCode::KeyA) {
                world.character.try_move_to(Direction::Left, &world.map);
            }
            if input.key_held(KeyCode::KeyS) {
                world.character.try_move_to(Direction::Down, &world.map);
            }
            if input.key_held(KeyCode::KeyD) {
                world.character.try_move_to(Direction::Right, &world.map);
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            map: Map::new(
                WIDTH,
                HEIGHT,
                TileMap::new("assets/minimal_8_v1.2.png".into()),
            ),
            character: Character {
                tile: TileType::Character,
                x: WIDTH / 2,
                y: HEIGHT / 2,
            },
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {}

    /// Draw the `World` state to the frame buffer.
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        let layered_tiles = self
            .map
            .layer(self.character.tile, self.character.x, self.character.y);
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
