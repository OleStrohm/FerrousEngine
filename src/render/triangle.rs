use crate::resources::Resources;
use super::*;

pub struct Triangle {
    program: Program,
    _vbo: ArrayBuffer,
    vao: VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources) -> Result<Triangle, Error> {


        unimplemented!();
    }

    pub fn render(&self) {
        unimplemented!("Triangle rendering is not implemented");
    }
}
