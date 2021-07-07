use amethyst::{
    assets::{Completion, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::palette::Srgb,
    ui::{UiCreator, UiLoader, UiPrefab},
    winit::VirtualKeyCode,
};

use crate::{game_data::CustomGameData, resources::Demo, resources::MainPrefab, states::Main};

#[derive(Default)]
pub struct Loading {
    progress: ProgressCounter,
    scene: Option<Handle<Prefab<MainPrefab>>>,
    load_ui: Option<Entity>,
    paused_ui: Option<Handle<UiPrefab>>,
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Loading {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        self.scene = Some(data.world.exec(|loader: PrefabLoader<'_, MainPrefab>| {
            loader.load("prefab/renderable.ron", RonFormat, &mut self.progress)
        }));

        self.load_ui = Some(data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", &mut self.progress);
            creator.create("ui/loading.ron", &mut self.progress)
        }));
        self.paused_ui = Some(
            data.world
                .exec(|loader: UiLoader<'_>| loader.load("ui/paused.ron", &mut self.progress)),
        );
        data.world.insert::<Demo>(Demo {
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
        data.data.update(&data.world, true);
        match self.progress.complete() {
            Completion::Failed => {
                eprintln!("Failed loading assets");
                Trans::Quit
            }
            Completion::Complete => {
                println!("Assets loaded, swapping state");
                if let Some(entity) = self.load_ui {
                    let _ = data.world.delete_entity(entity);
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
