use crate::util::*;

#[derive(Debug, Clone)]
#[repr(C, packed)]
pub struct Vertex {
    pub xyz: Vec3,
    pub rgba: Vec4,
    // uv
    // other shit lmao like specular etc
}

#[derive(Default)]
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

pub struct TriangleArgs {
    pub p: [Vec2; 3],
    pub z: f32,
    pub c: Vec4,
}


pub enum RenderCommand {
    Triangle(TriangleArgs),
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
                }), 
                0..3)
            },
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