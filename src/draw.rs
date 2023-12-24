use crate::state::State;
use crate::util;
use crate::assets;
use crate::shader;
use gl_matrix::common::*;
use gl_matrix::vec4;
use gl_matrix::mat4;
use glium::Surface;
use glium::uniforms;


// OPTI: group draw calls somehow
// position:    center of the quad, (0, 0) is bottom left, (1, wh) is the top right, last coordinate is z
// size:        size of the quad, 1 is screen height
pub fn draw_screen_billboard<'a, 'b>(position: Vec3, size: Vec2, rotation: f32, shader: &shader::Shader, uniforms: Option<uniforms::DynamicUniforms<'a, 'b>>, frame: &mut glium::Frame, state: &State) {
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

    match uniforms.to_owned() {
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

pub fn draw_line_world(a: &Vec3, b: &Vec3, color: &Vec4, width: f32, cheap: bool, frame: &mut glium::Frame, state: &State) {
    let mut a_screen: Vec4 = vec4::create();
    let mut b_screen: Vec4 = vec4::create();
    vec4::transform_mat4(&mut a_screen, &util::vec3_to_4(a), &state.camera_projection_mat);
    vec4::transform_mat4(&mut b_screen, &util::vec3_to_4(b), &state.camera_projection_mat);

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
    
    let shader = if cheap { assets::get_shader(&"line".to_string(), &state.assets)       } 
                 else     { assets::get_shader(&"cheap_line".to_string(), &state.assets) };

    draw_screen_billboard([position[0], position[1], 0.0], size, rotation, shader, Some(uniforms), frame, state);
}

pub fn draw_world_billboard<'a, 'b>(position: Vec3, size: Vec2,  rotation: f32, shader: &shader::Shader, uniforms: Option<uniforms::DynamicUniforms<'a, 'b>>, frame: &mut glium::Frame, state: &State) {
    let mut screen_position = vec4::create();
    vec4::transform_mat4(&mut screen_position, &util::vec3_to_4(&position), &state.camera_projection_mat);

    screen_position = util::divide_by_w(screen_position);

    screen_position[0] *= 0.5;
    screen_position[1] *= 0.5;

    screen_position[0] += 0.5;
    screen_position[1] += 0.5;

    // Take screen ratio into account
    screen_position[0] = screen_position[0] * state.resolution.x as f32 / state.resolution.y as f32;

    // Make smaller when distant
    let len = screen_position[2];
    let scale_screen = [size[0] / len, size[1] / len];

    screen_position[2] = 0.0;

    draw_screen_billboard(util::vec4_to_3(&screen_position), scale_screen, rotation, shader, uniforms, frame, state)
}
