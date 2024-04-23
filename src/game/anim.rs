use std::{collections::{HashMap, HashSet}, f32::consts::PI};
use self::render_context::*;
use super::*;
use crate::util::*;

pub type AnimDef = (
    &'static str, // key
    SpriteHandle,
    usize,  // width
    bool,   // looping
);


// single anim ie of 1 sprite
#[derive(Clone, Debug, Copy)]
pub struct Anim {
    pub h: SpriteHandle,
    pub n: u8,
    pub looping: bool,
}

#[derive(Debug)]
pub struct Layer {
    anims: HashMap<String, Anim>,
    curr_anim: String,
    // curr_playing: Option<(Anim, usize)>,
    z: f32,
    curr_frame: usize,
    t_last_update: f32,
    colour: Vec4,
}
impl Layer {
    pub fn new(colour: Vec4, z: f32, anims: impl Iterator<Item = AnimDef>) -> Self {
        let mut layer = Layer {
            anims: HashMap::from_iter(anims.map(|(key, h, width, looping)| (key.to_owned(), Anim { h, n: width as _, looping: false }))),
            curr_anim: "".to_owned(),
            z,
            curr_frame: 0,
            t_last_update: 0.0,
            colour,
        };
        layer.select_idle_anim(&HashSet::new());
        layer
    }
    // pub fn current_sprite(&self) -> SpriteHandle {
    //     let curr_anim = self.anims[&self.curr_anim].clone();
    //     let xy = curr_anim.h.xy + curr_anim.h.wh.projx() * self.curr_frame as f32;
    //     let wh = curr_anim.h.wh.projx() / self.curr_frame as f32 + curr_anim.h.wh.projy();
    //     SpriteHandle {xy, wh}
    // } 
    fn sprite_args(&self, center: Vec2, radians: f32) -> SpriteArgs {
        let curr_anim = self.anims[&self.curr_anim].clone();
        SpriteArgs { 
            center, 
            radians, 
            z: self.z, 
            h: curr_anim.h, 
            frame: self.curr_frame as _, 
            num_frames: curr_anim.n as _,
            c: self.colour,
         }
    }
    fn update(&mut self, t: f32, keys: &HashSet<String>) {
        let curr_anim = self.anims[&self.curr_anim].clone();
        // does curr anim match key?
        // keys maybe should be a vec so they have relative importance
        // and honestly maybe like a anim finished flag could be set or hashset be used if we wanted to know cast was done
        let t_update: f32 = 1.0/60.0 * 4.0;
        let should_update = t - self.t_last_update > t_update;
        if !should_update { return; }
        if keys.contains(&self.curr_anim) {
            self.curr_frame += 1;
            if self.curr_frame == curr_anim.n as _ {
                if curr_anim.looping {
                    self.curr_frame = 0;
                } else {
                    self.select_idle_anim(keys);
                }
            }
        } else {
            self.curr_frame = 0;
            self.select_idle_anim(keys);
        }
        self.t_last_update = t;
        println!("updating {} {} {}", self.z, self.curr_frame, self.curr_anim);
    }
    fn select_idle_anim(&mut self, keys: &HashSet<String>) {
        if keys.contains("walk") && self.anims.contains_key("walk") {
            self.curr_frame = 0;
            self.curr_anim = "walk".to_owned();
        } else if self.anims.contains_key("idle") {
            self.curr_frame = 0;
            self.curr_anim = "idle".to_owned();
        } else if self.anims.contains_key("walk") {
            self.curr_frame = 0;
            self.curr_anim = "walk".to_owned();
        }
    }
}

#[derive(Debug)]
pub struct EntityAppearance {
    layers: Vec<Layer>,
    keys: HashSet<String>,
}
impl EntityAppearance {
    pub fn new(layers: Vec<Layer>) -> Self {
        let mut ea = EntityAppearance {
            layers,
            keys: HashSet::new(),
        };
        // if any has idle its idle
        // otherwise if any has walk its walk
        if ea.layers.iter().any(|layer| layer.anims.iter().any(|(k, _)| k == "idle")) {
            ea.add_key("idle".to_owned())
        } else if ea.layers.iter().any(|layer| layer.anims.iter().any(|(k, _)| k == "walk")) {
            ea.add_key("walk".to_owned())
        }
        ea
    }
    pub fn add_key(&mut self, key: String) {
        self.keys.insert(key);
        // and change stuff etc
    }
    pub fn remove_key(&mut self, key: String) {
        self.keys.remove(&key);
        // and change stuff etc
    }
    pub fn update(&mut self, t: f32) {
        self.layers.iter_mut().for_each(|l| l.update(t, &self.keys))
    }
    pub fn draw(&self, pos: Vec2, facing: Vec2) -> impl Iterator<Item = RenderCommand> + '_ {
        self.layers.iter().map(move |layer| {
            RenderCommand::Sprite(
                layer.sprite_args(pos, facing.y.atan2(facing.x))
            )
        })
    }
}

// could also like randomize it etc
// equipped items could have layers too
// and tbf layers could be jsoned or something
pub fn necromancer_appearance(res: &HashMap<String, SpriteHandle>) -> EntityAppearance {
    let mut layers = vec![];
    layers.push(
        Layer::new(
        vec4(1.0, 0.0, 0.0, 1.0),
            -0.1,
            [
                ("walk", res["guy/feet/walk"], 10, true),
                ("idle", res["guy/feet/idle"], 1, true),
            ].into_iter(),
    ));
    layers.push(
        Layer::new(
        vec4(1.0, 0.0, 0.0, 1.0),
            -0.2,
            [
                ("walk", res["guy/body/walk"], 12, true),
                ("idle", res["guy/body/idle"], 9, true),
            ].into_iter(),
    ));
    layers.push(
        Layer::new(
        vec4(1.0, 0.0, 0.0, 1.0),
            -0.3,
            [
                ("walk", res["guy/head/walk"], 6, true),
                ("idle", res["guy/head/idle"], 9, true),
            ].into_iter(),
    ));
    EntityAppearance::new(layers)
}

#[test]
fn test_draw() {
    let layer = Layer::new(vec4(0.5, 1.0, 0.0, 1.0), 0.0, [("idle", SpriteHandle{xy: vec2(0.0, 0.0), wh: vec2(1.0, 1.0)}, 10, true)].into_iter());
    let mut appearance = EntityAppearance::new(vec![layer]);
    appearance.add_key("idle".to_owned());
    dbg!(&appearance);
    let rcs: Vec<RenderCommand> = appearance.draw(vec2(0.0, 0.0), vec2(1.0, 0.0)).collect();
    dbg!(&rcs);
    appearance.update(1.0);
    dbg!(&appearance);
    let rcs: Vec<RenderCommand> = appearance.draw(vec2(0.0, 0.0), vec2(1.0, 0.0)).collect();
    dbg!(&rcs);
}