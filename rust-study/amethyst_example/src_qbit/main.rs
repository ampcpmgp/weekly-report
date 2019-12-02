use amethyst::{
    animation::{
        Animation, AnimationBundle, AnimationSet, AnimationSetPrefab, InterpolationFunction,
        Sampler, SamplerPrimitive, TransformChannel,
    },
    assets::{HotReloadBundle, Loader, PrefabLoader, PrefabLoaderSystemDesc, RonFormat},
    core::{Transform, TransformBundle},
    ecs::Entity,
    prelude::*,
    renderer::{
        plugins::{RenderPbr3D, RenderToWindow},
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::{application_root_dir, scene::BasicScenePrefab},
    LogLevelFilter,
};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Deserialize, Clone, Copy, Serialize)]
enum AnimationId {
    Scale,
    Rotate,
    Translate,
    Test,
}

type MyPrefabData = (
    Option<BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>>,
    Option<AnimationSetPrefab<AnimationId, Transform>>,
);

struct ExampleState {
    pub sphere: Option<Entity>,
    rate: f32,
    current_animtion: AnimationId,
}

impl Default for ExampleState {
    fn default() -> Self {
        ExampleState {
            sphere: None,
            rate: 1.0,
            current_animtion: AnimationId::Translate,
        }
    }
}

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        let prefab_handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/animation.ron", RonFormat, ())
        });

        self.sphere = Some(world.create_entity().with(prefab_handle).build());

        let (animation_set, animation) = {
            let loader = world.read_resource::<Loader>();

            let sampler = loader.load_from_data(
                Sampler {
                    input: vec![0., 1.],
                    output: vec![
                        SamplerPrimitive::Vec3([0., 0., 0.]),
                        SamplerPrimitive::Vec3([0., 1., 0.]),
                    ],
                    function: InterpolationFunction::Step,
                },
                (),
                &world.read_resource(),
            );

            let animation = loader.load_from_data(
                Animation::new_single(0, TransformChannel::Translation, sampler),
                (),
                &world.read_resource(),
            );
            let mut animation_set: AnimationSet<AnimationId, Transform> = AnimationSet::new();
            animation_set.insert(AnimationId::Test, animation.clone());

            (animation_set, animation)
        };
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(amethyst::LoggerConfig {
        level_filter: LogLevelFilter::Off,
        ..Default::default()
    });

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display_qbit.ron");
    let asset_dir = app_root.join("assets/");

    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with_bundle(HotReloadBundle::default())?
        .with_bundle(AnimationBundle::<AnimationId, Transform>::new(
            "animation_control_system",
            "sampler_interpolation_system",
        ))?
        .with_bundle(TransformBundle::new().with_dep(&["sampler_interpolation_system"]))?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderPbr3D::default()),
        )?;

    let state: ExampleState = Default::default();
    let mut game = Application::new(asset_dir, state, game_data)?;
    game.run();

    Ok(())
}
