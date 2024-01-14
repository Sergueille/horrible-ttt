#![allow(dead_code)]

use gl_matrix::{vec3, quat};

use crate::gl_matrix::common::*;
use crate::Vertex;
use crate::state::State;

#[derive(Clone)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
} 

#[derive(Clone, PartialEq)]
pub struct Vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
} 

#[derive(Clone)]
pub struct Vec2u {
    pub x: u32,
    pub y: u32,
} 

pub fn vec2i(x: i32, y: i32) -> Vec2i {
    return Vec2i { x, y };
}

pub fn vec2u(x: u32, y: u32) -> Vec2u {
    return Vec2u { x, y };
}

pub fn vec3i(x: i32, y: i32, z: i32) -> Vec3i {
    return Vec3i { x, y, z };
}

pub fn vec3i_arr(coords: [i32; 3]) -> Vec3i {
    return Vec3i { x: coords[0], y: coords[1], z: coords[2] };
}

pub fn vec3i_to_arr(vec: &Vec3i) -> [i32; 3] {
    return [vec.x, vec.y, vec.z];
}

pub fn mat_to_uniform(mat: &Mat4) -> [[f32; 4]; 4] {
    return [
        [mat[0], mat[1], mat[2], mat[3]],
        [mat[4], mat[5], mat[6], mat[7]],
        [mat[8], mat[9], mat[10], mat[11]],
        [mat[12], mat[13], mat[14], mat[15]],
    ]
}

/*
pub fn mat_to_uniform(mat: Mat4) -> [[f32; 4]; 4] {
    return [
        [mat[0], mat[4], mat[8], mat[12]],
        [mat[1], mat[5], mat[9], mat[13]],
        [mat[2], mat[6], mat[10], mat[14]],
        [mat[3], mat[7], mat[11], mat[15]],
    ]
}
*/

pub fn print_mat(mat: Mat4) {
    for i in 0..4 {
        for j in 0..4 {
            print!("{} ", mat[i * 4 + j]);
        }
        println!("");
    }
}

pub fn print_vec4(vec: Vec4) {
    for i in 0..4 {
        print!("{} ", vec[i]);
    }
    println!("");
}

pub fn print_vec3(vec: Vec3) {
    for i in 0..3 {
        print!("{} ", vec[i]);
    }
    println!("");
}

pub fn print_vec2(vec: Vec2) {
    println!("{} {}", vec[0], vec[1]);
}

pub fn divide_by_w(vec: Vec4) -> Vec4 {
    return [
        vec[0] / vec[3],
        vec[1] / vec[3],
        vec[2] / vec[3],
        1.0,
    ];
}

pub const QUAD_VERTICES: [Vertex; 4] = [
    Vertex { pos: [-0.5, -0.5, 0.0],    uv: [0.0, 0.0] },
    Vertex { pos: [0.5, -0.5, 0.0],     uv: [1.0, 0.0] },
    Vertex { pos: [0.5, 0.5, 0.0],      uv: [1.0, 1.0] },
    Vertex { pos: [-0.5, 0.5, 0.0],     uv: [0.0, 1.0] },
];

pub const QUAD_INDICES: [u8; 6] = [
    0, 1, 2, 0, 2, 3,
];


pub const CUBE_VERTICES: [Vertex; 24] = [
    Vertex { pos: [-0.5, -0.5, 0.5], uv: [0.0, 0.0] },
    Vertex { pos: [-0.5, 0.5, 0.5], uv: [0.0, 1.0] },
    Vertex { pos: [-0.5, 0.5, -0.5], uv: [1.0, 1.0] },
    Vertex { pos: [-0.5, -0.5, -0.5], uv: [1.0, 0.0] },
    Vertex { pos: [-0.5, -0.5, -0.5], uv: [0.0, 0.0] },
    Vertex { pos: [-0.5, 0.5, -0.5], uv: [0.0, 1.0] },
    Vertex { pos: [0.5, 0.5, -0.5], uv: [1.0, 1.0] },
    Vertex { pos: [0.5, -0.5, -0.5], uv: [1.0, 0.0] },
    Vertex { pos: [0.5, -0.5, -0.5], uv: [1.0, 0.0] },
    Vertex { pos: [0.5, 0.5, -0.5], uv: [1.0, 1.0] },
    Vertex { pos: [0.5, 0.5, 0.5], uv: [0.0, 1.0] },
    Vertex { pos: [0.5, -0.5, 0.5], uv: [0.0, 0.0] },
    Vertex { pos: [0.5, -0.5, 0.5], uv: [1.0, 0.0] },
    Vertex { pos: [0.5, 0.5, 0.5], uv: [1.0, 1.0] },
    Vertex { pos: [-0.5, 0.5, 0.5], uv: [0.0, 1.0] },
    Vertex { pos: [-0.5, -0.5, 0.5], uv: [0.0, 0.0] },
    Vertex { pos: [-0.5, -0.5, -0.5], uv: [0.0, 1.0] },
    Vertex { pos: [0.5, -0.5, -0.5], uv: [1.0, 1.0] },
    Vertex { pos: [0.5, -0.5, 0.5], uv: [1.0, 0.0] },
    Vertex { pos: [-0.5, -0.5, 0.5], uv: [0.0, 0.0] },
    Vertex { pos: [0.5, 0.5, -0.5], uv: [1.0, 1.0] },
    Vertex { pos: [-0.5, 0.5, -0.5], uv: [0.0, 1.0] },
    Vertex { pos: [-0.5, 0.5, 0.5], uv: [0.0, 0.0] },
    Vertex { pos: [0.5, 0.5, 0.5], uv: [1.0, 0.0] },
];

pub const CUBE_INDICES: [u8; 36] = [
    3, 0, 1, 1, 2, 3, 7, 4, 5, 5, 6, 7, 11, 8, 9, 9, 10, 11, 15, 12, 13, 13, 14, 15, 19, 16, 17, 17, 18, 19, 23, 20, 21, 21, 22, 23
];

pub fn vec3_to_4(vec: &Vec3) -> Vec4 {
    return [vec[0], vec[1], vec[2], 1.0];
}

pub fn vec4_to_3(vec: &Vec4) -> Vec3 {
    return [vec[0], vec[1], vec[2]];
}

// NOTE: assumes that the camera is facing negative z axis
pub fn get_mouse_ray(state: &State) -> Vec3 {
    let ratio = state.resolution.x as f32 / state.resolution.y as f32;
    let tan = (crate::FOV / 2.0).tan();

    let direction = [
        (state.mouse_coords_normalized[0] - 0.5 * ratio) * tan,
        (state.mouse_coords_normalized[1] - 0.5) * tan,
        -0.5
    ];

    let mut res = vec3::create();
    vec3::normalize(&mut res, &direction);
    return res;
}

// Gets the intersection point between:
// - the sphere of center O and radius R 
// - the line that goes through A, with vector V
// returns the nearest point to A 
pub fn intersect_line_sphere(o: &Vec3, r: f32, a: &Vec3, v: &Vec3) -> Option<Vec3> {
    let mut c = vec3::create();
    vec3::sub(&mut c, &a, &o);

    let dot = vec3::dot(&v, &c);
    let vv = vec3::sqr_len(&v);
    let cc = vec3::sqr_len(&c);

    let delta = dot * dot - vv * (cc - r * r);
    if delta < 0.0 {
        return None;
    } 

    let sqrt_delta = delta.sqrt();
    let t1 = (-dot + sqrt_delta) / vv;
    let t2 = (-dot - sqrt_delta) / vv;
    let t = if t1 > t2 { t2 } else { t1 };

    let mut res = vec3::create();
    vec3::scale_and_add(&mut res, &a, &v, t);
    return Some(res);
}

// Same as above, but if not intersecting, returns the nearest point on sphere
pub fn intersect_line_sphere_always(o: &Vec3, r: f32, a: &Vec3, v: &Vec3) -> Vec3 {
    let mut c = vec3::create();
    vec3::sub(&mut c, &a, &o);

    let dot = vec3::dot(&v, &c);
    let vv = vec3::sqr_len(&v);
    let cc = vec3::sqr_len(&c);

    let mut delta = dot * dot - vv * (cc - r * r);
    if delta < 0.0 {
        delta = 0.0;
    } 

    let sqrt_delta = delta.sqrt();
    let t1 = (-dot + sqrt_delta) / vv;
    let t2 = (-dot - sqrt_delta) / vv;
    let t = if t1 > t2 { t2 } else { t1 };

    let mut res = vec3::create();
    vec3::scale_and_add(&mut res, &a, &v, t);
    return res;
}

// Gets the intersection point between:
// - the plane that goes through P, with normal N
// - the line that goes through A, with vector V
pub fn intersect_line_plane(p: &Vec3, n: &Vec3, a: &Vec3, v: &Vec3) -> Vec3 {
    let mut diff = vec3::create();
    vec3::sub(&mut diff, &a, &p);
    
    let t = -vec3::dot(&n, &diff) / vec3::dot(&v, &n);

    let mut res = vec3::create();
    vec3::scale_and_add(&mut res, &a, &v, t);
    return res;
}

// Gets the coordinates of A, on th plane with origin O tangent vectors t1 and t2
pub fn get_plane_coords(o: &Vec3, t1: &Vec3, t2: &Vec3, a: &Vec3) -> Vec2 {
    let mut sub = vec3::create();
    vec3::sub(&mut sub, a, o);

    return [
        vec3::dot(&sub, &t1),
        vec3::dot(&sub, &t2),
    ]
}

pub fn multiply_quat(quat: &Quat, scalar: f32) -> Quat {
    let mut id = quat::create();
    let mut res = quat::create();
    quat::identity(&mut id);
    quat::lerp(&mut res, &id, &quat, scalar); 
    return res;
}

pub fn is_pointing_towards_camera(center: &Vec3, normal: &Vec3) -> bool {
    return vec3::dot(&center, &normal) < 0.0;
}

pub fn identity_uniform() -> [[f32; 4]; 4] {
    return [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}
