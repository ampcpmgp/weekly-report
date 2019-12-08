use amethyst::{
    ecs::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    winit::VirtualKeyCode,
};

use crate::game_data::CustomGameData;

pub struct Paused {
    pub ui: Entity,
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Paused {
    fn handle_event(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                let _ = data.world.delete_entity(self.ui);
                Trans::Pop
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
        data.data.update(&data.world, false);
        Trans::None
    }
}
