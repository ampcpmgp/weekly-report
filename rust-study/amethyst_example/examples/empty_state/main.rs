use amethyst::prelude::*;

struct GameState;

impl EmptyState for GameState {
    fn on_start(&mut self, _data: StateData<()>) {
        println!("Starting Game");
    }

    fn on_stop(&mut self, _data: StateData<'_, ()>) {
        println!("End");
    }

    fn update(&mut self, _data: StateData<()>) -> EmptyTrans {
        println!("Computing..");

        Trans::Quit
    }
}

pub fn main() -> amethyst::Result<()> {
    let asset_dir = "assets/";
    let mut game = Application::new(asset_dir, GameState, ())?;

    game.run();

    Ok(())
}
