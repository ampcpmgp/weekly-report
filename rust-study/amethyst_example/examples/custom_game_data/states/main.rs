use amethyst::{
    assets::{Handle, Prefab},
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::UiPrefab,
    winit::VirtualKeyCode,
};

use crate::{game_data::CustomGameData, resources::MainPrefab, states::Paused};

pub struct Main {
    pub scene: Handle<Prefab<MainPrefab>>,
    pub paused_ui: Handle<UiPrefab>,
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Main {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        data.world.create_entity().with(self.scene.clone()).build();
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                Trans::Push(Box::new(Paused {
                    ui: data
                        .world
                        .create_entity()
                        .with(self.paused_ui.clone())
                        .build(),
                }))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);
        Trans::None
    }
}
