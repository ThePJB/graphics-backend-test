use glutin::event_loop::EventLoop;
use glutin::ContextWrapper;
use glutin::PossiblyCurrent;
use glutin::window::Window;
use glutin::event::VirtualKeyCode;
use std::collections::HashSet;
use crate::util::*;
use glutin::event::{Event , WindowEvent};
use glutin::event::KeyboardInput;
use glutin::event::MouseButton;
use glutin::event::ElementState;


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

pub struct Context {
    event_loop: EventLoop<()>,
    window: ContextWrapper<PossiblyCurrent, Window>,
    input: Input,
}

pub trait App {
    fn frame(&mut self, input: Input);
}

impl Context {
    pub fn new(name: &'static str) -> Self {
        unsafe {
            let event_loop = glutin::event_loop::EventLoop::new();
            let window_builder = glutin::window::WindowBuilder::new()
                .with_title(name)
                .with_inner_size(glutin::dpi::LogicalSize::new(INTERNAL_XRES, INTERNAL_YRES));
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();
            let input = Input::default();

            Self {
                window,
                event_loop,
                input,
            }
        }
    }

    pub fn get_gl(&self) -> glow::Context {
        unsafe {
            glow::Context::from_loader_function(|s| self.window.get_proc_address(s) as *const _)
        }
    }

    pub fn run<T: App + 'static>(mut self, mut t: T) {
        self.event_loop.run(move |event, _, control_flow| {
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
                            self.input.mouse_px = vec2(position.x as f32 / INTERNAL_XRES, position.y as f32 / INTERNAL_YRES);
                        },
                        WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                            self.input.click_lmb = true;
                            self.input.held_lmb = true;
                        },
                        WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                            self.input.held_lmb = false;
                        },
                        WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Right, .. } => {
                            self.input.click_rmb = true;
                            self.input.held_rmb = true;
                        },
                        WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Right, .. } => {
                            self.input.held_rmb = false;
                        },
                        WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(keycode), state: ElementState::Pressed, ..}, .. } => {
                            self.input.pressed_keys.push(keycode.clone());
                            self.input.held_keys.insert(keycode.clone());
                        },
                        WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(keycode), state: ElementState::Released, ..}, .. } => {
                            self.input.held_keys.remove(&keycode);
                        },
                        WindowEvent::Focused(true) => self.input.pause = true,
                        WindowEvent::Focused(false) => self.input.pause = false,
                        _ => {},
                    }
                }
                //glutin::event::Event::RedrawRequested(_) => window.swap_buffers().unwrap(),
                Event::MainEventsCleared => {
                    let input2 = self.input.clone();
                    t.frame(input2);
                    self.window.swap_buffers().unwrap();
                    self.input.reset_for_frame();
                }
                _ => {},
            }
        });
    }
}