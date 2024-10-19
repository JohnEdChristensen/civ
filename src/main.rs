#![deny(clippy::all)]
#![forbid(unsafe_code)]

use civ::tile::{pattern_1, Tile, TileMap};
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: usize = 32;
const HEIGHT: usize = 24;

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
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
    tiles: Vec<Tile>,
    tile_map: TileMap,
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
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
            tiles: pattern_1(WIDTH, HEIGHT),
            tile_map: TileMap::new("assets/minimal_8_v1.2.png".into()),
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        //if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
        //    self.velocity_x *= -1;
        //}
        //if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
        //    self.velocity_y *= -1;
        //}
        //
        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, tile) in self.tiles.iter().enumerate() {
            let sprite = self.tile_map.sprite(tile);
            let tx = i % WIDTH;
            let ty = i / WIDTH;
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

            //
            //    pixel.copy_from_slice(&rgba);
            //for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            //    let x = (i % WIDTH as usize) as i16;
            //    let y = (i / WIDTH as usize) as i16;
            //
            //    let inside_the_box = x >= self.box_x
            //        && x < self.box_x + BOX_SIZE
            //        && y >= self.box_y
            //        && y < self.box_y + BOX_SIZE;
            //
            //    let rgba = if inside_the_box {
            //        [0x5e, 0x48, 0xe8, 0xff]
            //    } else {
            //        [0x48, 0xb2, 0xe8, 0xff]
            //    };
            //
            //    pixel.copy_from_slice(&rgba);
            //}
        }
    }
}
