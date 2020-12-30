use glfw::Context;
use std::sync::mpsc::Receiver;

pub struct Window {
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::Resizable(false));
        let (mut window, events) = glfw
            .create_window(width, height, "Window!", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_resizable(true);
        window.make_current();
        window.set_key_polling(true);

        gl::load_with(|s| window.get_proc_address(s));
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        Window { window, events, glfw }
    }

    pub fn flush_messages<'a>(&'a mut self) -> impl Iterator<Item = glfw::WindowEvent> + 'a {
        self.glfw.poll_events();
        glfw::flush_messages(&self.events).map(|(_, e)| e)
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
}
