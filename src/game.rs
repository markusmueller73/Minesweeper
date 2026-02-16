mod config;
mod input;
mod res_manager;
mod render;
mod update;

use std::collections::HashMap;
use crate::minesweeper::board::{Board,BoardSize};
use crate::game::input::MouseState;
use crate::ui::main_menu::MainMenu;
use egor::{
    app::{App, FrameContext},
    math::Vec2
};

pub const APP_NAME: &str = "Minesweeper";
pub const WINDOW_WIDTH: u32 = 400;
pub const WINDOW_HEIGHT: u32 = 500;
pub const CELL_SIZE_SMALL_BOARD: f32 = 38.0;
pub const CELL_SIZE_MED_BOARD: f32 = 18.0;
pub const CELL_SIZE_LARGE_BOARD: f32 = 12.0;

#[derive(Default, PartialEq)]
pub enum GameMode {
    #[default]
    MainMenu,
    Game(BoardSize),
    GameWon,
    GameLost,
    ExitGame
}

#[derive(Default)]
pub struct GameState {
    pub mode: GameMode,
    pub screen_size: Vec2,
    pub texture: HashMap<String,isize>,
    pub mouse: MouseState,
    pub main_menu: MainMenu,
    pub board: Board,
    pub board_offset: Vec2,
    pub cell_size: f32,
    pub paused: bool,
    pub score: u32,
    pub time_elapsed: u32,
}

pub fn run() -> i32 {

    let config = config::Config::new();
    let mut state = GameState {
        paused: true,
        ..Default::default()
    };

    let mut game_timer = std::time::Instant::now();

    let game = App::new()
        .title(APP_NAME)
        .window_size(config.width, config.height)
        .resizable(config.resizeable)
        .vsync(config.vsync);

    //let mut texture: HashMap<String,isize> = HashMap::new();

    game.run(move |FrameContext {gfx, input, timer, events}| {

        // This happens once during game start, load all resources
        if timer.frame == 0 {
            res_manager::load_textures(gfx, &mut state.texture);
            state.main_menu.set_textures(&state.texture);
            state.screen_size = gfx.screen_size();
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
            println!("Bye!");
            std::process::exit(0);
        }

        if !state.paused && game_timer.elapsed().as_secs() >= 1 {
            state.time_elapsed += 1;
            game_timer = std::time::Instant::now();
        }


    });

    0

}
