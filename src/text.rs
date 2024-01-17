
use crate::state::State;
extern crate serde;
use gl_matrix::common::*;
use glium::Surface;
use serde::Deserialize;
use std::collections::HashMap;

pub const MAX_GLYPH_COUNT: usize = 5000;
pub const ADVANCE_MULTIPLIER: f32 =  0.015;
pub const LINE_HEIGHT: f32 =  1.0;

pub struct TextData {
    pub chars: HashMap<char, GlyphInfo>,
    pub line_height: f32,
    pub texts: Vec<TextInfo>,
    pub gpu_buffer: glium::vertex::VertexBuffer<GlyphAttr>,
    pub space_size: f32,
}

#[derive(Deserialize, Clone)]
pub struct GlyphInfo {
    pub char: char,
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub advance: f32,
    pub u_min: f32,
    pub u_max: f32,
    pub v_min: f32,
    pub v_max: f32,
}

pub struct TextInfo {
    pub content: String, 
    pub position: Vec2, 
    pub size: f32, 
    pub color: Vec4
}

#[derive(Clone, Copy)]
pub struct GlyphAttr {
    pub pos_attr: Vec4, // Center (xy), size (xy)
    pub uv_attr: Vec4, // Center (xy), size (xy)
    pub color_attr: Vec4,
}
implement_vertex!(GlyphAttr, pos_attr, uv_attr, color_attr);

pub fn empty_text_data(display: &glium::Display<glium::glutin::surface::WindowSurface>) -> TextData {
    return TextData {
        chars: HashMap::new(),
        line_height: 0.0, // TODO: not handled by text module
        texts: Vec::with_capacity(20),
        gpu_buffer: glium::vertex::VertexBuffer::empty(display, MAX_GLYPH_COUNT).unwrap(),
        space_size: 0.0
    }
}

pub fn init_text(state: &mut State) {
    println!("LOG: Loading font assets.");

    // Load atlas texture
    let tex = crate::texture::create("../font_atlas.png", state);
    state.assets.insert("font_atlas".to_string(), crate::assets::Asset::Texture(tex));

    // Load data
    state.text_data.chars.drain();
    let mut reader = csv::ReaderBuilder::new().escape(Some(b'\\')).has_headers(true).from_path("./assets/font_info.csv").unwrap();
    for info_maybe in reader.deserialize() {
        let info: GlyphInfo = info_maybe.unwrap();
        state.text_data.chars.insert(info.char, info);
    }

    // Advance of space is the same as 'l'
    state.text_data.space_size = state.text_data.chars[&'l'].advance;

    state.text_data.chars.shrink_to_fit();
}

pub fn draw_text(content: &str, position: Vec2, size: f32, color: Vec4, state: &mut State) {
    state.text_data.texts.push(TextInfo {
        content: content.to_string(),
        position,
        size,
        color,
    });
}

pub fn get_text_width(content: &str, size: f32, state: &State) -> f32 {
    let mut res = 0.0;

    for char in content.chars() {
        if char == ' ' {
            res += state.text_data.space_size * size * ADVANCE_MULTIPLIER;
            continue;
        }

        let info = match state.text_data.chars.get(&char) {
            Some(info) => info,
            None => {
                println!("ERR: Attempted to display character `{}`, which is not present in the font atlas", char);
                state.text_data.chars.get(&'?').expect("ERR: This fucking font atlas doesn't event contain a question mark!")
            }
        };

        res += info.advance * size * ADVANCE_MULTIPLIER;
    }

    return res;
}

pub fn draw_all_text(frame: &mut glium::Frame, state: &mut State) {
    // Create buffer
    let mut buffer: Vec<GlyphAttr> = Vec::with_capacity(MAX_GLYPH_COUNT); // Assume ~20 characters per text 
    for text in &state.text_data.texts {
        let ratio = state.resolution.y as f32 / state.resolution.x as f32;
        let mut cursor_pos = text.position;
        let actual_scale = text.size;

        for glyph in text.content.chars() {
            if glyph == ' ' {
                cursor_pos[0] += state.text_data.space_size * actual_scale * ADVANCE_MULTIPLIER;
                continue;
            }

            let glyph_info = match state.text_data.chars.get(&glyph) {
                Some(info) => info,
                None => {
                    println!("ERR: Attempted to display character `{}`, which is not present in the font atlas", glyph);
                    state.text_data.chars.get(&'?').expect("ERR: This fucking font atlas doesn't event contain a question mark!")
                }
            };

            if buffer.len() >= MAX_GLYPH_COUNT {
                panic!("Too much text on screen! Change text::MAX_GLYPH_COUNT")
            }

            buffer.push(GlyphAttr {
                pos_attr: [
                    (cursor_pos[0] + (glyph_info.x_min + glyph_info.x_max) / 2.0 * actual_scale) * ratio,
                    (cursor_pos[1] + (glyph_info.y_min + glyph_info.y_max) / 2.0 * actual_scale),
                    ((glyph_info.x_max - glyph_info.x_min) * actual_scale) * ratio,
                    (glyph_info.y_max - glyph_info.y_min) * actual_scale,
                ],
                uv_attr: [
                    (glyph_info.u_min + glyph_info.u_max) / 2.0,
                    (glyph_info.v_min + glyph_info.v_max) / 2.0,
                    (glyph_info.u_max - glyph_info.u_min),
                    (glyph_info.v_max - glyph_info.v_min),
                ],
                color_attr: text.color,
            });

            cursor_pos[0] += glyph_info.advance * actual_scale * ADVANCE_MULTIPLIER; // FIXME: why 0.001
        }
    }

    // TEST: fill buffer with trash
    for _ in buffer.len()..MAX_GLYPH_COUNT {
        buffer.push(GlyphAttr {
            pos_attr: [
                0.0, 0.0, 0.0, 0.0,
            ],
            uv_attr: [
                0.0, 0.0, 0.0, 0.0,
            ],
            color_attr: [0.0, 0.0, 0.0, 0.0],
        });
    }

    state.text_data.gpu_buffer.write(&buffer);

    let uniforms = uniform! {
        tex: &crate::assets::get_texture("font_atlas", &state.assets).texture,
    };

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::Overwrite,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    frame.draw(
        (&state.quad_vertices, state.text_data.gpu_buffer.per_instance().unwrap()), 
        &state.quad_indices, 
        &crate::assets::get_shader("text", &state.assets).program, 
        &uniforms, &params
    ).unwrap();

    state.text_data.texts.clear();
}
