use std::{iter::Sum, ops};

use derive_more::derive::Constructor;

pub type Point = Vec2;

#[derive(Debug, Constructor, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn sqr_magnitude(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn norm(&self) -> Vec2 {
        let mag = self.magnitude();
        if mag != 0. {
            self * (1. / mag)
        } else {
            panic!("can't normalize zero!")
        }
    }
}

impl Sum for Vec2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec2::new(0., 0.), |acc, e| acc + e)
    }
}
//impl Mul<f64> for Vec1 {
//    type Output = Vec1;
//
//    fn mul(self, rhs: f64) -> Self::Output {
//        Vec1::new(self.x * rhs, self.y * rhs)
//    }
//}

/// Generates the operations for vector methods. `let result = my_vec_3 + my_other_vec2`
/// Handles `Vec2, Vec2`, `Vec2, &Vec2`, `&Vec2, Vec2`, `&Vec2, &Vec2`
/// `vec2_vec2_op(ops::AddAssign, add_assign)` (note the camelcase add_assign name)
macro_rules! vec2_vec2_op {
    ($($path:ident)::+, $fn:ident) => {
        impl $($path)::+<Vec2> for Vec2 {
            type Output = Vec2;

            fn $fn(self, other: Vec2) -> Self::Output {
                Vec2 {
                    x: self.x.$fn(other.x),
                    y: self.y.$fn(other.y),
                }
            }
        }

        impl $($path)::+<&Vec2> for &Vec2 {
            type Output = Vec2;

            fn $fn(self, other: &Vec2) -> Self::Output {
                Vec2 {
                    x: self.x.$fn(other.x),
                    y: self.y.$fn(other.y),
                }
            }
        }

        impl $($path)::+<&Vec2> for Vec2 {
            type Output = Vec2;

            fn $fn(self, other: &Vec2) -> Self::Output {
                Vec2 {
                    x: self.x.$fn(other.x),
                    y: self.y.$fn(other.y),
                }
            }
        }

        impl $($path)::+<Vec2> for &Vec2 {
            type Output = Vec2;

            fn $fn(self, other: Vec2) -> Self::Output {
                Vec2 {
                    x: self.x.$fn(other.x),
                    y: self.y.$fn(other.y),
                }
            }
        }
    };
}

/// Generates the operations for vector method assignment. `my_vec += my_other_vec`
/// Handles `Vec2, Vec2` and `Vec2, &Vec2`
/// `vec2_vec2_opassign(ops::AddAssign, add_assign)` (note the camelcase add_assign name)
macro_rules! vec2_vec2_opassign {
    ($($path:ident)::+, $fn:ident) => {
        impl $($path)::+<Vec2> for Vec2 {
            fn $fn(&mut self, other: Vec2) {
                self.x.$fn(other.x);
                self.y.$fn(other.y);
            }
        }

        impl $($path)::+<&Vec2> for Vec2 {
            fn $fn(&mut self, other: &Vec2) {
                self.x.$fn(other.x);
                self.y.$fn(other.y);
            }
        }
    };
}

/// Generates the operations for method assignment. `my_vec += f32`
/// `vec2_opassign(ops:AddAssign, add_assign)` (note the camelcase add_assign name)
macro_rules! vec2_opassign {
    ($($path:ident)::+, $fn:ident, $ty:ty) => {
        impl $($path)::+<$ty> for Vec2 {
            fn $fn(&mut self, other: $ty) {
                self.x.$fn(other);
                self.y.$fn(other);
            }
        }
    }
}

/// Generates the operations for the method. `let result = my_vec + 4f32`
/// Handles `Vec2, T`, `T, Vec2`, `&Vec2, T`, `T, &Vec2`
/// `vec2_op!(ops:Add, add, f32)`
macro_rules! vec2_op {
    ($($path:ident)::+, $fn:ident, $ty:ty) => {
        // impl ops::Add::add for Vec2
        impl $($path)::+<$ty> for Vec2 {
            type Output = Vec2;

            // fn add(self, other: f32) -> Self::Output
            fn $fn(self, other: $ty) -> Self::Output {
                Vec2 {
                    // x: self.x.add(other)
                    x: self.x.$fn(other),
                    y: self.y.$fn(other),
                }
            }
        }

        impl $($path)::+<$ty> for &Vec2 {
            type Output = Vec2;

            fn $fn(self, other: $ty) -> Self::Output {
                Vec2 {
                    x: self.x.$fn(other),
                    y: self.y.$fn(other),
                }
            }
        }

        impl $($path)::+<Vec2> for $ty {
            type Output = Vec2;

            fn $fn(self, other: Vec2) -> Self::Output {
                Vec2 {
                    x: self.$fn(other.x),
                    y: self.$fn(other.y),
                }
            }
        }

        impl $($path)::+<&Vec2> for $ty {
            type Output = Vec2;

            fn $fn(self, other: &Vec2) -> Self::Output {
                Vec2 {
                    x: self.$fn(other.x),
                    y: self.$fn(other.y),
                }
            }
        }
    }
}

macro_rules! vec2_op_for {
    ($ty: ty) => {
        vec2_op!(ops::Add, add, $ty);
        vec2_op!(ops::Sub, sub, $ty);
        vec2_op!(ops::Mul, mul, $ty);
        vec2_op!(ops::Div, div, $ty);
        vec2_opassign!(ops::AddAssign, add_assign, $ty);
        vec2_opassign!(ops::SubAssign, sub_assign, $ty);
        vec2_opassign!(ops::MulAssign, mul_assign, $ty);
        vec2_opassign!(ops::DivAssign, div_assign, $ty);
    };
}

vec2_vec2_op!(ops::Add, add);
vec2_vec2_op!(ops::Sub, sub);
vec2_vec2_op!(ops::Mul, mul);
vec2_vec2_op!(ops::Div, div);
vec2_vec2_opassign!(ops::AddAssign, add_assign);
vec2_vec2_opassign!(ops::SubAssign, sub_assign);
vec2_vec2_opassign!(ops::MulAssign, mul_assign);
vec2_vec2_opassign!(ops::DivAssign, div_assign);
vec2_op_for!(f64);
