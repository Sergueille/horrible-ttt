
use crate::*;
use crate::game;
use crate::time;
use crate::util::*;
use gl_matrix::common::Mat4;
use gl_matrix::common::Vec2;
use std::collections::binary_heap;

use crate::Vertex;

pub struct State<'a> {
    // Engine
    pub time: time::Time,
    pub resolution: Vec2u,
    pub quad_vertices: glium::VertexBuffer<Vertex>,
    pub quad_indices: glium::IndexBuffer<u8>,
    pub cube_vertices: glium::VertexBuffer<Vertex>,
    pub cube_indices: glium::IndexBuffer<u8>,
    pub camera_projection_mat: Mat4,
    pub assets: crate::assets::AssetBase,
    pub display: glium::Display<glium::glutin::surface::WindowSurface>,
    pub mouse_coords_pixels: Vec2i,
    pub mouse_coords_normalized: Vec2,
    pub mouse_delta_normalized: Vec2,
    pub mouse_ray: Vec3,
    pub last_main_time: f32,
    
    pub quad_params: glium::DrawParameters<'a>,
    pub cube_params: glium::DrawParameters<'a>,
    pub draw_queue: binary_heap::BinaryHeap<draw::DrawCommand<'a>>,

    pub input: input::Input,
    pub game: game::GameInfo,
}

