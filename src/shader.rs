use glow::{HasContext};

pub fn create_program(gl: &glow::Context, vertex_src: &str, frag_src: &str) -> glow::NativeProgram {

    unsafe {
        // ------ Vertex Shader ------
        let vertex_shader: glow::NativeShader = gl.create_shader(glow::VERTEX_SHADER)
            .unwrap();
        gl.shader_source(vertex_shader, vertex_src);
        gl.compile_shader(vertex_shader);
        
        if !gl.get_shader_compile_status(vertex_shader) {
            panic!("[Vertex_shader] Erreur : {}", gl.get_shader_info_log(vertex_shader));
        }

        // ------ Fragement Shader ------
        let frag_shader: glow::NativeShader = gl.create_shader(glow::FRAGMENT_SHADER)
            .unwrap();
        gl.shader_source(frag_shader, frag_src);
        gl.compile_shader(frag_shader);

        if !gl.get_shader_compile_status(frag_shader) {
            panic!("[Frag_shader] Erreur : {}", gl.get_shader_info_log(frag_shader));
        }

        // -------- Programme --------
        let program: glow::NativeProgram = gl.create_program().unwrap();
        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, frag_shader);
        gl.link_program(program);

        if !gl.get_program_link_status(program) {
            panic!("[Program] Erreur : {}", gl.get_program_info_log(program));
        }

        // -------- Netoyage --------
        {
            gl.detach_shader(program, vertex_shader);
            gl.detach_shader(program, frag_shader);
            gl.delete_shader(vertex_shader);
            gl.delete_shader(frag_shader);
        }
        program
    } 
}
