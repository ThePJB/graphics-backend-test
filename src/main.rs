use glutin_wrapper::Event;

mod renderer;
mod game;
mod glutin_wrapper;

fn main() {
    let mut game = game::Game::default();
    let (gl, window, event_loop) = unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Hello triangle!")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl =
            glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        (gl, window, event_loop)
    };
    event_loop.run(move |event, _, event_loop| {
        match event {
            glutin::event::Event::WindowEvent { event: glutin::event::WindowEvent::Resized(size), .. } => window.resize(size),
            glutin::event::Event::RedrawRequested(_) => window.swap_buffers().unwrap(),
            _ => {},
        };
        let event: Event = event.into();
        game.handle_event(&gl, event.into(), event_loop)
    });
}
