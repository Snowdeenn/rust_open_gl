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

use std::time::Instant;

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

    let time = Instant::now();
    
    event_loop.run(move |event: Event<()>, window_target:&winit::event_loop::ActiveEventLoop| {
        window_target.set_control_flow(ControlFlow::Poll);
        match event {

            Event::WindowEvent {event, .. } => {

                match event {
                    WindowEvent::CloseRequested => window_target.exit(),
                    WindowEvent::RedrawRequested => {
                        let current_time = time.elapsed().as_secs_f32();

                        let r = (current_time.sin() + 1.0) / 2.0;
                        let g = (current_time.cos() + 1.0) / 2.0;
                        let b = (current_time.sin() + 1.0) / 2.0;

                        unsafe {
                            gl.clear_color(r, g, b, 1.0);
                            gl.clear(glow::COLOR_BUFFER_BIT);
                            gl_surface.swap_buffers(&gl_context).unwrap();
                        };
                        
                    },
                    _ => ()
                }
            },
            _ => window.request_redraw(),
        }
    }).unwrap();
}
