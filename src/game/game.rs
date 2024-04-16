use crate::util::*;
use super::context::*;
use super::*;

use super::render_context::*;

// this is pretty good, i should test it though eg whole atlas = some shit made of this and then add UVs and draw a UV triangle or a UV quad


pub struct SpriteHandle {
    pub xy: Vec2,
    pub wh: Vec2,
}

pub struct Resources {
    pub test: SpriteHandle,
}

impl Resources {
    pub fn new(atlas: &mut Atlas, gl: &glow::Context) -> Self {
        let mut image = ImageBuffer::new(ivec2(512,512));
        image.fill(vec4(1.0, 1.0, 1.0, 1.0));
        Self {
            // test: atlas.alloc(gl, &image),
            test: SpriteHandle {
                xy: vec2(0.0, 0.0),
                wh: vec2(1.0, 1.0),
            }
            // test: atlas.alloc(gl, &ImageBuffer::from_bytes(include_bytes!("../../assets/guy.png"))),
        }
    }
}

// yea this is like so easy to work with
// ok gonna sign off now



pub struct Game {
    rc: RenderContext,
}

impl App for Game {
    fn frame(&mut self, input: Input) {
        let render_list = self.render();
        let mut buf = VertexBufCPU::default();
        render_list.iter().for_each(|rc| rc.draw(&mut buf));
        self.rc.frame(buf);
    }
}

impl Game {
    pub fn new(gl: glow::Context) -> Self {
        Self {
            rc: RenderContext::new(gl)
        }
    }
    pub fn render(&self) -> Vec<RenderCommand> {
        vec![
            RenderCommand::Triangle(TriangleArgs {
                p: [vec2(-1.0, 0.0), vec2(0.0, 1.0), vec2(1.0, 0.0)],
                z: 0.0,
                c: vec4(1.0, 0.0, 0.0, 1.0),
            }),
            RenderCommand::Rect(RectArgs {
                xy: vec2(-0.1, -0.9),
                wh: vec2(0.1, 0.1),
                z: 0.0,
                c: vec4(0.0, 0.0, 1.0, 1.0),
                uv_xy: vec2(0.0, 0.0),
                uv_wh: vec2(1.0, 1.0),
            }),
        ]
    }
}