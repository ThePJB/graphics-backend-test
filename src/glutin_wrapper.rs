use glutin::dpi::PhysicalSize;
use glutin::event::{Event as GlutinEvent, WindowEvent};
use glutin::event::VirtualKeyCode;
use glutin::event::KeyboardInput;
use glutin::event::MouseButton;
use glutin::event::ElementState;
use glutin::window::Window as GlutinWindow;
use glutin::PossiblyCurrent;
use glutin::ContextWrapper;
use glam::{Vec2, vec2};

pub struct Window(ContextWrapper<PossiblyCurrent, GlutinWindow>);
impl From<ContextWrapper<PossiblyCurrent, GlutinWindow>> for Window {
    fn from(glutin_window: ContextWrapper<PossiblyCurrent, GlutinWindow>) -> Self {
        Window(glutin_window)
    }
}
impl Window {
    pub fn resize(&mut self, res: Vec2) {
        self.0.resize(PhysicalSize::new(res.x as u32, res.y as u32))
    }
}

pub enum Event {
    Quit,
    Step,
    Resized(Vec2),
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),
    MouseMotion(Vec2),
    MousePressed(MouseButton),
    MouseReleased(MouseButton),
    Unfocus,
    Focus,
    None,
}

impl From<GlutinEvent<'_, ()>> for Event {
    fn from(event: GlutinEvent<()>) -> Self {
        match event {
            GlutinEvent::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => Self::Quit,
                    WindowEvent::CursorMoved { position, .. } => Self::MouseMotion(vec2(position.x as _, position.y as _)),
                    WindowEvent::MouseInput { state: ElementState::Pressed, button, .. } => Self::MousePressed(button),
                    WindowEvent::MouseInput { state: ElementState::Released, button, .. } => Self::MouseReleased(button),
                    WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(keycode), state: ElementState::Pressed, ..}, .. } => Self::KeyPressed(keycode),
                    WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(keycode), state: ElementState::Released, ..}, .. } => Self::KeyReleased(keycode),
                    WindowEvent::Focused(true) => Self::Focus,
                    WindowEvent::Focused(false) => Self::Unfocus,
                    WindowEvent::Resized(size) => Self::Resized(vec2(size.width as f32, size.height as f32)),
                    _ => Self::None,
                }
            }
            GlutinEvent::MainEventsCleared => Self::Step,
            _ => Self::None,
        }
    }
}