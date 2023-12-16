
mod state;
mod time;
mod shader;
mod util;
mod texture;
mod assets;

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

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    
    // Create quad mesh
    let quad_vertices = glium::VertexBuffer::new(&display, &util::QUAD_VERTICES).unwrap();
    let quad_indices = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &util::QUAD_INDICES).unwrap();
    
    // Create cube mesh
    let cube_vertices = glium::VertexBuffer::new(&display, &util::CUBE_VERTICES).unwrap();
    let cube_indices = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &util::CUBE_INDICES).unwrap();

    // Initial state
    let mut state = State {
        time: time::init_time(),
        shaders: Vec::new(),
        resolution: vec2u(0, 0),
        quad_vertices, 
        quad_indices,
        cube_vertices, 
        cube_indices,
        camera_projection_mat: mat4::create(),
        assets: assets::crate_base(),
    };

    // Compile shaders
    shader::create_shaders(&display, &mut state);

    // TEST: create some textures
    texture::create_to_assets("x.png", &mut state);

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

        // If resolution changed, updates values
        let new_resolution = vec2u(_window.inner_size().width, _window.inner_size().height);
        if new_resolution.x != state.resolution.x || new_resolution.y != state.resolution.y {
            state.resolution = new_resolution;
            mat4::perspective(&mut state.camera_projection_mat, PI / 4.0, state.resolution.x as f32 / state.resolution.y as f32, 0.1, None);
        }

        draw(&display, &state);
    });
}

fn draw(display: &glium::Display<glium::glutin::surface::WindowSurface>, state: &State) {
    let mut frame = display.draw();

    frame.clear_all((0.8, f32::sin(state.time.time) * 0.5 + 0.5, 0.1, 1.0), 0.0, 0);


    let mut transform_mat: Mat4 = mat4::create();
    let mut test: Mat4 = mat4::create();
    mat4::translate(&mut test, &transform_mat, &[0.0,0.0, -5.0]);
    mat4::rotate_y(&mut transform_mat, &test, state.time.time);
    mat4::rotate_x(&mut test, &transform_mat, PI/4.0);
    mat4::rotate_z(&mut transform_mat, &test, PI/4.0);

    let uniforms = uniform!{
        projection: util::mat_to_uniform(state.camera_projection_mat),
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

    let test_shader = assets::get_shader(&"test".to_string(), &state);
    let test_shader2 = assets::get_shader(&"test2".to_string(), &state);

    frame.draw(&state.cube_vertices, &state.cube_indices, &test_shader.program, &uniforms, &params).expect("Failed to draw!");

    frame.clear_depth(0.0);

    // TODO: put the image here!
    draw_screen_billboard([0.5, 0.5, 0.0], [0.2, 0.2], &test_shader2, &mut frame, &state);

    frame.finish().expect("Uuh?");
}

// position:    center of the quad, (0, 0) is bottom left, (1, wh) is the top right, last coordinate is z
// size:        size of the quad, 1 is screen height
fn draw_screen_billboard(position: Vec3, size: Vec2, shader: &shader::Shader, frame: &mut glium::Frame, state: &State) {
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfMore,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };
    
    let mut no_projection_mat = mat4::create();
    mat4::identity(&mut no_projection_mat);

    let ratio = state.resolution.y as f32 / state.resolution.x as f32;

    let mut scale_mat = mat4::create();
    let mut translate_mat = mat4::create();
    let mut transform = mat4::create();
    mat4::from_scaling(&mut scale_mat, &[size[0] * 2.0 * ratio, size[1] * 2.0, 1.0]);
    mat4::from_translation(&mut translate_mat, &[-1.0 + position[0] * 2.0 * ratio, -1.0 + position[1] * 2.0, position[2]]);
    mat4::mul(&mut transform, &translate_mat, &scale_mat);

    let uniforms = uniform!{
        projection: util::mat_to_uniform(no_projection_mat),
        transform: util::mat_to_uniform(transform),
    };

    frame.draw(&state.cube_vertices, &state.cube_indices, &shader.program, &uniforms, &params)
        .expect("Failed to draw!");
}
