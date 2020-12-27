extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};
use std::collections::HashMap;

#[derive(Debug)]
struct InputHandler {
    clicked_keys: HashMap<Key, bool>,
    down_keys: HashMap<Key, bool>,
    repeat_keys: HashMap<Key, bool>,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            clicked_keys: HashMap::new(),
            down_keys: HashMap::new(),
            repeat_keys: HashMap::new(),
        }
    }

    pub fn update(&mut self, k: Key, a: &Action) {
        match a {
            Action::Press => {
                self.clicked_keys.insert(k, true);
                self.down_keys.insert(k, true);
            }
            Action::Repeat => {
                self.repeat_keys.insert(k, true);
            }
            Action::Release => {
                self.down_keys.insert(k, false);
                self.repeat_keys.insert(k, false);
            }
        }
    }

    pub fn clear(&mut self) {
        self.clicked_keys.clear();
    }

    pub fn clicked(&self, k: &Key) -> bool {
        *self.clicked_keys.get(k).unwrap_or(&false)
    }

    pub fn down(&self, k: &Key) -> bool {
        *self.down_keys.get(k).unwrap_or(&false)
    }

    pub fn repeat(&self, k: &Key) -> bool {
        *self.repeat_keys.get(k).unwrap_or(&false)
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

    let mut input = InputHandler::new();

    while !window.should_close() {
        input.clear();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(k, _, a, _) => {
                    input.update(k, &a);
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_basic() {
        let mut input = InputHandler::new();

        input.update(Key::Escape, &Action::Press);
        assert!(input.clicked(&Key::Escape));
        assert!(input.down(&Key::Escape));
        assert!(!input.repeat(&Key::Escape));

        input.clear();
        assert!(!input.clicked(&Key::Escape));
        assert!(input.down(&Key::Escape));
        assert!(!input.repeat(&Key::Escape));

        input.update(Key::Escape, &Action::Repeat);
        assert!(!input.clicked(&Key::Escape));
        assert!(input.down(&Key::Escape));
        assert!(input.repeat(&Key::Escape));

        input.update(Key::Escape, &Action::Release);
        assert!(!input.clicked(&Key::Escape));
        assert!(!input.down(&Key::Escape));
        assert!(!input.repeat(&Key::Escape));
    }
}
