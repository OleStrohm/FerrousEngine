extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key, Modifiers};

#[derive(Debug)]
struct InputHandler {
    pressed_keys: [bool; 64],
    down_keys: [bool; 64],
    repeat_keys: [bool; 64],
}

impl InputHandler {
    fn is_down(k: Key) -> bool {

    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Resizable(false));
    let (mut window, events) = glfw
        .create_window(300, 300, "Window!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_resizable(true);
    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s));
    unsafe {
        gl::ClearColor(0.25, 0.05, 0.5, 1.0);
    }

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::Key(Key::W, _, Action::Press, Modifiers::Control) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
