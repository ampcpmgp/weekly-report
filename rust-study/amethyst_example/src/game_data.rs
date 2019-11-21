use std::marker::PhantomData;

use amethyst::{
    core::{ArcThreadPool, SystemBundle, SystemDesc},
    ecs::prelude::{Dispatcher, DispatcherBuilder, System, World, WorldExt},
    error::Error,
    DataDispose, DataInit,
};

pub struct CustomGameData<'a, 'b> {
    pub base: Option<Dispatcher<'a, 'b>>,
    pub running: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> CustomGameData<'a, 'b> {
    pub fn update(&mut self, world: &World, running: bool) {
        if running {
            if let Some(running) = &mut self.running {
                running.dispatch(&world);
            }
        }

        if let Some(base) = &mut self.base {
            base.dispatch(&world);
        }
    }

    pub fn dispose(&mut self, world: &mut World) {
        if let Some(base) = self.base.take() {
            base.dispose(world);
        }

        if let Some(running) = self.running.take() {
            running.dispose(world);
        }
    }
}

pub struct CustomGameDataBuilder {}
