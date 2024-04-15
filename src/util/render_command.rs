use crate::util::*;

#[derive(Debug, Clone)]
#[repr(C, packed)]
pub struct Vertex {
    pub xyz: Vec3,
    pub rgba: Vec4,
    pub uv: Vec2,
    // uv
    // other shit lmao like specular etc
}

#[derive(Default, Debug)]
pub struct VertexBufCPU {
    pub verts: Vec<Vertex>,
    pub inds: Vec<u32>,
}

impl VertexBufCPU {
    pub fn extend(&mut self, verts: impl Iterator<Item = Vertex>, inds: impl Iterator<Item = u32>) {
        let offset = self.verts.len() as u32;
        self.verts.extend(verts);
        self.inds.extend(inds.map(|ind| ind + offset))
    }
}

#[derive(Debug)]
pub struct TriangleArgs {
    pub p: [Vec2; 3],
    pub z: f32,
    pub c: Vec4,
}

#[derive(Debug)]
pub struct RectArgs {
    pub xy: Vec2,
    pub wh: Vec2,
    pub z: f32,
    pub c: Vec4,
    pub uv_xy: Vec2,
    pub uv_wh: Vec2,
}


#[derive(Debug)]
pub enum RenderCommand {
    Triangle(TriangleArgs),
    Rect(RectArgs),
}

// lol would separate top and bottom vertex colour allow for cheesy gradients? might be kinda silly cause of the perspective or might be kinda cool
// anywhome easy to support

// but i guess the idea is like, im making triangle, i want to reuse triangle code for the other shapes too. except its indexed drawing actually so meh. might be able to recusrive the function though, build enum and call

impl RenderCommand {
    pub fn draw(&self, buf: &mut VertexBufCPU) {
        match self {
            Self::Triangle(args) => {
                buf.extend(args.p.iter().map(|p| Vertex {
                    xyz: vec3(p.x, p.y, args.z),
                    rgba: args.c,
                    uv: vec2(0.0, 0.0),
                }), 
                0..3)
            },
            Self::Rect(args) => {
                let uvs = [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)];
                // let points = [args.xy, args.xy + args.wh.projx(), args.xy + args.wh, args.xy + args.wh.projy()];
                let verts = uvs.iter().map(|uv| {
                    let p = args.xy + *uv*args.wh;
                    Vertex {
                        xyz: vec3(p.x, p.y, args.z),
                        rgba: args.c,
                        uv: *uv,    // and also this uv would need to be * by args uv
                        // uv: vec2(0.22, 0.222),
                    }
                });
                let inds = [0, 1, 2, 0, 2, 3].into_iter();
                buf.extend(verts, inds);
            }
        }
    }
}


// NB needs to be able to, Render Rect, Render Sprite, Render Text
// So its a UV sprite renderer.
// It needs a white square for render rect. render rect is actually render sprite where sprite = white square

// needs to render to texture.
// needs to render emission as well
// needs to bloom emission
// needs HDR pass

// there is something really nice about describing things at like the functional level