use glow::HasContext;
use std::slice::{from_raw_parts};

pub struct Mesh {
    pub vao: glow::NativeVertexArray,
    pub vbo: glow::NativeBuffer,
    pub ebo: glow::NativeBuffer,
}

pub struct VertexAttrib {
    pub index:  u32,
    pub size:   i32,
    pub offset: i32,
    pub stride: i32,
}

impl Mesh {
    pub fn new(gl: &glow::Context, vertices: &[f32], index: &[u32], attribs: &[VertexAttrib]) -> Mesh {
        let vao: glow::NativeVertexArray = unsafe {
            gl.create_vertex_array().unwrap()
        };
        let vbo: glow::NativeBuffer = unsafe {
            gl.create_buffer().unwrap()
        };
        let ebo: glow::NativeBuffer = unsafe {
            gl.create_buffer().unwrap()
        };

        let mesh: Mesh = Mesh { vao, vbo, ebo };

        unsafe {
            gl.bind_vertex_array(Some(mesh.vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(mesh.vbo));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(mesh.ebo));

            {
                let vertice_ptr: *const u8 = vertices.as_ptr() as *const u8;
                let vertice_size: usize = vertices.len() * size_of::<f32>();

                gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, from_raw_parts(vertice_ptr, vertice_size), glow::STATIC_DRAW);
            }

            // -------- Remplissage de l'EBO --------
            {
                let index_ptr: *const u8 = index.as_ptr() as *const u8;
                let index_size: usize = index.len() * size_of::<u32>();

                gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, from_raw_parts(index_ptr, index_size), glow::STATIC_DRAW);
            }

            // -------- Description pour le vao --------
            for attrib in attribs {
                gl.vertex_attrib_pointer_f32(attrib.index, attrib.size, glow::FLOAT, false, attrib.stride, attrib.offset);
                gl.enable_vertex_attrib_array(attrib.index);
            }

            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }
        mesh
    }
}