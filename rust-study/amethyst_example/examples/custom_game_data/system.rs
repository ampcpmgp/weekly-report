use amethyst::core::SystemDesc;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::{System, SystemData, World};

#[derive(Default, SystemDesc)]
pub struct ExampleSystem;

impl<'a> System<'a> for ExampleSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {}
}
