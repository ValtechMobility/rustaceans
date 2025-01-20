use glutin::event::{ElementState, VirtualKeyCode};

const MOVEMENT_SPEED: f32 = 0.1;

pub struct Player {
    pub x_position: f32,
    pub y_position: f32,
}

impl Player {
    fn new() -> Self {
        Self {
            x_position: 0.0,
            y_position: 0.0,
        }
    }
}

pub struct Game {
    pub player: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
        }
    }

    pub fn handle_keypress(&mut self, keycode: VirtualKeyCode, state: ElementState) {
        match (keycode, state) {
            (VirtualKeyCode::Left, ElementState::Pressed) => {
                self.player.x_position -= MOVEMENT_SPEED
            }
            (VirtualKeyCode::Right, ElementState::Pressed) => {
                self.player.x_position += MOVEMENT_SPEED
            }
            (VirtualKeyCode::Up, ElementState::Pressed) => self.player.y_position += MOVEMENT_SPEED,
            (VirtualKeyCode::Down, ElementState::Pressed) => {
                self.player.y_position -= MOVEMENT_SPEED
            }
            _ => (),
        }
    }
}
