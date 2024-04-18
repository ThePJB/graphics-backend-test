use glow::*;
use std::collections::HashMap;
use crate::util::ImageBuffer;
use std::path::Path;

pub const ATLAS_WH: IVec2 = ivec2(1024, 1024);

pub const FRAG: &str = r#"#version 330 core
in vec4 col;
in vec2 uv;
out vec4 frag_colour;

uniform sampler2D tex;


void main() {
    frag_colour = texture(tex, uv) * col;
    // frag_colour = col;
}
"#;

pub const VERT: &str = r#"#version 330 core
layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec4 in_col;
layout (location = 2) in vec2 in_uv;

out vec4 col;
out vec2 uv;

// uniform mat4 projection;
const mat4 projection = mat4(1.0);


void main() {
    col = in_col;
    uv = in_uv;
    gl_Position = projection * vec4(in_pos, 1.0);
}
"#;

pub struct RenderContext {
    pub gl: Context,
    pub program: NativeProgram,
    pub vao: VertexArray,
    pub vbo: Buffer,
    pub ebo: Buffer,
    pub texture: Texture,
    pub num_verts: usize,
    pub resource_handles: HashMap<String, SpriteHandle>,
}

impl RenderContext {
    pub fn new(gl: Context) -> Self {
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LEQUAL);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            gl.disable(glow::CULL_FACE);

            let vbo = gl.create_buffer().unwrap();
            let ebo = gl.create_buffer().unwrap();
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo)); // Bind the EBO
            // let vert_size: usize = std::mem::size_of::<Vertex>();
            let vert_size = 4*9;
            dbg!(vert_size);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, vert_size as i32, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, vert_size as i32, 3 * 4);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, vert_size as i32, 7 * 4);
            gl.enable_vertex_attrib_array(2);

            // self.atlas = Some(Atlas::new(gl));
            // self.res = Some(Resources::new(&mut self.atlas.unwrap_mut(), gl));

            let program = gl.create_program().expect("Cannot create program");
        
            let vs = gl.create_shader(glow::VERTEX_SHADER).expect("cannot create vertex shader");
            gl.shader_source(vs, VERT);
            gl.compile_shader(vs);
            if !gl.get_shader_compile_status(vs) {
                panic!("{}", gl.get_shader_info_log(vs));
            }
            gl.attach_shader(program, vs);
    
            let fs = gl.create_shader(glow::FRAGMENT_SHADER).expect("cannot create fragment shader");
            gl.shader_source(fs, FRAG);
            gl.compile_shader(fs);
            if !gl.get_shader_compile_status(fs) {
                panic!("{}", gl.get_shader_info_log(fs));
            }
            gl.attach_shader(program, fs);
    
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }
            gl.detach_shader(program, fs);
            gl.delete_shader(fs);
            gl.detach_shader(program, vs);
            gl.delete_shader(vs);

            let mut im = ImageBuffer::new(ATLAS_WH);
            im.fill(vec4(1.0, 0.0, 1.0, 1.0));
            let texture = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_image_2d(
                glow::TEXTURE_2D, 
                0, 
                glow::RGBA as i32, 
                im.wh.x as i32, im.wh.y as i32, 
                0, 
                RGBA, 
                glow::UNSIGNED_BYTE, 
                Some(&im.data)
            );
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

            gl.generate_mipmap(glow::TEXTURE_2D);

            RenderContext {
                gl,
                program,
                vao,
                vbo,
                ebo,
                texture,
                num_verts: 0,
                resource_handles: HashMap::new(),
            }
        }
    }

    pub fn frame(&mut self, buf: VertexBufCPU) {
        unsafe {
            self.gl.clear_color(0.5, 0.5, 0.5, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); 
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            self.gl.use_program(Some(self.program));
            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            self.num_verts = buf.inds.len();
            self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buf.verts.as_bytes(), glow::STATIC_DRAW);
            self.gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, buf.inds.as_bytes(), glow::STATIC_DRAW);
            self.gl.draw_elements(
                glow::TRIANGLES,
                self.num_verts as i32, // number of indices
                glow::UNSIGNED_INT,   // type of indices
                0                      // offset
            );
        }
    }
    // ok can i make a based file system abstraction like visit or iterator. .visit().flat_map().

    pub fn load_resources(&mut self, sprites_path: &std::path::Path) {
        let mut paths = vec![];
        dir_traverse(sprites_path, &mut |path| {
            if path.extension().unwrap() == "png" {
                paths.push(path.to_owned())
            }
        }).expect_with(|| sprites_path.to_string_lossy());
        dbg!(sprites_path, &paths);
        paths.sort();
        let img_buffers = paths.iter().map(|p| {
            let bytes = std::fs::read(p).unwrap();
            let img = ImageBuffer::from_bytes(&bytes);
            img
        });
        // yea this is pretty close just needs my patented paths to names function. xd
        let names = paths.iter().map(|p| path_to_name_fn(p, sprites_path));
        let resources = std::iter::zip(names, img_buffers);
        dbg!("begin pack sprites");
        self.pack_sprites(resources);
    }

    // sets the texture and the resource handles dictionary
    pub fn pack_sprites(&mut self, resources: impl Iterator<Item = (String, ImageBuffer)>) {
        let mut resource_tuples: Vec<(String, ImageBuffer)> = resources.collect();
        resource_tuples.sort_by(|a, b| a.1.wh.dot(&a.1.wh).cmp(&b.1.wh.dot(&b.1.wh)));
        // make a packing
        let wh = ATLAS_WH;
        let mut arena = Arena2D::new(wh);
        for (name, sprite) in resource_tuples.into_iter() {
            let xy = arena.alloc(sprite.wh);
            // make the uvs in uv space and then store handle
            // hmm what about using pixels vs storing the pixel w and h of the thing. pixels technically has full info
            // can we convert it at the end
            // na i dont think it matters i think we are in vertex land at this point
            // hmm i will need to know it
            let h = SpriteHandle { xy: xy.as_vec2() / wh.as_vec2(), wh: sprite.wh.as_vec2() / wh.as_vec2() };
            self.resource_handles.insert(name, h);
            //sub buffer 2d on the texture as well!

                    
            // update texture with new sprite data
            unsafe {
                self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
                self.gl.tex_sub_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    0,
                    0,
                    sprite.wh.x as i32,
                    sprite.wh.y as i32,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    glow::PixelUnpackData::Slice(&sprite.data.as_bytes()),
                );
            }
        }
    }
}

// lolz a string? I guess
fn path_to_name_fn(path: &Path, base: &Path) -> String {
    let components: Vec<String> = path
        .strip_prefix(base)
        .unwrap()
        .components()
        .filter_map(|c| match c {
            std::path::Component::Normal(s) => Some(s),
            _ => None,
        })
        .map(|x| x.to_str().unwrap())
        .map(|x| x.split_once(".").map(|x| x.0).unwrap_or(x)) // map such that asdf.png is asdf, and anything else is identity
        .map(|x| x.to_owned())
        .collect();
    components.join("/")
}

pub struct Arena2D {
    rects: Vec<(IVec2, IVec2)>,
    wh: IVec2,
}
impl Arena2D {
    pub fn new(wh: IVec2) -> Self {
        Arena2D {
            rects: vec![],
            wh,
        }
    }
    pub fn alloc(&mut self, wh: IVec2) -> IVec2 {
        let mut p = ivec2(0,0);
        loop {
            let mut r_idx = 0;
            loop {
                if p.x + wh.x > self.wh.x {
                    p.x = 0;
                    p.y += 1;
                    r_idx = 0;
                }
                // break loop when able to alloc
                if r_idx >= self.rects.len() {
                    break;
                }
                let other_r = self.rects[r_idx];
                if p + wh > other_r.0 && p < other_r.0 + other_r.1 {
                    p.x = other_r.0.x + other_r.1.x;
                    r_idx = 0;
                    continue;
                }
                r_idx += 1;
            }
            // clear inner loop is where thing gets actually allocated.
            self.rects.push((p, wh));
            return p;
        }
    }
}

trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl<T> AsBytes for Vec<T> {
    fn as_bytes(&self) -> &[u8] {
        // Get a pointer to the data and calculate the length in bytes
        let ptr = self.as_ptr() as *const u8;
        let len = self.len() * std::mem::size_of::<T>();

        // Convert the pointer and length to a byte slice
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

#[test]
fn test_as_bytes() {
    let a = vec![1i32, 1000i32, 0i32, 1i32];
    let b = a.as_bytes();
    dbg!(b);
}

use crate::{util::*, SpriteHandle};

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
    pub h: SpriteHandle,
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
                    let uv = args.h.xy + args.h.wh * *uv;
                    Vertex {
                        xyz: vec3(p.x, p.y, args.z),
                        rgba: args.c,
                        uv: uv,    // and also this uv would need to be * by args uv
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