mod program;
mod renderer;
mod shader;
mod shapes;

use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, ContextBuilder, GlRequest,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Learn OpenGL with Rust");

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let possibly_current_context;
    unsafe {
        possibly_current_context = gl_context.make_current().expect("something went wrong");
    };

    gl::load_with(|ptr| possibly_current_context.get_proc_address(ptr) as *const _);

    let renderer = renderer::Renderer::new().expect("Cannot create renderer");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    possibly_current_context.resize(physical_size)
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                renderer.draw();
                possibly_current_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
