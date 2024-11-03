use itertools::iproduct;

use crate::{
    math::{Point, Vec2},
    world::{PX_HEIGHT, PX_WIDTH},
};

const REPLUSE: f64 = 0.5;
const GRAVITY: f64 = 1.0; //0.01;
const COLLISION_DAMPING: f64 = 0.9;

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
                .map(|(x, y)| {
                    Particle::new(
                        0.5 + 5. * x as f64,
                        PX_HEIGHT as f64 - (0.5 + 5. * y as f64),
                    )
                })
                .collect(),
            left: 0.0,
            right: PX_WIDTH as f64,
            top: 0.0,
            bottom: PX_HEIGHT as f64,
        }
    }
    pub fn update(&mut self, dt: f64) {
        let prev_particles = &self.particles.clone();
        self.particles = prev_particles
            .iter()
            .enumerate()
            .map(|(i, particle)| {
                let init_pos = particle.position;
                let mut pos = particle.position;
                let mut vel = particle.velocity;

                let distances = prev_particles
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| i != *j)
                    .map(|(_, particle)| (init_pos - particle.position));
                //.filter(|d| d.sqr_magnitude() < (50_f64).powi(2)); // only worry about
                // nearby particles

                // move overlaping particles
                if distances
                    .clone()
                    .filter(|d| d.sqr_magnitude() < 0.1)
                    .count()
                    > 0
                {
                    pos.x += 0.01 * i as f64;
                    pos.y += 0.01 * i as f64;
                }

                let repulsion_force: Vec2 = distances
                    .filter(|d| d.sqr_magnitude() > 0.1)
                    .map(|d| {
                        let rsqr = d.sqr_magnitude();
                        let sigma: f64 = 3.;
                        let v = 4.
                            * REPLUSE
                            * (sigma.powi(12) / (rsqr.powi(6)) - sigma.powi(6) / (rsqr.powi(3)));

                        d.norm() * v
                    })
                    .sum();
                //prev_particles
                //    .iter()
                //    .enumerate()
                //    .map(|(j, particle)| {
                //        if i != j {
                //            let distance = pos - particle.position;
                //            if distance.sqr_magnitude() > 0.0000001 {
                //                REPLUSE * distance.norm() * (1. / distance.sqr_magnitude())
                //            } else {
                //                panic!("too close! {:?},{:?}", pos, particle.position)
                //            }
                //        } else {
                //            Vec2::new(0., 0.)
                //        }
                //    })
                //    .sum();
                //dbg!(repulsion_force);

                let gravity = Vec2::new(0., GRAVITY);
                //let drag = 0.0;
                let force = repulsion_force + gravity;
                //// integrate forces
                vel.x += force.x * dt;
                vel.y += force.y * dt;

                //let max_speed = 2.;
                //vel.x = f64::clamp(vel.x, -max_speed, max_speed);
                //vel.y = f64::clamp(vel.y, -max_speed, max_speed);
                //vel -= -1. * (vel * vel) * 0.01;
                vel *= 0.9;
                //// integrate velocity
                pos.x += vel.x * dt * 50.0;
                pos.y += vel.y * dt * 50.0;

                //// bounds check
                //// check bounds
                if pos.x < self.left {
                    vel.x = vel.x.abs() * (COLLISION_DAMPING);
                    pos.x = self.left;
                }
                if pos.x > self.right {
                    vel.x = vel.x.abs() * (-COLLISION_DAMPING);
                    pos.x = self.right - 0.001;
                }
                if pos.y < self.top {
                    vel.y = vel.y.abs() * (COLLISION_DAMPING);
                    pos.y = self.top;
                }
                if pos.y > self.bottom {
                    vel.y = vel.y.abs() * (-COLLISION_DAMPING);
                    pos.y = self.bottom - 0.001;
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
