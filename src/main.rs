
mod state;
mod time;
mod shader;
mod util;

#[macro_use]
extern crate glium;
extern crate gl_matrix;

use glium::Surface;
use state::State;
use gl_matrix::common::*;
use gl_matrix::mat4;
use util::*;

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 3],
    uv: [f32; 2],
}
implement_vertex!(Vertex, pos, uv);

const TEST_VERTICES: [Vertex; 8] = [
    Vertex { pos: [-0.5, -0.5, 0.5],    uv: [0.0, 0.0] },
    Vertex { pos: [0.5, -0.5, 0.5],     uv: [1.0, 0.0] },
    Vertex { pos: [0.5, 0.5, 0.5],      uv: [1.0, 1.0] },
    Vertex { pos: [-0.5, 0.5, 0.5],     uv: [0.0, 1.0] },
    Vertex { pos: [-0.5, -0.5, -0.5],     uv: [0.0, 0.0] },
    Vertex { pos: [0.5, -0.5, -0.5],     uv: [0.0, 0.0] },
    Vertex { pos: [0.5, 0.5, -0.5],     uv: [0.0, 0.0] },
    Vertex { pos: [-0.5, 0.5, -0.5],     uv: [0.0, 0.0] },
];

const TEST_INDICES: [u8; 36] = [
    0, 1, 2, 0, 2, 3,
    6, 5, 4, 7, 6, 4,
    6, 2, 1, 1, 5, 6,
    0, 3, 7, 7, 4, 0,
    5, 1, 0, 0, 4, 5,
    3, 2, 6, 6, 7, 3,
];

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let mut state = State {
        time: time::init_time(),
        shaders: Vec::new(),
        resolution: vec2u(_window.inner_size().width, _window.inner_size().height),
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

    let mut perspective_mat: Mat4 = mat4::create();
    mat4::perspective(&mut perspective_mat, PI / 4.0, state.resolution.x as f32 / state.resolution.y as f32, 0.001, Some(50.0));
    let mut transform_mat: Mat4 = mat4::create();
    let mut test: Mat4 = mat4::create();
    mat4::translate(&mut test, &transform_mat, &[0.0,0.0, 0.0]);
    mat4::rotate_y(&mut transform_mat, &test, state.time.time);
    mat4::rotate_z(&mut test, &transform_mat, PI/4.0);
    mat4::rotate_x(&mut transform_mat, &test, PI/4.0);

    let uniforms = uniform!{
        //projection: util::mat_to_uniform(perspective_mat),
        projection: util::mat_to_uniform(mat4::identity(&mut test)),
        transform: util::mat_to_uniform(transform_mat),
    };

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfMore,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };

    match &shader::get_shader("test", &state).program {
        None => println!("Uuh?"),
        Some(program) => {
            frame.draw(&vertex_buffer, &indices, program, &uniforms, &params)
                .expect("Failed to draw!");
        }
    }

    print_mat(perspective_mat);
    print_mat(transform_mat);
    println!("--");

    frame.finish().expect("Uuh?");
}

