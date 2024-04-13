use glutin::event::VirtualKeyCode;
use std::collections::HashSet;
use crate::util::*;

#[derive(Clone, Default)]
pub struct Input {
    pub held_keys: HashSet<VirtualKeyCode>,
    pub pressed_keys: Vec<VirtualKeyCode>,
    pub held_lmb: bool,
    pub held_rmb: bool,
    pub click_lmb: bool,
    pub click_rmb: bool,
    pub mouse_px: Vec2, // does everyone just want mouse_px in ndc coords?
    pub pause: bool,
    pub scroll: i32,                // ye just write here from game events
}

impl Input {
    pub fn reset_for_frame(&mut self) {
        self.click_lmb = false;
        self.click_rmb = false;
        self.pressed_keys = Vec::new();
    }
}