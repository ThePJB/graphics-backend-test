use std::f32::consts::PI;

use crate::util::*;
use crate::game::anim::*;
use super::context::*;
use super::render_context::*;

#[derive(Debug, Clone, Copy)]
pub struct SpriteHandle {
    pub xy: IVec2,
    pub wh: IVec2,
}

pub struct Game {
    rc: RenderContext,
    t: f32,
    guy: EntityAppearance,
}

impl App for Game {
    fn frame(&mut self, input: Input) {
        if let Some(resize) = input.resize {
            self.rc.resize(resize);
        }
        let dt = 0.016;
        self.t += dt;
        self.guy.update(self.t);
        let render_list = self.draw();
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

        let ea = necromancer_appearance(&rc.resource_handles);

        Self {
            rc,
            t: 0.0,
            guy: ea
        }
    }
    pub fn draw(&self) -> Vec<RenderCommand> {
        // let n = crate::game::anim::necromancer_appearance(&self.rc.resource_handles);
        let mut v = vec![
            // RenderCommand::Triangle(TriangleArgs {
            //     p: [vec2(-1.0, 0.0), vec2(0.0, 1.0), vec2(1.0, 0.0)],
            //     z: 0.0,
            //     c: vec4(1.0, 0.0, 0.0, 1.0),
            // }),
            // RenderCommand::Rect(RectArgs {
            //     xy: vec2(-0.1, -0.9),
            //     wh: vec2(0.1, 0.1),
            //     z: 0.0,
            //     c: vec4(0.0, 0.0, 1.0, 1.0),
            //     h: *self.rc.resource_handles.get("guy").expect_with(|| {
            //         let keys: Vec<String> = self.rc.resource_handles.keys().cloned().collect();
            //         keys.join("\n")
            //     } ),
            // }),
            // RenderCommand::Sprite(SpriteArgs { center: vec2(0.0, 0.0), radians: PI/8.0, z: -0.2, c: vec4(0.5, 1.0, 0.0, 1.0), h: *self.rc.resource_handles.get("guy/body/idle").unwrap(), num_frames: 9, frame: 0 }),
            // RenderCommand::Sprite(SpriteArgs { center: vec2(0.0, 0.0), radians: PI/8.0, z: -0.3, c: vec4(0.5, 1.0, 0.0, 1.0), h: *self.rc.resource_handles.get("guy/head/idle").unwrap(), num_frames: 9, frame: 0 }),
        ];
        // n.render(&mut v, vec2(0.0, 0.0), vec2(1.0, 0.0));
        v.extend(self.guy.draw(vec2(0.0, 0.0), vec2(1.0, 0.0)));
        v
    }
}