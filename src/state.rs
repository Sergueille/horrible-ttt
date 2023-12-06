
use crate::time;
use crate::shader;

pub struct State<'a> {
    pub time: time::Time,
    pub shaders: Vec<shader::Shader<'a>>,
}
