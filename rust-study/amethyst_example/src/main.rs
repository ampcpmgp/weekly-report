mod example_system;
mod game_data;

use crate::{
    example_system::ExampleSystem,
    game_data::{CustomGameData, CustomGameDataBuilder},
};
use amethyst::{
    input::is_key_down, prelude::*, utils::application_root_dir, window::WindowBundle,
    winit::VirtualKeyCode, LogLevelFilter, LoggerConfig,
};
use std::sync::Once;

static START: Once = Once::new();

struct ExampleState;

impl EmptyState for ExampleState {
    fn on_start(&mut self, _: StateData<'_, ()>) {
        println!("Begin!");
    }
    fn on_stop(&mut self, _: StateData<'_, ()>) {
        println!("End!");
    }
    fn update(&mut self, _: StateData<'_, ()>) -> EmptyTrans {
        START.call_once(|| {
            println!("Hello from Amethyst!");
        });
        Trans::None
    }
}

impl SimpleState for ExampleState {
    fn on_start(&mut self, _: StateData<'_, GameData<'_, '_>>) {
        println!("Begin!");
    }
    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

fn main() -> amethyst::Result<()> {
    let mut config = LoggerConfig::default();
    config.level_filter = LogLevelFilter::Off;
    amethyst::start_logger(config);

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("./config/display.ron");
    let assets_dir = "./assets/";
    let path = WindowBundle::from_config_path(display_config_path);
    let game_data = GameDataBuilder::default().with_bundle(path)?;
    let mut game = Application::new(assets_dir, ExampleState, game_data)?;
    game.run();

    Ok(())
}
