use crate::impl_vec;

#[derive(Clone, Copy, Debug, Eq, Hash, Default)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

pub const fn ivec2(x: i32, y: i32) -> IVec2 {
    IVec2 { x, y }
}

impl_vec!(IVec2, i32, x, y);

// vecs refactor, colour is ivec4 of u8s, etc
