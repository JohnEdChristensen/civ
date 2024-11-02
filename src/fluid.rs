use itertools::iproduct;

use crate::{
    math::{Point, Vec2},
    world::{PX_HEIGHT, PX_WIDTH},
};

const REPLUSE: f64 = 0.0000001;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

#[derive(Clone, Copy)]
pub struct Particle {
    pub position: Point,
    pub velocity: Vec2,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Particle {
            position: Vec2::new(x, y),
            velocity: Vec2::new(0.0, 0.0),
        }
    }
}

pub struct Fluid {
    pub particles: Vec<Particle>,
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}
impl Fluid {
    pub fn new() -> Self {
        Self {
            particles: iproduct!(1..19, 1..19)
                .map(|(x, y)| Particle::new(0.5 + 5. * x as f64, 0.5 + 5. * y as f64))
                .collect(),
            left: 0.0,
            right: PX_WIDTH as f64,
            top: 0.0,
            bottom: PX_HEIGHT as f64,
        }
    }
    pub fn update(&mut self, dt: f64) {
        println!("update!");
        let prev_particles = &self.particles.clone();
        self.particles = prev_particles
            .iter()
            .enumerate()
            .map(|(i, particle)| {
                let mut pos = particle.position;
                let mut vel = particle.velocity;
                //let mut x = particle.position.x;
                //let mut y = particle.position.y;
                //let mut vx = particle.velocity.x;
                //let mut vy = particle.velocity.y;
                // set forces
                let repulsion_force: Vec2 = prev_particles
                    .iter()
                    .enumerate()
                    .map(|(j, particle)| {
                        if i != j {
                            let distance = pos - particle.position;
                            if distance.sqr_magnitude() > 0.0000001 {
                                REPLUSE * distance.norm() * (1. / distance.sqr_magnitude())
                            } else {
                                panic!("too close! {:?},{:?}", pos, particle.position)
                            }
                        } else {
                            Vec2::new(0., 0.)
                        }
                    })
                    .sum();
                dbg!(repulsion_force);
                //// integrate forces

                //// integrate velocity
                pos.x += vel.x * dt * 50.0;
                pos.y += vel.y * dt * 50.0;

                //// bounds check
                //// check bounds
                if pos.x < self.left {
                    vel.x *= -1.;
                    pos.x = self.left;
                }
                if pos.x >= self.right {
                    vel.x *= -1.;
                    pos.x = self.right - 1.;
                }
                if pos.y < self.top {
                    vel.y *= -1.;
                    pos.y = self.top;
                }
                if pos.y >= self.bottom {
                    vel.y *= -1.;
                    pos.y = self.bottom - 1.;
                }

                Particle {
                    position: pos,
                    velocity: vel,
                }
            })
            .collect();
    }
}

impl Default for Fluid {
    fn default() -> Self {
        Self::new()
    }
}