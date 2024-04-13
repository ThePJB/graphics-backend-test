use crate::context::*;
use crate::util::*;
use crate::context::Game as GameTrait;
use glow::HasContext;

#[derive(Default)]
pub struct Game {
    program: Option<Program>,
    vertex_buf_handle: Option<VertexBufGPU>,
}

impl GameTrait for Game {
    fn setup(&mut self, gl: &glow::Context) {
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LEQUAL);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            self.program = Some(Program::new(gl, DEFAULT_VS, DEFAULT_FS));
            self.program.as_ref().map(|p| p.bind(gl));
            self.vertex_buf_handle = Some(VertexBufGPU::new(gl));
            // make dat shit n also do other setup related shizz
        }
    }
    fn frame(&mut self, input: Input, gl: &glow::Context) {
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            let render_list = self.render();
            let mut buf = VertexBufCPU::default();
            render_list.iter().for_each(|rc| rc.draw(&mut buf));
            self.program.as_ref().map(|p| p.bind(gl));
            self.vertex_buf_handle.as_ref().map(|h| h.bind(gl));
            self.vertex_buf_handle.as_mut().map(|h| h.update(gl, &buf));
            self.vertex_buf_handle.as_ref().map(|h| h.render(gl));
        }
    }
}

impl Game {
    pub fn render(&self) -> Vec<RenderCommand> {
        vec![
            RenderCommand::Triangle(TriangleArgs {
                p: [vec2(-1.0, 0.0), vec2(0.0, 1.0), vec2(1.0, 0.0)],
                z: 0.0,
                c: vec4(1.0, 0.0, 0.0, 1.0),
            })
        ]
    }
}