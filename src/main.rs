
mod state;
mod time;
mod shader;
mod util;
mod texture;
mod assets;

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
            let i_norm = 2.0 / 6.0 * i as f32 - 1.0;
            let j_norm = 2.0 / 6.0 * j as f32 - 1.0;

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

            draw_line_world(&a, &b, &color, width, corner, &mut frame, state);
            draw_line_world(&c, &d, &color, width, corner, &mut frame, state);
            draw_line_world(&e, &f, &color, width, corner, &mut frame, state);
        }
    }

    let quad_uniforms = dynamic_uniform!{
        tex: &assets::get_texture(&"sandwich.png".to_string(), &state).texture,
        color: &[1.0 as f32, 1.0, 1.0, 0.5],
    };

    frame.clear_depth(10000.0);

    draw_screen_billboard(
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

// position:    center of the quad, (0, 0) is bottom left, (1, wh) is the top right, last coordinate is z
// size:        size of the quad, 1 is screen height
fn draw_screen_billboard<'a, 'b>(position: Vec3, size: Vec2, rotation: f32, shader: &shader::Shader, uniforms: Option<uniforms::DynamicUniforms<'a, 'b>>, frame: &mut glium::Frame, state: &State) {
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        blend: glium::Blend::alpha_blending(),        
        .. Default::default()
    };
    
    let mut no_projection_mat = mat4::create();
    mat4::identity(&mut no_projection_mat);

    let ratio = state.resolution.y as f32 / state.resolution.x as f32;

    // OPTI: hum
    let mut scale_mat = mat4::create();
    let mut translate_mat = mat4::create();
    let mut rotate_mat = mat4::create();
    let mut ratio_mat = mat4::create();
    let transform = &mut mat4::create();
    mat4::from_scaling(&mut scale_mat, &[size[0] * 2.0, size[1] * 2.0, 1.0]);
    mat4::from_scaling(&mut ratio_mat, &[ratio, 1.0, 1.0]);
    mat4::from_translation(&mut translate_mat, &[-1.0 + position[0] * 2.0 * ratio, -1.0 + position[1] * 2.0, position[2]]);
    mat4::from_z_rotation(&mut rotate_mat, rotation);
    mat4::mul(transform, &translate_mat, &ratio_mat,);
    mat4::mul(transform, &mat4::clone(transform), &rotate_mat);
    mat4::mul(transform, &mat4::clone(transform), &scale_mat);

    let transform_u = util::mat_to_uniform(*transform);
    let projection_u = util::mat_to_uniform(no_projection_mat);

    match uniforms {
        Some(mut u) => {
            u.add(&"transform", &transform_u);
            u.add(&"projection", &projection_u);

            frame.draw(&state.cube_vertices, &state.cube_indices, &shader.program, &u, &params)
                .expect("Failed to draw!");
        }
        None => {
            let final_uniforms = uniform! {
                transform: transform_u,
                projection: projection_u
            };

            frame.draw(&state.cube_vertices, &state.cube_indices, &shader.program, &final_uniforms, &params)
                .expect("Failed to draw!");
        }
    }


}

// FIXME: this is not working
fn draw_line_world(a: &Vec3, b: &Vec3, color: &Vec4, width: f32, cheap: bool, frame: &mut glium::Frame, state: &State) {
    let a_4 : Vec4 = [a[0], a[1], a[2], 1.0];
    let b_4 : Vec4 = [b[0], b[1], b[2], 1.0];

    let mut a_screen: Vec4 = vec4::create();
    let mut b_screen: Vec4 = vec4::create();
    vec4::transform_mat4(&mut a_screen, &a_4, &state.camera_projection_mat);
    vec4::transform_mat4(&mut b_screen, &b_4, &state.camera_projection_mat);

    a_screen = util::divide_by_w(a_screen);
    b_screen = util::divide_by_w(b_screen);

    a_screen[0] *= 0.5;
    a_screen[1] *= 0.5;
    b_screen[0] *= 0.5;
    b_screen[1] *= 0.5;

    a_screen[0] += 0.5;
    a_screen[1] += 0.5;
    b_screen[0] += 0.5;
    b_screen[1] += 0.5;

    // Take screen ratio into account
    a_screen[0] = a_screen[0] * state.resolution.x as f32 / state.resolution.y as f32;
    b_screen[0] = b_screen[0] * state.resolution.x as f32 / state.resolution.y as f32;

    let position = [
        (a_screen[0] + b_screen[0]) / 2.0,
        (a_screen[1] + b_screen[1]) / 2.0,
        0.0
    ];

    let mut dist = (a_screen[0] - b_screen[0]) * (a_screen[0] - b_screen[0])
                 + (a_screen[1] - b_screen[1]) * (a_screen[1] - b_screen[1]);
    dist = dist.sqrt();

    let size = [
        dist + width,
        width
    ];

    let rotation;
    if (a_screen[0] - b_screen[0]).abs() > 0.001 { 
        rotation = ((a_screen[1] - b_screen[1]) / (a_screen[0] - b_screen[0])).atan();
    }
    else {
        rotation = PI / 2.0;
    }

    let ratio = (dist + width) / width;

    let uniforms = dynamic_uniform!{   
        color: color,
        ratio: &ratio, 
    };
    
    let shader = if cheap { assets::get_shader(&"line".to_string(), state)       } 
                 else     { assets::get_shader(&"cheap_line".to_string(), state) };

    draw_screen_billboard([position[0], position[1], 0.0], size, rotation, shader, Some(uniforms), frame, state);
}

fn apply_cube_transform(vec: &Vec3, state: &State) -> Vec3 {
    let mut res = vec4::create();
    vec4::transform_mat4(&mut res, &[vec[0], vec[1], vec[2], 1.0], &state.cube_transform_matrix);
    return [res[0], res[1], res[2]];
}
