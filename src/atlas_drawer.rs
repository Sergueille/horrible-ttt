
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
pub const PIXEL_SIZE: u32 = 16;

pub unsafe fn load_font(state: &mut State) {
    uninit!(lib, FT_Library);
    uninit!(face, FT_Face);

    let error = FT_Init_FreeType(&mut lib);
    check_err!(error);

    state.freetype = Some(lib);

    let error = FT_New_Face(state.freetype.expect("Uuh?"), b"./assets/arial.ttf\0".as_ptr() as *const i8, 0, &mut face);
    check_err!(error);

    FT_Set_Pixel_Sizes(face, PIXEL_SIZE, PIXEL_SIZE);

    state.font_info.chars = Vec::with_capacity(CHARS_TO_LOAD.len());

    state.font_info.line_height = ((*face).ascender - (*face).descender) as f32 / 64.0;

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
    
        let tex = crate::texture::from_grayscale_buffer(buffer_vec.clone(), size_x, size_y, &tex_name,  &state);
        state.assets.insert(tex_name, crate::assets::Asset::Texture(tex));

        // image::save_buffer_with_format(format!("./assets/font/glyph_{}", char), &buffer_vec, size_x, size_y, image::ColorType::L8, image::ImageFormat::Png).unwrap();
        
        state.font_info.chars.push(text::GlyphInfo {
            char,
            x_min: glyph.bitmap_left as f32,
            x_max: glyph.bitmap_left as f32 + glyph.bitmap.width as f32,
            y_min: glyph.bitmap_top as f32 - glyph.bitmap.rows as f32,
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

    if print_coords {
        println!("char,\tx_min,\tx_max,\ty_min,\ty_max,\tadvance,\tu_min,\tu_max,\tv_min,\tv_max,");
    }

    for i in 0..state.font_info.chars.len() {
        let char = &state.font_info.chars[i];

        if pos[0] + char.x_max * scale > ratio - margin * scale {
            pos[0] = margin * scale;
            pos[1] += state.font_info.line_height * scale;
        }

        pos[0] -= char.x_min * scale;

        let rect_pos = [
            pos[0] + (char.x_max + char.x_min) / 2.0 * scale,
            pos[1] + (char.y_max + char.y_min) / 2.0 * scale,
            -1.0,
        ];

        let rect_size = [
            (char.x_max - char.x_min) * scale,
            (char.y_max - char.y_min) * scale,
        ];

        if print_coords {
            println!("\"{}\", {}, {}, {}, {}, {}, {}, {}, {}, {},", 
                if char.char == '"' { "\\\"".to_string() } else { char.char.to_string() }, // FIXME: escape backslash
                char.x_min,
                char.x_max,
                char.y_min,
                char.y_max,
                char.advance,
                (pos[0] + char.x_min * scale) / ratio, 
                (pos[0] + char.x_max * scale) / ratio,
                pos[1] + char.y_min * scale, 
                pos[1] + char.y_max * scale, 
            );
        }

        let tex_name = format!("glyph_{}", char.char);
        crate::draw::draw_screen_billboard(rect_pos, rect_size, 0.0, [1.0, 1.0, 1.0, 1.0].into_iter(), crate::draw::TexArg::One(tex_name), "default_tex", state);

        pos[0] += state.font_info.chars[i].x_max * scale;
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
        let data_vec: Vec<u8> = data_vec.into_iter().step_by(4).collect();

        image::save_buffer_with_format("./assets/font_atlas.png", &data_vec, state.resolution.x, state.resolution.y, image::ColorType::L8, image::ImageFormat::Png).unwrap();
    }
    
}
