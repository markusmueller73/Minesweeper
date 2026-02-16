use crate::game::GameState;
use egor::{
    app::WindowEvent,
    input::{Input, KeyCode, MouseButton},
    math::Vec2,
};

#[derive(Default)]
pub struct MouseState {
    pub position: Vec2,
    pub left: bool,
    pub middle: bool,
    pub right: bool,
}

pub fn handle_events(events: &mut Vec<WindowEvent>, game_state: &mut GameState) {
    for event in events {
        if event == &WindowEvent::CloseRequested {
            game_state.mode = super::GameMode::ExitGame;
        } else if event == &WindowEvent::Focused(true) {
            game_state.paused = false;
        }  else if event == &WindowEvent::Focused(false) {
            game_state.paused = true;
        }
    }
}

pub fn handle_input(input: &Input, game_state: &mut GameState) {

    if input.key_released(KeyCode::Escape) {
        game_state.mode = super::GameMode::ExitGame;
    }

    game_state.mouse.left = input.mouse_released(MouseButton::Left);
    game_state.mouse.middle = input.mouse_released(MouseButton::Middle);
    game_state.mouse.right = input.mouse_released(MouseButton::Right);

    let mouse_pos = input.mouse_position();
    game_state.mouse.position = Vec2::new(mouse_pos.0, mouse_pos.1);

}
