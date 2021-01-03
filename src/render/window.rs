use glfw::Context;
use std::sync::mpsc::Receiver;

pub struct Window {
    inner: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::Resizable(false));
        let (mut inner, events) = glfw
            .create_window(width, height, "Window!", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        inner.set_resizable(true);
        inner.make_current();
        inner.set_key_polling(true);
        inner.set_mouse_button_polling(true);
        inner.set_size_polling(true);

        gl::load_with(|s| inner.get_proc_address(s));
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        Window {
            inner,
            events,
            glfw,
        }
    }

    pub fn flush_messages<'a>(&'a mut self) -> impl Iterator<Item = glfw::WindowEvent> + 'a {
        self.glfw.poll_events();
        glfw::flush_messages(&self.events).map(|(_, e)| e)
    }

    pub fn should_close(&self) -> bool {
        self.inner.should_close()
    }

    pub fn swap_buffers(&mut self) {
        self.inner.swap_buffers();
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }
}
