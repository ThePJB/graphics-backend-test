https://ianjk.com/rust-gamejam/
cargo watch ran the build script automatically whenever the code was saved.
devserver hosted the local web page and automatically reloaded it when a file changed
Visual Studio Code was configured to automatically save

OK fuck this unless winit can do wasm

// https://github.com/dabreegster/minimal_websys_winit_glow_demo/blob/main/src/lib.rs

this is pretty based but yeh lookin pretty hard
like have to do asset loading and sound as well

what if we only targeted web lmao. the based platform.



now so for assets do i want like a master struct of asset handles? and instancing that is like loading all of it. and on that u do load and unload too

or do we auto load into a dictionary and reference by Rc<str>

maybe the struct is based

it needs 2d allocation into an atlas when the handle is issued.
handles get issued and stored in accessible struct thereafter.

ok tomorrows coding i reckon. Get sprite drawing (atlas resource system) load and delete etc.


vec macros need to make dot, unit



compare texture shit
uvs right?

yeh idfk maybe look in render doc

didnt do any texture unit 0 shit. have had to do that before
can game just own gl?
and made with new

yea its possible rendercontext just owns gl, vbo, vao, etc and lives in game.
and maybe goodbye a lot of the abstractions
not sure if im missing a call or something


colours are scrambled
uvs r cooked
its like it backward or something
quads uvs are 1 1 1 1
colours right uvs wrong

y tf does printing it seem to say the right thing and then in renderdoc it says a different thing
rewrite, simplify, etc
try to do agile ay


texture binding is wrong or something
or needs some texture unit 0 or something
bc the texture is right
vertex data seems good
but draw col * sample is black
    frag_colour = texture(tex, uv) * col;
    // frag_colour = col;