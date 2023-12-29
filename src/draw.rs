use crate::state::State;
use crate::util;
use crate::assets;
use gl_matrix::common::*;
use gl_matrix::vec4;
use gl_matrix::mat4;
use glium::Surface;

const PARAM_COUNT: usize = 8;

pub enum DrawType {
    Quad, Cube
}

pub struct DrawCommand<'a> {
    pub z_pos: f32,

    pub shader: &'a str,
    pub transform: Mat4,
    pub apply_projection: bool,
    pub draw_type: DrawType,

    pub params: [f32; PARAM_COUNT],
    pub tex: TexArg<'a>,
}

// REFACT: why so many traits?
impl<'a> Ord for DrawCommand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).expect("");
    }
}

impl<'a> PartialOrd for DrawCommand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.z_pos > other.z_pos {
            return Some(std::cmp::Ordering::Greater);
        }
        else {
            return Some(std::cmp::Ordering::Less);
        }
    }
}

impl<'a> Eq for DrawCommand<'a> { }
impl<'a> PartialEq for DrawCommand<'a> {
    fn eq(&self, _: &Self) -> bool {
        return false;
    }
}

pub enum TexArg<'a> {
    None,
    One(&'a str)
}

// OPTI: group draw calls somehow
// position:    center of the quad, (0, 0) is bottom left, (1, wh) is the top right, last coordinate is z
// size:        size of the quad, 1 is screen height
pub fn draw_screen_billboard<'a, A>(position: Vec3, size: Vec2, rotation: f32, params: A, tex: TexArg<'a>, shader: &'a str, state: &mut State<'a>) where A: Iterator<Item = f32>{
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

    let mut params_arr: [f32; PARAM_COUNT] = [0.0; PARAM_COUNT];
    for (i, val) in params.enumerate() {
        params_arr[i] = val;
    }

    state.draw_queue.push(DrawCommand {
        z_pos: position[2],
        shader,
        params: params_arr,
        draw_type: DrawType::Quad,
        transform: *transform,
        apply_projection: false,
        tex,
    });
}

pub fn draw_line_world<'a>(a: &Vec3, b: &Vec3, color: Vec4, width: f32, cheap: bool, state: &mut State<'a>) {
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
        (a_screen[2] + b_screen[2]) / 2.0,
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

    let ratio = (dist + width) / width; // TODO: color2
    
    let shader = if cheap { "cheap_line" } 
                 else     { "line"       };

    draw_screen_billboard(position, size, rotation, [color[0], color[1], color[2], color[3], ratio].into_iter(), TexArg::None, shader, state);
}

pub fn draw_world_billboard<'a>(position: Vec3, size: Vec2, rotation: f32, color: Vec4, tex: TexArg<'a>, shader: &'a str, state: &mut State<'a>) {
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

    draw_screen_billboard(util::vec4_to_3(&screen_position), scale_screen, rotation, color.into_iter(), tex, shader, state);
}

pub fn draw_cube<'a>(transform: Mat4, color: Vec4, shader: &'a str, state: &mut State<'a>) {
    // Get z coordinate
    let mut center = [0.0, 0.0, 0.0, 1.0];
    let mut tmp = [0.0, 0.0, 0.0, 1.0];
    vec4::transform_mat4(&mut tmp, &center, &transform);
    vec4::transform_mat4(&mut center, &tmp.clone(), &state.camera_projection_mat);

    let mut params: [f32; PARAM_COUNT] = [0.0; PARAM_COUNT];
    for i in 0..4 {
        params[i] = color[i];
    }

    state.draw_queue.push(DrawCommand {
        z_pos: center[2] - 5.0, // TODO: add shift depending on the size of the cube
        shader,
        transform: transform,
        apply_projection: true,
        draw_type: DrawType::Cube,
        params,
        tex: TexArg::None
    });
}

pub fn draw_immediate(command: DrawCommand<'_>, frame: &mut glium::Frame, state: &mut State) {
    let projection;
    if command.apply_projection { 
        projection = util::mat_to_uniform(&state.camera_projection_mat); 
    } 
    else { 
        projection = util::identity_uniform();
    }

    let transform = util::mat_to_uniform(&command.transform);

    let mut color: [f32; 4] = [0.0; 4];
    let mut color2: [f32; 4] = [0.0; 4];
    for i in 0..4 {
        color[i] = command.params[i];
        color2[i] = command.params[i + 4];
    }

    let mut uniforms = dynamic_uniform! {
        transform: &transform,
        projection: &projection,
        color: &color,
        color2: &color2,
    };  

    // OPTI: crate non-dynamic uniforms somehow
    match command.tex {
        TexArg::None => { },
        TexArg::One(t1) => {
            uniforms.add("tex", &assets::get_texture(t1, &state.assets).texture);
        }
    }

    let shader = assets::get_shader(command.shader, &state.assets);

    match command.draw_type {
        DrawType::Quad => {
            frame.draw(&state.quad_vertices, &state.quad_indices, &shader.program, &uniforms, &state.quad_params)
                .expect("Failed to draw!");
        }
        DrawType::Cube => {
            frame.draw(&state.cube_vertices, &state.cube_indices, &shader.program, &uniforms, &state.cube_params)
                .expect("Failed to draw!");
        }
    }
}

pub fn draw_all(frame: &mut glium::Frame, state: &mut State) {
    loop {
        let next = state.draw_queue.pop();
        match next {
            Some(command) => {
                draw_immediate(command, frame, state);
            },
            None => break, // TEST: potential crash here
        }
    }
}

