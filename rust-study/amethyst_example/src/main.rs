mod example_system;
mod game_data;

use crate::{
    example_system::ExampleSystem,
    game_data::{CustomGameData, CustomGameDataBuilder},
};

use amethyst::{
    assets::{
        Completion, Handle, Prefab, PrefabLoader, PrefabLoaderSystemDesc, ProgressCounter,
        RonFormat,
    },
    core::transform::TransformBundle,
    ecs::{
        prelude::{Component, Entity},
        NullStorage, WorldExt,
    },
    input::{is_close_requested, is_key_down, InputBundle, StringBindings},
    prelude::*,
    renderer::{
        palette::Srgb,
        plugins::{RenderShaded3D, RenderToWindow},
        rendy::mesh::{Normal, Position, TexCoord},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle, UiCreator, UiLoader, UiPrefab},
    utils::{application_root_dir, fps_counter::FpsCounterBundle, scene::BasicScenePrefab},
    window::WindowBundle,
    winit::VirtualKeyCode,
    Error, LogLevelFilter, LoggerConfig,
};

use std::sync::Once;

static START: Once = Once::new();
type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, _: StateData<'_, GameData<'_, '_>>) {
        println!("Begin!");
    }

    fn on_stop(&mut self, _: StateData<'_, GameData<'_, '_>>) {
        println!("End!");
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        START.call_once(|| {
            println!("Hello from Amethyst!");
        });

        Trans::None
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

pub struct DemoState {
    light_angle: f32,
    light_color: Srgb,
    camera_angle: f32,
}

#[derive(Default)]
struct Loading {
    progress: ProgressCounter,
    scene: Option<Handle<Prefab<MyPrefabData>>>,
    load_ui: Option<Entity>,
    paused_ui: Option<Handle<UiPrefab>>,
}
impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Loading {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        let StateData { world, .. } = data;

        self.scene = Some(world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/renderable.ron", RonFormat, &mut self.progress)
        }));

        self.load_ui = Some(world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", &mut self.progress);
            creator.create("ui/loading.ron", &mut self.progress)
        }));

        self.paused_ui = Some(
            world.exec(|loader: UiLoader<'_>| loader.load("ui/paused.ron", &mut self.progress)),
        );

        world.insert::<DemoState>(DemoState {
            light_angle: 0.0,
            light_color: Srgb::new(1.0, 1.0, 1.0),
            camera_angle: 0.0,
        });
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        let StateData { data, world } = data;

        data.update(&world, true);

        match self.progress.complete() {
            Completion::Failed => {
                eprintln!("Failed loading assets");
                Trans::Quit
            }
            Completion::Complete => {
                println!("Assets loaded, swapping state");
                if let Some(entity) = self.load_ui {
                    let _ = world.delete_entity(entity);
                }

                Trans::Switch(Box::new(Main {
                    scene: self.scene.as_ref().unwrap().clone(),
                    paused_ui: self.paused_ui.as_ref().unwrap().clone(),
                }))
            }
            Completion::Loading => Trans::None,
        }
    }
}

struct Paused {
    ui: Entity,
}
impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Paused {}

struct Main {
    scene: Handle<Prefab<MyPrefabData>>,
    paused_ui: Handle<UiPrefab>,
}
impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Main {
    fn on_start(&mut self, data: StateData<'_, CustomGameData>) {
        data.world.create_entity().with(self.scene.clone()).build();
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(Paused {
                    ui: data
                        .world
                        .create_entity()
                        .with(self.paused_ui.clone())
                        .build(),
                }));
            }
        }

        Trans::None
    }
}

#[derive(Default)]
struct Tag;

impl Component for Tag {
    type Storage = NullStorage<Self>;
}

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(LoggerConfig {
        level_filter: LogLevelFilter::Off,
        ..Default::default()
    })
    .start();

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("./config/display.ron");
    let assets_dir = "./assets/";
    let path = WindowBundle::from_config_path(display_config_path);

    let game_data = GameDataBuilder::default().with_bundle(path)?;
    let mut game = Application::new(assets_dir, ExampleState, game_data)?;
    game.run();

    Ok(())
}
