use amethyst::{
    renderer::rendy::mesh::{Normal, Position, TexCoord},
    utils::scene::BasicScenePrefab,
};

pub type MainPrefab = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;
