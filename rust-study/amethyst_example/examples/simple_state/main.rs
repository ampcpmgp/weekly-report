use amethyst::prelude::*;

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Starting Game");
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("Computing..");

        Trans::Quit
    }
}

pub fn main() -> amethyst::Result<()> {
    let asset_dir = "assets/";
    let mut game = Application::new(asset_dir, GameState, GameDataBuilder::default())?;

    game.run();

    Ok(())
}
