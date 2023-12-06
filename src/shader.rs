use std::fs;
use crate::state;

macro_rules! define_program {
    ($name_s:expr, $frag_s:expr, $vtex_s:expr) => {
        Shader { name: $name_s, frag_filename: concat!("./shaders/", $frag_s, ".glsl"), vtex_filename: concat!("./shaders/", $vtex_s, ".glsl"), program: None }
    };
}

#[derive(Debug)]
pub struct Shader<'a> {
    pub name: &'a str,
    pub frag_filename: &'a str,
    pub vtex_filename: &'a str,
    pub program: Option<glium::Program>
}

pub fn create_shaders(display: &glium::Display<glium::glutin::surface::WindowSurface>, state: &mut state::State) {
    state.shaders = vec![
        define_program!("test", "frag", "vtex"),
    ];

    for prog in &mut state.shaders {
        println!("{}", prog.vtex_filename);
        let vtex_content = fs::read_to_string(prog.vtex_filename).expect("Failed to read the shader source file");
        let frag_content = fs::read_to_string(prog.frag_filename).expect("Failed to read the shader source file");

        prog.program = Some(glium::Program::from_source(display, &vtex_content, &frag_content, None).unwrap());
    }
}

// OPTI: this is way too slow, should not iterate and compare strings every time
//       maybe I should use a macro to replace string with ids
pub fn get_shader<'a>(name: &str, state: &'a state::State<'a>) -> &'a Shader<'a> {
    for shader in &state.shaders {
        if shader.name == name {
            return &shader;
        }
    }

    println!("ERR: could not find a shader with the name {}. Replaced with default one.", name);
    return &state.shaders[0];
}
