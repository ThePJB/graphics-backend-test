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

impl_vec!(Vec2, f32, x, y);