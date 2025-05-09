#![allow(clippy::type_complexity)]
#![doc = include_str!("../README.MD")]

pub mod anim;
mod error;
mod loader;

use anim::AsepriteAnimation;
use bevy::{
    app::{Plugin, Update},
    asset::{Asset, AssetApp, Handle},
    ecs::{
        bundle::Bundle, component::Component, schedule::{IntoSystemConfigs, SystemSet}
    },
    reflect::TypePath,
    sprite::TextureAtlas,
    transform::components::{GlobalTransform, Transform},
};

use bevy_aseprite_reader as reader;

pub use bevy::sprite::TextureAtlasBuilder;
pub use bevy_aseprite_derive::aseprite;
use reader::AsepriteInfo;

pub struct AsepritePlugin;

#[derive(Debug, SystemSet, Clone, Hash, PartialEq, Eq)]
enum AsepriteSystems {
    InsertSpriteSheet,
}

impl Plugin for AsepritePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<Aseprite>()
            .register_asset_loader(loader::AsepriteLoader)
            .add_systems(Update, loader::process_load)
            .add_systems(
                Update,
                anim::update_animations.after(AsepriteSystems::InsertSpriteSheet),
            );
    }
}

#[derive(Debug, Default, Clone, TypePath, Asset, Component)]
pub struct Aseprite {
    // Data is dropped after the atlas is built
    data: Option<reader::Aseprite>,
    // Info stores data such as tags and slices
    info: Option<AsepriteInfo>,
    // TextureAtlasBuilder might shift the index order when building so
    // we keep a mapping of frame# -> atlas index here
    frame_to_idx: Vec<usize>,
}

/// A bundle defining a drawn aseprite
#[derive(Debug, Bundle, Default)]
pub struct AsepriteBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub animation: AsepriteAnimation,
    pub aseprite: Aseprite,
}
