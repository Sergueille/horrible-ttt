use std::fs;
use crate::state;
use crate::assets;

macro_rules! define_program {
    ($name_s:expr, $frag_s:expr, $vtex_s:expr) => {
        ShaderInfo { name: $name_s, frag_filename: concat!("./assets/shaders/", $frag_s, ".glsl"), vtex_filename: concat!("./assets/shaders/", $vtex_s, ".glsl") }
    };
}

#[derive(Debug)]
pub struct ShaderInfo {
    pub name: &'static str,
    pub frag_filename: &'static str,
    pub vtex_filename: &'static str,
}

pub struct Shader {
    pub info: ShaderInfo,
    pub program: glium::Program
}

pub fn create_shaders<'a>(state: &'a mut state::State) {
    println!("LOG: Loading shaders.");

    let shader_infos = vec![
        define_program!("default_tex", "texture", "vtex"),
        define_program!("default_color", "color", "vtex"),
        define_program!("line", "line_frag", "vtex"),
        define_program!("cheap_line", "cheap_line_frag", "vtex"),
        define_program!("text", "text_frag", "text_vtex"),
        define_program!("bg_cube", "bg_frag", "vtex"),
    ];

    for info in shader_infos.into_iter() {
        let vtex_content = fs::read_to_string(info.vtex_filename).expect("Failed to read the shader source file");
        let frag_content = fs::read_to_string(info.frag_filename).expect("Failed to read the shader source file");

        let name = info.name.to_string();

        let shader = Shader {
            info,
            program: glium::Program::from_source(&state.display, &vtex_content, &frag_content, None).unwrap(),
        };

        state.assets.insert(name, assets::Asset::Shader(shader));
    }
}
