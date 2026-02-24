use crate::game::{constants::MENUBAR_HEIGHT, game_state::{GameMode, GameState}};
use egor::{
    app::{WindowEvent, egui::{Context, MenuBar, TopBottomPanel}},
    input::{Input, KeyCode, MouseButton},
    math::Vec2,
};

#[derive(Debug, Default)]
pub struct MouseState {
    pub position: Vec2,
    pub pressed: [bool; 3],
    pub held: [bool; 3],
    pub released: [bool; 3],
    pub wheel: f32,
}

pub fn handle_events(events: &mut Vec<WindowEvent>, game_state: &mut GameState) {

    for event in events {

        if event == &WindowEvent::CloseRequested {
            game_state.mode = GameMode::QuitGame;
        } else if event == &WindowEvent::Focused(true) {
            game_state.focused = true;
        }  else if event == &WindowEvent::Focused(false) {
            game_state.focused = false;
        }

    }
}

pub fn handle_input(input: &Input, game_state: &mut GameState) {

    if input.key_released(KeyCode::KeyQ) || input.key_released(KeyCode::Escape) {
        game_state.mode = GameMode::QuitGame;
    } else if input.key_released(KeyCode::KeyP) || input.key_released(KeyCode::Pause) {
        game_state.paused = !game_state.paused;
    }

    game_state.mouse.pressed[0] = input.mouse_released(MouseButton::Left);
    game_state.mouse.pressed[1] = input.mouse_released(MouseButton::Middle);
    game_state.mouse.pressed[2] = input.mouse_released(MouseButton::Right);

    game_state.mouse.held[0] = input.mouse_held(MouseButton::Left);
    game_state.mouse.held[1] = input.mouse_held(MouseButton::Middle);
    game_state.mouse.held[2] = input.mouse_held(MouseButton::Right);

    game_state.mouse.released[0] = input.mouse_released(MouseButton::Left);
    game_state.mouse.released[1] = input.mouse_released(MouseButton::Middle);
    game_state.mouse.released[2] = input.mouse_released(MouseButton::Right);

    game_state.mouse.wheel = input.mouse_scroll();

    let mouse_pos = input.mouse_position();
    game_state.mouse.position = Vec2::new(mouse_pos.0, mouse_pos.1);

}

pub fn handle_ui(context: &mut &Context, game_state: &mut GameState) {

    TopBottomPanel::top("menu_bar").exact_height(MENUBAR_HEIGHT).show(context, |ui| {

        MenuBar::new().ui(ui, |ui| {

            ui.menu_button(t!("menu.title_game"), |ui| {

                if ui.button(t!("menu.start_easy_game")).clicked() {
                    game_state.mode = GameMode::GameInit(crate::game::board::BoardSize::Small);
                }
                if ui.button(t!("menu.start_med_game")).clicked() {
                    game_state.mode = GameMode::GameInit(crate::game::board::BoardSize::Medium);
                }
                if ui.button(t!("menu.start_hard_game")).clicked() {
                    game_state.mode = GameMode::GameInit(crate::game::board::BoardSize::Large);
                }
                ui.separator();
                if ui.button(t!("menu.quit_game")).clicked() {
                    game_state.mode = GameMode::QuitGame;
                }

            });
        });
    });

}
