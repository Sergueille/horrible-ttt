
use crate::*;
use crate::game;
use crate::time;
use crate::shader;
use crate::util::*;
use gl_matrix::common::Mat4;
use gl_matrix::common::Vec2;

use crate::Vertex;

pub struct State {
    // Engine
    pub time: time::Time,
    pub shaders: Vec<shader::Shader>,
    pub resolution: Vec2u,
    pub quad_vertices: glium::VertexBuffer<Vertex>,
    pub quad_indices: glium::IndexBuffer<u8>,
    pub cube_vertices: glium::VertexBuffer<Vertex>,
    pub cube_indices: glium::IndexBuffer<u8>,
    pub camera_projection_mat: Mat4,
    pub assets: crate::assets::AssetBase,
    pub display: glium::Display<glium::glutin::surface::WindowSurface>,
    pub mouse_coords_pixels: Vec2u,
    pub mouse_coords_normalized: Vec2,
    pub mouse_delta_normalized: Vec2,
    pub last_main_time: f32,

    pub lmb: crate::input::ButtonInfo,
    pub rmb: crate::input::ButtonInfo,
    pub mmb: crate::input::ButtonInfo,

    // Game
    pub cube_transform_matrix: Mat4,
    pub cube_rotation: Quat,
    pub cube_size: f32,
    pub blocks: [game::BlockType; (ROW_COUNT * ROW_COUNT * ROW_COUNT) as usize],
    pub cube_rotation_velocity: Quat,

    // TODO: should also retain the sphere radius
    pub last_mouse_sphere_intersection: Option<Vec3>,
}

