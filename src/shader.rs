use std::fs;
use crate::state;

macro_rules! define_program {
    ($name_s:expr, $frag_s:expr, $vtex_s:expr) => {
        ShaderInfo { name: $name_s, frag_filename: concat!("./shaders/", $frag_s, ".glsl"), vtex_filename: concat!("./shaders/", $vtex_s, ".glsl") }
    };
}

#[derive(Debug)]
pub struct ShaderInfo<'a> {
    pub name: &'a str,
    pub frag_filename: &'a str,
    pub vtex_filename: &'a str,
}

pub struct Shader<'a> {
    pub info: ShaderInfo<'a>,
    pub program: glium::Program
}

pub fn create_shaders(display: &glium::Display<glium::glutin::surface::WindowSurface>, state: &mut state::State) {
    let shader_infos = vec![
        define_program!("test", "frag", "vtex"),
        define_program!("test2", "frag2", "vtex"),
    ];

    state.shaders = Vec::with_capacity(shader_infos.len());

    for info in shader_infos.into_iter() {
        println!("{}", info.vtex_filename);
        let vtex_content = fs::read_to_string(info.vtex_filename).expect("Failed to read the shader source file");
        let frag_content = fs::read_to_string(info.frag_filename).expect("Failed to read the shader source file");

        state.shaders.push(Shader {
            info,
            program: glium::Program::from_source(display, &vtex_content, &frag_content, None).unwrap(),
        });
    }
}

// OPTI: this is way too slow, should not iterate and compare strings every time
//       maybe I should use a macro to replace string with ids
pub fn get_shader<'a>(name: &str, state: &'a state::State<'a>) -> &'a Shader<'a> {
    for shader in &state.shaders {
        if shader.info.name == name {
            return &shader;
        }
    }

    println!("ERR: could not find a shader with the name {}. Replaced with default one.", name);
    return &state.shaders[0];
}
