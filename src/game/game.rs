use crate::context::*;
use crate::util::*;
use crate::context::Game as GameTrait;
use glow::HasContext;


pub struct Texture {
    id: Option<glow::Texture>,
    wh: IVec2,
}

impl Texture {
    pub fn new(gl: &glow::Context, res: IVec2) -> Self {
        unsafe {
            let id = Some(gl.create_texture().expect("Failed to create texture"));
            gl.bind_texture(glow::TEXTURE_2D, id);
            // gl.tex_storage_2d(glow::TEXTURE_2D, 0, glow::RGBA, res.x as i32, res.y as i32);
            gl.tex_storage_2d(glow::TEXTURE_2D, 1, glow::RGBA8, res.x as i32, res.y as i32);
            Self {
                id,
                wh: res,
            }
        }
    }
    pub fn new_from_image(gl: &glow::Context, image: &ImageBuffer) -> Self {
        unsafe {
            let id = Some(gl.create_texture().expect("Failed to create texture"));
            gl.bind_texture(glow::TEXTURE_2D, id);
            // gl.tex_storage_2d(glow::TEXTURE_2D, 0, glow::RGBA, res.x as i32, res.y as i32);
            // gl.tex_storage_2d(glow::TEXTURE_2D, 1, glow::RGBA8, image.wh.x as i32, image.wh.y as i32);
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, image.wh.x as i32, image.wh.y as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, Some(&image.data));
            Self {
                id,
                wh: image.wh,
            }
        }
    }
        //gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, res.x as i32, res.y as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, Some(&image.data));
    // maybe make new with storage function or something
    // new from image.

    // let texture = gl.create_texture().unwrap();
    //     gl.bind_texture(glow::TEXTURE_2D, Some(texture));
//     gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, image.w as i32, image.h as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, Some(&image.data));
//     gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
//     gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
//     gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
//     gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

    pub fn resize(&mut self, gl: &glow::Context, res: IVec2) {
        unsafe {
            self.wh = res;
            gl.bind_texture(glow::TEXTURE_2D, self.id);
            gl.tex_storage_2d(glow::TEXTURE_2D, 1, glow::RGBA8, res.x as i32, res.y as i32);
        }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, self.id);
        }
    }

    pub fn free(&self, gl: &glow::Context) {
        unsafe {
            if let Some(id) = self.id {
                gl.delete_texture(id);
            }
        }
    }

    pub fn sub_update(&mut self, gl: &glow::Context, src: &ImageBuffer, dst_xy: IVec2) {
        unsafe {
            self.bind(gl);

            // can u do sub image on nothing?
            dbg!(dst_xy, src.wh, src.data.len());

            // Assuming src is an array of u8 with RGBA format
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                dst_xy.x as i32,
                dst_xy.y as i32,
                src.wh.x as i32,
                src.wh.y as i32,
                glow::RGBA8,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(&src.data),
            );
        }
    }
}

// this is pretty good, i should test it though eg whole atlas = some shit made of this and then add UVs and draw a UV triangle or a UV quad

pub struct Atlas {
    pub s: usize,
    pub texture: Texture,
}

// and whether its a generational index allocator of (xy, wh) and also does the corresponding sub buffer,
// and also do vec shit to grow when size gets exceeded. Or just set the size initially.

// need to bring gen idx alloc, and image buffer here
// need to wrap texture

impl Atlas {
    pub fn new(gl: &glow::Context) -> Self {
        let mut image = ImageBuffer::new(ivec2(512,512));
        image.fill(vec4(1.0, 0.0, 1.0, 1.0));
        Atlas {
            s: 512,
            // texture: Texture::new(gl, ivec2(512, 512)),
            texture: Texture::new_from_image(gl, &image),
        }
    }
    pub fn alloc(&mut self, gl: &glow::Context, image: &ImageBuffer) -> SpriteHandle {
        let xy = ivec2(0, 0);
        self.texture.sub_update(gl, &image, xy);
        let xy = xy.as_vec2() / self.texture.wh.as_vec2();
        let wh = image.wh.as_vec2() / self.texture.wh.as_vec2();
        SpriteHandle {
            xy,
            wh,
        }

        // sheit what if we wanna also sort the things
        // arena stores index in other list that we sort? idk
        // we dont really plan to actually delete shit so why dont we like not do that although we could type of thing
        // just do it like that other one for now
    }
}

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

#[derive(Default)]
pub struct Game {
    program: Option<Program>,
    vertex_buf_handle: Option<VertexBufGPU>,
    res: Option<Resources>,
    atlas: Option<Atlas>,
}

impl GameTrait for Game {
    fn setup(&mut self, gl: &glow::Context) {
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LEQUAL);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            gl.disable(glow::CULL_FACE);
            self.program = Some(Program::new(gl, DEFAULT_VS, DEFAULT_FS));
            self.program.as_ref().map(|p| p.bind(gl));
            self.vertex_buf_handle = Some(VertexBufGPU::new(gl));
            // make dat shit n also do other setup related shizz
            self.atlas = Some(Atlas::new(gl));
            self.res = Some(Resources::new(&mut self.atlas.unwrap_mut(), gl));
        }
    }
    fn frame(&mut self, input: Input, gl: &glow::Context) {
        unsafe {
            gl.clear_color(0.5, 0.5, 0.5, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            let render_list = self.render();
            let mut buf = VertexBufCPU::default();
            render_list.iter().for_each(|rc| rc.draw(&mut buf));
            dbg!(&render_list);
            dbg!(&buf);
            self.program.as_ref().map(|p| p.bind(gl));
            self.atlas.as_ref().map(|a| a.texture.bind(gl));
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