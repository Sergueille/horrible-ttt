use std::fs;
use crate::state;
use crate::assets;

macro_rules! define_program {
    ($name_s:expr, $frag_s:expr, $vtex_s:expr) => {
        ShaderInfo { name: $name_s, frag_filename: concat!("./shaders/", $frag_s, ".glsl"), vtex_filename: concat!("./shaders/", $vtex_s, ".glsl") }
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
    let shader_infos = vec![
        define_program!("test", "frag", "vtex"),
        define_program!("test2", "textest", "vtex"),
        define_program!("line", "line_frag", "vtex"),
    ];

    state.shaders = Vec::with_capacity(shader_infos.len());

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
