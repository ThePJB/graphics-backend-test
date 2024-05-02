use glow::Texture as GPUTexture;
use glow::*;
use std::{collections::HashMap, f32::consts::PI};
use crate::util::ImageBuffer;


// Texture 0: Atlas Albedo
// Texture 1: Atlas Emission
// Texture 2: Light map

pub const ATLAS_WH: IVec2 = ivec2(1024, 1024);
// pub const LIGHT_WH: IVec2 = INTERNAL_WH_I/4;
pub const LIGHT_WH: IVec2 = ivec2(INTERNAL_WH_I.x/4, INTERNAL_WH_I.y/4);
pub const FRAG_ALBEDO: &str = r#"#version 330 core
in vec4 col;
in vec2 uv;
in vec4 gl_FragCoord;
out vec4 frag_colour;

uniform vec2 res;
uniform sampler2D tex;
uniform sampler2D light;

void main() {
    vec2 screen_uv = gl_FragCoord.xy / res;
    vec4 light_col = texture(light, screen_uv);
    // frag_colour = light_col;
    frag_colour = texture(tex, uv) * col * light_col;
    // frag_colour = col;
}
"#;

pub const VERT_ALBEDO: &str = r#"#version 330 core
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

pub const FRAG_EMIT: &str = r#"#version 330 core
in vec4 col;
in vec2 uv;
out vec4 frag_colour;

uniform sampler2D tex;

void main() {
    // frag_colour = vec4(1.0, 0.0, 0.0, 1.0);
    frag_colour = texture(tex, uv) * col;
    // frag_colour = col;
}
"#;

pub const VERT_EMIT: &str = r#"#version 330 core
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

// pub trait Draw {
//     fn draw(&self) -> impl Iterator<Item = RenderCommand>;
// }

pub struct RenderContext {
    pub gl: Context,
    pub program_emit: NativeProgram,
    pub program_albedo: NativeProgram,
    pub vao: VertexArray,
    pub vbo: Buffer,
    pub ebo: Buffer,
    pub texture: GPUTexture,
    pub texture_emit: GPUTexture,
    pub light_texture: GPUTexture,
    pub light_fbo: Framebuffer,
    pub resource_handles: HashMap<String, SpriteHandle>,
    pub wh: IVec2,
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

            // albedo 
            let program_albedo = {
                let program_albedo = gl.create_program().expect("Cannot create program");
            
                let vs = gl.create_shader(glow::VERTEX_SHADER).expect("cannot create vertex shader");
                gl.shader_source(vs, VERT_ALBEDO);
                gl.compile_shader(vs);
                if !gl.get_shader_compile_status(vs) {
                    panic!("{}", gl.get_shader_info_log(vs));
                }
                gl.attach_shader(program_albedo, vs);
        
                let fs = gl.create_shader(glow::FRAGMENT_SHADER).expect("cannot create fragment shader");
                gl.shader_source(fs, FRAG_ALBEDO);
                gl.compile_shader(fs);
                if !gl.get_shader_compile_status(fs) {
                    panic!("{}", gl.get_shader_info_log(fs));
                }
                gl.attach_shader(program_albedo, fs);
        
                gl.link_program(program_albedo);
                if !gl.get_program_link_status(program_albedo) {
                    panic!("{}", gl.get_program_info_log(program_albedo));
                }
                gl.detach_shader(program_albedo, fs);
                gl.delete_shader(fs);
                gl.detach_shader(program_albedo, vs);
                gl.delete_shader(vs);
                program_albedo
            };

            // emission
            let program_emit = {
                let program_emit = gl.create_program().expect("Cannot create program");
            
                let vs = gl.create_shader(glow::VERTEX_SHADER).expect("cannot create vertex shader");
                gl.shader_source(vs, VERT_EMIT);
                gl.compile_shader(vs);
                if !gl.get_shader_compile_status(vs) {
                    panic!("{}", gl.get_shader_info_log(vs));
                }
                gl.attach_shader(program_emit, vs);
        
                let fs = gl.create_shader(glow::FRAGMENT_SHADER).expect("cannot create fragment shader");
                gl.shader_source(fs, FRAG_EMIT);
                gl.compile_shader(fs);
                if !gl.get_shader_compile_status(fs) {
                    panic!("{}", gl.get_shader_info_log(fs));
                }
                gl.attach_shader(program_emit, fs);
        
                gl.link_program(program_emit);
                if !gl.get_program_link_status(program_emit) {
                    panic!("{}", gl.get_program_info_log(program_emit));
                }
                gl.detach_shader(program_emit, fs);
                gl.delete_shader(fs);
                gl.detach_shader(program_emit, vs);
                gl.delete_shader(vs);
                program_emit
            };


            gl.active_texture(glow::TEXTURE0);
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

            gl.active_texture(glow::TEXTURE1);
            let mut im = ImageBuffer::new(ATLAS_WH);
            im.fill(vec4(1.0, 0.0, 1.0, 0.0));
            let texture_emit = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(texture_emit));
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
            
            // Create the light texture
            gl.active_texture(glow::TEXTURE2);
            let mut im = ImageBuffer::new(LIGHT_WH);
            im.fill(vec4(1.0, 0.0, 1.0, 0.0));
            let light_texture = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(light_texture));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                im.wh.x as i32,
                im.wh.y as i32,
                0,
                RGBA,
                glow::UNSIGNED_BYTE,
                Some(&im.data),
            );
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
            gl.generate_mipmap(glow::TEXTURE_2D);

            // Create the framebuffer for rendering to the light texture
            let light_fbo = gl.create_framebuffer().expect("Cannot create framebuffer");
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(light_fbo));
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(light_texture),
                0,
            );
            let status = gl.check_framebuffer_status(glow::FRAMEBUFFER);
            if status != glow::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer incomplete: {:?}", status);
            }
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);

            RenderContext {
                gl,
                program_emit,
                program_albedo,
                vao,
                vbo,
                ebo,
                texture,
                texture_emit,
                resource_handles: HashMap::new(),
                light_texture,
                light_fbo,
                wh: INTERNAL_WH_I,
            }
        }
    }

    pub fn frame(&mut self, render_list: &Vec<RenderCommand>) {
        unsafe {
            self.gl.clear_color(0.5, 0.5, 0.5, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); 
            
            self.emission(render_list);
            self.gl.clear(glow::DEPTH_BUFFER_BIT);
            self.albedo(render_list);
        }
    }

    // passes: program, inputs, outputs, etc
    pub fn albedo(&mut self, render_list: &Vec<RenderCommand>) {
        let mut buf = VertexBufCPU::default();
        render_list.iter().for_each(|rc| rc.draw_albedo(&mut buf));
        unsafe {
            self.gl.use_program(Some(self.program_albedo));
            // so lol technically u dont have to do this but maybe u do
            self.gl.active_texture(glow::TEXTURE0);
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            self.gl.active_texture(glow::TEXTURE2);
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.light_texture));
            self.gl.uniform_1_i32(self.gl.get_uniform_location(self.program_albedo, "tex").as_ref(), 0);
            self.gl.uniform_1_i32(self.gl.get_uniform_location(self.program_albedo, "light").as_ref(), 2);
            self.gl.uniform_2_f32(self.gl.get_uniform_location(self.program_albedo, "res").as_ref(), self.wh.x as f32, self.wh.y as f32);
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            self.gl.viewport(0, 0, self.wh.x, self.wh.y);
            // self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            let num_verts = buf.inds.len();
            self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buf.verts.as_bytes(), glow::STATIC_DRAW);
            self.gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, buf.inds.as_bytes(), glow::STATIC_DRAW);
            self.gl.draw_elements(
                glow::TRIANGLES,
                num_verts as i32,       // number of indices
                glow::UNSIGNED_INT,   // type of indices
                0                           // offset
            );
        }
    }

    pub fn emission(&mut self, render_list: &Vec<RenderCommand>) {
        let mut buf = VertexBufCPU::default();
        render_list.iter().for_each(|rc| rc.draw_emission(&mut buf));
        unsafe {
            self.gl.use_program(Some(self.program_emit));
            self.gl.active_texture(glow::TEXTURE1);
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture_emit));
            self.gl.uniform_1_i32(self.gl.get_uniform_location(self.program_emit, "tex").as_ref(), 1);
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.light_fbo));
            self.gl.viewport(0, 0, LIGHT_WH.x, LIGHT_WH.y);
            // Clear the framebuffer
            self.gl.clear_color(0.0, 1.0, 0.0, 1.0); // Clear with black color
            self.gl.clear(glow::COLOR_BUFFER_BIT); // dont think depth needed

            // this shit did show up before, the glowy rune from the hat, showed in rdoc texture

            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            let num_verts = buf.inds.len();
            self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buf.verts.as_bytes(), glow::STATIC_DRAW);
            self.gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, buf.inds.as_bytes(), glow::STATIC_DRAW);
            self.gl.draw_elements(
                glow::TRIANGLES,
                num_verts as i32,       // number of indices
                glow::UNSIGNED_INT,   // type of indices
                0                           // offset
            );
        }
    }

    pub fn resize(&mut self, wh: IVec2) {
        self.wh = wh;
        unsafe {
            self.gl.viewport(0, 0, wh.x, wh.y);
        }
    }
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

// #[derive(Debug)]
// pub struct TriangleArgs {
//     pub p: [Vec2; 3],
//     pub z: f32,
//     pub c: Vec4,
// }

// #[derive(Debug)]
// pub struct RectArgs {
//     pub xy: Vec2,
//     pub wh: Vec2,
//     pub z: f32,
//     pub c: Vec4,
//     pub h: SpriteHandle,
// }
#[derive(Debug)]
pub struct SpriteArgs {
    pub center: Vec2,
    pub radians: f32,
    pub z: f32,
    pub colour: Vec4,
    pub colour_emit: Vec4,
    pub h: SpriteHandle,
    pub frame: u8,
    pub num_frames: u8,
}

#[derive(Debug)]
pub enum RenderCommand {
    // Triangle(TriangleArgs),
    // Rect(RectArgs),
    Sprite(SpriteArgs),
}

impl RenderCommand {
    pub fn draw_albedo(&self, buf: &mut VertexBufCPU) {
        match self {
            Self::Sprite(args) => {
                let h = args.h;
                let mut wh = h.wh.as_vec2() / ATLAS_WH.as_vec2();
                wh.x /= args.num_frames as f32;
                let mut xy = h.xy.as_vec2() / ATLAS_WH.as_vec2();
                xy.x += args.frame as f32 * wh.x;
                let yy = vec2(INTERNAL_YRES, INTERNAL_YRES);
                let mut wh_ndc = 2.0 * h.wh.as_vec2() / yy;
                wh_ndc.x /= args.num_frames as f32;
                let uvs = [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)];
                let uv_rots = oriented_rect_2d_points(args.radians, wh_ndc, vec2(0.0, 0.0));
                let stretch_vec = INTERNAL_WH / yy;
                let verts = (0..4).map(|i| (uvs[i], uv_rots[i])).map(|(uv, uv_rot)| {
                    let p = args.center + uv_rot/stretch_vec;
                    let uv = xy + wh * uv;
                    Vertex {
                        xyz: vec3(p.x, p.y, args.z),
                        rgba: args.colour,
                        uv: uv,
                    }
                });
                let inds = [0, 1, 2, 0, 2, 3].into_iter();
                buf.extend(verts, inds);
            }
        }
    }
    pub fn draw_emission(&self, buf: &mut VertexBufCPU) {
        match self {
            Self::Sprite(args) => {
                if args.colour_emit.w == 0.0 { return; }
                let h = args.h;
                let mut wh = h.wh.as_vec2() / ATLAS_WH.as_vec2();
                wh.x /= args.num_frames as f32;
                let mut xy = h.xy.as_vec2() / ATLAS_WH.as_vec2();
                xy.x += args.frame as f32 * wh.x;
                let yy = vec2(INTERNAL_YRES, INTERNAL_YRES);
                let mut wh_ndc = 2.0 * h.wh.as_vec2() / yy;
                wh_ndc.x /= args.num_frames as f32;
                let uvs = [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)];
                let uv_rots = oriented_rect_2d_points(args.radians, wh_ndc, vec2(0.0, 0.0));
                let stretch_vec = INTERNAL_WH / yy;
                let verts = (0..4).map(|i| (uvs[i], uv_rots[i])).map(|(uv, uv_rot)| {
                    let p = args.center + uv_rot/stretch_vec;
                    let uv = xy + wh * uv;
                    Vertex {
                        xyz: vec3(p.x, p.y, args.z),
                        rgba: args.colour_emit,
                        uv: uv,
                    }
                });
                let inds = [0, 1, 2, 0, 2, 3].into_iter();
                buf.extend(verts, inds);
            }
        }
    }
}

fn oriented_rect_2d_points(theta: f32, wh: Vec2, center: Vec2) -> [Vec2; 4] {
    let vo = wh.projx().rotate(theta);
    let vn = wh.projy().rotate(theta);
    let uv_rots = [center-vo/2.0 - vn/2.0, center+vo/2.0-vn/2.0, center+vo/2.0+vn/2.0, center-vo/2.0+vn/2.0];
    uv_rots
}

#[test]
fn test_opoints() {
    dbg!(oriented_rect_2d_points(PI/2.0, vec2(2.0, 1.0), vec2(0.0, 0.0)));
}


// NB needs to be able to, Render Rect, Render Sprite, Render Text
// So its a UV sprite renderer.
// It needs a white square for render rect. render rect is actually render sprite where sprite = white square

// needs to render to texture.
// needs to render emission as well
// needs to bloom emission
// needs HDR pass

// there is something really nice about describing things at like the functional level