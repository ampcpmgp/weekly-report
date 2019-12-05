mod game_data;

use game_data::CustomGameDataBuilder;

pub fn run() -> amethyst::Result<()> {
    let app_root = amethyst::utils::application_root_dir()?;
    let display_config_path = app_root.join("config/diplay.ron");

    let game_data = CustomGameDataBuilder {};

    Ok(())
}
