
use crate::time;
use crate::shader;
use crate::util::*;
use gl_matrix::common::Mat4;
use gl_matrix::common::Vec2;

use crate::Vertex;

pub struct State {
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
}
