OK What am I doing
Its not until I have working characters that I can put this into the game.
Clean up. Do we still like the anim shit? Just need to make sure I can define a necromancer hey.


ok i mean anim seems pretty good now just have to test it, impl Render for EntityAppearance that spits out the render commands

then render a lil guy and change his look etc
each layer may technically have own look, i was str8 hackin that in before

more toiling in the mines
get lil guy workin

maybe i need to render directly a sprite first

ohh its failing to locate asset folder indebugger. ok i done for today

its a bit of like why dont i have a render trait
which in a way im impling render for game.

and the impl is probably calling render. yea thats pretty sick

Hey its pretty working
but, how are we determining size and also why isnt it only a slice

Ok its kinda working
 - determine size of vertices from handle wh * res / atlas res or whatever
 - not sure about keys. bool walking, bool idle? not idle maybe casting
 - not sure its working for the key - unit test through stages of animation yea
 honestly probably keep own count of where in animation cycle so speed can be modified like if walking speed has a modification eg slow terrain

 ok layer height zs must be negative lol. keep it in mind
 but yea its like working. what about vertex size eh

 and the render to texture thing
 and gl viewport etc
 and scaling

 ya cool so render to texture, and gl viewport

 eventual maybe caring about aspect ratio idk lol. yea honestly rendercontext having wh and stuff just reads it from there seems good

 yea im defs not rendering pixel perfect tho
 seems to like scale which is kinda what i want anyway but still

 hmm one solution is to use pixel coordinates in the system and then dvide for UVs right at the end
 tbh i kinda like subdividing in uv space
 arena 2d is pixel rects
 original implementation was pixel rects lol.
 yeh well i mean it will have like pixel rect coordinates, num of frames, and frame number. So from that u calculate the uv coordinates




can we make a really awesome 2d postprocessing filter / compositor?
i reckon we downsample lighting and stuff a fair bit and maybe marching squares or fade it or something. /4 or /8




 hmm is there actually any downside to making image / pixel functions in UV space instead? pixel precision does feel kinda exact

 -----------

 Todo - defer changing to uvs to later in the pipeline
 hopefully fixing the scaling issue


