extern crate glfw;

use glfw::{Action, Key};
use std::collections::HashMap;

#[derive(Debug)]
pub struct InputHandler {
    pressed_keys: HashMap<Key, bool>,
    released_keys: HashMap<Key, bool>,
    down_keys: HashMap<Key, bool>,
    repeat_keys: HashMap<Key, bool>,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            pressed_keys: HashMap::new(),
            released_keys: HashMap::new(),
            down_keys: HashMap::new(),
            repeat_keys: HashMap::new(),
        }
    }

    pub fn update(&mut self, k: Key, a: &Action) {
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

    pub fn clear(&mut self) {
        self.pressed_keys.clear();
        self.released_keys.clear();
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_basic() {
        let mut input = InputHandler::new();

        input.update(Key::Escape, &Action::Press);
        assert!(input.pressed(&Key::Escape));
        assert!(!input.released(&Key::Escape));
        assert!(input.down(&Key::Escape));
        assert!(!input.repeat(&Key::Escape));

        input.clear();
        assert!(!input.pressed(&Key::Escape));
        assert!(!input.released(&Key::Escape));
        assert!(input.down(&Key::Escape));
        assert!(!input.repeat(&Key::Escape));

        input.update(Key::Escape, &Action::Repeat);
        assert!(!input.pressed(&Key::Escape));
        assert!(!input.released(&Key::Escape));
        assert!(input.down(&Key::Escape));
        assert!(input.repeat(&Key::Escape));

        input.clear();
        input.update(Key::Escape, &Action::Release);
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
    fn input_multiple() {
        let mut input = InputHandler::new();

        input.update(Key::W, &Action::Press);
        input.update(Key::D, &Action::Press);
        assert!(input.down(&Key::W));
        assert!(!input.down(&Key::A));
        assert!(!input.down(&Key::S));
        assert!(input.down(&Key::D));

        input.clear();
        input.update(Key::A, &Action::Press);
        input.update(Key::S, &Action::Press);
        assert!(input.down(&Key::W));
        assert!(input.down(&Key::A));
        assert!(input.down(&Key::S));
        assert!(input.down(&Key::D));

        input.clear();
        input.update(Key::W, &Action::Release);
        input.update(Key::D, &Action::Press);
        assert!(!input.down(&Key::W));
        assert!(input.down(&Key::A));
        assert!(input.down(&Key::S));
        assert!(input.down(&Key::D));

        input.clear();
        input.update(Key::W, &Action::Release);
        input.update(Key::A, &Action::Release);
        input.update(Key::S, &Action::Release);
        input.update(Key::D, &Action::Release);
        assert!(!input.down(&Key::W));
        assert!(!input.down(&Key::A));
        assert!(!input.down(&Key::S));
        assert!(!input.down(&Key::D));
    }
}
