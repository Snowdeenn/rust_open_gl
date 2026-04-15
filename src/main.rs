use glow::{HasContext};
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
use std::mem::{size_of};

mod math;
use crate::math::_mat4_::Mat4;
use crate::math::_vec3_::Vec3;

mod shader;
mod texture;
mod mesh;

use crate::shader::create_program;
use crate::mesh::{Mesh, VertexAttrib};
use crate::texture::load_texture;


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
    let program:glow::NativeProgram = create_program(&gl, include_str!("shader/vertex.glsl"), include_str!("shader/fragement.glsl"));

    unsafe {

    
        // -------- Construction des matrices --------
        let view: Mat4 = Mat4::look_at(Vec3{x: 0.0,y: 0.0,z: 3.0}, Vec3{x: 0.0,y: 0.0,z: 0.0}, Vec3{x: 0.0,y: 1.0,z: 0.0});
        let perspective: Mat4 = Mat4::perspective(45.0_f32.to_radians(), 800.0 / 600.0, 0.1, 100.0);

        // --------- Localisation des matrices dans le vertex shader --------
        let loctaion_model: glow::NativeUniformLocation = gl.get_uniform_location(program, "uModel").unwrap();
        let location_view: glow::NativeUniformLocation = gl.get_uniform_location(program, "uView").unwrap();
        let location_proj: glow::NativeUniformLocation = gl.get_uniform_location(program, "uProjection").unwrap();

        // Schema par ligne :
        // 3 premier position
        // 3 suivant couleur
        // 2 suivant uv

        //=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=
        // (-0.5, 0.5, 0.5) -> Position du point (x, y, z)
        // (1.0, 0.0, 0.0) -> Normale de la face
        // (1.0, 0.0, 0.0) -> Couleur du point (RGB)
        // (0.0) -> uv du point
        //=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=
        let vertices: [f32; 264] = [
            // Face avant (z = +0.5)
            -0.5, -0.5,  0.5,  0.0, 0.0, 1.0,   1.0, 0.0, 0.0,  0.0, 0.0,
            0.5, -0.5,  0.5,   0.0, 0.0, 1.0,  1.0, 0.0, 0.0,  1.0, 0.0,
            0.5,  0.5,  0.5,   0.0, 0.0, 1.0,  1.0, 0.0, 0.0,  1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 0.0, 1.0,   1.0, 0.0, 0.0,  0.0, 1.0,

            // Face arrière (z = -0.5)
            0.5, -0.5, -0.5,  0.0, 0.0, -1.0,  0.0, 1.0, 0.0,  0.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, 0.0, -1.0,   0.0, 1.0, 0.0,  1.0, 0.0,
            -0.5,  0.5, -0.5, 0.0, 0.0, -1.0,   0.0, 1.0, 0.0,  1.0, 1.0,
            0.5,  0.5, -0.5,  0.0, 0.0, -1.0,  0.0, 1.0, 0.0,  0.0, 1.0,

            // Face droite (x = +0.5)
            0.5, -0.5,  0.5,  1.0, 0.0, 0.0,  0.0, 0.0, 1.0,  0.0, 0.0,
            0.5, -0.5, -0.5,  1.0, 0.0, 0.0,  0.0, 0.0, 1.0,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 0.0, 0.0,  0.0, 0.0, 1.0,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 0.0, 0.0,  0.0, 0.0, 1.0,  0.0, 1.0,

            // Face gauche (x = -0.5)
            -0.5, -0.5, -0.5,  -1.0, 0.0, 0.0,  1.0, 1.0, 0.0,  0.0, 0.0,
            -0.5, -0.5,  0.5,  -1.0, 0.0, 0.0,  1.0, 1.0, 0.0,  1.0, 0.0,
            -0.5,  0.5,  0.5,  -1.0, 0.0, 0.0,  1.0, 1.0, 0.0,  1.0, 1.0,
            -0.5,  0.5, -0.5,  -1.0, 0.0, 0.0,  1.0, 1.0, 0.0,  0.0, 1.0,

            // Face haut (y = +0.5)
            -0.5,  0.5,  0.5, 0.0, 1.0, 0.0,   1.0, 0.0, 1.0,  0.0, 0.0,
            0.5,  0.5,  0.5,  0.0, 1.0, 0.0,  1.0, 0.0, 1.0,  1.0, 0.0,
            0.5,  0.5, -0.5,  0.0, 1.0, 0.0,  1.0, 0.0, 1.0,  1.0, 1.0,
            -0.5,  0.5, -0.5, 0.0, 1.0, 0.0,   1.0, 0.0, 1.0,  0.0, 1.0,

            // Face bas (y = -0.5)
            -0.5, -0.5, -0.5, 0.0, -1.0, 0.0,   0.0, 1.0, 1.0,  0.0, 0.0,
            0.5, -0.5, -0.5,  0.0, -1.0, 0.0,  0.0, 1.0, 1.0,  1.0, 0.0,
            0.5, -0.5,  0.5,  0.0, -1.0, 0.0,  0.0, 1.0, 1.0,  1.0, 1.0,
            -0.5, -0.5,  0.5, 0.0, -1.0, 0.0,   0.0, 1.0, 1.0,  0.0, 1.0,
        ];

        let index: [u32; 36] = [
            0,  1,  2,   0,  2,  3,  // avant
            4,  5,  6,   4,  6,  7,  // arrière
            8,  9, 10,   8, 10, 11,  // droite
            12, 13, 14,  12, 14, 15,  // gauche
            16, 17, 18,  16, 18, 19,  // haut
            20, 21, 22,  20, 22, 23,  // bas
        ];

        let stride: i32 = 11 * size_of::<f32>() as i32;
        let mut attribs: Vec<VertexAttrib> = Vec::new();
            // -- Position --
            attribs.push(VertexAttrib {index: 0, size: 3, offset: 0, stride: stride});
            // -- Normale --
            attribs.push(VertexAttrib { index: 1, size: 3, offset: 3 * size_of::<f32>() as i32, stride: stride });
            // -- Couleur --
            attribs.push(VertexAttrib {index: 2, size: 3, offset: 6 * size_of::<f32>() as i32, stride: stride});
            // -- UV --
            attribs.push(VertexAttrib {index: 3, size: 2, offset: 9 * size_of::<f32>() as i32, stride: stride});
        
        
        let mesh: Mesh = Mesh::new(&gl, &vertices, &index, &attribs);

        let location_u_texture: glow::NativeUniformLocation = gl.get_uniform_location(program, "uTexture").unwrap();
        let _texture_1: glow::NativeTexture = load_texture(&gl, "image.webp", glow::TEXTURE0);
        
        let location_u_texture_2: glow::NativeUniformLocation = gl.get_uniform_location(program, "uTexture2").unwrap();
        let _texture_2: glow::NativeTexture = load_texture(&gl, "image2.webp", glow::TEXTURE1);

        gl.enable(glow::DEPTH_TEST);
        
        gl.use_program(Some(program));
        // Envoie des Texture
        gl.uniform_1_i32(Some(&location_u_texture), 0);
        gl.uniform_1_i32(Some(&location_u_texture_2), 1);

        // Envoie de la couleur lumière 
        let location_light: glow::NativeUniformLocation = gl.get_uniform_location(program, "uLightColor").unwrap();
        gl.uniform_3_f32(Some(&location_light), 1.0, 1.0, 1.0);
        // Envoie de la position de la lumière
        let location_light_pos: glow::NativeUniformLocation = gl.get_uniform_location(program, "uLightPos").unwrap();
        gl.uniform_3_f32(Some(&location_light_pos), 2.0, 2.0, 2.0);

        //Envoie de la position de la camera
        let location_cam_pos: glow::NativeUniformLocation = gl.get_uniform_location(program, "uCamPos").unwrap();
        gl.uniform_3_f32(Some(&location_cam_pos), 0.0, 0.0, 3.0);
    
        event_loop.run(move |event: Event<()>, window_target:&winit::event_loop::ActiveEventLoop| {
            window_target.set_control_flow(ControlFlow::Poll);
            match event {

                Event::WindowEvent {event, .. } => {

                    match event {
                        WindowEvent::CloseRequested => window_target.exit(),
                        WindowEvent::RedrawRequested => {
                            let current_time: f32 = time.elapsed().as_secs_f32();

                            gl.clear_color(0.0, 0.0, 0.0, 1.0);
                            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                            
                            let model: Mat4 = Mat4::rotation_y(current_time * 0.5);
                
                            // -------- Envoie des valeurs au GPU --------
                            gl.uniform_matrix_4_f32_slice(Some(&loctaion_model), false, &model.columns);
                            gl.uniform_matrix_4_f32_slice(Some(&location_view), false, &view.columns);
                            gl.uniform_matrix_4_f32_slice(Some(&location_proj), false, &perspective.columns);
                        
                            gl.bind_vertex_array(Some(mesh.vao));
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
