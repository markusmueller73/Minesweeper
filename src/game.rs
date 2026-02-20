pub mod board;
pub mod cell;
mod config;
pub mod constants;

use crate::game::{board::{Board, BoardSize}, config::Config};
use crate::engine::{assets, input, render, ui, update};
use egor::{app::{App, FrameContext}, math::Vec2, render::Color};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum GameMode {
    #[default]
    MainMenu,
    Game(BoardSize),
    GameWon,
    GameLost,
    ExitGame
}

#[derive(Clone, Debug, Default)]
pub struct GameState {
    pub assets: assets::Assets,
    pub config: Config,
    pub mouse: input::MouseState,
    pub mode: GameMode,
    pub main_menu: ui::main_menu::MainMenu,
    pub board: Board,
    pub board_offset: Vec2,
    pub cell_size: f32,
    pub score: u32,
    pub time_elapsed: u32,
    pub paused: bool,
    pub resized: bool,
}

pub fn run() -> i32 {

    let mut state = GameState {
        paused: true,
        ..Default::default()
    };

    let mut game_timer = std::time::Instant::now();

    let game = App::new()
        .title(state.config.title.as_str())
        .window_size(state.config.width, state.config.height)
        .resizable(state.config.resizeable)
        .vsync(state.config.vsync);

    game.run(move |FrameContext {app, gfx, input, timer, events}| {

        // This happens once during game start, load all resources
        if timer.frame == 0 {
            state.assets.load_textures(gfx);
            state.assets.load_fonts(gfx);
            return;
        }

        // Handle window events
        input::handle_events(events, &mut state);

        // Update user input
        input::handle_input(input, &mut state);

        // Render scene
        render::render_scene(gfx, &state);

        // update game state
        update::update(&mut state);

        // check win or end condition
        if state.mode == GameMode::ExitGame {
            std::process::exit(0);
        }

        if state.resized {
            //app.set_vsync(state.config.vsync);
            app.set_size(state.config.width, state.config.height);
            state.resized = false;
        }

        if !state.paused && game_timer.elapsed().as_secs() >= 1 {
            state.time_elapsed += 1;
            game_timer = std::time::Instant::now();
        }

    });

    0

}
