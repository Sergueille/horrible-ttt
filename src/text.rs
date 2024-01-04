
use crate::state::State;
extern crate serde;
use serde::Deserialize;

pub struct FontInfo {
    pub chars: Vec<GlyphInfo>,
    pub line_height: f32,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct Test {
    pub char: char,
}

pub fn empty_font_info() -> FontInfo {
    return FontInfo {
        chars: vec![],
        line_height: 0.0, // TODO: not handled by text module
    }
}

pub fn init_text(state: &mut State) {
    println!("LOG: Loading font assets.");

    // Load atlas texture
    crate::texture::create_to_assets("../font_atlas.png", state);

    // Load data
    state.font_info.chars = Vec::new();
    let mut reader = csv::ReaderBuilder::new().delimiter(b',').escape(Some(b'\\')).has_headers(true).from_path("./assets/font_info.csv").unwrap();
    for info_maybe in reader.deserialize() {
        let info: Test = info_maybe.unwrap();
        //state.font_info.chars.push(info);
    }

    state.font_info.chars.shrink_to_fit();
}
