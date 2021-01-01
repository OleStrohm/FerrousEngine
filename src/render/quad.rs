use super::*;
use crate::resources::Resources;

pub struct Quad {
    program: Program,
    _vbo: VertexBufferObject,
    vao: VertexArray,
    ibo: IndexBufferObject,
}

impl Quad {
    pub fn new(res: &Resources) -> Result<Quad, Error> {
        let program = Program::from_res(res, "shaders/triangle")?;

        let vertices: Vec<Vertex> = vec![
            Vertex::new((-0.5,  0.5, 0.0).into(), (1.0, 0.0, 1.0).into()),
            Vertex::new((-0.5, -0.5, 0.0).into(), (1.0, 0.0, 0.0).into()),
            Vertex::new(( 0.5, -0.5, 0.0).into(), (0.0, 1.0, 0.0).into()),
            Vertex::new(( 0.5,  0.5, 0.0).into(), (0.0, 0.0, 1.0).into()),
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2,
            0, 2, 3,
        ];

        let vbo = ArrayBuffer::new();
        vbo.bind();
        vbo.buffer_static_data(&vertices);

        let vao = VertexArray::new();
        vao.bind();
        Vertex::vertex_attrib_pointers();
        vbo.unbind();
        vao.unbind();

        let ibo = ElementArrayBuffer::new();
        ibo.bind();
        ibo.buffer_static_data(&indices);

        Ok(Quad {
            program,
            _vbo: vbo,
            vao,
            ibo,
        })
    }

    pub fn render(&self) {
        self.program.bind();
        self.ibo.bind();
        self.vao.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);
        }
    }
}

