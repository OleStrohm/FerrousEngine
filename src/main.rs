extern crate gl;
pub mod input_handler;
pub mod render;
pub mod resources;
pub mod utils;

use input_handler::InputHandler;
use resources::Resources;
use std::path::Path;
use render::Vertex;

fn main() {
    println!("Welcome to a rusty engine!");

    let mut window = render::Window::new(600, 600);

    unsafe {
        gl::ClearColor(0.25, 0.05, 0.5, 1.0);
    }

    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let shader_program = render::Program::from_res(&res, "shaders/triangle").unwrap();

    let vertices: Vec<Vertex> = vec![
        Vertex::new((-0.5, -0.5, 0.0).into(), (1.0, 0.0, 0.0).into()),
        Vertex::new((0.5, -0.5, 0.0).into(), (0.0, 1.0, 0.0).into()),
        Vertex::new((0.0, 0.5, 0.0).into(), (0.0, 0.0, 1.0).into())
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        Vertex::vertex_attrib_pointers();

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let mut input = InputHandler::new();
    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        input.clear();
        for event in window.flush_messages() {
            match event {
                glfw::WindowEvent::Key(k, _, a, _) => input.update(k, &a),
                _ => println!("{:?}", event),
            }
        }
    }
}
