use crate::game::{GameMode, GameState};
use egor::{
    app::WindowEvent,
    input::{Input, KeyCode, MouseButton},
    math::Vec2,
};

#[derive(Clone, Debug, Default)]
pub struct MouseState {
    pub position: Vec2,
    pub released: [bool; 3],
    pub held: [bool; 3],
    pub wheel: f32,
}

pub fn handle_events(events: &mut Vec<WindowEvent>, game_state: &mut GameState) {

    for event in events {

        if event == &WindowEvent::CloseRequested {
            game_state.mode = GameMode::ExitGame;
        } else if event == &WindowEvent::Focused(true) {
            game_state.paused = false;
        }  else if event == &WindowEvent::Focused(false) {
            game_state.paused = true;
        }

    }
}

pub fn handle_input(input: &Input, game_state: &mut GameState) {

    if input.key_released(KeyCode::Escape) {
        game_state.mode = GameMode::ExitGame;
    } else if input.key_released(KeyCode::KeyP) {
        game_state.paused = !game_state.paused;
    }

    game_state.mouse.released[0] = input.mouse_released(MouseButton::Left);
    game_state.mouse.released[1] = input.mouse_released(MouseButton::Middle);
    game_state.mouse.released[2] = input.mouse_released(MouseButton::Right);

    game_state.mouse.held[0] = input.mouse_held(MouseButton::Left);
    game_state.mouse.held[1] = input.mouse_held(MouseButton::Middle);
    game_state.mouse.held[2] = input.mouse_held(MouseButton::Right);

    game_state.mouse.wheel = input.mouse_scroll();

    let mouse_pos = input.mouse_position();
    game_state.mouse.position = Vec2::new(mouse_pos.0, mouse_pos.1);

}
