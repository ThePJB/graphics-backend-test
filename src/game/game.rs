use crate::util::*;
use super::context::*;
use super::*;

use super::render_context::*;

// this is pretty good, i should test it though eg whole atlas = some shit made of this and then add UVs and draw a UV triangle or a UV quad

#[derive(Debug, Clone, Copy)]
pub struct SpriteHandle {
    pub xy: Vec2,
    pub wh: Vec2,
}

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
        dbg!("game new");
        let mut rc = RenderContext::new(gl);
        dbg!("begin load resources");
        rc.load_resources(std::path::Path::new("./assets"));
        dbg!("end load resources");
        Self {
            rc
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
                h: *self.rc.resource_handles.get("guy").expect_with(|| {
                    let keys: Vec<String> = self.rc.resource_handles.keys().cloned().collect();
                    keys.join("\n")
                } ),
            }),
        ]
    }
}