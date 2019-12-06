use amethyst::prelude::*;
use amethyst::utils::application_root_dir;
use amethyst::window::WindowBundle;

struct GameState;

impl SimpleState for GameState {}

pub fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets/");

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?;

    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}
