use super::DemoState;
use amethyst::{
    core::{
        math::{UnitQuaternion, Vector3},
        Time, Transform,
    },
    derive::SystemDesc,
    ecs::prelude::{
        Entity, Join, Read, ReadStorage, System, SystemData, WriteExpect, WriteStorage,
    },
    renrerer::{camera::Camera, light::Light},
    ui::{UiFinder, UiText},
    utils::fps_counter::FpsCounter,
};

pub struct CustomGameData {}
pub struct CustomGameDataBuilder {}
