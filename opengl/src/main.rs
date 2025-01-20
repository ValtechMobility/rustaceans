mod debug;
mod game;
mod program;
mod renderer;
mod shader;
mod texture;
mod vertex;

use game::Game;
use glutin::{
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, ContextBuilder, GlRequest,
};
use renderer::RendererError;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Learn OpenGL with Rust")
        .with_inner_size(glutin::dpi::PhysicalSize::new(1600.0, 1200.0));

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let window_size = gl_context.window().inner_size();
    println!(
        "Initial window size: {}x{}",
        window_size.width, window_size.height
    );

    let possibly_current_context;
    unsafe {
        possibly_current_context = gl_context.make_current().expect("something went wrong");
    };

    gl::load_with(|ptr| possibly_current_context.get_proc_address(ptr) as *const _);

    let mut game = Game::new();

    let mut renderer = renderer::Renderer::new(window_size.width as i32, window_size.height as i32)
        .expect("Cannot create renderer");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    possibly_current_context.resize(physical_size);
                    if let Err(RendererError::ResizeError) =
                        renderer.resize(physical_size.width as i32, physical_size.height as i32)
                    {
                        let actual_size = possibly_current_context.window().inner_size();
                        renderer
                            .resize(actual_size.width as i32, actual_size.height as i32)
                            .expect("Actual window size should be valid.");
                    }
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(keycode),
                            state,
                            ..
                        },
                    ..
                } => game.handle_keypress(keycode, state),
                _ => (),
            },
            Event::RedrawRequested(_) => {
                renderer.update(&game);
                renderer.draw();
                possibly_current_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => {
                renderer.update(&game);
                possibly_current_context.window().request_redraw();
            }

            _ => (),
        }
    });
}
