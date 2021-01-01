extern crate glfw;

use glfw::{Action, Key, MouseButton};
use std::collections::HashMap;

#[derive(Debug)]
pub struct InputHandler {
    pressed_keys: HashMap<Key, bool>,
    released_keys: HashMap<Key, bool>,
    down_keys: HashMap<Key, bool>,
    repeat_keys: HashMap<Key, bool>,

    pressed_buttons: HashMap<MouseButton, bool>,
    released_buttons: HashMap<MouseButton, bool>,
    down_buttons: HashMap<MouseButton, bool>,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            pressed_keys: HashMap::new(),
            released_keys: HashMap::new(),
            down_keys: HashMap::new(),
            repeat_keys: HashMap::new(),
            pressed_buttons: HashMap::new(),
            released_buttons: HashMap::new(),
            down_buttons: HashMap::new(),
        }
    }

    pub fn update_keys(&mut self, k: Key, a: &Action) {
        match a {
            Action::Press => {
                self.pressed_keys.insert(k, true);
                self.down_keys.insert(k, true);
            }
            Action::Repeat => {
                self.repeat_keys.insert(k, true);
            }
            Action::Release => {
                self.released_keys.insert(k, true);
                self.down_keys.insert(k, false);
                self.repeat_keys.insert(k, false);
            }
        }
    }

    pub fn update_mouse_buttons(&mut self, m: MouseButton, a: &Action) {
        match a {
            Action::Press => {
                self.pressed_buttons.insert(m, true);
                self.down_buttons.insert(m, true);
            }
            Action::Release => {
                self.released_buttons.insert(m, true);
                self.down_buttons.insert(m, false);
            }
            Action::Repeat => panic!("Mouse buttons can't repeat!"),
        }
    }

    pub fn clear(&mut self) {
        self.pressed_keys.clear();
        self.released_keys.clear();
        self.pressed_buttons.clear();
        self.released_buttons.clear();
    }

    #[allow(dead_code)]
    pub fn pressed(&self, k: &Key) -> bool {
        *self.pressed_keys.get(k).unwrap_or(&false)
    }

    #[allow(dead_code)]
    pub fn released(&self, k: &Key) -> bool {
        *self.released_keys.get(k).unwrap_or(&false)
    }

    #[allow(dead_code)]
    pub fn down(&self, k: &Key) -> bool {
        *self.down_keys.get(k).unwrap_or(&false)
    }

    #[allow(dead_code)]
    pub fn repeat(&self, k: &Key) -> bool {
        *self.repeat_keys.get(k).unwrap_or(&false)
    }

    #[allow(dead_code)]
    pub fn pressed_button(&self, m: &MouseButton) -> bool {
        *self.pressed_buttons.get(m).unwrap_or(&false)
    }

    #[allow(dead_code)]
    pub fn down_button(&self, m: &MouseButton) -> bool {
        *self.down_buttons.get(m).unwrap_or(&false)
    }

    #[allow(dead_code)]
    pub fn released_button(&self, m: &MouseButton) -> bool {
        *self.released_buttons.get(m).unwrap_or(&false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_key_basic() {
        let mut input = InputHandler::new();

        input.update_keys(Key::Escape, &Action::Press);
        assert!(input.pressed(&Key::Escape));
        assert!(!input.released(&Key::Escape));
        assert!(input.down(&Key::Escape));
        assert!(!input.repeat(&Key::Escape));

        input.clear();
        assert!(!input.pressed(&Key::Escape));
        assert!(!input.released(&Key::Escape));
        assert!(input.down(&Key::Escape));
        assert!(!input.repeat(&Key::Escape));

        input.update_keys(Key::Escape, &Action::Repeat);
        assert!(!input.pressed(&Key::Escape));
        assert!(!input.released(&Key::Escape));
        assert!(input.down(&Key::Escape));
        assert!(input.repeat(&Key::Escape));

        input.clear();
        input.update_keys(Key::Escape, &Action::Release);
        assert!(!input.pressed(&Key::Escape));
        assert!(input.released(&Key::Escape));
        assert!(!input.down(&Key::Escape));
        assert!(!input.repeat(&Key::Escape));

        input.clear();
        assert!(!input.pressed(&Key::Escape));
        assert!(!input.released(&Key::Escape));
        assert!(!input.down(&Key::Escape));
        assert!(!input.repeat(&Key::Escape));
    }

    #[test]
    fn input_key_multiple() {
        let mut input = InputHandler::new();

        input.update_keys(Key::W, &Action::Press);
        input.update_keys(Key::D, &Action::Press);
        assert!(input.down(&Key::W));
        assert!(!input.down(&Key::A));
        assert!(!input.down(&Key::S));
        assert!(input.down(&Key::D));

        input.clear();
        input.update_keys(Key::A, &Action::Press);
        input.update_keys(Key::S, &Action::Press);
        assert!(input.down(&Key::W));
        assert!(input.down(&Key::A));
        assert!(input.down(&Key::S));
        assert!(input.down(&Key::D));

        input.clear();
        input.update_keys(Key::W, &Action::Release);
        input.update_keys(Key::D, &Action::Press);
        assert!(!input.down(&Key::W));
        assert!(input.down(&Key::A));
        assert!(input.down(&Key::S));
        assert!(input.down(&Key::D));

        input.clear();
        input.update_keys(Key::W, &Action::Release);
        input.update_keys(Key::A, &Action::Release);
        input.update_keys(Key::S, &Action::Release);
        input.update_keys(Key::D, &Action::Release);
        assert!(!input.down(&Key::W));
        assert!(!input.down(&Key::A));
        assert!(!input.down(&Key::S));
        assert!(!input.down(&Key::D));
    }

    #[test]
    fn input_mouse_basic() {
        let mut input = InputHandler::new();

        input.update_mouse_buttons(MouseButton::Button1, &Action::Press);
        assert!(input.pressed_button(&MouseButton::Button1));
        assert!(input.down_button(&MouseButton::Button1));
        assert!(!input.released_button(&MouseButton::Button1));

        input.clear();
        assert!(!input.pressed_button(&MouseButton::Button1));
        assert!(input.down_button(&MouseButton::Button1));
        assert!(!input.released_button(&MouseButton::Button1));

        input.update_mouse_buttons(MouseButton::Button1, &Action::Release);
        assert!(!input.pressed_button(&MouseButton::Button1));
        assert!(!input.down_button(&MouseButton::Button1));
        assert!(input.released_button(&MouseButton::Button1));

        input.clear();
        assert!(!input.pressed_button(&MouseButton::Button1));
        assert!(!input.down_button(&MouseButton::Button1));
        assert!(!input.released_button(&MouseButton::Button1));
    }

    #[test]
    fn input_mouse_multiple() {
        let mut input = InputHandler::new();

        input.update_mouse_buttons(MouseButton::Button1, &Action::Press);
        input.update_mouse_buttons(MouseButton::Button2, &Action::Press);
        assert!(input.down_button(&MouseButton::Button1));
        assert!(input.down_button(&MouseButton::Button2));
        assert!(!input.down_button(&MouseButton::Button3));

        input.update_mouse_buttons(MouseButton::Button2, &Action::Release);
        input.update_mouse_buttons(MouseButton::Button3, &Action::Press);
        assert!(input.down_button(&MouseButton::Button1));
        assert!(!input.down_button(&MouseButton::Button2));
        assert!(input.down_button(&MouseButton::Button3));

        input.update_mouse_buttons(MouseButton::Button2, &Action::Press);
        assert!(input.down_button(&MouseButton::Button1));
        assert!(input.down_button(&MouseButton::Button2));
        assert!(input.down_button(&MouseButton::Button3));

        input.update_mouse_buttons(MouseButton::Button1, &Action::Release);
        input.update_mouse_buttons(MouseButton::Button2, &Action::Release);
        input.update_mouse_buttons(MouseButton::Button3, &Action::Release);
        assert!(!input.down_button(&MouseButton::Button1));
        assert!(!input.down_button(&MouseButton::Button2));
        assert!(!input.down_button(&MouseButton::Button3));
    }
}
