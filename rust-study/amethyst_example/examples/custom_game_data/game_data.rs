use amethyst::core::{ArcThreadPool, SystemBundle};
use amethyst::ecs::prelude::*;
use amethyst::{DataDispose, DataInit, Error};

pub struct CustomGameData<'a, 'b> {
    pub core_dispatcher: Dispatcher<'a, 'b>,
    pub running_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> CustomGameData<'a, 'b> {
    pub fn update(&mut self, world: &World, running: bool) {
        if running {
            self.running_dispatcher.dispatch(&world);
        }

        self.core_dispatcher.dispatch(&world);
    }

    pub fn dispose(&mut self, _world: &mut World) {}
}

impl DataDispose for CustomGameData<'_, '_> {
    fn dispose(&mut self, world: &mut World) {
        self.dispose(world);
    }
}

pub struct CustomGameDataBuilder<'a, 'b> {
    pub core: DispatcherBuilder<'a, 'b>,
    pub running: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> CustomGameDataBuilder<'a, 'b> {
    pub fn default() -> Self {
        CustomGameDataBuilder::new()
    }
}

impl<'a, 'b> CustomGameDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        CustomGameDataBuilder {
            core: DispatcherBuilder::new(),
            running: DispatcherBuilder::new(),
        }
    }

    pub fn with_base_bundle<B>(mut self, world: &mut World, bundle: B) -> Result<Self, Error>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle.build(world, &mut self.core)?;
        Ok(self)
    }

    pub fn with_running<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.running.add(system, name, dependencies);
        self
    }
}

impl<'a, 'b> DataInit<CustomGameData<'a, 'b>> for CustomGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> CustomGameData<'a, 'b> {
        let pool = (*world.read_resource::<ArcThreadPool>()).clone();

        let mut core_dispatcher = self.core.with_pool(pool.clone()).build();
        let mut running_dispatcher = self.running.with_pool(pool.clone()).build();

        core_dispatcher.setup(world);
        running_dispatcher.setup(world);

        CustomGameData {
            core_dispatcher,
            running_dispatcher,
        }
    }
}
