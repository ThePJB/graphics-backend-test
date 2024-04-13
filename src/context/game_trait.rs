use super::*;

pub trait Game {
    fn setup(&mut self, gl: &glow::Context);
    fn frame(&mut self, input: Input, gl: &glow::Context);
    // maybe even a resize trait or idk
}