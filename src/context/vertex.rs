use crate::util::*;


// so is drawing just to a &mut buffer?
// well we also want to stage it as a list of commands that can be sorted to be honest
// so the enum implements ord and also needs to be able to draw to a VertexBufCPU

// mmm but to implement this do I want it so theres a function of 

// so what if the drawing function was like a static function pointer... lol or like a closure currying stuff into that, oh god, idk



use glow::HasContext;

// now why is this here

pub struct VertexBufGPU {
    pub vao: glow::VertexArray,
    pub vbo: glow::Buffer,
    pub ebo: glow::Buffer,
    pub num_verts: usize,
}
impl VertexBufGPU {
    pub fn new(gl: &glow::Context) -> Self {
        unsafe {
            let vbo = gl.create_buffer().unwrap();
            let ebo = gl.create_buffer().unwrap();
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo)); // Bind the EBO
            // let vert_size: usize = std::mem::size_of::<Vertex>();
            let vert_size = 4*9;
            dbg!(vert_size);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, vert_size as i32, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, vert_size as i32, 3 * 4);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, vert_size as i32, 7 * 4);
            gl.enable_vertex_attrib_array(2);
            VertexBufGPU { vao, vbo, ebo, num_verts: 0 }
        }
    }
    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo)); // Bind the EBO
        }
    }

    pub fn render(&self, gl: &glow::Context) {
        unsafe {
            self.bind(gl); // Bind both VAO, VBO, and EBO
            gl.draw_elements(
                glow::TRIANGLES,
                self.num_verts as i32, // number of indices
                glow::UNSIGNED_INT,   // type of indices
                0                      // offset
            );
        }
    }

    pub fn free(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
            gl.delete_buffer(self.ebo);
        }
    }
    pub fn update(&mut self, gl: &glow::Context, buf: &VertexBufCPU) {
        self.bind(gl);
        self.num_verts = buf.inds.len();
        unsafe {
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buf.verts.as_bytes(), glow::STATIC_DRAW);
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, buf.inds.as_bytes(), glow::STATIC_DRAW);
        }
    }
}

trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl<T> AsBytes for Vec<T> {
    fn as_bytes(&self) -> &[u8] {
        // Get a pointer to the data and calculate the length in bytes
        let ptr = self.as_ptr() as *const u8;
        let len = self.len() * std::mem::size_of::<T>();

        // Convert the pointer and length to a byte slice
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

#[test]
fn test_as_bytes() {
    let a = vec![1i32, 1000i32, 0i32, 1i32];
    let b = a.as_bytes();
    dbg!(b);
}