use itertools::Itertools;
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

use crate::fluid::{Color, Fluid, WHITE};

pub const PX_WIDTH: usize = 100;
pub const PX_HEIGHT: usize = 150;

#[derive(Default)]
pub struct World {
    frame_count: i32,
    pub fluid: Fluid,
}

impl World {
    pub fn draw(&self, frame: &mut [u8]) {
        ////init
        if self.frame_count == 0 {
            frame
                .chunks_exact_mut(4)
                .chunks(PX_WIDTH)
                .into_iter()
                .enumerate()
                .for_each(|(j, row)| {
                    row.enumerate().for_each(|(i, c)| {
                        c.copy_from_slice(&[
                            (50 + j * 255 / (PX_WIDTH * 4)) as u8,
                            (15 + j * 255 / (PX_HEIGHT * 4)) as u8,
                            (25 + (i + j) * 255 / ((PX_WIDTH + PX_HEIGHT) * 4)) as u8,
                            255,
                        ])
                    });
                });
        }
        //// draw
        self.fluid
            .particles
            .iter()
            .for_each(|p| set_pixel(frame, p.position.x as usize, p.position.y as usize, WHITE));
    }
    pub fn update(&mut self, event: &WinitInputHelper) -> bool {
        if event.key_pressed(KeyCode::KeyW) {
            println!("thats a 'w'!");
        }
        //let dt = event.delta_time().unwrap().as_secs_f64();

        //// step
        //self.fluid.update(dt);

        true
    }
}

fn set_pixel(frame: &mut [u8], x: usize, y: usize, color: Color) {
    let i = (x + y * PX_WIDTH) * 4;
    frame[i..i + 4].copy_from_slice(&[color.r, color.g, color.b, color.a]);
}
