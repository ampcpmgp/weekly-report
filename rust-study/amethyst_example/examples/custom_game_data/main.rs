//! Demonstrates how to use a custom game data structure

use crate::{example_system::ExampleSystem, game_data::CustomGameDataBuilder};
use amethyst::{
    assets::PrefabLoaderSystemDesc,
    core::transform::TransformBundle,
    ecs::{prelude::Component, NullStorage},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
    Error, LogLevelFilter,
};

pub mod example_system;
pub mod game_data;
pub mod resources;
pub mod states;

use resources::MainPrefab;
use states::Loading;

#[derive(Default)]
struct Tag;

impl Component for Tag {
    type Storage = NullStorage<Self>;
}

fn main() -> Result<(), Error> {
    amethyst::Logger::from_config(amethyst::LoggerConfig {
        level_filter: LogLevelFilter::Error,
        ..Default::default()
    })
    .start();

    let app_root = application_root_dir()?;

    // Add our meshes directory to the asset loader.
    let assets_dir = app_root.join("assets");

    let display_config_path = app_root.join("config/display.ron");

    let game_data = CustomGameDataBuilder::default()
        .with_base(PrefabLoaderSystemDesc::<MainPrefab>::default(), "", &[])
        .with_running(ExampleSystem::default(), "example_system", &[])
        .with_base_bundle(TransformBundle::new())
        .with_base_bundle(UiBundle::<StringBindings>::new())
        .with_base_bundle(FpsCounterBundle::default())
        .with_base_bundle(InputBundle::<StringBindings>::new())
        .with_base_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderUi::default()),
        );

    let mut game = Application::build(assets_dir, Loading::default())?.build(game_data)?;
    game.run();

    Ok(())
}
