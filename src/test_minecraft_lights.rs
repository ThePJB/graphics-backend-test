use crate::util::*;

// So we need a bool texture for like collisions map
// Then we need a vec3 texture for light
// then we do minecraft light shit with a list of lights and look at it
// it kinda is like fake soft shadows or whatever
// and do we do inverse square law, i reckon so ay

#[test]
pub fn test_minecraft_lights() {
    let wh = ivec2(128, 128);
    let mut collision_stencil: Texture<bool> = Texture::new(wh, false);
    collision_stencil.draw_circle(ivec2(64, 64), 8, true);

    let mut colour_buf = Texture::<Vec3>::new(wh, vec3(0.0, 0.0, 0.0));

    // so yea we could have like minecraft cellular logic to propagate the light to neighbours
    // like flood fill it with decreasing intensity

    // we could also implement like, jump flood algorithm, get sdf, then get ray casted
    // but also minecraft light doesnt really blend
    // but you could like alow max of each channel to go through or something

}