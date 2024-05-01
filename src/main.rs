
mod util;
mod game;
use crate::game::*;

// well it looks like main is doing the switchy switchy not run, this is ok
// maybe can use wasm-pack or something
// how to set target

fn main() {
    let context = Context::new("GL test");
    let gl = context.get_gl();
    let game = game::Game::new(gl);
    context.run(game);
}

// a architecture for this that works with all the cooked web stuff:
/* maybe if it was literally like Run<T: Game> 
Where Game {
    step(input, dt)
    render -> Vec<RenderCommand>
}
// or literally step(inputs) -> outputs // render commands, play sound commands etc
*/
// then we can hide all dumb event loop stuff etc.
// what about sound and portability of sound to web? cpal on web?
// https://github.com/nannou-org/cpal_wasm_example

// yea i just wouldnt wanna do this n lock myself into something that wasnt gonna work. or eg 
// occlusion and shit, multiple render buffers, fbo shit etc.

// i could also just expand this in the non web direction

// theres an element also of like, does the render function just take a reference to gl? game store its own assets etc?
// might be better for doing multipass rendering and shit more directly in which case render list etc is not context any more
// maybe better as middleware
// maybe I add quads and hdr etc

// the idea of like make it 1 function is cute
// frame(input, &gl, &mut audio context thing?) or -> Vec<AudioCommand>




// test minecraft light!!!
// on image
// how many ms to do a frame
// 2d 1d convolution decomposition
// but also u kinda only want to do it from the lit sources yeah
// can u do u64 shit lol

// otherwise what jump flooding raymarched occlusion test?
// kinda want the sdf / raymarching ability for ai to player check
// what about the occlusion of players vision and fog of war?
// youve got like the tiles grid