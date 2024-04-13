use glow::HasContext;
use super::vertex::*;

pub struct Program {
    program: glow::NativeProgram,
}

impl Program {
    pub unsafe fn new(gl: &glow::Context, vert: &str, frag: &str) -> Self {
        let program = gl.create_program().expect("Cannot create program");
    
        let vs = gl.create_shader(glow::VERTEX_SHADER).expect("cannot create vertex shader");
        gl.shader_source(vs, vert);
        gl.compile_shader(vs);
        if !gl.get_shader_compile_status(vs) {
            panic!("{}", gl.get_shader_info_log(vs));
        }
        gl.attach_shader(program, vs);

        let fs = gl.create_shader(glow::FRAGMENT_SHADER).expect("cannot create fragment shader");
        gl.shader_source(fs, frag);
        gl.compile_shader(fs);
        if !gl.get_shader_compile_status(fs) {
            panic!("{}", gl.get_shader_info_log(fs));
        }
        gl.attach_shader(program, fs);

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }
        gl.detach_shader(program, fs);
        gl.delete_shader(fs);
        gl.detach_shader(program, vs);
        gl.delete_shader(vs);

        Program {
            program
        }
    }
    pub unsafe fn default(gl: &glow::Context) -> Self {
        Self::new(gl, DEFAULT_VS, DEFAULT_FS)
    }
    pub unsafe fn bind(&self, gl: &glow::Context) {
        gl.use_program(Some(self.program))
    }
    pub unsafe fn set_proj(&self, proj: &[f32; 16], gl: &glow::Context) {
        gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(self.program, "projection").as_ref(), true, proj);
    }
}
pub const DEFAULT_FS: &str = r#"#version 330 core
in vec4 col;
out vec4 frag_colour;

void main() {
    frag_colour = col;
}
"#;
pub const DEFAULT_VS: &str = r#"#version 330 core
layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec4 in_col;

out vec4 col;

// uniform mat4 projection;
const mat4 projection = mat4(1.0);


void main() {
    col = in_col;
    gl_Position = projection * vec4(in_pos, 1.0);
}
"#;