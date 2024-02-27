use glam::{Vec3, Vec4};
use glow::{
    HasContext, 
    NativeVertexArray,
    NativeBuffer, 
    NativeProgram, 
};
// next thing is ebo
// meed srgb?
pub struct Vertex {
    pos: Vec3,
    colour: Vec4,
}


pub const DEFAULT_FS: &str = r#"#version 330 core
in vec4 col;
out vec4 frag_colour;

void main() {
    frag_colour = col;
}
"#;
pub const DEFAULT_VS: &str = r#"#version 330 core
layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec4 in_col;

out vec4 col;

uniform mat4 projection;

void main() {
    col = in_col;
    gl_Position = projection * vec4(in_pos, 1.0);
}
"#;
// but do i do indexed rendering str8 up

// Albedo rendering is like regular sprites
// Emission rendering is the same kind of buffer
// 
// can I use same geometry for albedo as for emission?
// emission may be lower res etc

// also drawing particles...
// On Textures - R channel is value_base, G channel is value_tint, B channel is normalized emission, A channel is transparency
// How late can I tint? Can I render albedo and emission in 1 pass?
// V1 V2 H1 H2 S1 S2, unless we make it so S1 S2 or some of the parameters are auto made according to aesthetic palette
// and thus reducing number of parameters

// first crunch minimal set of data but essentially if the vertex shit was
// pos h1 h2 s1 s2 uv, maybe with render divisor into quads in which case rotation theta and w and h
// where v1 v2 emission and alpha are in the texture
// any runtime value fuckery can go in as well

// needing to compute 

// There will be some god damn stencil shit and skylight uniform
// Compositing pass needs to bind albedo, emission, stencil, skylight and add that shit and do gamma

// (potentially in future its like, jump flooding sdf shader, obstruction, occlusion, estimated normals etc, vignette)

// vbo shit 

pub struct Renderer {
    program: Program
}
impl Renderer {
    pub fn new(gl: &glow::Context) -> Self {
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LEQUAL);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            let vbo = gl.create_buffer().unwrap();        
        }
        Renderer {
            program: Program,
        }
    }
}