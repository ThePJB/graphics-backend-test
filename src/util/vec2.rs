use crate::impl_vec;
use std::cmp::{PartialEq, Eq};

#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl Vec2 {
    pub fn projx(&self) -> Vec2 {
        vec2(self.x, 0.0)
    }
    pub fn projy(&self) -> Vec2 {
        vec2(0.0, self.y)
    }
    pub fn rotate(&self, theta: f32) -> Vec2 {
        let c = theta.cos();
        let s = theta.sin();
        let c1 = vec2(c, s);
        let c2 = vec2(-s, c);
        vec2(c1.dot(&self), c2.dot(&self))
    }
}


impl_vec!(Vec2, f32, x, y);