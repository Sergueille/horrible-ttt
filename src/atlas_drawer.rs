
use crate::text;
use gl_matrix::common::*;

use crate::bindings::*;
use std::mem::MaybeUninit;
use crate::state::State;

macro_rules! uninit {
    ($name: ident, $t: tt) => {
        #[allow(invalid_value)]
        let mut $name: $t = MaybeUninit::uninit().assume_init();
    };
}

macro_rules! check_err {
    ($err: expr) => {
        if $err != 0 { println!("FreeType is not happy: {}", $err); panic!() };
    };
}

pub const CHARS_TO_LOAD: &str = "0123456789azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN,;:!?./%$*&\"'(-_)=+[]{}#~^\\`|<>éèàùâêîôûäëïöüÿ";
pub const PIXEL_SIZE: u32 = 32;

pub unsafe fn load_font(state: &mut State) {
    uninit!(lib, FT_Library);
    uninit!(face, FT_Face);

    let error = FT_Init_FreeType(&mut lib);
    check_err!(error);

    state.freetype = Some(lib);

    let error = FT_New_Face(state.freetype.expect("Uuh?"), b"./assets/arial.ttf\0".as_ptr() as *const i8, 0, &mut face);
    check_err!(error);

    FT_Set_Pixel_Sizes(face, PIXEL_SIZE, PIXEL_SIZE);

    state.text_data.chars = std::collections::HashMap::new();

    state.text_data.line_height = ((*face).ascender - (*face).descender) as f32 / PIXEL_SIZE as f32 * 0.6;

    // Characters to load
    for char in CHARS_TO_LOAD.chars() {
        let error = FT_Load_Char(face, char as u32, 0 /* Default?? */);
        check_err!(error);
    
        let error = FT_Render_Glyph((*face).glyph, FT_Render_Mode__FT_RENDER_MODE_SDF);
        check_err!(error);

        let glyph = &(*(*face).glyph);

        let buffer = glyph.bitmap.buffer;
        let size_x = glyph.bitmap.width;
        let size_y = glyph.bitmap.rows;
    
        // Copy raw memory buffer into a vec
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
    
        let tex = crate::texture::from_grayscale_buffer(buffer_vec.clone(), size_x, size_y, &tex_name, &state);
        state.assets.insert(tex_name, crate::assets::Asset::Texture(tex));
    
        state.text_data.chars.insert(char, text::GlyphInfo {
            char,
            x_min: glyph.bitmap_left as f32,
            x_max: (glyph.bitmap_left as f32 + glyph.bitmap.width as f32),
            y_min: (glyph.bitmap_top as f32 - glyph.bitmap.rows as f32),
            y_max: glyph.bitmap_top as f32,
            advance: glyph.metrics.horiAdvance as f32,
            u_min: 0.0,
            u_max: 0.0,
            v_min: 0.0,
            v_max: 0.0,
        });

        println!("Done {}", char);
    }
}

fn draw_atlas<T>(frame: &mut T, print_coords: bool, state: &mut State) where T: glium::Surface {
    // Clear buffer
    frame.clear_all((0.0, 0.0, 0.0, 1.0), 100000.0, 0);

    //const SCALE: f32 = 0.005;
    let scale: f32 = 1.0 / state.resolution.y as f32;
    let margin = 10.0;
    let mut pos: Vec2 = [margin * scale, margin * scale];

    let ratio = state.resolution.x as f32 / state.resolution.y as f32;

    let mut file_string = String::new();
    if print_coords {
        file_string.push_str("\"char\",\"x_min\",\"x_max\",\"y_min\",\"y_max\",\"advance\",\"u_min\",\"u_max\",\"v_min\",\"v_max\"\n");
    }

    // FIXME: the characters are different this time???
    for char in CHARS_TO_LOAD.chars() {
        let info = &state.text_data.chars[&char];

        if pos[0] + info.x_max * scale > ratio - margin * scale {
            pos[0] = margin * scale;
            pos[1] += state.text_data.line_height * scale;
        }

        pos[0] -= info.x_min * scale;

        let rect_pos = [
            pos[0] + (info.x_max + info.x_min) / 2.0 * scale,
            pos[1] + (info.y_max + info.y_min) / 2.0 * scale,
            -1.0,
        ];

        let rect_size = [
            (info.x_max - info.x_min) * scale,
            (info.y_max - info.y_min) * scale,
        ];

        if print_coords {
            // Escape delimiters and backslash
            let escaped_char = if info.char == '"' || info.char == '\\' {
                format!("\\{}", info.char)
            }
            else {
                info.char.to_string()
            };

            let line_text = format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n", // What a beautiful code
                escaped_char,
                info.x_min / PIXEL_SIZE as f32,
                info.x_max / PIXEL_SIZE as f32,
                info.y_min / PIXEL_SIZE as f32,
                info.y_max / PIXEL_SIZE as f32,
                info.advance / PIXEL_SIZE as f32,
                (pos[0] + info.x_min * scale) / ratio, 
                (pos[0] + info.x_max * scale) / ratio,
                pos[1] + info.y_min * scale, 
                pos[1] + info.y_max * scale, 
            );
            file_string.push_str(&line_text);
        }

        let tex_name = format!("glyph_{}", info.char);
        crate::draw::draw_screen_billboard(rect_pos, rect_size, 0.0, [1.0, 1.0, 1.0, 1.0].into_iter(), crate::draw::TexArg::One(tex_name), "default_tex", state);

        pos[0] += state.text_data.chars[&char].x_max * scale;
    }

    if print_coords {
        std::fs::write("./assets/font_info.csv", file_string).unwrap();
        println!("Exported atlas!");
    }

    crate::draw::draw_all(frame, state);
}

pub fn preview_atlas(state: &mut State) {
    let mut frame = state.display.draw();
    draw_atlas(&mut frame, false, state);
    frame.finish().expect("Uuh?");

    if state.input.rmb.down {
        // Export!
        let target_tex = glium::texture::SrgbTexture2d::empty_with_format(
            &state.display, 
            glium::texture::SrgbFormat::U8U8U8U8, 
            glium::texture::MipmapsOption::NoMipmap, 
            state.resolution.x, 
            state.resolution.y
        ).unwrap();

        let depth_tex = glium::texture::DepthTexture2d::empty_with_format(
            &state.display, 
            glium::texture::DepthFormat::I16, 
            glium::texture::MipmapsOption::NoMipmap, 
            state.resolution.x, 
            state.resolution.y
        ).unwrap();

        let mut framebuffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&state.display, &target_tex, &depth_tex).unwrap();
        draw_atlas(&mut framebuffer, true, state);
        let data = target_tex.read::<glium::texture::RawImage2d<u8>>().data;
        let data_vec = Vec::from(data);
        let data_vec: Vec<u8> = data_vec.into_iter()
            .step_by(4)
            .collect::<Vec<u8>>()
            .chunks(state.resolution.x as usize)
            .rev()
            .flat_map(|row| row.iter()).cloned()
            .collect();

        image::save_buffer_with_format("./assets/font_atlas.png", &data_vec, state.resolution.x, state.resolution.y, image::ColorType::L8, image::ImageFormat::Png).unwrap();
    }
    
}
