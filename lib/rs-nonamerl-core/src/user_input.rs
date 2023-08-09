use bevy_ecs::system::Resource;
use macroquad::prelude::{
    get_last_key_pressed, is_mouse_button_down, mouse_position, mouse_wheel, KeyCode, MouseButton,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyInput {
    None,
    Up,
    Down,
    Left,
    Right,
    Quit,
    Key(KeyCode),
    CtrlKey(KeyCode),
    AltKey(KeyCode),
    ShiftKey(KeyCode),
}

impl Default for KeyInput {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct MouseState {
    pub x: f32,
    pub y: f32,
    pub left_button: bool,
    pub right_button: bool,
    pub middle_button: bool,
    pub scroll: f32,
}

#[derive(Resource, Default, Debug, Copy, Clone)]
pub struct UserInput {
    pub key_input: KeyInput,
    pub mouse_state: MouseState,
}

impl UserInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self) {
        self.key_input = match get_last_key_pressed() {
            None => KeyInput::None,
            Some(key) => match key {
                KeyCode::A | KeyCode::Left => KeyInput::Left,
                KeyCode::D | KeyCode::Right => KeyInput::Right,
                KeyCode::W | KeyCode::Up => KeyInput::Up,
                KeyCode::S | KeyCode::Down => KeyInput::Down,
                KeyCode::Escape => KeyInput::Quit,
                _ => KeyInput::None,
            },
        };

        self.mouse_state = MouseState {
            x: mouse_position().0,
            y: mouse_position().1,
            left_button: is_mouse_button_down(MouseButton::Left),
            right_button: is_mouse_button_down(MouseButton::Right),
            middle_button: is_mouse_button_down(MouseButton::Middle),
            scroll: mouse_wheel().1,
        };
    }
}
