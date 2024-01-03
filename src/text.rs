
#![allow(invalid_value)]

use crate::bindings::*;
use std::mem::MaybeUninit;
use crate::state::State;

macro_rules! uninit {
    ($name: ident, $t: tt) => {
        let mut $name: $t = MaybeUninit::uninit().assume_init();
    };
}

macro_rules! check_err {
    ($err: expr) => {
        if $err != 0 { println!("FreeType is not happy: {}", $err); panic!() };
    };
}

// TEST
pub unsafe fn create_glyph_textures(state: &mut State) {
    uninit!(lib, FT_Library);
    uninit!(face, FT_Face);

    let error = FT_Init_FreeType(&mut lib);
    check_err!(error);

    state.freetype = Some(lib);

    let error = FT_New_Face(state.freetype.expect("Uuh?"), b"./assets/arial.ttf\0".as_ptr() as *const i8, 0, &mut face);
    check_err!(error);

    let chars = "0123456789azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN,;:!?./%$*&\"'(-_)=+[]{}#~^\\`|<>";

    for char in chars.chars() {
        let error = FT_Load_Char(face, char as u32, 0 /* Default?? */);
        check_err!(error);
    
        let error = FT_Render_Glyph((*face).glyph, FT_Render_Mode__FT_RENDER_MODE_SDF);
        check_err!(error);
    
        let buffer = (*(*face).glyph).bitmap.buffer;
        let size_x = (*(*face).glyph).bitmap.width;
        let size_y = (*(*face).glyph).bitmap.rows;
        
        // Create a vec from raw memory zone
        // let buffer_vec = Vec::from_raw_parts(buffer, (size_x * size_y) as usize, (size_x * size_y) as usize);
    
        let pixel_count = size_x * size_y;
        let mut buffer_vec = Vec::<u8>::with_capacity(pixel_count as usize);
        buffer_vec.set_len(pixel_count as usize);
        for i in 0..pixel_count {
            buffer_vec[i as usize] = *buffer.add(i as usize);
        }

        // Put the texture upside down
        let buffer_vec = buffer_vec            
            .chunks(size_x as usize)
            .rev()
            .flat_map(|row| row.iter()).cloned()
            .collect::<Vec<u8>>();

        let tex_name = format!("glyph_{}", char);
    
        let tex = crate::texture::from_grayscale_buffer(buffer_vec.clone(), size_x, size_y, &tex_name,  &state);
        state.assets.insert(tex_name, crate::assets::Asset::Texture(tex));

        // std::mem::forget(buffer_vec); // Prevent rust from deallocating this value
        
        println!("Done {}", char);
    }

}

