
mod state;
mod time;
mod shader;
mod util;
mod texture;
mod assets;
mod draw;

#[macro_use]
extern crate glium;
extern crate gl_matrix;

use gl_matrix::vec2;
use glium::Surface;
use glium::uniforms;
use state::State;
use gl_matrix::common::*;
use gl_matrix::mat4;
use gl_matrix::vec4;
use util::*;

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 3],
    uv: [f32; 2],
}
implement_vertex!(Vertex, pos, uv);

static MAX_FPS: i32 = 100; 
static ROW_COUNT: i32 = 6;

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
    let mut state = state::State {
        time: time::init_time(),
        shaders: Vec::new(),
        resolution: vec2u(0, 0),
        quad_vertices, 
        quad_indices,
        cube_vertices, 
        cube_indices,
        camera_projection_mat: mat4::create(),
        assets: assets::crate_base(),
        display,
        mouse_coords_normalized: [0.0, 0.0],
        mouse_coords_pixels: vec2u(0, 0),
        mouse_delta_normalized: [0.0, 0.0],
        last_main_time: 0.0,
        cube_transform_matrix: mat4::create(),
        cube_size: 2.0,
    };

    // Compile shaders
    shader::create_shaders(&mut state);

    // TEST: create some textures
    texture::create_to_assets("x.png", &mut state);
    texture::create_to_assets("sandwich.png", &mut state);

    event_loop.run(move |ev, _, control_flow| {

        // Remember last mouse position
        let old_mous_pos = state.mouse_coords_normalized;

        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                },
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    state.mouse_coords_pixels = vec2u(position.x as u32, state.resolution.y - position.y as u32);
                    state.mouse_coords_normalized = [state.mouse_coords_pixels.x as f32 / state.resolution.y as f32, state.mouse_coords_pixels.y as f32 / state.resolution.y as f32];
                }
                _ => (),
            },
            _ => (),
        }

        // Get mouse delta position
        vec2::sub(&mut state.mouse_delta_normalized, &state.mouse_coords_normalized, &old_mous_pos);

        state.time.update();

        // If resolution changed, updates values
        let new_resolution = vec2u(_window.inner_size().width, _window.inner_size().height);
        if new_resolution.x != state.resolution.x || new_resolution.y != state.resolution.y {
            state.resolution = new_resolution;
            mat4::perspective(&mut state.camera_projection_mat, PI / 4.0, state.resolution.x as f32 / state.resolution.y as f32, 0.1, Some(100.0));
        }

        // Prevent calling loop too many times, wait a bit if necessary
        if state.time.time - state.last_main_time > 1.0 / (MAX_FPS as f32) {
            main_loop(&mut state);
            state.last_main_time = state.time.time;
        }
    });
}

fn main_loop(state: &mut State) {
    let mut frame = state.display.draw();

    frame.clear_all((0.1, 0.3, 0.05, 1.0), 100000.0, 0);

    let mut transform_mat: Mat4 = mat4::create();
    let mut test: Mat4 = mat4::create();
    mat4::translate(&mut test, &transform_mat, &[0.0,0.0, -5.0]);
    mat4::rotate_y(&mut transform_mat, &test, state.time.time);
    mat4::rotate_x(&mut test, &transform_mat, PI/4.0);
    mat4::rotate_z(&mut transform_mat, &test, PI/4.0);
    
    state.cube_transform_matrix = transform_mat;

    let uniforms = uniform!{
        projection: util::mat_to_uniform(state.camera_projection_mat),
        transform: util::mat_to_uniform(transform_mat),
        tex: &assets::get_texture(&"x.png".to_string(), &state).texture
    };

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };

    let test_shader = assets::get_shader(&"test".to_string(), &state);
    let test_shader2 = assets::get_shader(&"test2".to_string(), &state);

    // frame.draw(&state.cube_vertices, &state.cube_indices, &test_shader2.program, &uniforms, &params).expect("Failed to draw!");

    for i in [0, 6, 1, 2, 3, 4, 5] {
        for j in [0, 6, 1, 2, 3, 4, 5] {
            let i_norm = state.cube_size / (ROW_COUNT as f32) * i as f32 - 1.0;
            let j_norm = state.cube_size / (ROW_COUNT as f32) * j as f32 - 1.0;

            let a = apply_cube_transform(&[i_norm, -1.0, j_norm], &state);
            let b = apply_cube_transform(&[i_norm,  1.0, j_norm], &state);
            let c = apply_cube_transform(&[-1.0, i_norm, j_norm], &state);
            let d = apply_cube_transform(&[ 1.0, i_norm, j_norm], &state);
            let e = apply_cube_transform(&[i_norm, j_norm, -1.0], &state);
            let f = apply_cube_transform(&[i_norm, j_norm,  1.0], &state);

            let corner = (i == 0 || i == 6) && 
                         (j == 0 || j == 6);

            let width = if corner { 0.01 } else { 0.002 };
            
            let alpha = if corner { 1.0 } else { 0.5};
            let color = [1.0, 1.0, 0.5, alpha];

            draw::draw_line_world(&a, &b, &color, width, corner, &mut frame, state);
            draw::draw_line_world(&c, &d, &color, width, corner, &mut frame, state);
            draw::draw_line_world(&e, &f, &color, width, corner, &mut frame, state);
        }
    }

    // TEST: draw crosses
    let billboard_shader = assets::get_shader(&"test2".to_string(), &state);    
    let billboard_uniforms = dynamic_uniform!{
        tex: &assets::get_texture(&"x.png".to_string(), &state).texture,
    };
    for i in 0..ROW_COUNT {
        for j in 0..ROW_COUNT {
            for k in 0..ROW_COUNT {
                let position = apply_cube_transform(&get_block_coords(&vec3i(i, j, k), &state), &state);
                draw::draw_world_billboard(position, [0.05, 0.05], 0.0, billboard_shader, Some(billboard_uniforms.clone()), &mut frame, state)
            }
        }
    }

    let quad_uniforms = dynamic_uniform!{
        tex: &assets::get_texture(&"sandwich.png".to_string(), &state).texture,
        color: &[1.0 as f32, 1.0, 1.0, 0.5],
    };

    frame.clear_depth(10000.0);

    draw::draw_screen_billboard(
        [state.mouse_coords_normalized[0], 
        state.mouse_coords_normalized[1], 0.0], 
        [0.2, 0.2], 
        state.time.time, 
        &test_shader2, 
        Some(quad_uniforms), 
        &mut frame, state
    );

    frame.finish().expect("Uuh?");
}

fn apply_cube_transform(vec: &Vec3, state: &State) -> Vec3 {
    let mut res = vec4::create();
    vec4::transform_mat4(&mut res, &[vec[0], vec[1], vec[2], 1.0], &state.cube_transform_matrix);
    return [res[0], res[1], res[2]];
}

fn get_block_coords(pos: &Vec3i, state: &State) -> Vec3 {
    let block_size = state.cube_size / ROW_COUNT as f32;
    
    return [
        -(block_size * (ROW_COUNT - 1) as f32 / 2.0) + pos.x as f32 * block_size,
        -(block_size * (ROW_COUNT - 1) as f32 / 2.0) + pos.y as f32 * block_size,
        -(block_size * (ROW_COUNT - 1) as f32 / 2.0) + pos.z as f32 * block_size,
    ];
}
