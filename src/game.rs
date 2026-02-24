pub mod board;
pub mod cell;
pub mod config;
pub mod constants;
pub mod game_state;

use crate::game::game_state::{GameMode, GameState};
use crate::engine::{event_manager, rendering, updateing};
use egor::app::{App, FrameContext};

pub fn run() -> i32 {

    let mut state = GameState::new();dbg!(state.paused);
    let mut game_timer = std::time::Instant::now();

    let game = App::new()
        .title(state.config.title.as_str())
        .window_size(state.config.width, state.config.height)
        .resizable(state.config.resizeable)
        .vsync(state.config.vsync);

    game.run(move |FrameContext {app, gfx, input, timer, events, egui_ctx}| {

        // This happens once during game start, load all resources
        if timer.frame == 0 {
            state.assets.load_textures(gfx);
            state.assets.load_fonts(gfx);
            return;
        }

        // Handle window events
        event_manager::handle_events(events, &mut state);

        // Update user input
        event_manager::handle_input(input, &mut state);

        // Update and render the UI
        event_manager::handle_ui(egui_ctx, &mut state);

        // Render scene
        rendering::render_scene(gfx, &state);

        // update game state
        updateing::update(&mut state);

        // check win or end condition
        if state.mode == GameMode::QuitGame {
            std::process::exit(0);
        }

        if state.resized {
            app.set_size(state.config.width, state.config.height);
            state.resized = false;
        }

        if state.focused && !state.paused && game_timer.elapsed().as_millis() >= 1000 {
            state.time_elapsed += 1;
            game_timer = std::time::Instant::now();
        }

    });

    0

}
