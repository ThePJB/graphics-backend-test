use crate::util::*;
use glutin::event::{Event , WindowEvent};
use glutin::event::KeyboardInput;
use glutin::event::MouseButton;
use glutin::event::ElementState;
use glutin::event_loop::{ControlFlow, EventLoop};
use crate::{util::*, Input};
use super::program::Program;
use super::vertex::*;
use glow::HasContext;


// um if i eliminate context struct how do I handle resizes?
// its like, what I have now sans of run shit is set to actually work

// i reckon game maybe does actually own the direct gl handles and shit
// input is abstracted bc it varies between backends
// other than that abstraction is as thing and platformy as possible

// honestly once we move that shit into game this is looking pretty streamlined, pretty set up, and pretty able to be turned into web
// and it does just like straight up need the gl context.
// i can delete a lot of my shitty wrappers too

// yo by the time that gl shit not here this is just like, context creation basically
// abstracting winit / wasm basically

// in b4 no longer a module and just inside game

pub trait Game {
    fn setup(&mut self, gl: &glow::Context);
    fn frame(&mut self, input: Input, gl: &glow::Context);
    // maybe even a resize trait or idk
}

// just 1 procedure should be amenable to cfg wasm
// and collapses much of the structure down helpfully
// this is basically run winit
// can also make a run wasm that basically also calls setup, marshals input, and calls frame.
pub fn run<T: Game + 'static>(mut game: T) {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
                .with_title("Hello triangle!")
                .with_inner_size(glutin::dpi::LogicalSize::new(INTERNAL_XRES, INTERNAL_YRES));
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();
            let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
            game.setup(&gl);
            let mut input = Input::default();
            event_loop.run(move |event, _, control_flow| {
                match event {
                    Event::WindowEvent { event, .. } => {
                        match event {
                            WindowEvent::Resized(size) => {
                                // self.resize(size.width as f32, size.height as f32)
                                // glViewport etc
                            },
                            WindowEvent::CloseRequested => {
                                *control_flow = glutin::event_loop::ControlFlow::Exit;
                            },
                            WindowEvent::CursorMoved { position, .. } => {
                                // also dis wrong, x * internal / actual bra
                                input.mouse_px = vec2(position.x as f32 / INTERNAL_XRES, position.y as f32 / INTERNAL_YRES);
                            },
                            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                                input.click_lmb = true;
                                input.held_lmb = true;
                            },
                            WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                                input.held_lmb = false;
                            },
                            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Right, .. } => {
                                input.click_rmb = true;
                                input.held_rmb = true;
                            },
                            WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Right, .. } => {
                                input.held_rmb = false;
                            },
                            WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(keycode), state: ElementState::Pressed, ..}, .. } => {
                                input.pressed_keys.push(keycode.clone());
                                input.held_keys.insert(keycode.clone());
                            },
                            WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(keycode), state: ElementState::Released, ..}, .. } => {
                                input.held_keys.remove(&keycode);
                            },
                            WindowEvent::Focused(true) => input.pause = true,
                            WindowEvent::Focused(false) => input.pause = false,
                            _ => {},
                        }
                    }
                    //glutin::event::Event::RedrawRequested(_) => window.swap_buffers().unwrap(),
                    Event::MainEventsCleared => {
                        let input2 = input.clone();
                        game.frame(input2, &gl);
                        window.swap_buffers().unwrap();
                        input.reset_for_frame();
                    }
                    _ => {},
                }
            });
    }
}