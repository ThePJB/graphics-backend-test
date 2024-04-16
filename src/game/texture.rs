use crate::util::*;
use glow::HasContext;


pub struct Texture {
    pub id: Option<glow::Texture>,
    pub wh: IVec2,
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