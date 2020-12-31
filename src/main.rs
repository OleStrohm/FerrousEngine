extern crate gl;
extern crate glfw;
#[macro_use]
extern crate render_derive;

pub mod input_handler;
pub mod render;
pub mod resources;
pub mod utils;

use input_handler::InputHandler;
use render::buffer::{ArrayBuffer, VertexArray};
use render::Vertex;
use resources::Resources;
use std::path::Path;

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
        Vertex::new((0.0, 0.5, 0.0).into(), (0.0, 0.0, 1.0).into()),
    ];

    let vbo = ArrayBuffer::new();
    vbo.buffer_static_data(&vertices);

    let vao = VertexArray::new();
    vao.bind();
    vbo.bind();
    Vertex::vertex_attrib_pointers();
    vbo.unbind();
    vao.unbind();

    let mut input = InputHandler::new();
    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.bind();
        unsafe {
            vao.bind();
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
