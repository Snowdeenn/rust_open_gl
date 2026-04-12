use glow::HasContext;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::WindowAttributes;

use glutin::config::{ConfigTemplateBuilder, Config};
use glutin::context::{ContextApi, ContextAttributes, ContextAttributesBuilder, NotCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder};

use glutin_winit::{DisplayBuilder, GlWindow};

use core::f32;
use std::time::Instant;
use std::mem::size_of;
use std::slice::{from_raw_parts};

mod math;
use crate::math::_mat4_::Mat4;
use crate::math::_vec3_::Vec3;
fn main() {
    let event_loop: EventLoop<()> = EventLoop::new().unwrap();
    
    let window_attributes: WindowAttributes = WindowAttributes::default()
        .with_title("Hello OpenGL Rust")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

    let template: ConfigTemplateBuilder = ConfigTemplateBuilder::new()
        .with_stencil_size(8)
        .with_depth_size(24);

    // Création du display et de la window
    let display_builder: DisplayBuilder = DisplayBuilder::new()
        .with_window_attributes(Some(window_attributes));

    let (window, gl_config) = display_builder
        .build(&event_loop, template, |config| { config
                            .reduce(|accum: Config, config: Config | { 
                                if config.num_samples() > accum.num_samples() {config} else {accum}
                            }).unwrap()
        }
    ).unwrap();

    let window: winit::window::Window = window.expect("[Window] Echec de la création de la fenêtre");
    let raw_window_handle:  winit::raw_window_handle::RawWindowHandle = window.window_handle().unwrap().as_raw();
                        
    let context_attribute: ContextAttributes  = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version::new(3, 3))))
        .build(Some(raw_window_handle));

    let not_current_gl_context: NotCurrentContext = unsafe {
        gl_config.display().create_context(&gl_config, &context_attribute)
            .expect("[gl_context] Impossible de créer le context openGL")
    };

    let surface_attribute: glutin::surface::SurfaceAttributes<glutin::surface::WindowSurface> = window
        .build_surface_attributes(SurfaceAttributesBuilder::default())
        .expect("[Surface] Erreur lors de la création des attribues de la surface");

    let gl_surface: Surface<glutin::surface::WindowSurface> = unsafe {
        gl_config.display().create_window_surface(&gl_config, &surface_attribute)
            .expect("[gl_surface] Erreur lors de la création de la surface")
    };

    let gl_context: glutin::context::PossiblyCurrentContext = not_current_gl_context.make_current(&gl_surface)
        .expect("[gl_context] Impossible de passer le context en current");

    let gl: glow::Context = unsafe {
        glow::Context::from_loader_function(|s| {
            gl_config.display().get_proc_address(std::ffi::CString::new(s).unwrap().as_c_str())
        })
    };

    let time: Instant = Instant::now();

    unsafe {
        // ------ Vertex Shader ------
        let vertex_shader: glow::NativeShader = gl.create_shader(glow::VERTEX_SHADER)
            .unwrap();
        gl.shader_source(vertex_shader, include_str!("shader/vertex.glsl"));
        gl.compile_shader(vertex_shader);
        
        if !gl.get_shader_compile_status(vertex_shader) {
            panic!("[Vertex_shader] Erreur : {}", gl.get_shader_info_log(vertex_shader));
        }

        // ------ Fragement Shader ------
        let frag_shader: glow::NativeShader = gl.create_shader(glow::FRAGMENT_SHADER)
            .unwrap();
        gl.shader_source(frag_shader, include_str!("shader/fragement.glsl"));
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

    
        // -------- Construction des matrices --------
        let view: Mat4 = Mat4::look_at(Vec3{x: 0.0,y: 0.0,z: 3.0}, Vec3{x: 0.0,y: 0.0,z: 0.0}, Vec3{x: 0.0,y: 1.0,z: 0.0});
        let perspective: Mat4 = Mat4::perspective(45.0_f32.to_radians(), 800.0 / 600.0, 0.1, 100.0);

        // --------- Localisation des matrices dans le vertex shader --------
        let loctaion_model: glow::NativeUniformLocation = gl.get_uniform_location(program, "uModel").unwrap();
        let location_view: glow::NativeUniformLocation = gl.get_uniform_location(program, "uView").unwrap();
        let location_proj: glow::NativeUniformLocation = gl.get_uniform_location(program, "uProjection").unwrap();


        // -------- Netoyage --------
        {
            gl.detach_shader(program, vertex_shader);
            gl.detach_shader(program, frag_shader);
            gl.delete_shader(vertex_shader);
            gl.delete_shader(frag_shader);
        }

        // -------- Mise en place VAO et VBO --------
        let vao: glow::NativeVertexArray = gl.create_vertex_array().unwrap();
        let vbo: glow::NativeBuffer = gl.create_buffer().unwrap();
        let ebo:glow::NativeBuffer = gl.create_buffer().unwrap();
        {
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));

            let vertices: [f32; 48] = [
                -0.5, -0.5, 0.5, 1.0, 0.0, 0.0, //0
                0.5, -0.5, 0.5, 0.0, 1.0, 0.0,  //1
                -0.5, 0.5, 0.5, 0.0, 0.0, 1.0,  //2
                0.5, 0.5, 0.5, 0.0, 1.0, 0.0,   //3
                -0.5, -0.5, -0.5, 1.0, 0.0, 0.0,//4
                0.5, -0.5, -0.5, 0.0, 1.0, 0.0, //5
                -0.5, 0.5, -0.5, 0.0, 0.0, 1.0, //6
                0.5, 0.5, -0.5, 0.0, 1.0, 0.0,  //7
            ];

            let index: [u32; 36] = [
                // face avant
                0, 1, 2,
                1, 3, 2,
                //face droite
                1, 5, 7,
                1, 3, 7,
                //face gauche
                0, 2, 6,
                0, 4, 6,
                //face arrière
                4, 5, 6,
                5, 7, 6,
                //face bas
                0, 4, 1,
                4, 5, 1,
                //face haut
                2, 3, 6,
                3, 6, 7
            ];

            // -------- Remplissage du VBO --------
            let vertice_ptr: *const u8 = &vertices as *const f32 as *const u8;
            let vertice_size = vertices.len() * size_of::<f32>();

            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, from_raw_parts(vertice_ptr, vertice_size), glow::STATIC_DRAW);
        
            // -------- Remplissage de l'EBO --------
            let index_ptr: *const u8 = &index  as *const u32 as *const u8;
            let index_size = index.len() * size_of::<u32>();

            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, from_raw_parts(index_ptr, index_size), glow::STATIC_DRAW);

            // -------- Description pour le vao --------
            let stride: i32 = 6 * size_of::<f32>() as i32;
            let offset: i32 = 3 * size_of::<f32>() as i32; 

            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, stride, offset);
            gl.enable_vertex_attrib_array(0);
            gl.enable_vertex_attrib_array(1);

            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }

        event_loop.run(move |event: Event<()>, window_target:&winit::event_loop::ActiveEventLoop| {
            window_target.set_control_flow(ControlFlow::Poll);
            match event {

                Event::WindowEvent {event, .. } => {

                    match event {
                        WindowEvent::CloseRequested => window_target.exit(),
                        WindowEvent::RedrawRequested => {
                            let current_time: f32 = time.elapsed().as_secs_f32();

                            gl.clear_color(0.0, 0.0, 0.0, 1.0);
                            gl.clear(glow::COLOR_BUFFER_BIT);
                            gl.use_program(Some(program));

                            let model: Mat4 = Mat4::rotation_y(current_time * 0.5)
                                    .multiply(Mat4::rotation_x(current_time))
                                    .multiply(Mat4::rotation_z(current_time * 0.25));
                
                            // -------- Envoie des valeurs au GPU --------
                            gl.uniform_matrix_4_f32_slice(Some(&loctaion_model), false, &model.columns);
                            gl.uniform_matrix_4_f32_slice(Some(&location_view), false, &view.columns);
                            gl.uniform_matrix_4_f32_slice(Some(&location_proj), false, &perspective.columns);
    
                            gl.bind_vertex_array(Some(vao));
                            gl.draw_elements(glow::TRIANGLES, 36, glow::UNSIGNED_INT, 0);

                            gl_surface.swap_buffers(&gl_context).unwrap();
                            
                        },
                        _ => ()
                    }
                },
                _ => window.request_redraw(),
            }
        }).unwrap();
    };
    
}
