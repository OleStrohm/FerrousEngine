use super::*;
use crate::resources::Resources;

pub struct Triangle {
    program: Program,
    _vbo: ArrayBuffer,
    vao: VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources) -> Result<Triangle, Error> {
        let program = Program::from_res(res, "shaders/triangle")?;

        let vertices: Vec<Vertex> = vec![
            Vertex::new((-0.5, -0.5, 0.0).into(), (1.0, 0.0, 0.0).into()),
            Vertex::new((0.5, -0.5, 0.0).into(), (0.0, 1.0, 0.0).into()),
            Vertex::new((0.0, 0.5, 0.0).into(), (0.0, 0.0, 1.0).into()),
        ];

        let vbo = ArrayBuffer::new();
        vbo.bind();
        vbo.buffer_static_data(&vertices);

        let vao = VertexArray::new();
        vao.bind();
        Vertex::vertex_attrib_pointers();
        vbo.unbind();
        vao.unbind();

        Ok(Triangle {
            program,
            _vbo: vbo,
            vao,
        })
    }

    pub fn render(&self) {
        self.program.bind();
        self.vao.bind();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
