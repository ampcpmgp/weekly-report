mod game_data;
mod state;
mod system;

use amethyst::core::transform::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::prelude::*;
use amethyst::renderer::{plugins::RenderToWindow, types::DefaultBackend, RenderingBundle};
use amethyst::ui::UiBundle;
use game_data::CustomGameDataBuilder;
use state::Main;
use system::ExampleSystem;

pub fn main() -> amethyst::Result<()> {
    let app_root = amethyst::utils::application_root_dir()?;
    let display_config_path = app_root.join("config/diplay.ron");
    let assets_directory = app_root.join("assets");

    let mut world = World::new();
    let game_data = CustomGameDataBuilder::default()
        .with_running(ExampleSystem, "example_system", &[])
        .with_base_bundle(
            &mut world,
            RenderingBundle::<DefaultBackend>::new().with_plugin(
                RenderToWindow::from_config_path(display_config_path).with_clear([0, 0, 0, 1]),
            ),
        )?
        .with_base_bundle(&mut world, TransformBundle::new())?
        .with_base_bundle(&mut world, UiBundle::<StringBindings>::new())?
        .with_base_bundle(&mut world, InputBundle::<StringBindings>::new())?;

    let mut game = Application::build(assets_directory, Main)?.build(game_data)?;
    game.run();

    Ok(())
}
