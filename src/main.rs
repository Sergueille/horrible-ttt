
mod state;
mod time;
mod shader;
mod util;
mod texture;
mod assets;
mod draw;
mod game;
mod input;

#[macro_use]
extern crate glium;
extern crate gl_matrix;

use std::env;

use gl_matrix::quat;
use gl_matrix::vec2;
use gl_matrix::vec3;
use glium::Surface;
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

const MAX_FPS: i32 = 100; 
const ROW_COUNT: i32 = 6;
const ROTATE_MULT: f32 = 1.8; // HACK: temporary solution
const ROTATE_SPEED_DECREASE: f32 = 250.0; // TEST
static FOV: f32 = PI / 4.0;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

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

        lmb: input::get_info(),
        rmb: input::get_info(),
        mmb: input::get_info(),

        cube_transform_matrix: mat4::create(),
        cube_rotation: quat::create(),
        cube_rotation_velocity: quat::create(),
        cube_size: 2.0,

        blocks: [game::BlockType::Empty; (ROW_COUNT * ROW_COUNT * ROW_COUNT) as usize],
        last_mouse_sphere_intersection: None,
    };

    // Reset rotation
    quat::identity(&mut state.cube_rotation);
    quat::identity(&mut state.cube_rotation_velocity);

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
                },
                winit::event::WindowEvent::MouseInput { device_id: _, state: action, button, .. } => {
                    match button {
                        winit::event::MouseButton::Left => input::update_info(&action, &mut state.lmb),
                        winit::event::MouseButton::Right => input::update_info(&action, &mut state.rmb),
                        winit::event::MouseButton::Middle => input::update_info(&action, &mut state.mmb),
                        winit::event::MouseButton::Other(_) => {},
                    }
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
            mat4::perspective(&mut state.camera_projection_mat, FOV, state.resolution.x as f32 / state.resolution.y as f32, 0.1, Some(100.0));
        }

        // Prevent calling loop too many times, wait a bit if necessary
        if state.time.time - state.last_main_time > 1.0 / (MAX_FPS as f32) {
            main_loop(&mut state);
            state.last_main_time = state.time.time;
            
            // reset button states
            input::reset_info(&mut state.lmb);
            input::reset_info(&mut state.rmb);
            input::reset_info(&mut state.mmb);
        }
    });
}

fn main_loop(state: &mut State) {
    let mut frame = state.display.draw();

    frame.clear_all((0.1, 0.3, 0.05, 1.0), 100000.0, 0);

    let mut translate_mat: Mat4 = mat4::create();
    mat4::from_translation(&mut translate_mat, &[0.0,0.0, -5.0]);
    
    let mut rotation_mat = mat4::create();
    mat4::from_quat(&mut rotation_mat, &state.cube_rotation);
    
    let mut transform_mat: Mat4 = mat4::create();
    mat4::mul(&mut transform_mat, &translate_mat, &rotation_mat);
    
    state.cube_transform_matrix = transform_mat;

    // frame.draw(&state.cube_vertices, &state.cube_indices, &test_shader2.program, &uniforms, &params).expect("Failed to draw!");

    // Draw lines
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
            
            let color = if corner { [1.0, 1.0, 0.5, 1.0] } else { [1.0, 1.0, 1.0, 0.5] };

            draw::draw_line_world(&a, &b, &color, width, corner, &mut frame, state);
            draw::draw_line_world(&c, &d, &color, width, corner, &mut frame, state);
            draw::draw_line_world(&e, &f, &color, width, corner, &mut frame, state);
        }
    }

    // TEST: draw crosses
    let billboard_shader = assets::get_shader(&"default".to_string(), &state);    
    let billboard_uniforms = dynamic_uniform!{
        tex: &assets::get_texture(&"x.png".to_string(), &state).texture,
    };
    for i in 0..ROW_COUNT {
        for j in 0..ROW_COUNT {
            for k in 0..ROW_COUNT {
                let pos = vec3i(i, j, k);
                let block_type = game::get_block(&pos, &state);
                let position = apply_cube_transform(&get_block_coords(&pos, &state), &state);

                if block_type != game::BlockType::Empty {
                    draw::draw_world_billboard(position, [0.05, 0.05], 0.0, billboard_shader, Some(billboard_uniforms.clone()), &mut frame, state)
                }
            }
        }
    }

    frame.clear_depth(10000.0);

    // Mouse control
    if state.lmb.hold {
        let mouse_ray = util::get_mouse_ray(&state);
        let intersection = util::intersect_line_sphere(&[0.0, 0.0, -5.0], 2.0, &[0.0, 0.0, 0.0], &mouse_ray);

        // TODO: looks like when delta mouse is too small, rotation is ignored because of precision issues,
        //       so the rotation doesn't follow the mouse
        if !state.lmb.down { // Already down last frame
            if intersection != None && state.last_mouse_sphere_intersection != None {
                // Get delta angle
                let from = intersection.expect("");
                let to = state.last_mouse_sphere_intersection.expect("");

                let mut from_n = vec3::create();
                let mut to_n = vec3::create();
                vec3::normalize(&mut from_n, &from);
                vec3::normalize(&mut to_n, &to);

                let mut delta = quat::create();
                quat::rotation_to(&mut delta, &from_n, &to_n);

                let multiplied = util::multiply_quat(&delta, ROTATE_MULT); // ~ "Multiply" the quaternion

                let mut new_rotation = quat::create();
                quat::mul(&mut new_rotation, &multiplied, &state.cube_rotation);
                state.cube_rotation = new_rotation;
                
                let test_shader = assets::get_shader(&"test".to_string(), &state);      
                draw::draw_world_billboard(intersection.expect(""), [0.04, 0.04], 0.0, test_shader, None, &mut frame, state);

                state.cube_rotation_velocity = multiplied;
            }   
        }

        state.last_mouse_sphere_intersection = intersection;
    }
    else {
        // Decrease velocity
        state.cube_rotation_velocity = util::multiply_quat(&state.cube_rotation_velocity, (-state.time.delta_time * ROTATE_SPEED_DECREASE).exp()) ;

        // Apply velocity rotation
        let mut new_rotation = quat::create();
        quat::mul(&mut new_rotation, &state.cube_rotation_velocity, &state.cube_rotation);
        state.cube_rotation = new_rotation;
    }

    // Make sure the cube rotation is normalized (sometimes isn't because of precision issues) 
    let mut normalized = quat::create();
    quat::normalize(&mut normalized, &state.cube_rotation);
    state.cube_rotation = normalized;

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
