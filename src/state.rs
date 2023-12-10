
use crate::time;
use crate::shader;
use crate::util::*;

pub struct State<'a> {
    pub time: time::Time,
    pub shaders: Vec<shader::Shader<'a>>,
    pub resolution: Vec2u,
}
