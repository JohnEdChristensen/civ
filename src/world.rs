use itertools::Itertools;
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

pub const PX_WIDTH: usize = 100;
pub const PX_HEIGHT: usize = 150;

pub struct World {}

impl World {
    pub fn draw(&self, frame: &mut [u8]) {
        frame
            .chunks_exact_mut(4)
            .chunks(PX_WIDTH)
            .into_iter()
            .enumerate()
            .for_each(|(j, row)| {
                row.enumerate().for_each(|(i, c)| {
                    c.copy_from_slice(&[
                        (i * 255 / PX_WIDTH) as u8,
                        (j * 255 / PX_HEIGHT) as u8,
                        ((i + j) * 255 / (PX_WIDTH + PX_HEIGHT)) as u8,
                        255,
                    ])
                });
            });
    }
    pub fn update(&mut self, event: &WinitInputHelper) -> bool {
        if event.key_pressed(KeyCode::KeyW) {
            println!("thats a 'w'!");
        }
        true
    }
}
