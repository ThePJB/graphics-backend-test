use glow::*;
use std::{collections::HashMap, f32::consts::PI};
use crate::util::*;
use crate::*;
use std::path::Path;
use super::render_context::*;

impl RenderContext {
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
        let names = paths.iter().map(|p| path_to_name_fn(p, sprites_path));
        let resources = std::iter::zip(names, img_buffers);
        self.pack_sprites(resources);
    }

    // sets the texture and the resource handles dictionary
    pub fn pack_sprites(&mut self, resources: impl Iterator<Item = (String, ImageBuffer)>) {
        dbg!("begin pack sprites");
        let mut resource_tuples: Vec<(String, ImageBuffer)> = resources.collect();
        resource_tuples.sort_by(|a, b| {
            let a = a.1.wh.dot(&a.1.wh);
            let b = b.1.wh.dot(&b.1.wh);
            b.cmp(&a)
        });
        // make a packing
        let wh = ATLAS_WH;
        let mut arena = Arena2D::new(wh);
        resource_tuples.iter().for_each(|rt| {dbg!(&rt.0);});
        for (name, sprite) in resource_tuples.into_iter() {
            let r = arena.alloc(sprite.wh);
            println!("finish alloc with xy {:?} for {}", r, &name);
            let h = SpriteHandle { xy: r.xy.as_vec2() / wh.as_vec2(), wh: r.wh.as_vec2() / wh.as_vec2() };
            self.resource_handles.insert(name, h);
            //sub buffer 2d on the texture as well!
            unsafe {
                self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
                self.gl.tex_sub_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    r.xy.x,
                    r.xy.y,
                    r.wh.x as i32,
                    r.wh.y as i32,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    glow::PixelUnpackData::Slice(&sprite.data.as_bytes()),
                );
            }
        }
        dbg!("end pack sprites");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc() {
        let mut arena = Arena2D::new(ivec2(1024, 1024)); // Assuming an ATLAS_WH of 1024x1024

        arena.alloc(ivec2(768,64));
        let b = arena.alloc(ivec2(576,64));
        assert_eq!(b.xy.x, 0);
        assert_eq!(b.xy.y, 64);
        arena.alloc(ivec2(480,48));
        arena.alloc(ivec2(384,64));
        
        // Check if the allocated rectangle matches the expected size
        dbg!(&arena.rects);

        // Allocate a 66x56 rectangle
        let rect2_wh = ivec2(66, 56);
        let rect2 = arena.alloc(rect2_wh);
        
        // Check if the allocated rectangle matches the expected size
        assert_eq!(rect2.xy, ivec2(768, 0));
    }

    // Add more tests here if needed
}

pub struct Arena2D {
    rects: Vec<IRect2>,
    wh: IVec2,
}
impl Arena2D {
    pub fn new(wh: IVec2) -> Self {
        Arena2D {
            rects: vec![],
            wh,
        }
    }
    fn alloc_x(&self, y: i32, wh: IVec2) -> Option<IRect2> {
        println!("alloc x y={}",y);
        let mut x = 0;
        loop {
            let candidate_r = IRect2 {
                xy: ivec2(x, y),
                wh,
            };
            if x+wh.x >= self.wh.x {
                return None;
            }
            println!("loop candidate_r = {:?}",candidate_r);
            let overlapper = self.rects.iter().find(|r| r.overlaps(&candidate_r));
            match overlapper {
                Some(r) => x = r.xy.x + r.wh.x - 1,
                None => return Some(candidate_r),
            }
            x += 1;
        }
    }
    pub fn alloc(&mut self, wh: IVec2) -> IRect2 {
        println!("alloc {:?}", wh);
        for j in 0..(ATLAS_WH.y-wh.y) {
            if let Some(r) = self.alloc_x(j, wh) {
                self.rects.push(r);
                return r;
            }
        }
        panic!("unable to alloc {:?}, contents of rects: {:?}", wh, self.rects);
    }
}
