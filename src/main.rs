
mod state;
mod time;
mod shader;

#[macro_use]
extern crate glium;

use glium::Surface;
use state::State;

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 3],
    uv: [f32; 2],
}
implement_vertex!(Vertex, pos, uv);

const TEST_VERTICES: [Vertex; 4] = [
    Vertex { pos: [-0.5, -0.5, 0.0],    uv: [0.0, 0.0] },
    Vertex { pos: [0.5, -0.5, 0.0],     uv: [0.0, 0.0] },
    Vertex { pos: [0.5, 0.5, 0.0],      uv: [0.0, 0.0] },
    Vertex { pos: [-0.5, 0.5, 0.0],     uv: [0.0, 0.0] },
];

const TEST_INDICES: [u8; 6] = [
    0, 1, 2, 0, 2, 3
];

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let mut state = State {
        time: time::init_time(),
        shaders: Vec::new(),
    };

    shader::create_shaders(&display, &mut state);

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                },
                _ => (),
            },
            _ => (),
        }

        state.time.update();
        draw(&display, &state);
    });
}

fn draw(display: &glium::Display<glium::glutin::surface::WindowSurface>, state: &State) {
    let mut frame = display.draw();

    frame.clear_all((0.8, f32::sin(state.time.time) * 0.5 + 0.5, 0.1, 1.0), 0.0, 0);

    let vertex_buffer = glium::VertexBuffer::new(display, &TEST_VERTICES).unwrap();
    let indices = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &TEST_INDICES).unwrap();

    match &shader::get_shader("test", &state).program {
        None => println!("Uuh?"),
        Some(program) => {
            frame.draw(&vertex_buffer, &indices, program, &glium::uniforms::EmptyUniforms, &Default::default())
                .expect("Failed to draw!");
        }
    }

    frame.finish().expect("Uuh?");
}

