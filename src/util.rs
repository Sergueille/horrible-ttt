use crate::gl_matrix::common::*;
use crate::Vertex;

pub struct Vec2i {
    pub x: i32,
    pub y: i32,
} 

pub struct Vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
} 

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

pub fn mat_to_uniform(mat: Mat4) -> [[f32; 4]; 4] {
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

pub fn divide_by_w(vec: Vec4) -> Vec4 {
    return [
        vec[0] / vec[3],
        vec[1] / vec[3],
        vec[2] / vec[3],
        vec[3],
    ]
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


pub const CUBE_VERTICES: [Vertex; 8] = [
    Vertex { pos: [-0.5, -0.5, 0.5],    uv: [0.0, 0.0] },
    Vertex { pos: [0.5, -0.5, 0.5],     uv: [1.0, 0.0] },
    Vertex { pos: [0.5, 0.5, 0.5],      uv: [1.0, 1.0] },
    Vertex { pos: [-0.5, 0.5, 0.5],     uv: [0.0, 1.0] },
    Vertex { pos: [-0.5, -0.5, -0.5],     uv: [0.0, 1.0] },
    Vertex { pos: [0.5, -0.5, -0.5],     uv: [1.0, 1.0] },
    Vertex { pos: [0.5, 0.5, -0.5],     uv: [1.0, 0.0] },
    Vertex { pos: [-0.5, 0.5, -0.5],     uv: [0.0, 0.0] },
];

pub const CUBE_INDICES: [u8; 36] = [
    0, 1, 2, 0, 2, 3,
    6, 5, 4, 7, 6, 4,
    6, 2, 1, 1, 5, 6,
    0, 3, 7, 7, 4, 0,
    5, 1, 0, 0, 4, 5,
    3, 2, 6, 6, 7, 3,
];



