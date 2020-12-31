use gl::types::*;

pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;

pub trait BufferType {
    const BUFFER_TYPE: GLuint;
}

pub struct Buffer<B>
where
    B: BufferType,
{
    vbo: GLuint,
    _buffer_marker: ::std::marker::PhantomData<B>,
}

pub struct VertexArray {
    vao: GLuint,
}

pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: GLuint = gl::ARRAY_BUFFER;
}

pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

impl<B> Buffer<B>
where
    B: BufferType,
{
    pub fn new() -> Buffer<B> {
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        Buffer {
            vbo,
            _buffer_marker: ::std::marker::PhantomData,
        }
    }

    pub fn buffer_static_data<T>(&self, data: &[T]) {
        self.bind();
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,
                (data.len() * ::std::mem::size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
        self.unbind();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, 0);
        }
    }
}

impl<B> Drop for Buffer<B>
where
    B: BufferType,
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        VertexArray { vao }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}
