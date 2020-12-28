extern crate gl;
extern crate glfw;

mod input_handler;
mod render;

use glfw::Context;

use input_handler::InputHandler;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Resizable(false));
    let (mut window, events) = glfw
        .create_window(600, 400, "Window!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_resizable(true);
    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s));
    unsafe {
        gl::Viewport(0, 0, 600, 400);
        gl::ClearColor(0.25, 0.05, 0.5, 1.0);
    }

    let mut input = InputHandler::new();

    use std::ffi::CString;

    let vert_shader =
        render::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();
    let frag_shader =
        render::Shader::from_vert_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    shader_program.set_used();

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
        input.clear();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(k, _, a, _) => input.update(k, &a),
                _ => println!("{:?}", event),
            }
        }
    }
}
