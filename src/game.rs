use glow::Context;
use crate::renderer::*;
use crate::glutin_wrapper::*;



#[derive(Default)]
pub struct Game {
    renderer: Renderer,
}

impl Game {
    pub fn handle_event(&mut self, gl: &Context, event: Event, control_flow: &mut glutin::event_loop::ControlFlow) {
        match event {
            Event::Quit => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            },
            Event::Step => {

            },
            Event::Resized(res) => {

            },
            Event::KeyPressed(kc) => {},
            Event::KeyReleased(kc) => {},
            Event::MouseMotion(p) => {},
            Event::MousePressed(MouseButton) => {},
            Event::MouseReleased(MouseButton) => {},
            Event::Unfocus => {},
            Event::Focus => {},
            Event::None => {},
        }
    }
}