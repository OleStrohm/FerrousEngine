extern crate gl;
extern crate glfw;
#[macro_use]
extern crate render_derive;

pub mod input_handler;
pub mod render;
pub mod resources;
pub mod utils;

use input_handler::InputHandler;
use render::Quad;
use resources::Resources;
use std::path::Path;

fn main() {
    println!("Welcome to a rusty engine!");

    let mut window = render::Window::new(600, 600);
    window.set_clear_color(0.15, 0.0, 0.5, 1.0);

    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let quad = Quad::new(&res).unwrap();

    let mut input = InputHandler::new();
    while !window.should_close() {
        window.clear();

        quad.render();

        window.swap_buffers();
        input.clear();
        for event in window.flush_messages() {
            match event {
                glfw::WindowEvent::Key(k, _, a, _) => input.update_keys(k, &a),
                glfw::WindowEvent::MouseButton(m, a, _) => input.update_mouse_buttons(m, &a),
                glfw::WindowEvent::Size(w, h) => unsafe { gl::Viewport(0, 0, w, h); },
                _ => println!("{:?}", event),
            }
        }
    }
}
