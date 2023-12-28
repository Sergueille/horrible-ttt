
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

use game::{GameInfo, GameState, BlockType};
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
const COUNT_TO_WIN: i32 = 5;
const ROTATE_SPEED_DECREASE: f32 = 500.0;
const CUBE_POS: [f32; 3] = [0.0, 0.0, -5.0];
static FOV: f32 = PI / 4.0;

// TEST
const CROSS_COLOR: Vec4 = [0.9, 0.2, 0.2, 1.0];
const CIRCLE_COLOR: Vec4 = [0.2, 0.2, 0.9, 1.0];

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
    let game_state = GameInfo {
        cube_transform_matrix: mat4::create(),
        cube_rotation: quat::create(),
        cube_rotation_velocity: quat::create(),
        cube_size: 2.0,

        blocks: [game::BlockType::None; (ROW_COUNT * ROW_COUNT * ROW_COUNT) as usize],
        start_mouse_sphere_intersection: None,
        last_mouse_sphere_intersection: None,
        mouse_sphere_radius: 0.0,
        drag_start_rotation: quat::create(),
        last_face_id: -1,
        depth: 0,

        state: GameState::Turn(BlockType::Cross),
    };

    let mut state = state::State {
        time: time::init_time(),
        resolution: vec2u(0, 0),
        quad_vertices, 
        quad_indices,
        cube_vertices, 
        cube_indices,
        camera_projection_mat: mat4::create(),
        assets: assets::crate_base(),
        display,
        mouse_coords_normalized: [0.0, 0.0],
        mouse_coords_pixels: vec2i(0, 0),
        mouse_delta_normalized: [0.0, 0.0],
        mouse_ray: vec3::create(),
        last_main_time: 0.0,
        input: input::get_input(),

        quad_params: glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        },
        cube_params: glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        },
        draw_queue: std::collections::binary_heap::BinaryHeap::new(),
        game: game_state,
    };

    // Reset rotation
    quat::identity(&mut state.game.cube_rotation);
    quat::identity(&mut state.game.cube_rotation_velocity);

    // Compile shaders
    shader::create_shaders(&mut state);

    // TEST: create some textures
    texture::create_to_assets("x.png", &mut state);
    texture::create_to_assets("o.png", &mut state);
    texture::create_to_assets("sandwich.png", &mut state);

    event_loop.run(move |event, _, control_flow| {

        // Remember last mouse position
        let old_mous_pos = state.mouse_coords_normalized;

        input::get_events(&event, control_flow, &mut state);

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
            input::reset_input(&mut state);
        }
    });
}

fn main_loop(state: &mut State) {
    let mut frame = state.display.draw();

    // Clear buffer
    frame.clear_all((0.1, 0.3, 0.05, 1.0), 100000.0, 0);

    // Create the transform matrix of the cube
    let mut translate_mat: Mat4 = mat4::create();
    mat4::from_translation(&mut translate_mat, &CUBE_POS);
    
    let mut rotation_mat = mat4::create();
    mat4::from_quat(&mut rotation_mat, &state.game.cube_rotation);
    
    let mut transform_mat: Mat4 = mat4::create();
    mat4::mul(&mut transform_mat, &translate_mat, &rotation_mat);
    
    state.game.cube_transform_matrix = transform_mat;

    // Get intersection with cube
    let pos_on_cube = get_mouse_pos_on_cube(&state);

    // Draw lines
    for i in [0, 6, 1, 2, 3, 4, 5] {
        for j in [0, 6, 1, 2, 3, 4, 5] {
            let i_norm = state.game.cube_size / (ROW_COUNT as f32) * i as f32 - 1.0;
            let j_norm = state.game.cube_size / (ROW_COUNT as f32) * j as f32 - 1.0;

            let a = apply_cube_transform(&[i_norm, -1.0, j_norm], &state);
            let b = apply_cube_transform(&[i_norm,  1.0, j_norm], &state);
            let c = apply_cube_transform(&[-1.0, i_norm, j_norm], &state);
            let d = apply_cube_transform(&[ 1.0, i_norm, j_norm], &state);
            let e = apply_cube_transform(&[i_norm, j_norm, -1.0], &state);
            let f = apply_cube_transform(&[i_norm, j_norm,  1.0], &state);

            let corner = (i == 0 || i == 6) && 
                         (j == 0 || j == 6);
            let border = (i == 0 || i == 6) ||
                         (j == 0 || j == 6);

            if border {
                let width = if corner { 0.01 } else { 0.002 };
                let color = if corner { [1.0, 1.0, 0.5, 1.0] } else { [1.0, 1.0, 1.0, 0.2] };
    
                draw::draw_line_world(&a, &b, color, width, !corner, state);
                draw::draw_line_world(&c, &d, color, width, !corner, state);
                draw::draw_line_world(&e, &f, color, width, !corner, state);
            } 
        }
    }

    // Draw crosses and circles
    for i in 0..ROW_COUNT {
        for j in 0..ROW_COUNT {
            for k in 0..ROW_COUNT {
                let pos = vec3i(i, j, k);
                let block_type = game::get_block(&pos, &state);
                let position = apply_cube_transform(&get_block_coords(&pos, &state), &state);

                if block_type != game::BlockType::None {
                    let mut color = if block_type == game::BlockType::Cross { CROSS_COLOR } else { CIRCLE_COLOR };
                    color[3] = 0.2;
                    draw::draw_world_billboard(position, [0.05, 0.05], 0.0, [1.0, 1.0, 1.0, 1.0], 
                        draw::TexArg::One(&get_symbol_texture(block_type)), &"default_tex", state);
                    draw_cube_on_block(&pos, color, &"default_color", state);
                }
            }
        }
    }

    // Mouse control
    let mut moving_cube = false;

    if state.input.lmb.hold || state.input.mmb.hold {
        let intersection = util::intersect_line_sphere(&CUBE_POS, state.game.mouse_sphere_radius, &[0.0, 0.0, 0.0], &state.mouse_ray);
        moving_cube = state.game.start_mouse_sphere_intersection != None && intersection != None;

        if !state.input.lmb.down && !state.input.mmb.down { // Already down last frame
            if moving_cube {
                // Get delta angle
                let from = state.game.start_mouse_sphere_intersection.expect("");
                let mut to = intersection.expect("");

                let mut to_tmp = vec3::create();
                vec3::sub(&mut to_tmp, &to, &CUBE_POS);
                vec3::normalize(&mut to, &to_tmp);

                let mut delta = quat::create();
                quat::rotation_to(&mut delta, &from, &to); // Rotation from old vector to new vector

                quat::mul(&mut state.game.cube_rotation, &delta, &state.game.drag_start_rotation);

                state.game.cube_rotation_velocity = delta;

                match state.game.last_mouse_sphere_intersection {
                    Some(vec) => {
                        quat::rotation_to(&mut state.game.cube_rotation_velocity, &vec, &to);
                    },
                    None => {
                        quat::identity(&mut state.game.cube_rotation_velocity);
                    }
                }

                state.game.last_mouse_sphere_intersection = Some(to);
            }
        }
        else {

            match pos_on_cube {
                Some(ref pos) => {
                    let mut tmp1 = vec3::create();
                    let mut tmp2 = vec3::create();

                    vec3::sub(&mut tmp1, &pos.world_pos, &CUBE_POS);
                    vec3::normalize(&mut tmp2, &tmp1);
                    state.game.start_mouse_sphere_intersection = Some(tmp2);
                    state.game.last_mouse_sphere_intersection = Some(tmp2);

                    vec3::sub(&mut tmp1, &pos.world_pos, &CUBE_POS);
                    state.game.mouse_sphere_radius = vec3::len(&tmp1);

                }
                None => {
                    state.game.start_mouse_sphere_intersection = None;
                    state.game.last_mouse_sphere_intersection = None;
                    state.game.mouse_sphere_radius = 0.0;
                }
            }

            state.game.drag_start_rotation = state.game.cube_rotation.clone();
        }
    }

    match state.game.state {
        GameState::GameWon(_) => {
            // TODO
        },
        GameState::Turn(_) => {
            handle_turn(pos_on_cube, state);
        }
    };

    if !moving_cube {
        // Decrease velocity
        state.game.cube_rotation_velocity = util::multiply_quat(&state.game.cube_rotation_velocity, (-state.time.delta_time * ROTATE_SPEED_DECREASE).exp()) ;

        // Apply velocity rotation
        let mut new_rotation = quat::create();
        quat::mul(&mut new_rotation, &state.game.cube_rotation_velocity, &state.game.cube_rotation);
        state.game.cube_rotation = new_rotation;
    }

    // Make sure the cube rotation is normalized (sometimes isn't because of precision issues) 
    let mut normalized = quat::create();
    quat::normalize(&mut normalized, &state.game.cube_rotation);
    state.game.cube_rotation = normalized;

    draw::draw_all(&mut frame, state);

    frame.finish().expect("Uuh?");
}

fn handle_turn(pos_on_cube: Option<CubePosition>, state: &mut State) {
    if pos_on_cube.is_some() {
        let pos = pos_on_cube.expect("");

        // Handle wheel
        if pos.face_id != state.game.last_face_id {
            state.game.depth = 0;
        }

        if state.input.wheel_down {
            if state.game.depth > 0 {
                state.game.depth -= 1;
            }
        }
        else if state.input.wheel_up {
            if state.game.depth < ROW_COUNT - 1 {
                state.game.depth += 1;
            }
        }

        let mut block_pos = pos.coords;
        if pos.is_wheel_inverted {
            block_pos[pos.wheel_direction] = ROW_COUNT - 1 - state.game.depth;
        }
        else {
            block_pos[pos.wheel_direction] = state.game.depth;
        }

        let pos_vec = util::vec3i_arr(block_pos);

        let billboard_pos = apply_cube_transform(&get_block_coords(&pos_vec, &state), &state);
        let color = vec4::from_values(1.0, 1.0, 1.0, (state.time.time * 10.0).sin() * 0.25 + 0.75);
        draw::draw_world_billboard(billboard_pos, [0.05, 0.05], 0.0, color,
            draw::TexArg::One(&get_symbol_texture_of_turn(state)), "default_tex", state);

        block_pos[pos.wheel_direction] = 0;
        let mut a = get_block_coords(&util::vec3i_arr(block_pos), &state);
        block_pos[pos.wheel_direction] = ROW_COUNT - 1;
        let mut b = get_block_coords(&util::vec3i_arr(block_pos), &state);

        let half_block_size = state.game.cube_size / ROW_COUNT as f32 / 2.0;

        let depth_delta = if pos.wheel_direction == 2 { -half_block_size } else { half_block_size };
        a[pos.wheel_direction] -= depth_delta;
        b[pos.wheel_direction] += depth_delta;

        let helper_color = [1.0, 0.0, 0.0, 1.0];
        let helper_width = 0.005;

        let mut a_points: [Vec3; 4] = [[0.0; 3]; 4];
        let mut b_points: [Vec3; 4] = [[0.0; 3]; 4];
        let deltas = [[1, 1], [-1, 1], [-1, -1], [1, -1]];
        for i in 0..4 {
            let mut line_a = a.clone();
            let mut line_b = b.clone();
            line_a[pos.tangent1] += deltas[i][0] as f32 * half_block_size;
            line_a[pos.tangent2] += deltas[i][1] as f32 * half_block_size;
            line_b[pos.tangent1] += deltas[i][0] as f32 * half_block_size;
            line_b[pos.tangent2] += deltas[i][1] as f32 * half_block_size;

            a_points[i] = apply_cube_transform(&line_a, &state);
            b_points[i] = apply_cube_transform(&line_b, &state);

            draw::draw_line_world(&a_points[i], &b_points[i], helper_color, helper_width, false, state);

            if i > 0 {
                draw::draw_line_world(&a_points[i], &a_points[i-1], helper_color, helper_width, false, state);
                draw::draw_line_world(&b_points[i], &b_points[i-1], helper_color, helper_width, false, state);
            }
        }

        draw::draw_line_world(&a_points[0], &a_points[3], helper_color, helper_width, false, state);
        draw::draw_line_world(&b_points[0], &b_points[3], helper_color, helper_width, false, state);

        // Submit
        if state.input.rmb.up {
            game::submit_click(&pos_vec, state);
        }
        
        state.game.last_face_id = pos.face_id;
    }
    else {
        state.game.last_face_id = -1;
    }
}

fn apply_cube_transform(vec: &Vec3, state: &State) -> Vec3 {
    let mut res = vec4::create();
    vec4::transform_mat4(&mut res, &[vec[0], vec[1], vec[2], 1.0], &state.game.cube_transform_matrix);
    res = util::divide_by_w(res);

    return [res[0], res[1], res[2]];
}

fn apply_cube_rotation(vec: &Vec3, state: &State) -> Vec3 {
    let mut res = vec3::create();
    vec3::transform_quat(&mut res, &vec, &state.game.cube_rotation);
    return res;
}

fn get_block_coords(pos: &Vec3i, state: &State) -> Vec3 {
    let block_size = state.game.cube_size / ROW_COUNT as f32;
    
    return [
        -(block_size * (ROW_COUNT - 1) as f32 / 2.0) + pos.x as f32 * block_size,
        -(block_size * (ROW_COUNT - 1) as f32 / 2.0) + pos.y as f32 * block_size,
        (block_size * (ROW_COUNT - 1) as f32 / 2.0) - pos.z as f32 * block_size,
    ];
}

pub struct CubePosition {
    pub world_pos: Vec3,
    pub coords: [i32; 3],
    pub wheel_direction: usize,
    pub tangent1: usize,
    pub tangent2: usize,
    pub is_wheel_inverted: bool,
    pub face_id: i32,
}

fn get_mouse_pos_on_cube(state: &State) -> Option<CubePosition> {
    let normals = [
        [-1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, 1.0],
    ];

    let tangent1 = [
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
    ];
    let tangent2 = [
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ];

    let origin = [
        [-1.0, -1.0, 1.0],
        [1.0, -1.0, 1.0],
        [-1.0, -1.0, 1.0],
        [-1.0, 1.0, 1.0],
        [-1.0, -1.0, -1.0],
        [-1.0, -1.0, 1.0],
    ];

    // 1: x, 2: y, 3: z, negative means inverted
    let coords: [[i32; 3]; 6] = [
        [3, 2, -1],
        [3, 2, 1],
        [3, 1, -2],
        [3, 1, 2],
        [1, 2, 3],
        [1, 2, -3],
    ];

    for i in 0..6 {
        let transformed_normal = apply_cube_rotation(&normals[i], &state);
        let transformed_origin = apply_cube_transform(&origin[i], &state);

        // Ignore if facing away
        if !is_pointing_towards_camera(&transformed_origin, &transformed_normal) {
            continue;
        }  

        let transformed_t1 = apply_cube_rotation(&tangent1[i], &state);
        let transformed_t2 = apply_cube_rotation(&tangent2[i], &state);

        let intersection = util::intersect_line_plane(&transformed_origin, &transformed_normal, &[0.0, 0.0, 0.0], &state.mouse_ray);
        let plane_coords = util::get_plane_coords(&transformed_origin, &transformed_t1, &transformed_t2, &intersection);

        if plane_coords[0] < 0.0 || plane_coords[0] > state.game.cube_size || plane_coords[1] < 0.0 || plane_coords[1] > state.game.cube_size {
            continue;
        }

        let block_coords = [
            (plane_coords[0] / state.game.cube_size * 6.0).floor() as i32,
            (plane_coords[1] / state.game.cube_size * 6.0).floor() as i32,
        ];

        let mut res = [0, 0, 0];
        res[(coords[i][0] - 1) as usize] = block_coords[0];
        res[(coords[i][1] - 1) as usize] = block_coords[1];

        return Some(CubePosition {
            world_pos: intersection,
            coords: res,
            wheel_direction: (coords[i][2].abs() - 1) as usize,
            tangent1: (coords[i][0] - 1) as usize,
            tangent2: (coords[i][1] - 1) as usize,
            is_wheel_inverted: coords[i][2] > 0,
            face_id: i as i32,
        });
    }

    return None;
}

fn get_symbol_texture(t: game::BlockType) -> &'static str {
    if t == game::BlockType::Cross {
        return "x.png";
    }
    else {
        return "o.png";
    }
}

fn get_symbol_texture_of_turn(state: &State) -> &'static str {
    let block_type = match state.game.state {
        GameState::Turn(block_type) => block_type,
        GameState::GameWon(block_type) => block_type,
    };

    return get_symbol_texture(block_type);
}

fn draw_cube_on_block<'a>(pos: &Vec3i, color: Vec4, shader: &'a str, state: &mut State<'a>) {
    let mut translate_mat = mat4::create();
    mat4::from_translation(&mut translate_mat, &get_block_coords(pos, state));

    
    let mut result_transform = mat4::create();
    mat4::mul(&mut result_transform, &state.game.cube_transform_matrix, &translate_mat);
    
    let scale_amount = state.game.cube_size / ROW_COUNT as f32;

    let cloned = result_transform.clone();
    mat4::scale(&mut result_transform, &cloned, &[scale_amount, scale_amount, scale_amount]);

    draw::draw_cube(result_transform, color, shader, state)
}

