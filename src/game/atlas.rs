use crate::util::*;
use glow::HasContext;
use super::*;

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